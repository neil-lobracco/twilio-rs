use crate::{Client, FromMap, TwilioError, POST};
use serde::Deserialize;
use std::collections::BTreeMap;

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
    pub from: String,
    pub to: String,
    pub sid: String,
    pub status: CallStatus,
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
    fn from_map(mut m: BTreeMap<String, String>) -> Result<Box<Call>, TwilioError> {
        let from = match m.remove("From") {
            Some(v) => v,
            None => return Err(TwilioError::ParsingError),
        };
        let to = match m.remove("To") {
            Some(v) => v,
            None => return Err(TwilioError::ParsingError),
        };
        let sid = match m.remove("CallSid") {
            Some(v) => v,
            None => return Err(TwilioError::ParsingError),
        };
        let stat = match m.get("CallStatus").map(|s| s.as_str()) {
            Some("queued") => CallStatus::Queued,
            Some("ringing") => CallStatus::Ringing,
            Some("in-progress") => CallStatus::InProgress,
            Some("canceled") => CallStatus::Canceled,
            Some("completed") => CallStatus::Completed,
            Some("failed") => CallStatus::Failed,
            Some("busy") => CallStatus::Busy,
            Some("no-answer") => CallStatus::NoAnswer,
            _ => return Err(TwilioError::ParsingError),
        };
        Ok(Box::new(Call {
            from,
            to,
            sid,
            status: stat,
        }))
    }
}
