use crate::blocker::Blocker;
use crate::commands::*;

use scraper::{Html, Selector};

const COOKIENOTICE_REJECT_BTN: &str = "#cn-refuse-cookie";

pub struct CookieNotice {}

impl Blocker for CookieNotice {
    fn matches(&self, _url: &str, html: &Html) -> bool {
        let btn_reject = Selector::parse(COOKIENOTICE_REJECT_BTN).unwrap();

        html.select(&btn_reject).next().is_some()
    }

    fn block(&self, _url: &str, _html: &Html) -> Vec<QuteCommand> {
        vec![QuteCommand::JsEval(Js::Click(
            COOKIENOTICE_REJECT_BTN.to_string(),
        ))]
    }
}
