extern crate crypto;
extern crate url;
use self::crypto::hmac::Hmac;
use self::crypto::sha1::Sha1;
use self::crypto::mac::{MacResult,Mac};
use hyper::server::request::Request;
use hyper::header::Host;
use hyper::uri::RequestUri::AbsolutePath;
use hyper::method::Method::{Get,Post};
use rustc_serialize::base64::FromBase64;
use std::io::Read;
use std::collections::HashMap;

fn parse_object<T : ::FromMap>(args: &[(String,String)]) -> Result<T,::TwilioError> {
    let mut m = HashMap::new();
    for t in args {
        m.insert(t.0.as_ref(),t.1.as_ref());
    }
    T::from_map(&m)
}

fn get_args(path: &str) -> Vec<(String,String)> {
    let url_segments: Vec<&str> = path.split('?').collect();
    if url_segments.len() != 2 {
        return vec![]
    }
    let query_string = url_segments[1];
    args_from_urlencoded(query_string)
}

fn args_from_urlencoded(enc: &str) -> Vec<(String,String)> {
    url::form_urlencoded::parse(enc.as_bytes())
}

impl ::Client {
    pub fn parse_request<T : ::FromMap>(&self, req: &mut Request) -> Result<T,::TwilioError> {
        let mut bod = "".to_string();
        req.read_to_string(&mut bod).unwrap();
        let request_path: &str = match req.uri {
            AbsolutePath(ref s) => s,
            _ => return Err(::TwilioError::BadRequest),
        };
        let (args,post_append) = match req.method {
            Get => (get_args(request_path),"".to_string()),
            Post => {
                let mut postargs = args_from_urlencoded(&bod);
                postargs.sort_by(|p1,p2| {
                    let k1 = &p1.0;
                    let k2 = &p2.0;
                    k1.cmp(&k2)
                });
                let append = postargs.iter()
                    .map(|t| {
                        let (k,v) = (&t.0,&t.1);
                        format!("{}{}",k,v)
                    })
                    .fold("".to_string(), |mut acc, item| {
                        acc.push_str(&item);
                        acc
                    });
                (postargs,append)
            },
            _ => return Err(::TwilioError::BadRequest),
        };
        // Twilio can provide encrypted communication with an application server, but only for
        // a server using HTTPS with a certificate signed by a Certificate Authority.
        // For details, see: https://www.twilio.com/docs/api/security
        // The following code performs the authentication for this library. To bypass this check,
        // call disable_authentication() on the Client. 
        if self.authenticate {
            let sig = match req.headers.get_raw("X-Twilio-Signature") {
                None => return Err(::TwilioError::AuthError),
                Some(d) => match d.len() {
                    1 => match d[0].from_base64() {
                        Ok(v) => v,
                        Err(_) => return Err(::TwilioError::BadRequest),
                    },
                    _ => return Err(::TwilioError::BadRequest),
                }
            };
            let host: &str = match req.headers.get::<Host>() {
                None => return Err(::TwilioError::BadRequest),
                Some(h) => &h.hostname,
            };
            let effective_uri = format!("https://{}{}{}",host,request_path,post_append);
            let mut hmac = Hmac::new(Sha1::new(),self.auth_token.as_bytes());
            hmac.input(effective_uri.as_bytes());
            let result = hmac.result();
            let expected = MacResult::new(&sig);
            if result != expected {
                return Err(::TwilioError::AuthError);
            }
        }
        parse_object::<T>(&args)
    }
}
