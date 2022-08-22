mod app;
mod batch;
mod process;
mod operation;

#[macro_use]
extern crate text_io;

use crate::app::run_app;
use std::{io, error::Error};
use std::io::stdin;
use std::str::FromStr;
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
    let processes = data_input();
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal);

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

fn data_input() -> Vec<Process> {
    print!("\x1B[2J\x1B[1;1H");
    println!("Ingrese el numero de procesos:");
    let max_processes: u8 = read!();
    print!("\x1B[2J\x1B[1;1H");
    let mut processes: Vec<Process> = Vec::new();

    for n in 0..max_processes {
        println!("Ingrese el id del {}# proceso", n+1);
        let id: u8 = read!();
        print!("\x1B[2J\x1B[1;1H");

        println!("Ingrese el nombre del proceso {}", id);
        let name: String = read!();
        print!("\x1B[2J\x1B[1;1H");

        println!("Ingrese el tiempo estimado del proceso {}", id);
        let expected_time: u8 = read!();
        print!("\x1B[2J\x1B[1;1H");

        println!("Ingrese la operacion del proceso {}. (Ej. \"5 + 3\"", id);
        let mut operation_str = String::new();
        stdin().read_line(&mut operation_str).unwrap(); // including '\n'
        print!("\x1B[2J\x1B[1;1H");

        processes.push(Process::new(id, name, expected_time, operation_str.trim_end()));
    }

    processes
}