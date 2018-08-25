use twiml::{Action, format_xml_string, Method};
use std::default::Default;

pub struct Dial {
    pub number: String,
    pub action: Option<String>,
    pub method: Method,
    pub timeout_seconds: u32,
    pub caller_id: Option<String>,
}

impl Action for Dial {
    fn as_twiml(&self) -> String {
        let timeout_string = format!("{}",self.timeout_seconds);
        let mut attrs = Vec::new();
        let method_str = match self.method {
            Method::Get => "GET",
            Method::Post => "POST",
        };
        attrs.push(("method", method_str));
        if let Some(ref a) = self.action {
            attrs.push(("action", a));
        }
        if let Some(ref cid) = self.caller_id {
            attrs.push(("caller_id", cid));
        }
        attrs.push(("timeout", timeout_string.as_ref()));
        format_xml_string("Dial", &attrs, self.number.as_ref())
    }
}

impl Default for Dial {
    fn default() -> Dial {
        Dial { number: "".to_string(), action: None, method: Method::Post, timeout_seconds: 5, caller_id: None }
    }
}
