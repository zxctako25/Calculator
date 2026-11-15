use super::theme::Theme;
use eframe::egui;

pub struct DisplayPanel;

impl DisplayPanel {
    pub fn show(ctx: &egui::Context, theme: &Theme, history: &str, text: &str) {
        egui::TopBottomPanel::top("display_panel")
            .show_separator_line(false)
            .frame(
                egui::Frame::none()
                    .fill(theme.bg)
                    .inner_margin(egui::Margin::symmetric(theme.margin + 8.0, theme.margin)),
            )
            .exact_height(130.0)
            .show(ctx, |ui| {
                let len = text.chars().count().max(1) as f32;
                let size = (ui.available_width() / (len * 0.6)).clamp(14.0, 48.0);

                ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                    ui.add_space(8.0);
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(history).size(16.0).color(theme.history_text),
                        )
                            .selectable(false),
                    );
                    ui.add_space(4.0);
                    ui.add(
                        egui::Label::new(egui::RichText::new(text).size(size).color(theme.text))
                            .selectable(false),
                    );
                });
            });
    }
}
