use crate::{
    event::{Event, Events},
    workspace::Workspace,
};
use chrono::Local;
use convert_case::{Case, Casing};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Terminal,
};

impl<'r> From<&Workspace> for Row<'r> {
    fn from(workspace: &Workspace) -> Row<'r> {
        Row::new(vec![
            workspace.story().clone(),
            workspace
                .modified()
                .with_timezone(&Local)
                .format("%b %d %Y %I:%M:%S %P")
                .to_string(),
            format!("{}", workspace.status()),
            workspace.description().clone().to_case(Case::Title),
        ])
    }
}

pub struct StatefulTable {
    row_state: TableState,
    items: Vec<Workspace>,
}

impl StatefulTable {
    fn new(workspaces: Vec<Workspace>) -> StatefulTable {
        let mut workspaces = workspaces;
        workspaces.sort_by(|a, b| b.modified().cmp(&a.modified()));
        StatefulTable {
            row_state: TableState::default(),
            items: workspaces,
        }
    }

    pub fn current(&mut self) -> Workspace {
        let i = match self.row_state.selected() {
            Some(i) => i,
            None => 0,
        };
        self.items.remove(i)
    }

    pub fn next(&mut self) {
        let i = match self.row_state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.row_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.row_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.row_state.select(Some(i));
    }
}

pub fn run(workspaces: Vec<Workspace>) -> Result<Option<Workspace>, Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    let mut table = StatefulTable::new(workspaces);

    // Input
    loop {
        terminal.draw(|f| {
            let rects = Layout::default()
                .constraints([Constraint::Percentage(100)].as_ref())
                .margin(5)
                .split(f.size());

            let selected_style = Style::default().add_modifier(Modifier::REVERSED);
            let normal_style = Style::default().bg(Color::Blue);
            let header_cells = ["Story", "Last Modified", "Status", "Description"]
                .iter()
                .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
            let header = Row::new(header_cells)
                .style(normal_style)
                .height(1)
                .bottom_margin(1);
            let rows = table.items.iter().map(Into::into);
            let t = Table::new(rows)
                .header(header)
                .block(Block::default().borders(Borders::ALL).title("Workspaces"))
                .highlight_style(selected_style)
                .widths(&[
                    Constraint::Percentage(10),
                    Constraint::Percentage(30),
                    Constraint::Percentage(10),
                    Constraint::Percentage(50),
                ]);
            f.render_stateful_widget(t, rects[0], &mut table.row_state);
        })?;

        if let Event::Input(key) = events.next()? {
            match key {
                Key::Char('q') => {
                    return Ok(None);
                }
                Key::Down => {
                    table.next();
                }
                Key::Up => {
                    table.previous();
                }
                Key::Char('\n') => {
                    return Ok(Some(table.current()));
                }
                _ => {}
            }
        };
    }
}
