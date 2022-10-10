use std::{time::Duration, io::Error};

use crossterm::{execute, event::{poll, Event, DisableMouseCapture, DisableFocusChange, KeyCode, read}};
use tui::{backend::Backend, Frame, Terminal};
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Cell, Paragraph, Row, Table, Wrap};
use crate::process::Process;
use crate::scheduler::Scheduler;

struct UI {
    app_block: Block<'static>,
    main_layout: Layout,
    process_layout: Layout,
    process_block: Block<'static>,
    waiting_block: Block<'static>,
    executing_block: Block<'static>,
    blocked_block: Block<'static>,
    finished_block: Block<'static>,
}

impl UI {
    pub fn new() -> Self {
        let app_block = Block::default()
            .title("First Come First Served")
            .borders(Borders::ALL);

        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(80),
                ]
                    .as_ref(),
            );

        let process_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                ]
                    .as_ref(),
            );

        let process_block = Block::default()
            .title("Procesos")
            .borders(Borders::ALL);

        let waiting_block = Block::default()
            .title("Esperando")
            .borders(Borders::ALL);

        let executing_block = Block::default()
            .title("Ejecutando")
            .borders(Borders::ALL);

        let blocked_block = Block::default()
            .title("Bloqueado")
            .borders(Borders::ALL);

        let finished_block = Block::default()
            .title("Finalizado")
            .borders(Borders::ALL);

        Self {
            app_block,
            main_layout,
            process_layout,
            process_block,
            waiting_block,
            executing_block,
            blocked_block,
            finished_block,
        }
    }

    pub fn build_frame(self, f: &mut Frame<impl Backend>, scheduler: &Scheduler) {
        f.render_widget(self.app_block, f.size());
        let chunks = self.main_layout.split(f.size());
        let process_chunks = self.process_layout.split(chunks[1]);

        let pending_paragraphs = Paragraph::new(
            format!("Hay {} procesos pendientes\n Tiempo: {}",
                    scheduler.get_processes().len(),
                    scheduler.get_time()
            )
        )
            .block(self.process_block)
            .wrap(Wrap { trim: true });

        let waiting_processes = scheduler.get_waiting_processes();
        let mut waiting_text = String::new();
        for waiting_process in waiting_processes {
            let stats = waiting_process.get_stats();
            waiting_text += &*format!(
                "Proceso {}:\n\t - TME: {}\n\t - Tiempo en espera: {}\n",
                waiting_process.get_id(),
                stats.get_expected_time(),
                stats.get_waited_time()
            );
        }
        let waiting_paragraph = Paragraph::new(
            waiting_text
        )
            .block(self.waiting_block)
            .wrap(Wrap { trim: true });

        let executing_process = scheduler.get_executing_process();
        let executing_text = match executing_process {
            None => {
                "No hay procesos en ejecucion".to_string()
            }
            Some(process) => {
                let stats = process.get_stats();
                format!(
                    "Proceso {}:\n\t - Operacion: {}\n\t - TME: {}\n - Tiempo ejecutado: {}\n\t - Tiempo restante: {}\n",
                    process.get_id(),
                    process.get_operation(),
                    stats.get_expected_time(),
                    stats.get_executed_time(),
                    stats.get_remaining_time()
                )
            }
        };
        let executing_paragraph = Paragraph::new(executing_text)
            .block(self.executing_block)
            .wrap(Wrap { trim: true });

        let blocked_processes = scheduler.get_blocked_processes();
        let mut blocked_text = String::new();
        for blocked_process in blocked_processes {
            let stats = blocked_process.get_stats();
            blocked_text += &*format!(
                "Proceso {}:\n - Tiempo bloqueado: {}\n",
                blocked_process.get_id(),
                stats.get_blocked_time()
            );
        }
        let blocked_paragraph = Paragraph::new(blocked_text)
            .block(self.blocked_block)
            .wrap(Wrap { trim: true });

        let finished_processes = scheduler.get_finished_processes();
        let mut finished_text = String::new();
        for finished_process in finished_processes {
            let operation = finished_process.get_operation();
            finished_text += &*format!(
                "Proceso {}: {}\n",
                finished_process.get_id(),
                operation,
            );
        }
        let finished_paragraph = Paragraph::new(finished_text)
            .block(self.finished_block)
            .wrap(Wrap { trim: true });

        f.render_widget(pending_paragraphs, chunks[0]);
        f.render_widget(waiting_paragraph, process_chunks[0]);
        f.render_widget(executing_paragraph, process_chunks[1]);
        f.render_widget(blocked_paragraph, process_chunks[2]);
        f.render_widget(finished_paragraph, process_chunks[3]);
    }

    pub fn bcp_frame<B: Backend>(self, f: &mut Frame<B>, processes_info: Vec<Vec<String>>) {
        let rects = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .margin(5)
            .split(f.size());

        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let normal_style = Style::default().bg(Color::Blue);
        let header_cells = [
            "ID", "Estado", "Operacion", "LLegada", "Finalizacion",
            "Retorno", "Espera", "Servicio", "Restante", "Respuesta"
        ]
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
        let header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);
        let rows = processes_info.iter().map(|item| {
            let height = item
                .iter()
                .map(|content| content.chars().filter(|c| *c == '\n').count())
                .max()
                .unwrap_or(0)
                + 1;
            let cells = item.iter().map(|c| Cell::from(c.as_str()));
            Row::new(cells).height(height as u16).bottom_margin(1)
        });
        let t = Table::new(rows)
            .header(header)
            .block(Block::default().borders(Borders::ALL).title("Table"))
            .highlight_style(selected_style)
            .highlight_symbol(">> ")
            .widths(&[
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
            ]);
        f.render_widget(t, rects[0]);
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, num_processes: usize) -> Result<Vec<Process>, Error> {
    execute!(std::io::stdout(), DisableMouseCapture).unwrap();
    execute!(std::io::stdout(), DisableFocusChange).unwrap();
    let mut scheduler = Scheduler::new(Process::random_vector(num_processes));
    let mut pause = false;
    let mut bcp_mode = false;
    loop {
        let ui = UI::new();

        if poll(Duration::from_secs(1))? {
            if let Event::Key(key) = read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('p') => pause = true,
                    KeyCode::Char('c') => {
                        pause = false;
                        bcp_mode = false;
                    },
                    KeyCode::Char('b') => {
                        pause = true;
                        bcp_mode = true;
                    },
                    KeyCode::Char('n') => {
                        scheduler.add_new();
                    },
                    KeyCode::Char('w') => scheduler.error(),
                    KeyCode::Char('e') => scheduler.block(),
                    _ => {}
                }
            }
        }

        if !pause {
            scheduler.execute();
        }

        if !bcp_mode {
            terminal.draw(|f| ui.build_frame(f, &scheduler))?;
        } else {
            terminal.draw(|f| ui.bcp_frame(f, scheduler.get_bcp()))?;
        }
    }
    return Ok(scheduler.get_finished_processes().clone());
}
