use crate::logic::action::Action;
use crate::logic::operation::Operation;
use eframe::egui;

pub struct KeyboardHandler;

impl KeyboardHandler {
    pub fn poll(ctx: &egui::Context) -> Vec<Action> {
        let events = ctx.input(|i| i.events.clone());
        let mut actions = Vec::new();

        for ev in events {
            match ev {
                egui::Event::Text(t) => {
                    for ch in t.chars() {
                        match ch {
                            '0'..='9' => actions.push(Action::Digit(ch)),
                            '.' | ',' => actions.push(Action::Decimal),
                            '+' => actions.push(Action::Operator(Operation::Add)),
                            '-' => actions.push(Action::Operator(Operation::Sub)),
                            '*' => actions.push(Action::Operator(Operation::Mul)),
                            '/' => actions.push(Action::Operator(Operation::Div)),
                            '%' => actions.push(Action::Percent),
                            '=' => actions.push(Action::Equals),
                            _ => {}
                        }
                    }
                }
                egui::Event::Key {
                    key, pressed: true, .. 
                } => match key {
                    egui::Key::Enter => actions.push(Action::Equals),
                    egui::Key::Backspace => actions.push(Action::Backspace),
                    egui::Key::Escape => actions.push(Action::ClearAll),
                    egui::Key::Delete => actions.push(Action::ClearEntry),
                    _ => {}
                },
                _ => {}
            }
        }
        actions
    }
}
