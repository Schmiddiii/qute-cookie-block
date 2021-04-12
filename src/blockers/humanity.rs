use crate::blocker::Blocker;
use crate::commands::*;

use scraper::{Html, Selector};

const HUMANITY_REJECT_BTN: &str = ".hu-notice-reject";

pub struct HuManity {}

impl Blocker for HuManity {
    fn matches(&self, _url: &str, html: &Html) -> bool {
        let btn_reject = Selector::parse(HUMANITY_REJECT_BTN).unwrap();

        html.select(&btn_reject).next().is_some()
    }

    fn block(&self, _url: &str, _html: &Html) -> Vec<QuteCommand> {
        vec![QuteCommand::JsEval(Js::Click(
            HUMANITY_REJECT_BTN.to_string(),
        ))]
    }
}
