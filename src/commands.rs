use std::fmt::{Display, Formatter, Result};

pub enum QuteCommand {
    JsEval(Js),
}

pub enum Js {
    Click(String),
    Raw(String),
}

impl Display for QuteCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            QuteCommand::JsEval(js) => {
                write!(f, "jseval -q {}", js)
            }
        }
    }
}

impl Display for Js {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Js::Click(selector) => {
                write!(f, r#"document.querySelector("{}").click()"#, selector)
            }
            Js::Raw(string) => write!(f, "{}", string),
        }
    }
}
