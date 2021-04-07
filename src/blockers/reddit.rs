use crate::blocker::Blocker;
use crate::commands::*;

use scraper::Html;

pub struct Reddit {}

impl Blocker for Reddit {
    fn new() -> Self {
        Reddit {}
    }

    fn matches(&self, url: &str, _html: &Html) -> bool {
        url.contains("reddit.com")
    }

    fn block(&self) -> Vec<QuteCommand> {
        vec![QuteCommand::JsEval(Js::Click("#SHORTCUT_FOCUSABLE_DIV > div:nth-child(6) > div._3q-XSJ2vokDQrvdG6mR__k > section > div > section > section > form:nth-child(1) > button".to_string()))]
    }
}
