mod message;

pub use message::{Message, OutboundMessage};
use reqwest::{Client, Method, StatusCode};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

pub const GET: Method = Method::GET;
pub const POST: Method = Method::POST;
pub const PUT: Method = Method::PUT;

pub struct TwilioClient {
    account_id: String,
    auth_token: String,
    client: Client,
}

#[derive(Debug)]
pub enum TwilioError {
    NetworkError(reqwest::Error),
    HTTPError(StatusCode),
    ParsingError,
    AuthError,
    BadRequest,
}

impl Display for TwilioError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            TwilioError::NetworkError(ref e) => e.fmt(f),
            TwilioError::HTTPError(ref s) => write!(f, "Invalid HTTP status code: {}", s),
            TwilioError::ParsingError => f.write_str("Parsing error"),
            TwilioError::AuthError => f.write_str("Missing `X-Twilio-Signature` header in request"),
            TwilioError::BadRequest => f.write_str("Bad request"),
        }
    }
}

impl Error for TwilioError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            TwilioError::NetworkError(ref e) => Some(e),
            _ => None,
        }
    }
}

pub trait FromMap {
    fn from_map(m: BTreeMap<String, String>) -> Result<Box<Self>, TwilioError>;
}

impl TwilioClient {
    pub fn new(account_id: &str, auth_token: &str) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap();
        Self {
            account_id: account_id.to_string(),
            auth_token: auth_token.to_string(),
            client,
        }
    }

    async fn send_request<T>(
        &self,
        method: Method,
        endpoint: &str,
        params: &[(&str, &str)],
    ) -> Result<T, TwilioError>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/{}.json",
            self.account_id, endpoint
        );

        let response = self
            .client
            .request(method, url)
            .form(&params)
            .basic_auth(self.account_id.clone(), Some(self.auth_token.clone()))
            .send()
            .await
            .map_err(TwilioError::NetworkError)?;

        // println!("Headers: {:?}", response.headers());
        // println!("URL: {:?}", response.url());
        // println!("Error: {:?}", response.text().await.expect("no text"));
        // // println!("Headers: {:?}", response.headers());

        // return Err(TwilioError::AuthError)
        match response.status() {
            StatusCode::CREATED | StatusCode::OK => {}
            other => return Err(TwilioError::HTTPError(other)),
        };

        let decoded: T = response
            .bytes()
            .await
            .map_err(TwilioError::NetworkError)
            .and_then(|bytes| {
                serde_json::from_slice(&bytes).map_err(|_| TwilioError::ParsingError)
            })?;

        Ok(decoded)
    }
}
