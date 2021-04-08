mod blocker;
mod blockers;
mod commands;
mod file_blocker;

use std::env;
use std::fs::OpenOptions;
use std::io::{Read, Write};

use scraper::Html;

const QUTE_USERSCRIPTS_DIR: &str = "~/.local/share/qutebrowser/userscripts/";

fn main() {
    match (
        env::var("QUTE_MODE"),
        env::var("QUTE_FIFO"),
        env::var("QUTE_DATA_DIR"),
        env::var("QUTE_URL"),
        env::var("QUTE_HTML"),
    ) {
        (Ok(mode), Ok(fifo_path), Ok(data_path), Ok(url), Ok(html_path)) => {
            let mut fifo_file = OpenOptions::new()
                .read(false)
                .write(true)
                .append(true)
                .open(fifo_path.clone())
                .unwrap();
            let mut html_file = OpenOptions::new()
                .read(true)
                .write(false)
                .open(html_path)
                .unwrap();
            if mode == "command" {
                let mut html_str = String::new();
                html_file
                    .read_to_string(&mut html_str)
                    .expect("Could not read HTML file");
                let html = Html::parse_document(&html_str);

                let blockers = blocker::get_blockers(&data_path);

                let steps_opt = blocker::get_blocking_steps(blockers, &url, &html);
                if let Some(steps) = steps_opt {
                    for step in steps {
                        writeln!(fifo_file, "{}", step).unwrap();
                        fifo_file.flush().unwrap();
                    }
                } else {
                    writeln!(
                        fifo_file,
                        "message-warning \"This website cannot be blocked yet. Please create a issue or pull request.\""
                    )
                    .unwrap();
                }
            } else {
                write!(
                    fifo_file,
                    "message-warning \"You cannot spawn this script when hinting\""
                )
                .unwrap();
            }
        }
        _ => println!(
            "This script was not run from qutebrowser. Please put the compiled binary into {}",
            QUTE_USERSCRIPTS_DIR
        ),
    }
}
