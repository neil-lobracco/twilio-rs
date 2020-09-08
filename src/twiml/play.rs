use super::{format_xml_string, Action};
use std::char::from_digit;

pub struct Digits {
    s: String,
}

impl Digits {
    pub fn new() -> Digits {
        Digits { s: "".to_string() }
    }
    pub fn add(&mut self, d: u32) -> &mut Digits {
        self.s.push(from_digit(d, 10).unwrap());
        self
    }
    pub fn add_wait(&mut self) -> &mut Digits {
        self.s.push('w');
        self
    }
    fn as_str(&self) -> &str {
        self.s.as_ref()
    }
}

pub enum Playable {
    Url(String),
    Digits(Digits),
}

pub struct Play {
    playable: Playable,
    loop_count: usize,
}

impl Action for Play {
    fn as_twiml(&self) -> String {
        let loop_string = format!("{}", self.loop_count);
        let mut atts = Vec::new();
        atts.push(("loop", &loop_string[..]));
        let inner = match self.playable {
            Playable::Url(ref s) => s.as_ref(),
            Playable::Digits(ref d) => {
                atts.push(("digits", d.as_str()));
                ""
            }
        };
        format_xml_string("Play", &atts, inner)
    }
}
