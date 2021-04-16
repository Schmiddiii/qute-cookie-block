use crate::blocker::Blocker;
use crate::commands::{Js, QuteCommand};

use std::collections::HashSet;
use std::path::PathBuf;

use scraper::{Html, Selector};

/// A blocker that uses xml files.
pub struct BlocklistBlocker {
    file: PathBuf,
}

impl BlocklistBlocker {
    /// Create the blocker. The block files must be `data_dir/cookie-blockers/blocklist.txt`.
    pub fn new(data_dir: &str) -> Self {
        let mut path = PathBuf::from(data_dir);
        path.push("cookie-blockers");

        if path.is_dir() {
            path.push("blocklist.txt");

            BlocklistBlocker { file: path }
        } else {
            panic!("Cannot find the directory containing the blockers");
        }
    }

    /// Returns the blocked ids and classes, in this order.
    fn get_blocked(&self) -> (HashSet<String>, HashSet<String>) {
        let file_content = std::fs::read_to_string(&self.file).expect("Failed to read blocklist");
        let valid_lines = file_content
            .lines()
            .filter(|s| !s.starts_with("!") && s.len() >= 2)
            .filter(|s| {
                s.chars()
                    .skip(3)
                    .all(|c| c == '_' || c == '-' || c.is_ascii_alphanumeric())
            })
            .map(|s| s.split_at(2).1);
        let ids = valid_lines
            .clone()
            .filter(|s| s.starts_with("#"))
            .map(|s| s.split_at(1).1.to_string())
            .collect();
        let classes = valid_lines
            .filter(|s| s.starts_with("."))
            .map(|s| s.split_at(1).1.to_string())
            .collect();
        (ids, classes)
    }
}

impl Blocker for BlocklistBlocker {
    fn matches(&self, _url: &str, html: &Html) -> bool {
        let (blocked_ids, blocked_classes) = self.get_blocked();

        let selector_ids = Selector::parse("[id]").unwrap();
        let selector_classes = Selector::parse("[class]").unwrap();

        let id_found = html
            .select(&selector_ids)
            .map(|e| e.value().id())
            .filter(|i| i.is_some())
            .any(|i| blocked_ids.get(i.unwrap()).is_some());

        if id_found {
            true
        } else {
            html.select(&selector_classes)
                .map(|e| e.value().classes().map(|s| s.to_string()).collect())
                .any(|cls| blocked_classes.intersection(&cls).last().is_none())
        }
    }

    fn block(&self, _url: &str, html: &Html) -> Vec<QuteCommand> {
        let (blocked_ids, blocked_classes) = self.get_blocked();

        let selector_ids = Selector::parse("[id]").unwrap();
        let selector_classes = Selector::parse("[class]").unwrap();

        let ids = html
            .select(&selector_ids)
            .map(|e| e.value().id())
            .filter(|i| i.is_some())
            .filter(|i| blocked_ids.get(i.unwrap()).is_some())
            .map(|i| QuteCommand::JsEval(Js::Remove(format!("#{}", i.unwrap()))));

        let classes = html
            .select(&selector_classes)
            .map(|e| e.value().classes().collect())
            .map(|cls: HashSet<&str>| {
                blocked_classes
                    .intersection(
                        &cls.iter()
                            .map(|s| s.to_string())
                            .collect::<HashSet<String>>()
                            .clone(),
                    )
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect()
            })
            .fold(HashSet::new(), |acc, v: HashSet<String>| {
                acc.union(&v).cloned().collect()
            });

        let classes_commands = classes
            .iter()
            .map(|c| QuteCommand::JsEval(Js::RemoveAny(format!(".{}", c))));

        ids.chain(classes_commands).collect()
    }
}
