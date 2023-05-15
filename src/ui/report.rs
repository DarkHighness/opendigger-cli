use crossterm::event;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::{Axis, Block, BorderType, Borders, Chart, Dataset, GraphType, Paragraph},
    Frame,
};

pub use crate::report::RepoOverview;

use super::{
    util::{create_terminal, destory_terminal},
    UIError,
};

pub struct RepoOverviewUI {
    report: RepoOverview,
}

impl RepoOverviewUI {
    pub fn new(report: RepoOverview) -> Self {
        Self { report }
    }

    fn ui<B: Backend>(&mut self, frame: &mut Frame<B>) {
        let root_layout = frame.size();
        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .margin(0)
            .split(root_layout);

        let chart_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .margin(0)
            .split(main_layout[1]);

        let title = format!(" {}'s Report ", self.report.owner);

        let _block = Block::default()
            .title(title.as_str())
            .borders(Borders::ALL)
            .border_type(BorderType::Double);

        {
            let block = Block::default()
                .title(" Info ")
                .borders(Borders::ALL)
                .border_type(BorderType::Thick);

            frame.render_widget(block, main_layout[0]);

            let info_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(1), Constraint::Length(1)].as_ref())
                .margin(1)
                .split(main_layout[0]);

            let info = Paragraph::new(Spans::from(vec![
                Span::styled(
                    "Owner: ",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(self.report.owner.as_str()),
            ]));

            frame.render_widget(info, info_layout[0]);
        }

        if let Some(trend) = &self.report.star_trend {
            let data = trend
                .iter()
                .enumerate()
                .map(|(index, (_, value))| (index as f64, *value as f64))
                .collect::<Vec<_>>();

            let dataset = Dataset::default()
                // .name("Star Trend")
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Cyan))
                .graph_type(GraphType::Line)
                .data(&data);

            let chart = Chart::new(vec![dataset])
                .block(
                    Block::default()
                        .title(Span::styled(
                            " Star Trend ",
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        ))
                        .borders(Borders::ALL)
                        .border_type(BorderType::Thick),
                )
                .x_axis(
                    Axis::default()
                        .title("Time")
                        .labels(vec![
                            Span::raw(trend[0].0.as_str()),
                            Span::raw(trend[trend.len() / 2].0.as_str()),
                            Span::raw(trend[trend.len() - 1].0.as_str()),
                        ])
                        .style(Style::default().fg(Color::Gray))
                        .bounds([0.0, trend.len() as f64]),
                )
                .y_axis(
                    Axis::default()
                        .title("Stars")
                        .labels(vec![
                            Span::styled(
                                trend[0].1.to_string(),
                                Style::default().fg(Color::LightGreen),
                            ),
                            Span::styled(
                                trend[trend.len() / 2].1.to_string(),
                                Style::default().fg(Color::LightGreen),
                            ),
                            Span::styled(
                                trend[trend.len() - 1].1.to_string(),
                                Style::default().fg(Color::LightGreen),
                            ),
                        ])
                        .labels_alignment(Alignment::Right)
                        .style(Style::default().fg(Color::Gray))
                        .bounds([0.0, trend[trend.len() - 1].1 as f64]),
                );

            frame.render_widget(chart, chart_layout[0]);
        }
    }

    pub fn run(mut self) -> Result<(), UIError> {
        let mut terminal = create_terminal()?;

        loop {
            terminal.draw(|frame| self.ui(frame))?;

            if let event::Event::Key(key) = event::read()? {
                if let event::KeyCode::Char('q') = key.code {
                    break;
                }
            }
        }

        destory_terminal(terminal)?;

        Ok(())
    }
}
