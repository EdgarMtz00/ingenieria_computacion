use std::io;

use crossterm::event::{KeyCode, Event, self};
use tui::{layout::{Alignment, Constraint, Direction, Layout}, style::{Style, Modifier, Color}, widgets::{Paragraph, Borders, Block, Wrap}, text::{Span, Spans}, backend::Backend, Terminal, Frame};

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(ui)?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>) {
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
                Constraint::Percentage(10),
                Constraint::Percentage(90),
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

    let paragraph = Paragraph::new(format!("Cantidad de batches a procesar: {}", 3))
        .style(Style::default().fg(Color::Black))
        .block(create_block("Batches"))
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

    let paragraph = Paragraph::new("Procesos pendientes")
        .style(Style::default().fg(Color::Black))
        .block(create_block("Procesos pendientes"))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, bottom_chunks[0]);

    let paragraph = Paragraph::new("Proceso en ejecucion")
        .style(Style::default().fg(Color::Black))
        .block(create_block("Proceso en ejecucion"))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, bottom_chunks[1]);

    let paragraph = Paragraph::new("relleno")
        .style(Style::default().fg(Color::Black))
        .block(create_block("Procesos finalizados"))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, bottom_chunks[2]);
}
