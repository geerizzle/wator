use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};
use ratatui::layout::{Constraint, Flex, Layout, Rect};

use super::Message;

pub fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}

pub fn handle_event() -> color_eyre::Result<Option<Message>> {
    if event::poll(Duration::from_millis(1))? {
        match event::read()? {
            Event::Key(k) => match k.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    return Ok(Some(Message::AppExit));
                }
                KeyCode::Char('s') => {
                    return Ok(Some(Message::SimulationStep));
                }
                KeyCode::Char('l') => {
                    return Ok(Some(Message::ToggleLoop));
                }
                _ => return Ok(None),
            },
            _ => return Ok(None),
        };
    }
    Ok(None)
}
