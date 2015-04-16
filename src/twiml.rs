pub enum Action {
    Message(String),
    Redirect(String),
}

impl Action {
    fn as_xml(&self) -> String {
        let (tag_name, inner) = match *self {
            Action::Message(ref s) => ("Message", s),
            Action::Redirect(ref s) => ("Redirect",s),
        };
        format!("<{}>{}</{}>",tag_name,inner,tag_name)
    }
}

pub struct Twiml {
    actions : Vec<Action>,
}

impl Twiml {
    pub fn new() -> Twiml {
        Twiml { actions : vec![] }
    }
    pub fn add(&mut self, a: Action) -> &Self {
        self.actions.push(a);
        self
    }
}
