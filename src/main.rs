use std::{
    io::{self},
    panic::{self, PanicInfo},
};

use crossterm::{execute, terminal::disable_raw_mode};
use crossterm::terminal::LeaveAlternateScreen;

use nmkill::{delete_directory, find_directories};

use crate::cli::Cli;

mod banner;
mod cli;

fn main() {
    panic::set_hook(Box::new(|info| {
        panic_hook(info);
    }));
    let mut cli = Cli::new();
    let clap_app = cli.get_clap_app();
    let matches = clap_app.get_matches();

    // parse CLI arguments
    if let Some(path) = matches.value_of("path") {
        cli.path = path.to_string();
    }
    if let Some(criteria) = matches.value_of("criteria") {
        cli.criteria = criteria.to_string();
    } else {
        cli.criteria = "node_modules".to_string();
    }

    println!("Searching for {}", cli.path);

    let projects = find_directories(&*cli.path, &*cli.criteria);

    println!("Found {} projects", projects.len());
    delete_directory(projects);
}

#[cfg(debug_assertions)]
fn panic_hook(info: &PanicInfo<'_>) {
    use backtrace::Backtrace;
    use crossterm::style::Print;

    let location = info.location().unwrap();

    let msg = match info.payload().downcast_ref::<&'static str>() {
        Some(s) => *s,
        None => match info.payload().downcast_ref::<String>() {
            Some(s) => &s[..],
            None => "Box<Any>",
        },
    };

    let stacktrace: String = format!("{:?}", Backtrace::new()).replace('\n', "\n\r");

    disable_raw_mode().unwrap();
    execute!(
        io::stdout(),
        LeaveAlternateScreen,
        Print(format!(
            "thread '<unnamed>' panicked at '{}', {}\n\r{}",
            msg, location, stacktrace
        )),
    )
    .unwrap();
}

#[cfg(not(debug_assertions))]
fn panic_hook(info: &PanicInfo<'_>) {
    use human_panic::{handle_dump, print_msg, Metadata};

    let meta = Metadata {
        version: env!("CARGO_PKG_VERSION").into(),
        name: env!("CARGO_PKG_NAME").into(),
        authors: env!("CARGO_PKG_AUTHORS").replace(":", ", ").into(),
        homepage: env!("CARGO_PKG_HOMEPAGE").into(),
    };
    let file_path = handle_dump(&meta, info);
    disable_raw_mode().unwrap();
    execute!(io::stdout(), LeaveAlternateScreen).unwrap();
    print_msg(file_path, &meta).expect("human-panic: printing error message to console failed");
}
