use std::{
    io::{self, stdout, Stdout},
    panic::{self, PanicInfo},
    sync::Arc,
};

use anyhow::Result;
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, terminal::disable_raw_mode};
use tokio::sync::Mutex;
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use app::App;
use event::Key;

use crate::cli::Cli;
use crate::operations::find_projects;

mod app;
mod banner;
mod cli;
mod event;
mod handlers;
mod operations;
mod ui;

#[tokio::main]
async fn main() -> Result<()> {
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

    let projects = find_projects(&*cli.path, &*cli.criteria);
    // Initialize app state
    let app = Arc::new(Mutex::new(App::new(projects, true)));

    start_ui(cli, &app).await?;

    Ok(())
}

async fn start_ui(_cli: Cli, app: &Arc<Mutex<App>>) -> Result<()> {
    // Terminal initialization
    let mut stdout = stdout();
    // not capturing mouse to make text select/copy possible
    execute!(stdout, EnterAlternateScreen)?;
    // see https://docs.rs/crossterm/0.17.7/crossterm/terminal/#raw-mode
    enable_raw_mode()?;
    // terminal backend for cross platform support
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;
    // custom events
    let events = event::Events::new(250u64);
    // main UI loop
    loop {
        let mut app = app.lock().await;

        // Get the size of the screen on each loop to account for resize event
        if let Ok(size) = terminal.backend().size() {
            // Reset the help menu if the terminal was resized
            if app.refresh || app.size != size {
                app.size = size;

                // Based on the size of the terminal, adjust how many cols are
                // displayed in the tables
                if app.size.width > 8 {
                    app.table_cols = app.size.width - 1;
                } else {
                    app.table_cols = 2;
                }
            }
        };
        // draw the UI layout
        terminal.draw(|f| ui::draw(f, &mut app))?;

        // handle key events
        match events.next()? {
            event::Event::Input(key) => {
                // quit on CTRL + C
                if key == Key::Ctrl('c') {
                    break;
                }
                // handle all other keys
                handlers::handle_key_events(key, &mut app).await
            }
            // handle mouse events
            event::Event::MouseInput(mouse) => handlers::handle_mouse_events(mouse, &mut app).await,
            _ => {}
        }
        if app.should_quit {
            break;
        }
    }

    terminal.show_cursor()?;
    shutdown(terminal)?;

    Ok(())
}

// shutdown the CLI and show terminal
fn shutdown(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    terminal.show_cursor()?;
    Ok(())
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
