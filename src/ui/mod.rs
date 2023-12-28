use std::{
    io,
    time::{Duration, Instant},
};

use async_std::task;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{prelude::*, widgets::*};
use ratatui::{
    prelude::{Backend, Constraint, Direction},
    style::{Color, Style},
    text::Line,
    widgets::ListItem,
    Frame, Terminal,
};

use self::db_explorer::{DBExplorer, DBObject};

pub mod db_explorer;
pub mod stateful_list;

pub struct App {
    pub db_explorer: DBExplorer,
}

impl App {
    pub fn new() -> App {
        App {
            db_explorer: db_explorer::new(),
        }
    }

    fn on_tick(&mut self) {}
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Left | KeyCode::Char('h') => {
                            app.db_explorer.items_visible.unselect()
                        }
                        KeyCode::Down | KeyCode::Char('j') => app.db_explorer.items_visible.next(),
                        KeyCode::Up | KeyCode::Char('k') => {
                            app.db_explorer.items_visible.previous()
                        }
                        KeyCode::Enter => {
                            app.db_explorer.show_system_objs = !app.db_explorer.show_system_objs
                        }
                        _ => {}
                    }
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let tables = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(50)])
        .split(f.size());

    app.db_explorer.update_visible_items();

    let items: Vec<ListItem> = app
        .db_explorer
        .items_visible
        .items
        .iter()
        .map(|i| match i {
            DBObject::Table(table) => {
                let lines = vec![Line::from(format!(
                    "{}.{}",
                    table.schema.clone(),
                    table.name.clone()
                ))];
                ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
            }
        })
        .collect();

    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        );
    let state = &mut app.db_explorer.items_visible.state;
    // We can now render the item list
    f.render_stateful_widget(items, tables[0], state);
}
