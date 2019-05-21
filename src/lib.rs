extern crate mime;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate url;
extern crate reqwest;

mod call;
mod message;
pub mod twiml;

pub use call::{Call, OutboundCall};
pub use message::{Message, OutboundMessage};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io::Read;
use std::io::Write;
use url::form_urlencoded;
use reqwest::header::CONTENT_TYPE;
use reqwest::header::HeaderValue;
use reqwest::StatusCode;

pub struct Client {
    account_id: String,
    auth_token: String,
}

fn url_encode(params: &[(&str, &str)]) -> String {
    let mut builder = form_urlencoded::Serializer::new(String::new());

    for (ref key, ref value) in params {
        builder.append_pair(key, value);
    }
    builder.finish()
}


#[derive(Debug, Deserialize, Serialize)]
pub enum TwilioError {
    NetworkError,
    HTTPError,
    ParsingError,
    AuthError,
    BadRequest,
    ValidationError {
        code: u64,
        message: String,
        more_info: String,
        status: u64,
    },
}

impl fmt::Display for TwilioError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())?;

        Ok(())
    }
}

impl Error for TwilioError {
    fn description(&self) -> &str {
        use TwilioError::*;
        match self {
            NetworkError => "NetworkError",
            HTTPError => "HTTPError",
            ParsingError => "ParsingError",
            AuthError => "AuthError",
            BadRequest => "BadRequest",
            ValidationError {
                code,
                message,
                more_info,
                status,
            } => "Validation Error",
        }
    }
}

impl From<serde_json::Error> for TwilioError {
    fn from(e: serde_json::Error) -> Self {
        TwilioError::ParsingError
    }
}

pub trait FromMap {
    fn from_map(_: &HashMap<&str, &str>) -> Result<Box<Self>, TwilioError>;
}

impl Client {
    pub fn new(account_id: &str, auth_token: &str) -> Client {
        Client {
            account_id: account_id.to_string(),
            auth_token: auth_token.to_string(),
        }
    }

    fn send_request<T>(
        &self,
        method: reqwest::Method,
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

        let client = reqwest::Client::new();

        let post_body = url_encode(params);
        let req = client
            .request(method, &url)
            .header(
                CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            )
            .basic_auth(self.account_id.clone(), Some(self.auth_token.clone()))
            .body(post_body);

        let mut resp = match req.send() {
            Ok(res) => res,
            Err(_) => return Err(TwilioError::NetworkError),
        };
        let mut body_str = "".to_string();
        match resp.read_to_string(&mut body_str) {
            Ok(_) => (),
            Err(_) => return Err(TwilioError::NetworkError),
        };
        match resp.status() {
            StatusCode::CREATED | StatusCode::OK => (),
            StatusCode::BAD_REQUEST => {
                let val: serde_json::Value = serde_json::from_str(&body_str)?;
                return Err(TwilioError::ValidationError {
                    code: val["code"].as_u64().unwrap_or_default(),
                    message: val["message"].as_str().unwrap_or("").to_string(),
                    more_info: val["more_info"].as_str().unwrap_or("").to_string(),
                    status: val["status"].as_u64().unwrap_or_default(),
                });
            }
            _ => {
                return Err(TwilioError::HTTPError);
            }
        };
        let decoded: T = match serde_json::from_str(&body_str) {
            Ok(obj) => obj,
            Err(_) => return Err(TwilioError::ParsingError),
        };
        Ok(decoded)
    }

}
