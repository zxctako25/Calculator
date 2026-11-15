use super::error::CalcError;

fn finite(r: f64) -> Result<f64, CalcError> {
    if r.is_finite() {
        Ok(r) 
    } else {
        Err(CalcError::Overflow)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    pub fn symbol(self) -> &'static str {
        match self {
            Operation::Add => "+",
            Operation::Sub => "−",
            Operation::Mul => "×",
            Operation::Div => "÷",
        }
    }

    pub fn apply(self, a: f64, b: f64) -> Result<f64, CalcError> {
        match self {
            Operation::Add => finite(a + b),
            Operation::Sub => finite(a - b),
            Operation::Mul => finite(a * b),
            Operation::Div => {
                if b == 0.0 {
                    return Err(if a == 0.0 { CalcError::Undefined } else { CalcError::DivideByZero });
                }
                finite(a / b)
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum UnaryOp {
    Sqrt,
    Sqr,
    Inverse,
    Negate,
}

impl UnaryOp {
    pub fn name(self) -> &'static str {
        match self {
            UnaryOp::Sqrt => "√",
            UnaryOp::Sqr => "sqr",
            UnaryOp::Inverse => "1/",
            UnaryOp::Negate => "negate",
        }
    }

    pub fn apply(self, x: f64) -> Result<f64, CalcError> {
        match self {
            UnaryOp::Sqrt => {
                if x < 0.0 {
                    return Err(CalcError::InvalidInput);
                }
                finite(x.sqrt())
            }
            UnaryOp::Sqr => finite(x * x),
            UnaryOp::Inverse => {
                if x == 0.0 {
                    return Err(CalcError::DivideByZero);
                }
                finite(1.0 / x)
            }
            UnaryOp::Negate => finite(-x),
        }
    }
}
