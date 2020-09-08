use super::{format_xml_string, Action, Method};
use std::default::Default;

pub enum Transcribe {
    DontTranscribe,
    StoreTranscription,
    CallbackTranscription(String),
}

pub struct Record {
    pub action: Option<String>,
    pub method: Method,
    pub timeout_seconds: u32,
    pub finish_on_key: char,
    pub max_length_seconds: u32,
    pub transcribe: Transcribe,
    pub play_beep: bool,
    pub trim: bool,
}

impl Action for Record {
    fn as_twiml(&self) -> String {
        let timeout_string = format!("{}", self.timeout_seconds);
        let finish_string = self.finish_on_key.to_string();
        let length_string = format!("{}", self.max_length_seconds);
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
        attrs.push(("maxLength", length_string.as_ref()));
        attrs.push(("playBeep", if self.play_beep { "true" } else { "false" }));
        attrs.push((
            "trim",
            if self.trim {
                "trim-silence"
            } else {
                "do-not-trim"
            },
        ));
        match self.transcribe {
            Transcribe::DontTranscribe => {
                attrs.push(("transcribe", "false"));
            }
            Transcribe::StoreTranscription => {
                attrs.push(("transcribe", "true"));
            }
            Transcribe::CallbackTranscription(ref s) => {
                /* transcribe=true is implied in this case */
                attrs.push(("transcribeCallback", s.as_ref()));
            }
        };
        format_xml_string("Record", &attrs, "")
    }
}

impl Default for Record {
    fn default() -> Record {
        Record {
            action: None,
            method: Method::Post,
            timeout_seconds: 5,
            finish_on_key: '*',
            max_length_seconds: 3600,
            transcribe: Transcribe::DontTranscribe,
            play_beep: true,
            trim: true,
        }
    }
}
