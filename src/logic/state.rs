use super::action::Action;
use super::error::CalcError;
use super::format::NumberFormatter;
use super::input_buffer::InputBuffer;
use super::operation::{
    Operation, UnaryOp
};

#[derive(Default)]
pub struct CalculatorState {
    input: InputBuffer,
    typing: bool,
    value: f64,
    acc: Option<f64>,
    op: Option<Operation>,
    awaiting: bool,
    rhs_expr: Option<String>,
    last_op: Option<(Operation, f64)>,
    history: String,
    error: Option<CalcError>,
}

fn show(v: f64) -> String {
    NumberFormatter::show(v)
}

impl CalculatorState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn display_text(&self) -> String {
        if let Some(e) = self.error {
            return e.message().to_string();
        }
        
        if self.typing {
            NumberFormatter::group(self.input.as_str())
        } else {
            show(self.value)
        }
    }

    pub fn history_text(&self) -> &str {
        &self.history
    }

    pub fn handle(&mut self, action: Action) {
        match action {
            Action::Digit(d) => self.input_digit(d),
            Action::Decimal => self.input_decimal(),
            Action::Operator(op) => self.input_operator(op),
            Action::Unary(u) => self.unary(u),
            Action::Percent => self.percent(),
            Action::Equals => self.equals(),
            Action::ClearEntry => self.clear_entry(),
            Action::ClearAll => self.clear_all(),
            Action::Backspace => self.backspace(),
        }
    }

    fn input_digit(&mut self, d: char) {
        if self.error.is_some() {
            return;
        }
        
        self.begin_entry();
        self.input.push_digit(d);
    }

    fn input_decimal(&mut self) {
        if self.error.is_some() {
            return;
        }
        
        self.begin_entry();
        self.input.push_decimal();
    }

    fn input_operator(&mut self, op: Operation) {
        if self.error.is_some() {
            return;
        }
        
        if self.op.is_some() && self.awaiting {
            self.op = Some(op);
            self.history = self.pending_history();
            return;
        }

        let cur = self.current();
        let acc = match (self.op, self.acc) {
            (Some(prev), Some(a)) => match prev.apply(a, cur) {
                Ok(r) => r,
                Err(e) => {
                    self.fail(e);
                    return;
                }
            },
            _ => cur,
        };

        self.commit();
        self.value = acc;
        self.acc = Some(acc);
        self.op = Some(op);
        self.awaiting = true;
        self.rhs_expr = None;
        self.last_op = None;
        self.history = self.pending_history();
    }

    fn equals(&mut self) {
        if self.error.is_some() {
            return;
        }
        
        let cur = self.current();
        let (o, lhs, rhs) = if let (Some(o), Some(a)) = (self.op, self.acc) {
            (o, a, cur)
        } else if let Some((o, r)) = self.last_op {
            (o, cur, r)
        } else {
            self.history = format!("{} =", self.rhs_expr.clone().unwrap_or_else(|| show(cur)));
            self.commit();
            self.rhs_expr = None;
            return;
        };

        let rhs_text = if self.op.is_some() {
            self.rhs_expr.clone().unwrap_or_else(|| show(rhs))
        } else {
            show(rhs)
        };
        
        self.history = format!("{} {} {} =", show(lhs), o.symbol(), rhs_text);
        match o.apply(lhs, rhs) {
            Ok(r) => {
                self.typing = false;
                self.value = r;
                self.acc = None;
                self.op = None;
                self.awaiting = false;
                self.rhs_expr = None;
                self.last_op = Some((o, rhs));
            }
            Err(e) => self.fail(e),
        }
    }

    fn percent(&mut self) {
        if self.error.is_some() {
            return;
        }
        
        let cur = self.current();
        let r = match (self.op, self.acc) {
            (Some(_), Some(a)) => a * cur / 100.0,
            _ => 0.0,
        };
        
        if !r.is_finite() {
            self.fail(CalcError::Overflow);
            return;
        }
        
        self.typing = false;
        self.value = r;
        self.awaiting = false;
        self.rhs_expr = Some(show(r));
        self.history = self.pending_history();
    }

    fn unary(&mut self, kind: UnaryOp) {
        if self.error.is_some() {
            return;
        }
        
        if kind == UnaryOp::Negate && self.typing {
            self.input.toggle_sign();
            return;
        }

        let cur = self.current();
        let res = match kind.apply(cur) {
            Ok(r) => r,
            Err(e) => {
                self.fail(e);
                return;
            }
        };

        let inner = self.rhs_expr.clone().unwrap_or_else(|| show(cur));
        self.typing = false;
        self.value = res;
        self.awaiting = false;
        self.rhs_expr = Some(format!("{}({})", kind.name(), inner));
        self.history = self.pending_history();
    }

    fn clear_all(&mut self) {
        *self = Self::default();
    }

    fn clear_entry(&mut self) {
        if self.error.is_some() || self.op.is_none() {
            self.clear_all();
            return;
        }
        
        self.typing = false;
        self.value = 0.0;
        self.awaiting = false;
        self.rhs_expr = None;
        self.history = self.pending_history();
    }

    fn backspace(&mut self) {
        if self.error.is_some() || !self.typing {
            return;
        }
        
        self.input.backspace();
    }

    fn current(&self) -> f64 {
        if self.typing { 
            self.input.value() 
        } else {
            self.value
        }
    }

    fn commit(&mut self) {
        if self.typing {
            self.value = self.input.value();
            self.typing = false;
        }
    }

    fn begin_entry(&mut self) {
        if self.typing {
            return;
        }
        
        if self.op.is_none() {
            self.history.clear();
            self.acc = None;
        }
        
        self.typing = true;
        self.input.reset();
        self.awaiting = false;
        self.rhs_expr = None;
    }

    fn pending_history(&self) -> String {
        match (self.acc, self.op) {
            (Some(a), Some(o)) => {
                let mut s = format!("{} {}", show(a), o.symbol());
                if let Some(e) = &self.rhs_expr {
                    s.push(' ');
                    s.push_str(e);
                }
                s
            }
            _ => self.rhs_expr.clone().unwrap_or_default(),
        }
    }

    fn fail(&mut self, err: CalcError) {
        let h = self.history.clone();
        *self = Self::default();
        self.history = h;
        self.error = Some(err);
    }
}
