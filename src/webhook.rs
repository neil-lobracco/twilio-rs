use crate::{Client, FromMap, TwilioError};
use crypto::hmac::Hmac;
use crypto::mac::{Mac, MacResult};
use crypto::sha1::Sha1;
use headers::{HeaderMapExt, Host};
use hyper::{Body, Method, Request};
use std::collections::HashMap;

fn parse_object<T: FromMap>(args: &[(String, String)]) -> Result<Box<T>, TwilioError> {
    let mut m = HashMap::new();
    for t in args {
        m.insert(t.0.as_ref(), t.1.as_ref());
    }
    T::from_map(&m)
}

fn get_args(path: &str) -> Vec<(String, String)> {
    let url_segments: Vec<&str> = path.split('?').collect();
    if url_segments.len() != 2 {
        return vec![];
    }
    let query_string = url_segments[1];
    args_from_urlencoded(query_string.as_bytes())
}

fn args_from_urlencoded(enc: &[u8]) -> Vec<(String, String)> {
    url::form_urlencoded::parse(enc)
        .into_owned()
        .collect::<Vec<(String, String)>>()
}

impl Client {
    pub async fn parse_request<T: FromMap>(
        &self,
        req: Request<Body>,
    ) -> Result<Box<T>, TwilioError> {
        let sig = req
            .headers()
            .get("X-Twilio-Signature")
            .ok_or_else(|| TwilioError::AuthError)
            .and_then(|d| match d.len() {
                1 => base64::decode(d.as_bytes()).map_err(|_| TwilioError::BadRequest),
                _ => Err(TwilioError::BadRequest),
            })?;

        let (parts, body) = req.into_parts();
        let body = hyper::body::to_bytes(body)
            .await
            .map_err(|_| TwilioError::NetworkError)?;
        let host = match parts.headers.typed_get::<Host>() {
            None => return Err(TwilioError::BadRequest),
            Some(h) => h.hostname().to_string(),
        };
        let request_path = match parts.uri.path() {
            "*" => return Err(TwilioError::BadRequest),
            path => path,
        };
        let (args, post_append) = match parts.method {
            Method::GET => (get_args(request_path), "".to_string()),
            Method::POST => {
                let mut postargs = args_from_urlencoded(&body);
                postargs.sort_by(|(k1, _), (k2, _)| k1.cmp(&k2));
                let append = postargs
                    .iter()
                    .map(|(k, v)| format!("{}{}", k, v))
                    .collect();
                (postargs, append)
            }
            _ => return Err(TwilioError::BadRequest),
        };

        let effective_uri = format!("https://{}{}{}", host, request_path, post_append);
        let mut hmac = Hmac::new(Sha1::new(), self.auth_token.as_bytes());
        hmac.input(effective_uri.as_bytes());
        let result = hmac.result();
        let expected = MacResult::new(&sig);
        if result != expected {
            return Err(TwilioError::AuthError);
        }

        parse_object::<T>(&args)
    }
}
