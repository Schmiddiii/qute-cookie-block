use crate::blocker::Blocker;
use crate::commands::*;

use scraper::{Html, Selector};

const FC_SETTINGS_BTN: &str = ".fc-cta-manage-options";
const FC_CHECKBOXES: &str =
    ".fc-legitimate-interest-preference-container > span > input[type='checkbox']";
const FC_CONFIM: &str = ".fc-save-continue";

pub struct Fc {}

impl Blocker for Fc {
    fn matches(&self, _url: &str, html: &Html) -> bool {
        let btn_settings = Selector::parse(FC_SETTINGS_BTN).unwrap();

        html.select(&btn_settings).next().is_some()
    }

    fn block(&self, _url: &str, _html: &Html) -> Vec<QuteCommand> {
        let mut commands = vec![];

        // Click on "settings" and wait.
        commands.push(QuteCommand::JsEval(Js::Click(FC_SETTINGS_BTN.to_string())));

        // Click on all checked boxes.
        commands.push(QuteCommand::JsEval(Js::ClickAny(FC_CHECKBOXES.to_string())));
        // Click on "save preferences"
        commands.push(QuteCommand::JsEval(Js::Click(FC_CONFIM.to_string())));

        commands
    }
}
