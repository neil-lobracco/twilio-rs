use crate::{Client, FromMap, TwilioError, POST};
use serde_derive::Deserialize;
use std::collections::HashMap;

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
#[serde(rename_all = "lowercase")]
pub enum CallStatus {
    Queued,
    Ringing,
    InProgress,
    Canceled,
    Completed,
    Failed,
    Busy,
    NoAnswer,
}

#[derive(Debug, Deserialize)]
pub struct Call {
    from: String,
    to: String,
    sid: String,
    status: CallStatus,
}

impl Client {
    pub async fn make_call(&self, call: OutboundCall<'_>) -> Result<Call, TwilioError> {
        let opts = [
            ("To", &*call.to),
            ("From", &*call.from),
            ("Url", &*call.url),
        ];
        self.send_request(POST, "Calls", &opts).await
    }
}

impl FromMap for Call {
    fn from_map(m: &HashMap<&str, &str>) -> Result<Box<Call>, TwilioError> {
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
            Some(&"queued") => CallStatus::Queued,
            Some(&"ringing") => CallStatus::Ringing,
            Some(&"in-progress") => CallStatus::InProgress,
            Some(&"canceled") => CallStatus::Canceled,
            Some(&"completed") => CallStatus::Completed,
            Some(&"failed") => CallStatus::Failed,
            Some(&"busy") => CallStatus::Busy,
            Some(&"no-answer") => CallStatus::NoAnswer,
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
