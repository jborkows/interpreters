use std::usize;

use crate::{
    code::{Byte, Instructions},
    object::{CompiledFunctionEntry, Object},
};

#[derive(Clone, Debug)]
pub(crate) struct Closure {
    pub(crate) function: CompiledFunctionEntry,
    pub(crate) free: Vec<Object>,
}

impl Closure {
    pub fn bytes(&self) -> Vec<Byte> {
        self.function.instructions.bytes()
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Frame {
    pub(crate) closure: Closure,
    pub(crate) instruction_pointer: usize,
    pub(crate) base_pointer: usize, //position of stack before starting new frame, it is not
                                    //instruction pointer since on stack there will be a place for local bindings so
                                    /*
                                     * Stack
                                     * call() - base_pointer
                                     * place for variable 1
                                     * place for variable 2
                                     * code - instruction_pointer
                                     */
}

impl Frame {
    pub(crate) fn new(closure: Closure, base_pointer: usize) -> Frame {
        Frame {
            closure: closure,
            instruction_pointer: 0,
            base_pointer: base_pointer,
        }
    }
}

pub(crate) const NIL_FRAME: Frame = Frame {
    closure: Closure {
        function: CompiledFunctionEntry {
            instructions: Instructions(vec![]),
            number_of_locals: 0,
            number_of_parameters: 0,
        },
        free: vec![],
    },
    instruction_pointer: 0,
    base_pointer: 0,
};
