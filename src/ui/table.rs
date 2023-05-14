use crossterm::event::{self};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
    Frame,
};

use crate::ui::render_table;

use super::{
    util::{create_terminal, destory_terminal},
    UIError, UIMode,
};

pub enum TableUI {
    Simple(SimpleTableUI),
    Interactive(InteractiveTableUI),
}

pub struct SimpleTableUI {
    name: String,
    header: Vec<String>,
    rows: Vec<Vec<String>>,
}

pub struct InteractiveTableUI {
    name: String,
    header: Vec<String>,
    rows: Vec<Vec<String>>,
    state: TableState,
}

impl TableUI {
    pub fn new(
        mode: UIMode,
        name: String,
        header: Vec<String>,
        rows: Vec<Vec<String>>,
    ) -> Result<Self, UIError> {
        let ui = match mode {
            UIMode::Simple => TableUI::Simple(SimpleTableUI::new(name, header, rows)?),
            UIMode::Interactive => {
                TableUI::Interactive(InteractiveTableUI::new(name, header, rows)?)
            }
        };

        Ok(ui)
    }

    pub fn run(self) -> Result<(), UIError> {
        match self {
            TableUI::Simple(ui) => ui.run(),
            TableUI::Interactive(ui) => ui.run(),
        }
    }
}

impl SimpleTableUI {
    pub fn new(name: String, header: Vec<String>, rows: Vec<Vec<String>>) -> Result<Self, UIError> {
        Ok(SimpleTableUI { name, header, rows })
    }

    pub fn run(self) -> Result<(), UIError> {
        let output = render_table(&self.header, &self.rows);

        println!("{}", output);

        Ok(())
    }
}

impl InteractiveTableUI {
    pub fn new(name: String, header: Vec<String>, rows: Vec<Vec<String>>) -> Result<Self, UIError> {
        let state = TableState::default();

        Ok(InteractiveTableUI {
            name,
            header,
            rows,
            state,
        })
    }

    fn select_next(&mut self) {
        let next = match self.state.selected() {
            Some(selected) => {
                if selected >= self.rows.len() - 1 {
                    0
                } else {
                    selected + 1
                }
            }
            None => 0,
        };

        self.state.select(Some(next));
    }

    fn select_previous(&mut self) {
        let previous = match self.state.selected() {
            Some(selected) => {
                if selected == 0 {
                    self.rows.len() - 1
                } else {
                    selected - 1
                }
            }
            None => 0,
        };

        self.state.select(Some(previous));
    }

    fn ui<B: Backend>(&mut self, frame: &mut Frame<B>) {
        let rects = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(99), Constraint::Length(1)].as_ref())
            .margin(1)
            .split(frame.size());

        let selected_style = Style::default().fg(Color::Green);
        let normal_style = Style::default().fg(Color::White);
        let header_style = Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD);

        let header_cells = self
            .header
            .iter()
            .map(|header| Cell::from(header.as_str()))
            .collect::<Vec<_>>();

        let header = Row::new(header_cells)
            .style(header_style)
            .height(1)
            .bottom_margin(1);

        let rows = self
            .rows
            .iter()
            .map(|row| {
                let height = row
                    .iter()
                    .map(|value| value.chars().filter(|c| *c == '\n').count())
                    .max()
                    .unwrap_or(0) as u16
                    + 1;

                let cells = row
                    .iter()
                    .map(|value| Cell::from(value.as_str()))
                    .collect::<Vec<_>>();

                Row::new(cells).height(height).bottom_margin(1)
            })
            .collect::<Vec<_>>();

        let width_percentages = self
            .header
            .iter()
            .map(|_| Constraint::Percentage(100 / self.header.len() as u16))
            .collect::<Vec<_>>();

        let table = Table::new(rows)
            .header(header)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(self.name.as_str()),
            )
            .widths(&width_percentages)
            .highlight_style(selected_style)
            .highlight_symbol("> ");

        frame.render_stateful_widget(table, rects[0], &mut self.state);

        let indicator_text = Text::styled(
            format!(
                "{} / {}",
                self.state.selected().unwrap_or(0) + 1,
                self.rows.len()
            ),
            normal_style,
        );

        let indicator = Paragraph::new(indicator_text).alignment(tui::layout::Alignment::Right);

        let helper_text = Text::styled(
            "Press <Up> or <Down> to navigate, <q> to quit",
            normal_style,
        );

        let helper = Paragraph::new(helper_text).alignment(tui::layout::Alignment::Left);

        let bottom_line_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(30)].as_ref())
            .split(rects[1]);

        frame.render_widget(helper, bottom_line_layout[0]);
        frame.render_widget(indicator, bottom_line_layout[1]);
    }

    pub fn run(mut self) -> Result<(), UIError> {
        let mut terminal = create_terminal()?;

        // Select default
        self.select_next();

        loop {
            terminal.draw(|frame| self.ui(frame))?;

            match event::read()? {
                event::Event::Key(key) => match key.code {
                    event::KeyCode::Char('q') => break,
                    event::KeyCode::Down => self.select_next(),
                    event::KeyCode::Up => self.select_previous(),
                    _ => {}
                },
                _ => {}
            }
        }

        destory_terminal(terminal)?;

        Ok(())
    }
}
