use crate::blockers::*;
use crate::blocklist_blocker::BlocklistBlocker;
use crate::commands::QuteCommand;
use crate::file_blocker::FileBlocker;

use scraper::Html;

/// A blocker for a specific website of cookie banner.
pub trait Blocker {
    /// Check whether that blocker can be used on the website with the given url with the html.
    fn matches(&self, url: &str, html: &Html) -> bool;

    /// Give the steps to block he cookies.
    fn block(&self, url: &str, html: &Html) -> Vec<QuteCommand>;
}

pub fn get_blockers(data_dir: &str) -> Vec<Box<dyn Blocker>> {
    vec![
        Box::new(FileBlocker::new(data_dir)),
        Box::new(OneTrust {}),
        Box::new(CookieNotice {}),
        Box::new(Truste {}),
        Box::new(HuManity {}),
        Box::new(Fc {}),
        Box::new(Fandom {}),
        Box::new(BlocklistBlocker::new(data_dir)),
    ]
}

pub fn get_blocking_steps(
    blockers: Vec<Box<dyn Blocker>>,
    url: &str,
    html: &Html,
) -> Option<Vec<QuteCommand>> {
    for blocker in blockers {
        if blocker.as_ref().matches(url, html) {
            return Some(blocker.as_ref().block(url, html));
        }
    }

    None
}
