use core::Wator;
use std::time::Duration;

use color_eyre::Result;
use ratatui::{DefaultTerminal, layout::Rect};
use ui::{AppState, helpers::handle_event, update, view};

mod core;
mod ui;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal: DefaultTerminal = ratatui::init();
    let Rect {
        x: _,
        y: _,
        width,
        height,
    } = terminal.get_frame().area();
    let mut wator = Wator::new(width, height);
    wator.initialize();
    while *wator.state() != AppState::Exit {
        terminal.draw(|frame| view(&mut wator, frame))?;
        if let Some(msg) = handle_event()? {
            update(&mut wator, msg);
        } else if wator.is_looping() {
            update(&mut wator, ui::Message::SimulationStep);
            std::thread::sleep(Duration::from_millis(wator.chronon()));
        }
    }
    ratatui::restore();
    Ok(())
}
