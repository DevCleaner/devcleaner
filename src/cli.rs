use clap::{App as ClapApp, Arg};
use std::env;

use super::banner::BANNER;

/// DevCleaner CLI
pub struct Cli {
    /// time in ms between two ticks.
    pub path: String,
    /// time in ms between two network calls.
    pub criteria: String,
    /// whether unicode symbols are used to improve the overall look of the app
    pub enhanced_graphics: bool,
}

impl Cli {
    pub fn new() -> Cli {
        Cli {
            path: env::current_dir().unwrap().to_str().unwrap().to_string(),
            criteria: "node_modules".into(),
            enhanced_graphics: true,
        }
    }

    /// create a new clapapp instance
    pub fn get_clap_app<'a, 'b>(&mut self) -> ClapApp<'a, 'b> {
        ClapApp::new(env!("CARGO_PKG_NAME"))
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .usage("Press `?` while running the app to see keybindings")
            .before_help(BANNER)
            .arg(
                Arg::with_name("path")
                    .short("p")
                    .long("path")
                    .help("Set the path to scan for the criteria.")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("criteria")
                    .short("c")
                    .long("criteria")
                    .help("Set the criteria to search.")
                    .default_value("node_modules")
                    .takes_value(true),
            )
    }
}
