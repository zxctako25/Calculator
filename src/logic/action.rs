use super::operation::{
    Operation, UnaryOp
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Action {
    Digit(char),
    Decimal,
    Operator(Operation),
    Unary(UnaryOp),
    Percent,
    Equals,
    ClearEntry,
    ClearAll,
    Backspace,
}
