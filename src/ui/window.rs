use super::display::DisplayPanel;
use super::keyboard::KeyboardHandler;
use super::keypad::Keypad;
use super::theme::Theme;
use crate::logic::state::CalculatorState;
use eframe::egui;

pub struct CalculatorWindow {
    state: CalculatorState,
    theme: Theme,
}

impl CalculatorWindow {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let theme = Theme::default();
        theme.apply(&cc.egui_ctx);
        Self {
            state: CalculatorState::new(), theme
        }
    }
}

impl eframe::App for CalculatorWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        for action in KeyboardHandler::poll(ctx) {
            self.state.handle(action);
        }

        let display = self.state.display_text();
        DisplayPanel::show(ctx, &self.theme, self.state.history_text(), &display);

        let m = self.theme.margin;
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(self.theme.bg).inner_margin(egui::Margin {
                left: m,
                right: m,
                top: 0.0,
                bottom: m,
            }))
            .show(ctx, |ui| {
                if let Some(action) = Keypad::show(ui, &self.theme) {
                    self.state.handle(action);
                }
            });
    }
}
