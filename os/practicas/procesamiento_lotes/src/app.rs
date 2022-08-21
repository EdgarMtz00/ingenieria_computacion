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
        .margin(5)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(size);

    // Words made "loooong" to demonstrate line breaking.
    let s = "Veeeeeeeeeeeeeeeery    loooooooooooooooooong   striiiiiiiiiiiiiiiiiiiiiiiiiing.   ";
    let mut long_line = s.repeat(usize::from(size.width) / s.len() + 4);
    long_line.push('\n');
    
    let text = vec![
        Spans::from("This is a line "),
        Spans::from(Span::styled(
            "This is a line   ",
            Style::default().fg(Color::Red),
        )),
        Spans::from(Span::styled(
            "This is a line",
            Style::default().bg(Color::Blue),
        )),
        Spans::from(Span::styled(
            "This is a longer line",
            Style::default().add_modifier(Modifier::CROSSED_OUT),
        )),
        Spans::from(Span::styled(&long_line, Style::default().bg(Color::Green))),
        Spans::from(Span::styled(
            "This is a line",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::ITALIC),
        )),
    ];

    let create_block = |title| {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::White).fg(Color::Black))
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            ))
    };

    let paragraph = Paragraph::new(text.clone())
        .style(Style::default().bg(Color::White).fg(Color::Black))
        .block(create_block("Left, no wrap"))
        .alignment(Alignment::Left);
    f.render_widget(paragraph, chunks[0]);

    let paragraph = Paragraph::new(text.clone())
        .style(Style::default().bg(Color::White).fg(Color::Black))
        .block(create_block("Left, wrap"))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[1]);

    let paragraph = Paragraph::new(text.clone())
        .style(Style::default().bg(Color::White).fg(Color::Black))
        .block(create_block("Center, wrap"))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[2]);

    let paragraph = Paragraph::new(text)
        .style(Style::default().bg(Color::White).fg(Color::Black))
        .block(create_block("Right, wrap"))
        .alignment(Alignment::Right)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[3]);
    
}
