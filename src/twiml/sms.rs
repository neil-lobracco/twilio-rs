use super::{format_xml_string, Action, Method};
use std::default::Default;

pub struct Sms {
    pub txt: String,
    pub action: Option<String>,
    pub method: Method,
    pub from: Option<String>,
    pub to: Option<String>,
    pub status_callback: Option<String>,
}

impl Action for Sms {
    fn as_twiml(&self) -> String {
        let mut attrs = Vec::new();
        let method_str = match self.method {
            Method::Get => "GET",
            Method::Post => "POST",
        };
        attrs.push(("method", method_str));
        if let Some(ref a) = self.action {
            attrs.push(("action", a));
        }
        if let Some(ref f) = self.from {
            attrs.push(("from", f));
        }
        if let Some(ref t) = self.to {
            attrs.push(("to", t));
        }
        if let Some(ref c) = self.status_callback {
            attrs.push(("statusCallback", c));
        }
        format_xml_string("Sms", &attrs, self.txt.as_ref())
    }
}

impl Default for Sms {
    fn default() -> Sms {
        Sms {
            txt: "".to_string(),
            action: None,
            method: Method::Post,
            from: None,
            to: None,
            status_callback: None,
        }
    }
}
