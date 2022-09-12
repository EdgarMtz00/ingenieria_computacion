mod app;
mod batch;
mod process;
mod operation;
mod state;

#[macro_use]
extern crate text_io;

use crate::app::run_app;
use std::{io, error::Error};
use tui::{
    backend::CrosstermBackend,
    Terminal,
};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use crate::operation::Operation;
use crate::process::Process;


fn main() -> Result<(), Box<dyn Error>> {
    print!("\x1B[2J\x1B[1;1H");
    println!("Ingrese el numero de procesos:");
    let num_processes: usize = read!();
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal, num_processes);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}