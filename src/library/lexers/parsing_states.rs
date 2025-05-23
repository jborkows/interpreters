use std::{cell::RefCell, rc::Rc};

use crate::lines::TextPosition;

#[derive(Debug)]
pub(super) enum LexerState {
    Idle,
    ReadingEquality {
        starting_position: TextPosition,
    },
    ReadingNegation {
        starting_position: TextPosition,
    },
    ReadingNumber {
        starting_position: TextPosition,
        value: u32,
    },
    ReadingInvalid {
        starting_position: TextPosition,
        reason: String,
    },
    ReadingText {
        starting_position: TextPosition,
        chars: Rc<RefCell<Vec<char>>>,
    },
    ReadingIdentifier {
        starting_position: TextPosition,
        chars: Vec<char>,
    },
}
