use crate::blockers::*;
use crate::commands::QuteCommand;

use scraper::Html;

/// A blocker for a specific website of cookie banner.
pub trait Blocker {
    /// Create a new instance of the blocker.
    fn new() -> Self
    where
        Self: Sized;

    /// Check whether that blocker can be used on the website with the given url with the html.
    fn matches(&self, url: &str, html: &Html) -> bool;

    /// Give the steps to block he cookies.
    fn block(&self) -> Vec<QuteCommand>;
}

fn get_blockers() -> Vec<Box<dyn Blocker>> {
    vec![Box::new(Reddit::new()), Box::new(Fandom::new())]
}

pub fn get_blocking_steps(url: &str, html: &Html) -> Option<Vec<QuteCommand>> {
    for blocker in get_blockers() {
        if blocker.as_ref().matches(url, html) {
            return Some(blocker.as_ref().block());
        }
    }

    None
}
