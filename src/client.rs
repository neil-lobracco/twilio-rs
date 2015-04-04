extern crate hyper;
extern crate rustc_serialize;
extern crate mime;
use std::io::Read;

use message;
pub struct Client {
    account_id : String,
    auth_token : String,
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

fn basic_auth_header(username: String, password: String) -> hyper::header::Authorization<hyper::header::Basic> {
    hyper::header::Authorization(hyper::header::Basic { username : username, password : Some(password) })
}

impl Client {
    pub fn new(account_id: String, auth_token: String) -> Client {
        Client { account_id : account_id, auth_token : auth_token }
    }
    pub fn send_message(&self, msg: message::OutboundMessage) -> Result<message::Message,hyper::status::StatusCode> {
        let url = format!("https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",self.account_id);
        let opts = [("To",&*msg.to),("From",&*msg.from),("Body",&*msg.body)];
        let mut http_client = hyper::Client::new();
        let post_body: &str = &*url_encode(&opts);
        println!("POST body: '{}'",post_body);
        let mime: mime::Mime = "application/x-www-form-urlencoded".parse().unwrap();
        let content_type_header = hyper::header::ContentType(mime);
        let req = http_client.post(&*url).body(post_body).header(basic_auth_header(self.account_id.clone(),self.auth_token.clone())).header(content_type_header);
        let mut resp = req.send().unwrap();
        let mut body_str = "".to_string();
        resp.read_to_string(&mut body_str).unwrap();
        match resp.status {
            hyper::status::StatusCode::Created => (),
            _ => {
                println!("{:?}",body_str);
                return Err(resp.status)
            }
        }
        let decoded: message::Message = rustc_serialize::json::decode(&body_str).unwrap();
        Ok(decoded)
    }
}
