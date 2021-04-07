use crate::blocker::Blocker;
use crate::commands::*;

use scraper::Html;

pub struct Fandom {}

impl Blocker for Fandom {
    fn new() -> Self {
        Fandom {}
    }

    fn matches(&self, url: &str, _html: &Html) -> bool {
        url.contains("fandom.com")
    }

    fn block(&self) -> Vec<QuteCommand> {
        vec![
            QuteCommand::JsEval(Js::Click("body > div:nth-child(17) > div > div > div._1r08nyekFdI7_2d8r3AIBf > div.bXhNiA09CDOuFH0Zi9NOx.XHcr6qf5Sub2F2zBJ53S_".to_string())),
            QuteCommand::JsEval(Js::Click("body > div:nth-child(17) > div > div > div._1r08nyekFdI7_2d8r3AIBf > div.hscaUBZW7rwd_U4uzKfml.XHcr6qf5Sub2F2zBJ53S_".to_string()))
        ]
    }
}
