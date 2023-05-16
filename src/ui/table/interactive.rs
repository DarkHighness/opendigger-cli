use std::collections::{BTreeMap, BTreeSet, HashMap};

use crossterm::event;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Text},
    widgets::{
        Axis, Block, Borders, Cell, Chart, Dataset, Gauge, GraphType, Paragraph, Row, Table,
        TableState,
    },
    Frame,
};

use crate::{
    engine::Value,
    ui::{
        format_human_readable_f64,
        util::{create_terminal, destory_terminal},
        UIError, UI,
    },
};

enum TableUIMode {
    Simple,
    Enhanced,
}

pub struct InteractiveTableUI {
    name: String,
    header: Vec<String>,
    rows: Vec<Vec<Value>>,
    state: TableState,
    mode: TableUIMode,
    chart_visible: bool,
    chart_trending: bool,
}

impl InteractiveTableUI {
    fn is_standard_form(header: &[String], rows: &[Vec<Value>]) -> bool {
        if header.len() != 3 {
            return false;
        }

        let header_set = header
            .iter()
            .map(String::as_str)
            .collect::<std::collections::HashSet<_>>();
        let standard_header_set = vec!["name", "month", "value"]
            .into_iter()
            .collect::<std::collections::HashSet<_>>();

        if header_set != standard_header_set {
            return false;
        }

        let value_index = header.iter().position(|e| e == "value").unwrap();

        let value_flag = rows
            .iter()
            .all(|row| matches!(row[value_index], Value::Float(_) | Value::Int(_)));

        value_flag
    }

    pub fn new(name: String, header: Vec<String>, rows: Vec<Vec<Value>>) -> Result<Self, UIError> {
        let state = TableState::default();
        let mode = if Self::is_standard_form(&header, &rows) {
            TableUIMode::Enhanced
        } else {
            TableUIMode::Simple
        };

        Ok(InteractiveTableUI {
            name,
            header,
            rows,
            state,
            mode,
            chart_visible: true,
            chart_trending: false,
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
        let dataset = if matches!(self.mode, TableUIMode::Enhanced) {
            let mut dataset = BTreeMap::new();

            let name_index = self.header.iter().position(|e| e == "name").unwrap();
            let month_index = self.header.iter().position(|e| e == "month").unwrap();
            let value_index = self.header.iter().position(|e| e == "value").unwrap();

            let mut sum = HashMap::new();

            for row in &self.rows {
                let name = row[name_index].to_string();
                let month = row[month_index].to_string();

                let mut value = row[value_index].clone();

                if self.chart_trending {
                    let psum = sum.entry(name.clone()).or_default();

                    *psum += TryInto::<f64>::try_into(value).unwrap();

                    value = Value::Float(*psum);
                }

                dataset
                    .entry(name)
                    .or_insert_with(Vec::new)
                    .push((month, value));
            }

            for (_, values) in dataset.iter_mut() {
                values.sort_by(|(a, _), (b, _)| a.cmp(b));
            }

            Some(dataset)
        } else {
            None
        };

        let month_range = if matches!(self.mode, TableUIMode::Enhanced) {
            let tree: BTreeSet<&str> = dataset
                .as_ref()
                .unwrap()
                .iter()
                .flat_map(|(_, values)| values.iter().map(|(month, _)| month.as_str()))
                .fold(BTreeSet::new(), |mut acc, e| {
                    acc.insert(e);
                    acc
                });

            Some((
                tree.iter().next().unwrap().to_string(),
                tree.iter().last().unwrap().to_string(),
            ))
        } else {
            None
        };

        let value_range = if matches!(self.mode, TableUIMode::Enhanced) {
            let tree: BTreeSet<i64> = dataset
                .as_ref()
                .unwrap()
                .iter()
                .flat_map(|(_, values)| {
                    values
                        .iter()
                        .map(|(_, value)| TryInto::<f64>::try_into(value.clone()).unwrap() as i64)
                })
                .fold(BTreeSet::new(), |mut acc, e: i64| {
                    acc.insert(e);
                    acc
                });

            Some((
                tree.iter().next().cloned().unwrap() as f64,
                tree.iter().last().cloned().unwrap() as f64,
            ))
        } else {
            None
        };

        let (_root_width, root_height) = (frame.size().width, frame.size().height);

        let root_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Max(root_height - 1), Constraint::Length(1)].as_ref())
            .margin(1)
            .split(frame.size());

        let (table_rect, chart_rect) =
            if matches!(self.mode, TableUIMode::Enhanced) && self.chart_visible {
                let rects = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .margin(0)
                    .split(root_layout[0]);

                (rects[0], rects[1])
            } else {
                let rects = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(100), Constraint::Max(0)].as_ref())
                    .margin(0)
                    .split(root_layout[0]);

                (rects[0], rects[1])
            };

        let selected_style = Style::default().fg(Color::Green);
        let normal_style = Style::default().fg(Color::White);
        let emphasize_style = Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD);

        {
            let block = Block::default().borders(Borders::ALL).title("Table");

            frame.render_widget(block, table_rect);

            let table_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Max(table_rect.height - 2), Constraint::Max(1)].as_ref())
                .margin(1)
                .split(table_rect);

            let header_cells = self
                .header
                .iter()
                .map(|header| Cell::from(header.as_str()))
                .collect::<Vec<_>>();

            let header = Row::new(header_cells)
                .style(emphasize_style)
                .height(1)
                .bottom_margin(1);

            let rows = self
                .rows
                .iter()
                .map(|row| {
                    let height = row
                        .iter()
                        .map(|value| value.to_string().chars().filter(|c| *c == '\n').count())
                        .max()
                        .unwrap_or(0) as u16
                        + 1;

                    let cells = row
                        .iter()
                        .map(|value| Cell::from(value.to_string()))
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
                        // .borders(Borders::ALL)
                        .title(self.name.as_str()),
                )
                .widths(&width_percentages)
                .highlight_style(selected_style)
                .highlight_symbol("> ");

            frame.render_stateful_widget(table, table_layout[0], &mut self.state);

            let offset = self.state.selected().unwrap_or(0);
            let len = self.rows.len();
            let gauge_label = format!("{}/{}", offset + 1, len);
            let gauge_percent = ((offset as f64 / len as f64) * 100.0) as u16;

            let gauge = Gauge::default()
                .block(Block::default().borders(Borders::empty()))
                .gauge_style(Style::default().fg(Color::LightGreen))
                .label(gauge_label)
                .percent(gauge_percent)
                .use_unicode(true);

            frame.render_widget(gauge, table_layout[1]);
        }

        {
            if matches!(self.mode, TableUIMode::Enhanced) && self.chart_visible {
                let colors = [
                    Color::Green,
                    Color::Yellow,
                    Color::Blue,
                    Color::Magenta,
                    Color::Cyan,
                ];

                let values = dataset
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|(_, value)| {
                        value
                            .iter()
                            .map(|(month, value)| {
                                let mut iter = month.split('-');
                                let year = iter.next().unwrap().parse::<f64>().unwrap();
                                let month = iter.next().unwrap().parse::<f64>().unwrap();

                                let x_value = year * 12.0f64 + month;
                                let y_value = value.clone().try_into().unwrap();

                                (x_value, y_value)
                            })
                            .collect::<Vec<(f64, f64)>>()
                    })
                    .collect::<Vec<_>>();

                let datasets = dataset
                    .as_ref()
                    .unwrap()
                    .iter()
                    .enumerate()
                    .map(|(index, (name, _))| {
                        Dataset::default()
                            .name(name)
                            .marker(tui::symbols::Marker::Braille)
                            .style(
                                Style::default()
                                    .fg(colors[index % colors.len()])
                                    .add_modifier(Modifier::SLOW_BLINK),
                            )
                            .graph_type(GraphType::Line)
                            .data(&values[index])
                    })
                    .collect::<Vec<_>>();

                let x_bounds = month_range
                    .as_ref()
                    .map(|(min, max)| {
                        let mut iter = min.split('-');
                        let min_year = iter.next().unwrap().parse::<f64>().unwrap();
                        let min_month = iter.next().unwrap().parse::<f64>().unwrap();

                        let mut iter = max.split('-');
                        let max_year = iter.next().unwrap().parse::<f64>().unwrap();
                        let max_month = iter.next().unwrap().parse::<f64>().unwrap();

                        let min = min_year * 12.0f64 + min_month;
                        let max = max_year * 12.0f64 + max_month;

                        [min, max]
                    })
                    .unwrap();

                let x_labels = month_range
                    .as_ref()
                    .map(|(min, max)| {
                        vec![
                            Span::styled(
                                min,
                                Style::default()
                                    .add_modifier(Modifier::BOLD)
                                    .fg(Color::White),
                            ),
                            Span::styled(
                                max,
                                Style::default()
                                    .add_modifier(Modifier::BOLD)
                                    .fg(Color::White),
                            ),
                        ]
                    })
                    .unwrap();

                let y_bounds = value_range.as_ref().map(|(min, max)| [*min, *max]).unwrap();

                let y_labels = value_range
                    .as_ref()
                    .map(|(min, max)| {
                        let min = format_human_readable_f64(*min);
                        let max = format_human_readable_f64(*max);

                        vec![
                            Span::styled(
                                min,
                                Style::default()
                                    .add_modifier(Modifier::BOLD)
                                    .fg(Color::White),
                            ),
                            Span::styled(
                                max,
                                Style::default()
                                    .add_modifier(Modifier::BOLD)
                                    .fg(Color::White),
                            ),
                        ]
                    })
                    .unwrap();

                let title = if self.chart_trending {
                    " Trending "
                } else {
                    " Value "
                };

                let chart = Chart::new(datasets)
                    .block(Block::default().borders(Borders::ALL).title(title))
                    .x_axis(
                        Axis::default()
                            .title("Month")
                            .style(emphasize_style)
                            .bounds(x_bounds)
                            .labels(x_labels),
                    )
                    .y_axis(
                        Axis::default()
                            .title("Value")
                            .style(emphasize_style)
                            .bounds(y_bounds)
                            .labels(y_labels),
                    )
                    .hidden_legend_constraints((
                        Constraint::Percentage(100),
                        Constraint::Percentage(100),
                    ));

                frame.render_widget(chart, chart_rect)
            }
        }

        {
            let helper_text = if matches!(self.mode, TableUIMode::Enhanced) {
                Text::styled(
                    "Press <Up> or <Down> to navigate, <t> to toggle chart, <u> to toggle chart type, <q> to quit",
                    normal_style,
                )
            } else {
                Text::styled(
                    "Press <Up> or <Down> to navigate, <q> to quit",
                    normal_style,
                )
            };

            let helper_widget = Paragraph::new(helper_text).alignment(tui::layout::Alignment::Left);

            frame.render_widget(helper_widget, root_layout[1]);
        }
    }
}

impl UI for InteractiveTableUI {
    fn render(mut self) -> Result<(), UIError> {
        let mut terminal = create_terminal()?;

        // Select default
        self.select_next();

        loop {
            terminal.draw(|frame| self.ui(frame))?;

            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    event::KeyCode::Char('q') => break,
                    event::KeyCode::Char('t') => {
                        self.chart_visible = !self.chart_visible;
                    }
                    event::KeyCode::Char('u') => {
                        self.chart_trending = !self.chart_trending;
                    }
                    event::KeyCode::Down => self.select_next(),
                    event::KeyCode::Up => self.select_previous(),
                    _ => {}
                }
            }
        }

        destory_terminal(terminal)?;

        Ok(())
    }
}
