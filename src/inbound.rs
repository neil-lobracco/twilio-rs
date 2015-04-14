extern crate sha1;
use self::sha1::Sha1;
use hyper::server::request::Request;
use hyper::header::Host;
use hyper::uri::RequestUri::AbsolutePath;
use hyper::method::Method::{Get,Post};
use rustc_serialize::base64::{ToBase64,STANDARD};
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
    enc.split('&').filter_map( |param| {
        let split: Vec<&str> = param.split('=').collect();
        match split.len() {
            1 => Some((split[0].to_string(),"".to_string())),
            2 => Some((split[0].to_string(),split[1].to_string())),
            _ => None,
        }
    }).collect()
}

pub fn parse_request<T : ::FromMap>(req: &mut Request) -> Result<T,::TwilioError> {
    let sig = match req.headers.get_raw("X-Twilio-Signature") {
        None => return Err(::TwilioError::AuthError),
        Some(d) => match d.len() {
            1 => match String::from_utf8(d[0].clone()) {
                Ok(s) => s,
                Err(_) => return Err(::TwilioError::BadRequest),
            },
            _ => return Err(::TwilioError::BadRequest),
        }
    };
    let mut bod = "".to_string();
    req.read_to_string(&mut bod).unwrap();
    let host: &str = match req.headers.get::<Host>() {
        None => return Err(::TwilioError::BadRequest),
        Some(h) => &h.hostname,
    };
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
    let effective_uri = format!("https://{}{}{}",host,request_path,post_append);
    let mut hasher = Sha1::new();
    hasher.update(effective_uri.as_bytes());
    let sha = hasher.digest();
    let b64 = sha.to_base64(STANDARD);
    if b64 != sig {
        return Err(::TwilioError::AuthError);
    }
    parse_object::<T>(&args)
}
