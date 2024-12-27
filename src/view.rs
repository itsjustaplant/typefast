use std::rc::Rc;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::{Backend, Terminal},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::constants::Screen;
use crate::state::State;

#[derive(Debug, Default)]
pub struct View {}

impl View {
    pub fn draw<B: Backend>(
        terminal: &mut Terminal<B>,
        state: &State,
    ) -> Result<(), Box<dyn std::error::Error>> {
        terminal.draw(|frame| {
            let area = frame.size();

            match state.screen {
                Screen::Main => View::draw_main_scene(frame, area, state),
            }
        })?;
        Ok(())
    }

    fn get_chunks(area: Rect) -> (Rc<[Rect]>, Rc<[Rect]>) {
        let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
            .split(area);

        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(outer_layout[1]);

        return (outer_layout, inner_layout);
    }

    fn draw_main_scene(frame: &mut Frame, area: Rect, state: &State) {
        let chunks = View::get_chunks(area);
        let outer_layout = chunks.0;
        let inner_layout = chunks.1;
        let message = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Morbi eu nulla et est sollicitudin iaculis. Nam pharetra eros nulla, lobortis accumsan sem laoreet finibus. Maecenas id convallis mauris, quis ultrices erat. Maecenas imperdiet maximus sapien in sodales. In posuere in lectus vel egestas. Praesent vestibulum convallis interdum. Vestibulum pellentesque libero non felis gravida, eu mollis orci cursus.";
        let counter = state.get_counter() as usize;
        let message_length = message.len();
        let line = Line::from(vec![
            Span::styled(
                &message[0..counter as usize],
                Style::default().fg(Color::Red),
            ),
            Span::styled(
                &message[counter..message_length],
                Style::default().fg(Color::Yellow),
            ),
        ]);
        let title = Line::from(" typefast ");
        let widget = Paragraph::new(line)
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title.centered()),
            )
            .wrap(Wrap { trim: true });

        frame.render_widget(widget, outer_layout[0]);
        View::draw_legend(
            frame,
            "esc: Exit, enter: Select, ↑: Up, ↓: Down",
            inner_layout[0],
        );
        View::draw_error(frame, &state, inner_layout[1]);
    }

    fn draw_legend(frame: &mut Frame, text: &str, area: Rect) {
        let widget = Paragraph::new(text)
            .alignment(Alignment::Left)
            .block(Block::default().borders(Borders::NONE));

        frame.render_widget(widget, area);
    }

    fn draw_error(frame: &mut Frame, state: &State, area: Rect) {
        let widget = Paragraph::new(state.get_error().as_str())
            .alignment(Alignment::Left)
            .block(Block::default().borders(Borders::NONE));

        frame.render_widget(widget, area);
    }
}
