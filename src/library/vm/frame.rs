use crate::code::Instructions;

#[derive(Clone, Debug)]
pub(crate) struct Frame {
    pub(crate) function: Instructions,
    pub(crate) instruction_pointer: usize,
}

impl Frame {
    pub(crate) fn new(instructions: Instructions) -> Frame {
        Frame {
            function: instructions,
            instruction_pointer: 0,
        }
    }
}

pub(crate) const NIL_FRAME: Frame = Frame {
    function: Instructions(vec![]),
    instruction_pointer: 0,
};
