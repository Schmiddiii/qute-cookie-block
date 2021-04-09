use std::{
    fmt::{Display, Formatter, Result},
    fs::File,
    io::Write,
};

const DEFAULT_TIMEOUT: u64 = 500;

pub enum QuteCommand {
    JsEval(Js),
    Timeout(Option<u64>),
}

pub enum Js {
    Click(String),
    Raw(String),
}

impl QuteCommand {
    pub fn execute(&self, fifo: &mut File) {
        match self {
            QuteCommand::Timeout(time) => std::thread::sleep(std::time::Duration::from_millis(
                time.unwrap_or(DEFAULT_TIMEOUT),
            )),
            _ => {
                writeln!(fifo, "{}", self).unwrap();
                fifo.flush().unwrap();
            }
        }
    }
}

impl Display for QuteCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            QuteCommand::JsEval(js) => {
                write!(f, "jseval -q {}", js)
            }
            _ => write!(f, ""),
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
