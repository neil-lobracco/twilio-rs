mod call;
mod message;
pub mod twiml;
mod webhook;

pub use call::{Call, OutboundCall};
use headers::authorization::{Authorization, Basic};
use headers::{ContentType, HeaderMapExt};
use hyper::client::connect::HttpConnector;
use hyper::{Body, Method, StatusCode};
use hyper_tls::HttpsConnector;
use serde::Deserialize;
pub use message::{Message, OutboundMessage};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use crate::TwilioError::ServiceError;

pub const GET: Method = Method::GET;
pub const POST: Method = Method::POST;
pub const PUT: Method = Method::PUT;

pub struct Client {
    account_id: String,
    auth_token: String,
    auth_header: Authorization<Basic>,
    http_client: hyper::Client<HttpsConnector<HttpConnector>>,
}

fn url_encode(params: &[(&str, &str)]) -> String {
    params
        .iter()
        .map(|&t| {
            let (k, v) = t;
            format!("{}={}", k, v)
        })
        .fold("".to_string(), |mut acc, item| {
            acc.push_str(&item);
            acc.push_str("&");
            acc.replace("+", "%2B")
        })
}

#[derive(Debug, Deserialize)]
pub struct TwilioServiceError {
    pub code: u32,
    pub status: u32,
    pub message: String,
    pub more_info: String,
}

#[derive(Debug)]
pub enum TwilioError {
    NetworkError(hyper::Error),
    HTTPError(StatusCode),
    ParsingError,
    AuthError,
    BadRequest,
    ServiceError(TwilioServiceError)
}

impl Display for TwilioError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            TwilioError::NetworkError(ref e) => e.fmt(f),
            TwilioError::HTTPError(ref s) => write!(f, "Invalid HTTP status code: {}", s),
            TwilioError::ParsingError => f.write_str("Parsing error"),
            TwilioError::AuthError => f.write_str("Missing `X-Twilio-Signature` header in request"),
            TwilioError::BadRequest => f.write_str("Bad request"),
            TwilioError::ServiceError(ref se) => write!(f, "Twilio service error: [{}] {}", se.code, se.message),
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

impl Client {
    pub fn new(account_id: &str, auth_token: &str) -> Client {
        Client {
            account_id: account_id.to_string(),
            auth_token: auth_token.to_string(),
            auth_header: Authorization::basic(account_id, auth_token),
            http_client: hyper::Client::builder().build(HttpsConnector::new()),
        }
    }

    async fn send_request<T>(
        &self,
        method: hyper::Method,
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
        let mut req = hyper::Request::builder()
            .method(method)
            .uri(&*url)
            .body(Body::from(url_encode(params)))
            .unwrap();

        let mime: mime::Mime = "application/x-www-form-urlencoded".parse().unwrap();
        req.headers_mut().typed_insert(ContentType::from(mime));
        req.headers_mut().typed_insert(self.auth_header.clone());

        let resp = self
            .http_client
            .request(req)
            .await
            .map_err(TwilioError::NetworkError)?;

        match resp.status() {
            StatusCode::CREATED | StatusCode::OK => {}
            StatusCode::BAD_REQUEST => {
                let decoded: TwilioServiceError = hyper::body::to_bytes(resp.into_body())
                    .await
                    .map_err(TwilioError::NetworkError)
                    .and_then(|bytes| {
                        serde_json::from_slice(&bytes).map_err(|_| TwilioError::ParsingError)
                    })?;

                return Err(ServiceError(decoded));
            },
            other => return Err(TwilioError::HTTPError(other)),
        };

        let decoded: T = hyper::body::to_bytes(resp.into_body())
            .await
            .map_err(TwilioError::NetworkError)
            .and_then(|bytes| {
                serde_json::from_slice(&bytes).map_err(|_| TwilioError::ParsingError)
            })?;

        Ok(decoded)
    }

    pub async fn respond_to_webhook<T: FromMap, F>(
        &self,
        req: hyper::Request<Body>,
        mut logic: F,
    ) -> hyper::Response<Body>
    where
        F: FnMut(T) -> twiml::Twiml,
    {
        let o: T = match self.parse_request::<T>(req).await {
            Ok(obj) => *obj,
            Err(_) => {
                let mut res = hyper::Response::new(Body::from("Error.".as_bytes()));
                *res.status_mut() = StatusCode::BAD_REQUEST;
                return res;
            }
        };

        let t = logic(o);
        let body = t.as_twiml();
        let len = body.len() as u64;
        let mut res = hyper::Response::new(Body::from(body));
        res.headers_mut().typed_insert(headers::ContentType::xml());
        res.headers_mut().typed_insert(headers::ContentLength(len));
        res
    }
}
