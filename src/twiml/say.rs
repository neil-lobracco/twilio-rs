use super::{format_xml_string, Action};

pub enum Voice {
    Man,
    Woman,
    Alice,
}

pub struct Say {
    pub txt: String,
    pub voice: Voice,
    pub language: String,
}

impl Action for Say {
    fn as_twiml(&self) -> String {
        let voice_str = match self.voice {
            Voice::Man => "man",
            Voice::Woman => "woman",
            Voice::Alice => "alice",
        };
        format_xml_string(
            "Say",
            &vec![("voice", voice_str), ("language", &self.language)],
            &self.txt,
        )
    }
}
