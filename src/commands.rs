use std::{
    convert::TryFrom,
    fmt::{Display, Formatter, Result},
    fs::File,
    io::Write,
};

const DEFAULT_TIMEOUT: u64 = 500;

pub enum QuteCommand {
    JsEval(Js),
    Timeout(Option<u64>),
    Debug(DebugType, String),
}

pub enum Js {
    Click(String),
    ClickAny(String),
    Remove(String),
    RemoveAny(String),
    Raw(String),
}

pub enum DebugType {
    Error,
    Info,
    Warning,
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
            QuteCommand::Debug(debug_type, message) => {
                write!(f, r#"message-{} "{}""#, debug_type, message)
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
            Js::ClickAny(selector) => {
                write!(
                    f,
                    r#"Array.prototype.slice.call(document.querySelectorAll("{}")).forEach(function(e, idx) {{e.click()}})"#,
                    selector
                )
            }
            Js::Remove(selector) => {
                write!(f, r#"document.querySelector("{}").remove()"#, selector)
            }
            Js::RemoveAny(selector) => {
                write!(
                    f,
                    r#"Array.prototype.slice.call(document.querySelectorAll("{}")).forEach(function(e, idx) {{e.remove()}})"#,
                    selector
                )
            }
            Js::Raw(string) => write!(f, "{}", string),
        }
    }
}

impl Display for DebugType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            DebugType::Error => write!(f, "error"),
            DebugType::Info => write!(f, "info"),
            DebugType::Warning => write!(f, "warning"),
        }
    }
}

impl TryFrom<String> for DebugType {
    type Error = ();
    fn try_from(string: String) -> std::result::Result<DebugType, ()> {
        match &string[..] {
            "error" => Ok(DebugType::Error),
            "info" => Ok(DebugType::Info),
            "warninng" => Ok(DebugType::Warning),
            _ => Err(()),
        }
    }
}
