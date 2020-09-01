use super::{format_xml_string, Action, Method, Play, Say};
use std::default::Default;

pub enum Prompt {
    Nothing,
    Play(Play),
    Say(Say),
}

pub struct Gather {
    pub action: Option<String>,
    pub method: Method,
    pub timeout_seconds: u32,
    pub finish_on_key: char,
    pub num_digits: Option<u32>,
    pub prompt: Prompt,
}

impl Action for Gather {
    fn as_twiml(&self) -> String {
        let timeout_string = format!("{}", self.timeout_seconds);
        let finish_string = self.finish_on_key.to_string();
        let digits_string = self.num_digits.map(|d| format!("{}", d));
        let mut attrs = Vec::new();
        let method_str = match self.method {
            Method::Get => "GET",
            Method::Post => "POST",
        };
        attrs.push(("method", method_str));
        if let Some(ref a) = self.action {
            attrs.push(("action", a));
        }
        attrs.push(("timeout", timeout_string.as_ref()));
        attrs.push(("finishOnKey", finish_string.as_ref()));
        if let Some(ref d) = digits_string {
            attrs.push(("numDigits", d.as_ref()));
        }
        let inner = match self.prompt {
            Prompt::Nothing => "".to_string(),
            Prompt::Play(ref p) => p.as_twiml(),
            Prompt::Say(ref s) => s.as_twiml(),
        };
        format_xml_string("Gather", &attrs, inner.as_ref())
    }
}

impl Default for Gather {
    fn default() -> Gather {
        Gather {
            action: None,
            method: Method::Post,
            timeout_seconds: 5,
            finish_on_key: '*',
            num_digits: None,
            prompt: Prompt::Nothing,
        }
    }
}
