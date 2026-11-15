use eframe::egui::{
    self, Color32, Stroke
};

pub struct Theme {
    pub bg: Color32,
    pub text: Color32,
    pub history_text: Color32,
    pub key_digit: Color32,
    pub key_function: Color32,
    pub key_equals: Color32,
    pub margin: f32,
    pub gap: f32,
    pub rounding: f32,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            bg: Color32::from_rgb(32, 32, 32),
            text: Color32::WHITE,
            history_text: Color32::from_gray(150),
            key_digit: Color32::from_rgb(59, 59, 59),
            key_function: Color32::from_rgb(50, 50, 50),
            key_equals: Color32::from_rgb(0, 120, 215),
            margin: 6.0,
            gap: 3.0,
            rounding: 5.0,
        }
    }
}

impl Theme {
    pub fn apply(&self, ctx: &egui::Context) {
        let mut v = egui::Visuals::dark();
        v.panel_fill = self.bg;
        v.window_fill = self.bg;
        v.widgets.noninteractive.bg_stroke = Stroke::NONE;
        ctx.set_visuals(v);
    }
}
