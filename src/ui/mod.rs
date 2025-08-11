use std::time::Duration;

use crate::core::{Wator, entity::Entity};
use helpers::center;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Style, Stylize},
    symbols::Marker,
    widgets::{
        Block, Clear, Padding,
        canvas::{Canvas, Rectangle},
    },
};

pub mod helpers;

#[derive(Default, PartialEq)]
pub enum AppState {
    #[default]
    InWaTor,
    Exit,
}

pub enum Message {
    SimulationStep,
    SimulationLoop,
    ToggleLoop,
    AppExit,
}

pub fn view(model: &mut Wator, frame: &mut Frame) {
    if *model.state() == AppState::InWaTor {
        let layout = Layout::vertical([Constraint::Percentage(100), Constraint::Min(3)]);
        let [sim_area, details] = layout.areas(frame.area());
        let sim_rect = sim_area.inner(Margin::new(1, 1));
        let sim_block = Block::bordered()
            .padding(Padding::new(1, 1, 1, 1))
            .title("Wa-Tor World Simulation")
            .title_style(Style::new().bold());
        let canvas = Canvas::default()
            .block(sim_block)
            .marker(Marker::Block)
            .paint(|ctx| {
                let amount = (sim_rect.width * sim_rect.height) as usize;
                for (i, entity) in model.world().iter().take(amount).enumerate() {
                    let (x, y) = (i as u16 % sim_rect.width, i as u16 / sim_rect.width);
                    let color = match entity {
                        Entity::Fish { .. } => ratatui::style::Color::Green,
                        Entity::Shark { .. } => ratatui::style::Color::LightBlue,
                        Entity::Empty => continue,
                    };
                    ctx.draw(&Rectangle {
                        x: x.into(),
                        y: y.into(),
                        width: 1.0,
                        height: 1.0,
                        color,
                    });
                }
            })
            .x_bounds([sim_rect.x.into(), sim_rect.width.into()])
            .y_bounds([sim_rect.y.into(), sim_rect.height.into()]);

        let details_block = Block::bordered()
            .title("Details")
            .title_style(Style::new().bold());

        let Rect {
            x: _,
            y: _,
            width: max_w,
            height: max_h,
        } = sim_rect;

        let details_text = format!(
            "Chronon: {}ms | Fish: {} | Sharks: {} | Press 's' to step, 'l' for loop, 'q' or Esc to exit",
            model.chronon(),
            model.num_fish_in_area(max_w, max_h),
            model.num_sharks_in_area(max_w, max_h)
        );

        let details_area = details.inner(Margin::default());
        let centered_details = center(
            details_area,
            Constraint::Length(details_text.len() as u16),
            Constraint::Length(1),
        );
        frame.render_widget(Clear, sim_area);
        frame.render_widget(canvas, sim_area);
        frame.render_widget(details_block, details);
        frame.render_widget(details_text, centered_details);
    }
}

pub fn update(model: &mut Wator, msg: Message) -> Option<Message> {
    match msg {
        Message::AppExit => *model.state_mut() = AppState::Exit,
        Message::SimulationStep => {
            model.simulate();
        }
        Message::ToggleLoop => {
            model.toggle_loop();
            if model.is_looping() {
                return Some(Message::SimulationLoop);
            }
        }
        Message::SimulationLoop => {
            model.simulate();
            return Some(Message::SimulationLoop);
        }
    };
    None
}
