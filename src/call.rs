use crate::{Client, FromMap, Post, TwilioError};
use serde_derive::Deserialize;

pub struct OutboundCall<'a> {
    pub from: &'a str,
    pub to: &'a str,
    pub url: &'a str,
}

impl<'a> OutboundCall<'a> {
    pub fn new(from: &'a str, to: &'a str, url: &'a str) -> OutboundCall<'a> {
        OutboundCall { from, to, url }
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum CallStatus {
    queued,
    ringing,
    inprogress,
    canceled,
    completed,
    failed,
    busy,
    noanswer,
}

#[derive(Debug, Deserialize)]
pub struct Call {
    from: String,
    to: String,
    sid: String,
    status: CallStatus,
}

impl Client {
    pub fn make_call(&self, call: OutboundCall) -> Result<Call, TwilioError> {
        let opts = [
            ("To", &*call.to),
            ("From", &*call.from),
            ("Url", &*call.url),
        ];
        self.send_request(Post, "Calls", &opts)
    }
}

impl FromMap for Call {
    fn from_map(m: &::std::collections::HashMap<&str, &str>) -> Result<Box<Call>, TwilioError> {
        let from = match m.get("From") {
            Some(&v) => v,
            None => return Err(TwilioError::ParsingError),
        };
        let to = match m.get("To") {
            Some(&v) => v,
            None => return Err(TwilioError::ParsingError),
        };
        let sid = match m.get("CallSid") {
            Some(&v) => v,
            None => return Err(TwilioError::ParsingError),
        };
        let stat = match m.get("CallStatus") {
            Some(&"queued") => CallStatus::queued,
            Some(&"ringing") => CallStatus::ringing,
            Some(&"in-progress") => CallStatus::inprogress,
            Some(&"canceled") => CallStatus::canceled,
            Some(&"completed") => CallStatus::completed,
            Some(&"failed") => CallStatus::failed,
            Some(&"busy") => CallStatus::busy,
            Some(&"no-answer") => CallStatus::noanswer,
            _ => return Err(TwilioError::ParsingError),
        };
        Ok(Box::new(Call {
            from: from.to_string(),
            to: to.to_string(),
            sid: sid.to_string(),
            status: stat,
        }))
    }
}
