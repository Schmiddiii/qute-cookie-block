use crate::blocker::Blocker;
use crate::commands::{Js, QuteCommand};

use std::path::PathBuf;
use std::{collections::HashMap, fs};

use quick_xml::events::Event;
use quick_xml::Reader;
use scraper::Html;
use url::Url;

/// A blocker that uses xml files.
pub struct FileBlocker {
    files: Vec<PathBuf>,
}

impl FileBlocker {
    /// Create the blocker. The xml files must be in `data_dir/cookie-blockers`.
    pub fn new(data_dir: &str) -> Self {
        let mut path = PathBuf::from(data_dir);
        path.push("cookie-blockers");

        let mut files = vec![];

        if path.is_dir() {
            for entry in fs::read_dir(&path).expect("Failed to read folder containing the blockers")
            {
                let entry = entry.unwrap();
                let entry_path = entry.path();
                if entry_path.is_file() {
                    files.push(entry_path);
                }
            }

            FileBlocker { files }
        } else {
            panic!("Cannot find the directory containing the blockers");
        }
    }

    /// Find a file path matching the given url.
    fn find_matching_file(&self, url: &str) -> Option<PathBuf> {
        let url = Url::parse(url).expect("Could not parse the url");
        let host_str = url.host_str().expect("Could not parse the url");

        for file in &self.files {
            if host_str.ends_with(file.file_stem().unwrap().to_str().unwrap()) {
                return Some(file.clone());
            }
        }
        None
    }

    /// Panics that the file could not be parsed.
    fn panic_file(path: &PathBuf) {
        panic!("Could not parse file {:?}", path);
    }
}

impl Blocker for FileBlocker {
    fn matches(&self, url: &str, _html: &Html) -> bool {
        self.find_matching_file(url).is_some()
    }

    fn block(&self, url: &str, _html: &Html) -> Vec<QuteCommand> {
        // Check for a matching file.
        if let Some(path) = self.find_matching_file(url) {
            let mut reader = Reader::from_file(path.clone()).unwrap();
            let mut buf = vec![];

            let mut commands = vec![];

            loop {
                match reader.read_event(&mut buf) {
                    // Pull each tag.
                    Ok(Event::Empty(tag)) => {
                        if let Ok(name) = std::str::from_utf8(tag.name()) {
                            let attributes = tag
                                .attributes()
                                .map(|a| {
                                    if a.is_err() {
                                        FileBlocker::panic_file(&path);
                                    }
                                    let attr = a.unwrap();
                                    (
                                        String::from_utf8(attr.clone().key.to_vec()).unwrap(),
                                        String::from_utf8(attr.value.into_owned()).unwrap(),
                                    )
                                })
                                .collect();

                            if let Some(command) = make_command(name, &attributes) {
                                commands.push(command);
                            } else {
                                FileBlocker::panic_file(&path);
                            }
                        } else {
                            FileBlocker::panic_file(&path);
                        }
                    }
                    Ok(Event::Eof) => break,
                    Err(_) => FileBlocker::panic_file(&path),
                    _ => (),
                }
            }

            commands
        } else {
            vec![]
        }
    }
}

fn make_command(name: &str, attributes: &HashMap<String, String>) -> Option<QuteCommand> {
    match name {
        "click" => {
            if let Some(element) = attributes.get("element") {
                return Some(QuteCommand::JsEval(Js::Click(element.to_string())));
            } else {
                return None;
            }
        }
        "js" => {
            if let Some(source) = attributes.get("source") {
                return Some(QuteCommand::JsEval(Js::Raw(source.to_string())));
            } else {
                return None;
            }
        }
        _ => {
            return None;
        }
    }
}
