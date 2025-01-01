use std::rc::Rc;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::{Backend, Terminal},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
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
                Screen::Menu => View::draw_menu_scene(frame, area, state),
                Screen::CountDown => View::draw_countdown_scene(frame, area, state),
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

        (outer_layout, inner_layout)
    }

    fn draw_main_scene(frame: &mut Frame, area: Rect, state: &State) {
        let chunks = View::get_chunks(area);
        let outer_layout = chunks.0;
        let inner_layout = chunks.1;

        let message = state.get_paragraph();
        let message_length = message.len();
        let position = state.get_position() as usize;

        let lines = Line::from(vec![
            Span::styled(&message[0..position], Style::default().fg(Color::Green)),
            Span::styled(
                &message[position..message_length],
                Style::default().fg(Color::Rgb(21, 21, 21)),
            ),
        ]);
        let title = Line::from(" typefast ");
        let widget = Paragraph::new(lines)
            .alignment(Alignment::Left)
            .block(
                Block::bordered()
                    .title(title.centered())
                    .padding(Padding::new(3, 3, 1, 1)),
            )
            .wrap(Wrap { trim: true });

        frame.render_widget(widget, outer_layout[0]);
        View::draw_legend(
            frame,
            format!("esc: Exit, timer: {}s", state.get_timer()).as_str(),
            inner_layout[0],
        );
        View::draw_error(frame, state, inner_layout[1]);
    }

    fn draw_menu_scene(frame: &mut Frame, area: Rect, state: &State) {
        let chunks = View::get_chunks(area);
        let outer_layout = chunks.0;
        let inner_layout = chunks.1;

        let title = Line::from(" typefast ");
        let widget = Paragraph::new("Menu")
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title(title.centered())
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Rgb(21, 21, 21))),
            )
            .wrap(Wrap { trim: true });

        frame.render_widget(widget, outer_layout[0]);
        View::draw_legend(frame, "Press enter to start", inner_layout[0]);
        View::draw_error(frame, state, inner_layout[1]);
    }

    fn draw_countdown_scene(frame: &mut Frame, area: Rect, state: &State) {
        let chunks = View::get_chunks(area);
        let outer_layout = chunks.0;
        let inner_layout = chunks.1;

        let title = Line::from(" typefast ");
        let widget = Paragraph::new("Get ready!")
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title(title.centered())
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Rgb(21, 21, 21))),
            )
            .wrap(Wrap { trim: true });

        frame.render_widget(widget, outer_layout[0]);
        View::draw_legend(
            frame,
            format!("Ready! {}", state.get_timer()).as_str(),
            inner_layout[0],
        );
        View::draw_error(frame, state, inner_layout[1]);
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