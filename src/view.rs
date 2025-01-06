use std::{io::Error as StandardError, rc::Rc};

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::{Backend, Terminal},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Padding, Paragraph, Wrap},
    Frame,
};
use thiserror::Error;

use crate::constants::{Page, MENU_ITEMS};
use crate::state::State;

#[derive(Debug, Default)]
pub struct View {}

#[derive(Error, Debug)]
pub enum ViewError {
    #[error("Could not draw view: {0} because of {1}")]
    DrawError(Page, StandardError),
}

impl View {
    pub fn draw<B: Backend>(terminal: &mut Terminal<B>, state: &State) -> Result<(), ViewError> {
        terminal
            .draw(|frame| {
                let area = frame.size();

                match state.page {
                    Page::Game => View::draw_main_page(frame, area, state),
                    Page::Menu => View::draw_menu_page(frame, area, state),
                    Page::CountDown => View::draw_countdown_page(frame, area, state),
                    Page::Records => View::draw_records_page(frame, area, state),
                    Page::GameResult => View::draw_game_result_page(frame, area, state),
                }
            })
            .map_err(|e| ViewError::DrawError(state.page, e))
            .ok();
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

    fn draw_main_page(frame: &mut Frame, area: Rect, state: &State) {
        let chunks = View::get_chunks(area);
        let outer_layout = chunks.0;
        let inner_layout = chunks.1;

        let message = state.get_paragraph();
        let message_length = message.len();
        let position = state.get_position() as usize;
        let timer = state.get_timer();
        let word_speed = state.get_word_speed();
        let char_speed = state.get_char_speed();

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
            format!(
                "esc: Exit, timer: {}s wpm: {}, cpm: {}",
                timer, word_speed, char_speed
            )
            .as_str(),
            inner_layout[0],
        );
        View::draw_error(frame, state, inner_layout[1]);
    }

    fn draw_menu_page(frame: &mut Frame, area: Rect, state: &State) {
        let chunks = View::get_chunks(area);
        let outer_layout = chunks.0;
        let inner_layout = chunks.1;
        let menu_index = state.get_menu_index();

        let title = Line::from(" typefast ");
        let list = List::new(
            MENU_ITEMS
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let indicator = if i == menu_index as usize { "•" } else { " " };
                    ListItem::new(format!("{} {}", indicator, item))
                })
                .collect::<Vec<ListItem>>(),
        )
        .block(
            Block::bordered()
                .title(title)
                .padding(Padding::new(3, 3, 1, 1)),
        );

        frame.render_widget(list, outer_layout[0]);
        View::draw_legend(
            frame,
            "esc: Exit, enter: Select, ↑: Up, ↓: Down",
            inner_layout[0],
        );
        View::draw_error(frame, state, inner_layout[1]);
    }

    fn draw_countdown_page(frame: &mut Frame, area: Rect, state: &State) {
        let chunks = View::get_chunks(area);
        let outer_layout = chunks.0;
        let inner_layout = chunks.1;

        let title = Line::from(" typefast ");
        let widget = Paragraph::new(format!("Get ready! {}s", state.get_timer()))
            .alignment(Alignment::Center)
            .block(
                Block::bordered()
                    .title(title)
                    .padding(Padding::new(3, 3, 1, 1)),
            )
            .wrap(Wrap { trim: true });

        frame.render_widget(widget, outer_layout[0]);

        View::draw_error(frame, state, inner_layout[1]);
    }

    fn draw_records_page(frame: &mut Frame, area: Rect, state: &State) {
        let chunks = View::get_chunks(area);
        let outer_layout = chunks.0;
        let inner_layout = chunks.1;

        let title = Line::from(" typefast ");
        let records = state.get_records();
        let list = List::new(if records.is_empty() {
            vec![ListItem::new("No records yet")]
        } else {
            records
                .iter()
                .map(|record| {
                    ListItem::new(format!(
                        "{} - {}wpm {}cpm",
                        record.date, record.wpm, record.cpm
                    ))
                })
                .collect::<Vec<ListItem>>()
        })
        .block(
            Block::bordered()
                .title(title)
                .padding(Padding::new(3, 3, 1, 1)),
        );

        frame.render_widget(list, outer_layout[0]);
        View::draw_legend(frame, "esc: Exit", inner_layout[0]);
        View::draw_error(frame, state, inner_layout[1]);
    }

    fn draw_game_result_page(frame: &mut Frame, area: Rect, state: &State) {
        let chunks = View::get_chunks(area);
        let outer_layout = chunks.0;
        let inner_layout = chunks.1;

        let title = Line::from(" typefast ");
        let widget = Paragraph::new(format!(
            "Congrats! You typed {} words and {} characters in 60s! Would you like to save this record?",
            state.get_word_count(),
            state.get_char_count(),
        ))
        .alignment(Alignment::Center)
        .block(
            Block::bordered()
                .title(title)
                .padding(Padding::new(3, 3, 1, 1)),
        )
        .wrap(Wrap { trim: true });

        frame.render_widget(widget, outer_layout[0]);
        View::draw_legend(frame, "esc: Exit, enter: Save", inner_layout[0]);
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
#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;

    #[test]
    fn test_draw_game_page() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let state = State {
            page: Page::Game,
            ..State::default()
        };

        let result = View::draw(&mut terminal, &state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_draw_menu_page() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let state = State {
            page: Page::Menu,
            ..State::default()
        };

        let result = View::draw(&mut terminal, &state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_draw_countdown_page() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let state = State {
            page: Page::CountDown,
            ..State::default()
        };

        let result = View::draw(&mut terminal, &state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_draw_records_page() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let state = State {
            page: Page::Records,
            ..State::default()
        };

        let result = View::draw(&mut terminal, &state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_draw_game_result_page() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let state = State {
            page: Page::GameResult,
            ..State::default()
        };

        let result = View::draw(&mut terminal, &state);
        assert!(result.is_ok());
    }
}
