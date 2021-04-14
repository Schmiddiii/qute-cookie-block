use crate::blocker::Blocker;
use crate::commands::*;

use scraper::{Html, Selector};

const TRUSTE_BANNER: &str = "#truste-consent-track";
const TRUSTE_OVERLAY: &str = ".truste_overlay";
const TRUSTE_BOX_OVERLAY: &str = ".truste_box_overlay";

// You can only hide the banner with truste as the cookie banner is in a iframe.
pub struct Truste {}

impl Blocker for Truste {
    fn matches(&self, _url: &str, html: &Html) -> bool {
        let btn_accept = Selector::parse(&format!(
            "{}, {}, {}",
            TRUSTE_BANNER, TRUSTE_OVERLAY, TRUSTE_BOX_OVERLAY
        ))
        .unwrap();

        html.select(&btn_accept).next().is_some()
    }

    fn block(&self, _url: &str, _html: &Html) -> Vec<QuteCommand> {
        vec![
            QuteCommand::JsEval(Js::Remove(TRUSTE_BANNER.to_string())),
            QuteCommand::JsEval(Js::Remove(TRUSTE_OVERLAY.to_string())),
            QuteCommand::JsEval(Js::Remove(TRUSTE_BOX_OVERLAY.to_string())),
        ]
    }
}
