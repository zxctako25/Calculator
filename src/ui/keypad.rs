use super::theme::Theme;
use crate::logic::action::Action;
use crate::logic::operation::{
    Operation, 
    UnaryOp
};

use eframe::egui::{
    self, pos2, vec2, Align2, Color32, FontId, Painter, Pos2, Rect, Sense, Shape, Stroke
};

#[derive(Clone, Copy)]
enum KeyKind {
    Digit,
    Function,
    Equals,
}

#[derive(Clone, Copy)]
struct Key {
    label: &'static str,
    action: Action,
    kind: KeyKind,
}

const fn key(label: &'static str, action: Action, kind: KeyKind) -> Key {
    Key {
        label, action, kind 
    }
}

const D: KeyKind = KeyKind::Digit;
const F: KeyKind = KeyKind::Function;
const E: KeyKind = KeyKind::Equals;

const LAYOUT: [[Key; 4]; 6] = [
    [
        key("%", Action::Percent, F),
        key("CE", Action::ClearEntry, F),
        key("C", Action::ClearAll, F),
        key("", Action::Backspace, F),
    ],
    [
        key("1/x", Action::Unary(UnaryOp::Inverse), F),
        key("x²", Action::Unary(UnaryOp::Sqr), F),
        key("√x", Action::Unary(UnaryOp::Sqrt), F),
        key("÷", Action::Operator(Operation::Div), F),
    ],
    [
        key("7", Action::Digit('7'), D),
        key("8", Action::Digit('8'), D),
        key("9", Action::Digit('9'), D),
        key("×", Action::Operator(Operation::Mul), F),
    ],
    [
        key("4", Action::Digit('4'), D),
        key("5", Action::Digit('5'), D),
        key("6", Action::Digit('6'), D),
        key("−", Action::Operator(Operation::Sub), F),
    ],
    [
        key("1", Action::Digit('1'), D),
        key("2", Action::Digit('2'), D),
        key("3", Action::Digit('3'), D),
        key("+", Action::Operator(Operation::Add), F),
    ],
    [
        key("±", Action::Unary(UnaryOp::Negate), D),
        key("0", Action::Digit('0'), D),
        key(",", Action::Decimal, D),
        key("=", Action::Equals, E),
    ],
];

pub struct Keypad;

impl Keypad {
    pub fn show(ui: &mut egui::Ui, theme: &Theme) -> Option<Action> {
        let area = ui.available_rect_before_wrap();
        let gap = theme.gap;
        let rows = LAYOUT.len() as f32;
        let cols = LAYOUT[0].len() as f32;

        let w = (area.width() - gap * (cols - 1.0)) / cols;
        let h = (area.height() - gap * (rows - 1.0)) / rows;
        let font = FontId::proportional((h * 0.32).clamp(15.0, 26.0));
        let mut clicked = None;

        for (r, row) in LAYOUT.iter().enumerate() {
            for (c, k) in row.iter().enumerate() {
                let min = area.min + vec2(c as f32 * (w + gap), r as f32 * (h + gap));
                let rect = Rect::from_min_size(min, vec2(w, h));
                let resp = ui.interact(rect, ui.id().with(("key", r, c)), Sense::click());
                let base = match k.kind {
                    KeyKind::Digit => theme.key_digit,
                    KeyKind::Function => theme.key_function,
                    KeyKind::Equals => theme.key_equals,
                };
                
                let fill = if resp.is_pointer_button_down_on() {
                    mix(base, Color32::BLACK, 0.2)
                } else if resp.hovered() {
                    mix(base, Color32::WHITE, 0.08)
                } else {
                    base
                };

                let painter = ui.painter();
                painter.rect_filled(rect, theme.rounding, fill);
                if k.action == Action::Backspace {
                    draw_backspace_icon(painter, rect.center(), font.size, theme.text);
                } else {
                    painter.text(rect.center(), Align2::CENTER_CENTER, k.label, font.clone(), theme.text);
                }

                if resp.clicked() {
                    clicked = Some(k.action);
                }
            }
        }
        clicked
    }
}

fn mix(a: Color32, b: Color32, t: f32) -> Color32 {
    let lerp = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t).round() as u8;
    Color32::from_rgb(lerp(a.r(), b.r()), lerp(a.g(), b.g()), lerp(a.b(), b.b()))
}

fn draw_backspace_icon(p: &Painter, c: Pos2, size: f32, color: Color32) {
    let w = size * 1.1;
    let h = size * 0.75;
    let tip = h * 0.5;
    let (left, right) = (c.x - w / 2.0, c.x + w / 2.0);
    let (top, bottom) = (c.y - h / 2.0, c.y + h / 2.0);
    let stroke = Stroke::new(1.5, color);

    p.add(Shape::line(
        vec![
            pos2(left, c.y),
            pos2(left + tip, top),
            pos2(right, top),
            pos2(right, bottom),
            pos2(left + tip, bottom),
            pos2(left, c.y),
        ],
        stroke,
    ));

    let cx = (left + tip + right) / 2.0;
    let r = h * 0.18;
    p.line_segment([pos2(cx - r, c.y - r), pos2(cx + r, c.y + r)], stroke);
    p.line_segment([pos2(cx - r, c.y + r), pos2(cx + r, c.y - r)], stroke);
}
