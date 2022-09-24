use std::{time::Duration, io::Error};

use crossterm::{execute, event::{poll, Event, DisableMouseCapture, DisableFocusChange, KeyCode, read}};
use tui::{backend::Backend, Terminal};



pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, num_processes: usize) -> Result<(), Error>{
    execute!(std::io::stdout(), DisableMouseCapture).unwrap();
    execute!(std::io::stdout(), DisableFocusChange).unwrap();
    //let mut state: State = State::new(num_processes);
    //terminal.draw(|f| ui(f, &state))?;
    let mut pause = false;
    loop {
        /*if !state.is_finished() && !pause {
            state.execute_step();
        }

        if poll(Duration::from_secs(1))? {
            if let Event::Key(key) = read()?{
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('p') => pause = true,
                    KeyCode::Char('c') => pause = false,
                    KeyCode::Char('w') => state.end_process_without_result(),
                    KeyCode::Char('e') => state.interrupt(),
                    _ => {}
                }
            }
        }*/

    //terminal.draw(|f| ui(f, &state))?;
    }
}
