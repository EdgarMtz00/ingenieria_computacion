use std::io;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use text_io::read;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use crate::app::run_app;

mod process_stats;
mod app;
mod process;
mod operation;
mod scheduler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("\x1B[2J\x1B[1;1H");
    println!("Ingrese el numero de procesos:");
    let num_processes: usize = read!();

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal, num_processes)?;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    for process in res {
        let stats = process.get_stats();
        let operation = process.get_operation();
        println!(
            "Proceso {}:\n\t- TME: {}\n\t- Hora de llegada: {}\n\t- Hora de inicio: {}\n\t\
            - Hora de finalizacion: {}\n\t- Tiempo de retorno: {}\n\t- Tiempo de respuesta: {}\n\t\
            - Tiempo de espera: {}\n\t- Tiempo de servicio: {}\n\t- Resultado {} = {}\n",
            process.get_id(),
            stats.get_expected_time(),
            stats.get_arrival_time().unwrap(),
            stats.get_start_time().unwrap(),
            stats.get_end_time().unwrap(),
            stats.arrival_to_end_time().unwrap(),
            stats.arrival_to_start_time().unwrap(),
            stats.get_waited_time(),
            stats.get_executed_time(),
            operation,
            operation.get_result()
        );
    }

    Ok(())
}
