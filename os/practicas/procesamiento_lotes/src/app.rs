use std::io;
use std::time::Duration;
use crossterm::event::{DisableFocusChange, DisableMouseCapture, Event, KeyCode, poll, read};
use crossterm::execute;

use tui::{layout::{Alignment, Constraint, Direction, Layout}, style::{Style, Modifier, Color}, widgets::{Paragraph, Borders, Block, Wrap}, text::{Span}, backend::Backend, Terminal, Frame};
use crate::state::State;


pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, num_processes: usize) -> io::Result<()> {
    execute!(std::io::stdout(), DisableMouseCapture).unwrap();
    execute!(std::io::stdout(), DisableFocusChange).unwrap();
    let mut state: State = State::new(num_processes);
    terminal.draw(|f| ui(f, &state))?;
    let mut pause = false;
    loop {
        if !state.is_finished() && !pause {
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
        }

    terminal.draw(|f| ui(f, &state))?;
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, state: &State){
    let size = f.size();
    let block = Block::default()
        .title("Proceasmiento por lotes")
        .borders(Borders::ALL);
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(80),
            ]
            .as_ref(),
        )
        .split(size);


    let create_block = |title| {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Gray).fg(Color::Black))
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            ))
    };

    let paragraph = Paragraph::new(
        format!("Cantidad de lotes a procesar: {}\nTiempo Transcurrido: {}",
                state.get_remaining_batches(), state.get_time_on_execution()))
        .style(Style::default().fg(Color::Black))
        .block(create_block("Lotes"))
        .alignment(Alignment::Left);
    f.render_widget(paragraph, chunks[0]);

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    // get current batch pending processes
    let current_batch = state.get_current_batch();
    let pending_processes = current_batch.get_pending_processes();
    let pending_processes_text = pending_processes.iter().fold(String::new(), |acc, process| {
        acc + &format!("{} \n", process.get_name_and_remaining_time())
    });

    let paragraph = Paragraph::new(pending_processes_text)
        .style(Style::default().fg(Color::Black))
        .block(create_block("Procesos pendientes"))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, bottom_chunks[0]);


    match current_batch.get_current_process(){
        Some(process) => {
            let paragraph = Paragraph::new(process.to_string())
                .style(Style::default().fg(Color::Black))
                .block(create_block("Proceso en ejecucion"))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true });
            f.render_widget(paragraph, bottom_chunks[1]);
        }
        None => {
            let paragraph = Paragraph::new("No hay procesos actualmente")
                .style(Style::default().fg(Color::Black))
                .block(create_block("Proceso actual"))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true });
            f.render_widget(paragraph, bottom_chunks[1]);
        }
    };

    let finished_processes = state.get_finished_processes();
    let pending_processes_text = finished_processes.iter().fold(String::new(), |acc, process| {
        acc + &format!("{} \n", process.get_name_operation_result())
    });
    let paragraph = Paragraph::new(pending_processes_text)
        .style(Style::default().fg(Color::Black))
        .block(create_block("Procesos finalizados"))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, bottom_chunks[2]);
}