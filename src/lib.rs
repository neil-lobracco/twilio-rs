extern crate hyper;
extern crate rustc_serialize;
extern crate mime;
mod message;
mod call;
mod webhook;
pub mod twiml;
use std::io::Read;
use self::hyper::header::{Authorization,Basic};
use self::hyper::status::StatusCode;
pub use self::hyper::method::Method::{Post,Get,Put};
pub use message::{Message,OutboundMessage};
pub use call::{Call,OutboundCall};
use std::collections::HashMap;
use std::io::Write;

pub struct Client {
    account_id : String,
    auth_token : String,
    auth_header : Authorization<Basic>,
}
fn url_encode(params: &[(&str,&str)]) -> String {
    params.iter().map(|&t| {
        let (k,v) = t;
        format!("{}={}",k,v)
    }).fold("".to_string(), |mut acc, item| {
        acc.push_str(&item);
        acc.push_str("&");
        acc
    })
}

fn basic_auth_header(username: String, password: String) -> Authorization<Basic> {
    Authorization(Basic { username : username, password : Some(password) })
}

#[derive(Debug)]
pub enum TwilioError {
    NetworkError,
    HTTPError,
    ParsingError,
    AuthError,
    BadRequest,
}

pub trait FromMap {
    fn from_map(&HashMap<&str,&str>) -> Result<Self,TwilioError>;
}

impl Client {
    pub fn new(account_id: &str, auth_token: &str) -> Client {
        Client {
            account_id : account_id.to_string(),
            auth_token : auth_token.to_string(),
            auth_header : basic_auth_header(account_id.to_string(),auth_token.to_string()),
        }
    }
    fn send_request<T : rustc_serialize::Decodable>(&self, method: hyper::method::Method, endpoint: &str, params: &[(&str,&str)]) -> Result<T,TwilioError> {
        let url = format!("https://api.twilio.com/2010-04-01/Accounts/{}/{}.json",self.account_id,endpoint);
        let mut http_client = hyper::Client::new();
        let post_body: &str = &*url_encode(params);
        let mime: mime::Mime = "application/x-www-form-urlencoded".parse().unwrap();
        let content_type_header = hyper::header::ContentType(mime);
        let req = http_client
            .request(method,&*url)
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
            _ => {
                return Err(TwilioError::HTTPError)
            }
        };
        let decoded: T = match rustc_serialize::json::decode(&body_str) {
            Ok(obj) => obj,
            Err(_)  => return Err(TwilioError::ParsingError),
        };
        Ok(decoded)
    }

    pub fn respond_to_webhook<T:FromMap,F>(&self, req: &mut hyper::server::Request, mut res: hyper::server::Response, mut logic:  F)
    where F: FnMut(T) -> twiml::Twiml {
        let o: T = match self.parse_request::<T>(req){
            Err(_) => {
                *res.status_mut() = hyper::BadRequest;
                let mut res = res.start().unwrap();
                res.write_all("Error.".as_bytes()).unwrap();
                res.end().unwrap();
                return
            },
                Ok(obj) => obj,
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
