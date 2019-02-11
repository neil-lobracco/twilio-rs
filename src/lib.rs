extern crate hyper;
extern crate hyper_native_tls;
extern crate mime;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate url;

mod message;
mod call;
mod webhook;
pub mod twiml;

use std::io::Read;
use self::hyper::header::{Authorization, Basic};
use hyper::net::HttpsConnector;
use self::hyper::status::StatusCode;
pub use self::hyper::method::Method::{Post, Get, Put};
use hyper_native_tls::NativeTlsClient;
pub use message::{Message, OutboundMessage};
pub use call::{Call, OutboundCall};
use std::collections::HashMap;
use std::io::Write;
use std::error::Error;
use url::form_urlencoded;
use std::fmt;

pub struct Client {
    account_id: String,
    auth_token: String,
    auth_header: Authorization<Basic>,
}

fn url_encode(params: &[(&str, &str)]) -> String {
    let mut builder = form_urlencoded::Serializer::new(String::new());

    for (ref key, ref value) in params {
        builder.append_pair(key, value);
    }
    builder.finish()

}

fn basic_auth_header(username: String, password: String) -> Authorization<Basic> {
    Authorization(Basic { username: username, password: Some(password) })
}

#[derive(Debug, Deserialize, Serialize)]
pub enum TwilioError {
    NetworkError,
    HTTPError,
    ParsingError,
    AuthError,
    BadRequest,
    ValidationError { code: u64, message: String, more_info: String, status: u64 },
}

impl fmt::Display for TwilioError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}", self.description()
        )?;

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
            ValidationError{code, message, more_info, status} => "Validation Error"
        }
    }
}

impl From<serde_json::Error> for TwilioError{
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
            auth_header: basic_auth_header(account_id.to_string(), auth_token.to_string()),
        }
    }

    fn send_request<T>(&self, method: hyper::method::Method, endpoint: &str, params: &[(&str, &str)]) -> Result<T, TwilioError>
        where T: serde::de::DeserializeOwned,
    {
        let url = format!("https://api.twilio.com/2010-04-01/Accounts/{}/{}.json", self.account_id, endpoint);
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let mut http_client = hyper::Client::with_connector(connector);
        let post_body: &str = &*url_encode(params);
        let mime: mime::Mime = "application/x-www-form-urlencoded".parse().unwrap();
        let content_type_header = hyper::header::ContentType(mime);
        let req = http_client
            .request(method, &*url)
            .body(post_body)
            .header(self.auth_header.clone())
            .header(content_type_header);
        let mut resp = match req.send() {
            Ok(res) => res,
            Err(_) => return Err(TwilioError::NetworkError),
        };
        let mut body_str = "".to_string();
        match resp.read_to_string(&mut body_str) {
            Ok(_) => (),
            Err(_) => return Err(TwilioError::NetworkError),
        };
        match resp.status {
            StatusCode::Created | StatusCode::Ok => (),
            StatusCode::BadRequest => {
                let val: serde_json::Value = serde_json::from_str(&body_str)?;
                return Err(TwilioError::ValidationError { code: val["code"].as_u64().unwrap_or_default(), message: val["message"].as_str().unwrap_or("").to_string(), more_info: val["more_info"].as_str().unwrap_or("").to_string(), status: val["status"].as_u64().unwrap_or_default() });
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

    pub fn respond_to_webhook<T: FromMap, F>(&self, req: &mut hyper::server::Request, mut res: hyper::server::Response, mut logic: F)
        where F: FnMut(T) -> twiml::Twiml {
        let o: T = match self.parse_request::<T>(req) {
            Err(_) => {
                *res.status_mut() = hyper::BadRequest;
                let mut res = res.start().unwrap();
                res.write_all("Error.".as_bytes()).unwrap();
                res.end().unwrap();
                return;
            }
            Ok(obj) => *obj,
        };
        let t = logic(o);
        let body = t.as_twiml();
        let mime: mime::Mime = "text/xml".parse().unwrap();
        let content_type_header = hyper::header::ContentType(mime);
        res.headers_mut().set(content_type_header);
        res.headers_mut().set(hyper::header::ContentLength(body.len() as u64));
        let mut res = res.start().unwrap();
        res.write_all(body.as_bytes()).unwrap();
        res.end().unwrap();
    }
}
