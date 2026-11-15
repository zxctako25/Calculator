#![windows_subsystem = "windows"]
mod logic;
mod ui;

use ui::window::CalculatorWindow;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([340.0, 520.0])
            .with_min_inner_size([280.0, 420.0])
            .with_resizable(true)
            .with_title("Calculator"),
        ..Default::default()
    };

    eframe::run_native(
        "calculator",
        options,
        Box::new(|cc| Box::new(CalculatorWindow::new(cc))),
    )
}
