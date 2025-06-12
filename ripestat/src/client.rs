pub use reqwest::{
    header::{self, HeaderValue},
    Client as ReqwestClient,
    Error as ReqwestError,
    StatusCode,
};
use {
    chrono::{DateTime, Utc},
    serde::{Deserialize, Serialize},
};
use tracing::{debug, info,};

use crate::RipestatClientError;
use ripestat_common::VERSION;

/// Used by the request functions.
#[derive(Clone, Copy)]
pub struct RequestOptions {
    pub(crate) max_retry_secs: u32,
    pub(crate) def_retry_secs: u32,
    pub(crate) max_retries: u16,
}

impl Default for RequestOptions {
    fn default() -> Self {
        Self {
            max_retry_secs: 120,
            def_retry_secs: 60,
            max_retries: 1,
        }
    }
}

/// Represents the data from HTTP responses.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct HttpData {
    pub content_length: Option<u64>,
    pub content_type: Option<String>,
    pub scheme: Option<String>,
    pub host: String,
    pub expires: Option<String>,
    pub cache_control: Option<String>,
    pub received: DateTime<Utc>,
    pub status_code: u16,
    pub location: Option<String>,
    pub access_control_allow_origin: Option<String>,
    pub strict_transport_security: Option<String>,
    pub retry_after: Option<String>,
    pub request_uri: Option<String>,
}

/// Configures the HTTP client.
pub struct ReqwestClientConfig {
    /// This string is appended to the user agent.
    ///
    /// It is provided so
    /// library users may identify their programs.
    pub user_agent_suffix: String,

    /// If set to true, connections will be required to use HTTPS.
    pub https_only: bool,

    /// If set to true, invalid host names will be accepted.
    pub accept_invalid_host_names: bool,

    /// If set to true, invalid certificates will be accepted.
    pub accept_invalid_certificates: bool,

    /// If true, HTTP redirects will be followed.
    pub follow_redirects: bool,

    /// Specify Host
    pub host: Option<HeaderValue>,

    /// Specify the value of the origin header.
    ///
    /// Most browsers ignore this by default.
    pub origin: Option<HeaderValue>,

    /// Query timeout in seconds.
    ///
    /// This corresponds to the total timeout of the request (connection plus reading all the data).
    pub timeout_secs: u64,
}

impl Default for ReqwestClientConfig {
    fn default() -> Self {
        Self {
            user_agent_suffix: "library".to_string(),
            https_only: true,
            accept_invalid_host_names: false,
            accept_invalid_certificates: false,
            follow_redirects: true,
            host: None,
            origin: None,
            timeout_secs: 60,
        }
    }
}

/// Creates an HTTP client using Reqwest. The Reqwest
/// client holds its own connection pools, so in many
/// uses cases creating only one client per process is
/// necessary.
pub fn create_reqwest_client(config: &ReqwestClientConfig) -> Result<ReqwestClient, ReqwestError> {
    let default_headers = default_headers(config);

    let mut client = reqwest::Client::builder();

    let redirects = if config.follow_redirects {
        reqwest::redirect::Policy::default()
    } else {
        reqwest::redirect::Policy::none()
    };
    client = client
        .timeout(std::time::Duration::from_secs(config.timeout_secs))
        .user_agent(format!(
            "library {VERSION} {}",
            config.user_agent_suffix
        ))
        .redirect(redirects)
        .https_only(config.https_only)
        .danger_accept_invalid_hostnames(config.accept_invalid_host_names)
        .danger_accept_invalid_certs(config.accept_invalid_certificates);

    let client = client.default_headers(default_headers).build()?;
    Ok(client)
}

fn default_headers(config: &ReqwestClientConfig) -> header::HeaderMap {
    let mut default_headers = header::HeaderMap::new();
    default_headers.insert(
        header::ACCEPT,
        HeaderValue::from_static("application/json"),
    );
    if let Some(host) = &config.host {
        default_headers.insert(header::HOST, host.into());
    };
    if let Some(origin) = &config.origin {
        default_headers.insert(header::ORIGIN, origin.into());
    }
    default_headers
}

/// Configures the HTTP client.
#[derive(Default)]
pub struct ClientConfig {
    /// Config for the Reqwest client.
    client_config: ReqwestClientConfig,

    /// Request options.
    request_options: RequestOptions,
}

/// A wrapper around Reqwest client to give additional features when used with the request functions.
pub struct Client {
    /// The reqwest client.
    pub(crate) reqwest_client: ReqwestClient,

    /// Request options.
    pub(crate) request_options: RequestOptions,
}

impl Client {
    pub fn new(reqwest_client: ReqwestClient, request_options: RequestOptions) -> Self {
        Self {
            reqwest_client,
            request_options,
        }
    }
}

/// Creates a wrapped HTTP client. The wrapped
/// client holds its own connection pools, so in many
/// uses cases creating only one client per process is
/// necessary.
pub fn create_client(config: &ClientConfig) -> Result<Client, RipestatClientError> {
    let client = create_reqwest_client(&config.client_config)?;
    Ok(Client::new(client, config.request_options))
}

pub(crate) struct WrappedResponse {
    #[allow(dead_code)]
    pub(crate) http_data: HttpData,
    pub(crate) text: String,
}

pub(crate) async fn wrapped_request(
    request_uri: reqwest::Url,
    client: &Client,
) -> Result<WrappedResponse, ReqwestError> {
    // send request and loop for possible retries
    #[allow(unused_mut)] //because of wasm32 exclusion below
    let mut response = client.reqwest_client.get(request_uri.clone()).send().await?;

    // this doesn't work on wasm32 because tokio doesn't work on wasm
    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut tries: u16 = 0;
        loop {
            debug!("HTTP version: {:?}", response.version());
            // don't repeat the request
            if !matches!(response.status(), StatusCode::TOO_MANY_REQUESTS) {
                break;
            }
            // loop if HTTP 429
            let retry_after_header = response
                .headers()
                .get(header::RETRY_AFTER)
                .map(|value| value.to_str().unwrap().to_string());
            let retry_after = if let Some(rt) = retry_after_header {
                info!("Server says too many requests and to retry-after '{rt}'.");
                rt
            } else {
                info!("Server says too many requests but does not offer 'retry-after' value.");
                client.request_options.def_retry_secs.to_string()
            };
            let mut wait_time_seconds = if let Ok(date) = DateTime::parse_from_rfc2822(&retry_after)
            {
                (date.with_timezone(&Utc) - Utc::now()).num_seconds() as u64
            } else if let Ok(seconds) = retry_after.parse::<u64>() {
                seconds
            } else {
                info!(
                    "Unable to parse retry-after header value. Using {}",
                    client.request_options.def_retry_secs
                );
                client.request_options.def_retry_secs.into()
            };
            if wait_time_seconds == 0 {
                info!("Given {wait_time_seconds} for retry-after. Does not make sense.");
                wait_time_seconds = client.request_options.def_retry_secs as u64;
            }
            if wait_time_seconds > client.request_options.max_retry_secs as u64 {
                info!(
                    "Server is asking to wait longer than configured max of {}.",
                    client.request_options.max_retry_secs
                );
                wait_time_seconds = client.request_options.max_retry_secs as u64;
            }
            info!("Waiting {wait_time_seconds} seconds to retry.");
            tokio::time::sleep(tokio::time::Duration::from_secs(wait_time_seconds + 1)).await;
            tries += 1;
            if tries > client.request_options.max_retries {
                info!("Max query retries reached.");
                break;
            } else {
                // send the query again
                response = client.reqwest_client.get(request_uri.clone()).send().await?;
            }
        }
    }

    // throw an error if not 200 OK
    let response = response.error_for_status()?;

    // get the response
    let content_type = response
        .headers()
        .get(header::CONTENT_TYPE)
        .map(|value| value.to_str().unwrap().to_string());
    let expires = response
        .headers()
        .get(header::EXPIRES)
        .map(|value| value.to_str().unwrap().to_string());
    let cache_control = response
        .headers()
        .get(header::CACHE_CONTROL)
        .map(|value| value.to_str().unwrap().to_string());
    let location = response
        .headers()
        .get(header::LOCATION)
        .map(|value| value.to_str().unwrap().to_string());
    let access_control_allow_origin = response
        .headers()
        .get(header::ACCESS_CONTROL_ALLOW_ORIGIN)
        .map(|value| value.to_str().unwrap().to_string());
    let strict_transport_security = response
        .headers()
        .get(header::STRICT_TRANSPORT_SECURITY)
        .map(|value| value.to_str().unwrap().to_string());
    let retry_after = response
        .headers()
        .get(header::RETRY_AFTER)
        .map(|value| value.to_str().unwrap().to_string());
    let content_length = response.content_length();
    let status_code = response.status().as_u16();
    let url = response.url().to_owned();
    let text = response.text().await?;

    let http_data = HttpData {
        status_code: status_code,
        content_length: content_length,
        content_type: content_type,
        scheme: Some(url.scheme().to_owned()),
        host: url.host_str().expect("URL has no host. This shouldn't happen.").to_owned(),
        expires: expires,
        cache_control: cache_control,
        received: Utc::now(),
        location: location,
        access_control_allow_origin: access_control_allow_origin,
        strict_transport_security: strict_transport_security,
        retry_after: retry_after,
        request_uri: Some(String::from(request_uri)),
    };

    Ok(WrappedResponse { http_data, text })
}