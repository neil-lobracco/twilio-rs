use twiml::{Action,format_xml_string};
pub enum Voice {
    Man,
    Woman,
    Alice,
}
pub struct Say {
    txt: String,
    voice: Voice,
    language: String,
}
impl Action for Say {
    fn as_twiml(&self) -> String {
        let voice_str = match self.voice {
            Voice::Man => "man",
            Voice::Woman => "woman",
            Voice::Alice => "alice",
        };
        format_xml_string("Say",&vec![("voice",voice_str),("language",&self.language)],&self.txt)
    }
}
impl Say {
    pub fn new(txt: String, voice: Voice, language: String) -> Say {
        Say { txt: txt, voice: voice, language: language }
    }
}
