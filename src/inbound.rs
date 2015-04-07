extern crate sha1;
use hyper::server::request::Request;
use hyper::header::Host;
use hyper::uri::RequestUri::AbsolutePath;
use hyper::method::Method::Post;
use rustc_serialize::Decodable;

fn post_args(req: &Request) -> Vec<(String,String)> {
    vec![]
}

pub fn parse_request<T : Decodable>(req: Request) -> Result<T,::TwilioError> {
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
    let host: &str = match req.headers.get::<Host>() {
        None => return Err(::TwilioError::BadRequest),
        Some(h) => &h.hostname,
    };
    let request_path: &str = match req.uri {
        AbsolutePath(ref s) => s,
        _ => return Err(::TwilioError::BadRequest),
    };

    let post_append = match req.method {
        Post => {
            let mut postargs = post_args(&req);
            postargs.sort_by(|p1,p2| {
                let k1 = &p1.0;
                let k2 = &p2.0;
                k1.cmp(&k2)
            });
            postargs.iter()
                .map(|t| {
                    let (k,v) = (&t.0,&t.1);
                    format!("{}{}",k,v)
                })
                .fold("".to_string(), |mut acc, item| {
                    acc.push_str(&item);
                    acc
                })
        },
        _ => "".to_string(),
    };
    let effective_uri = format!("https://{}{}{}",host,request_path,post_append);
    Err(::TwilioError::NetworkError)
}
