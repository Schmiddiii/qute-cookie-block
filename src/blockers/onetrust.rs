use crate::blocker::Blocker;
use crate::commands::*;

use scraper::{Html, Selector};

const ONETRUST_ACCEPT_BTN: &str = "#onetrust-accept-btn-handler";
const ONETRUST_REJECT_BTN: &str = "#onetrust-reject-all-handler";
const ONETRUST_SETTINGS_BTN: &str = "#onetrust-pc-btn-handler";
const ONETRUST_TOGGLES: [&str; 2] = [".ot-tgl", ".ot-toggle"];
const ONETRUST_CHECKBOXES: &str = "input[type=checkbox][aria-checked=true]";
const ONETRUST_CONFIM: &str = ".save-preference-btn-handler.onetrust-close-btn-handler";

pub struct OneTrust {}

impl Blocker for OneTrust {
    fn matches(&self, _url: &str, html: &Html) -> bool {
        let btn_accept = Selector::parse(ONETRUST_ACCEPT_BTN).unwrap();

        html.select(&btn_accept).next().is_some()
    }

    fn block(&self, _url: &str, html: &Html) -> Vec<QuteCommand> {
        let mut commands = vec![];

        let btn_reject = Selector::parse(ONETRUST_REJECT_BTN).unwrap();

        // Check for a available "reject all" button
        if html.select(&btn_reject).next().is_some() {
            commands.push(QuteCommand::JsEval(Js::Click(
                ONETRUST_REJECT_BTN.to_string(),
            )));
        } else {
            // Click on "settings" and wait.
            commands.push(QuteCommand::JsEval(Js::Click(
                ONETRUST_SETTINGS_BTN.to_string(),
            )));
            commands.push(QuteCommand::Timeout(None));

            let mut toggles_selector: Vec<String> = ONETRUST_TOGGLES
                .iter()
                .map(|i| format!("{} {}", i, ONETRUST_CHECKBOXES))
                .collect();
            let first = toggles_selector.remove(0);
            let combined_selectors: String = toggles_selector
                .iter()
                .fold(first, |acc, item| format!("{}, {}", acc, item));

            // Click on all checked boxes.
            commands.push(QuteCommand::JsEval(Js::ClickAny(combined_selectors)));
            // Click on "save preferences"
            commands.push(QuteCommand::JsEval(Js::Click(ONETRUST_CONFIM.to_string())));
        }

        commands
    }
}
