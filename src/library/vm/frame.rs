use crate::code::Instructions;

#[derive(Clone, Debug)]
pub(crate) struct Frame {
    pub(crate) function: Instructions,
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
    pub(crate) fn new(instructions: Instructions, base_pointer: usize) -> Frame {
        Frame {
            function: instructions,
            instruction_pointer: 0,
            base_pointer: base_pointer,
        }
    }
}

pub(crate) const NIL_FRAME: Frame = Frame {
    function: Instructions(vec![]),
    instruction_pointer: 0,
    base_pointer: 0,
};
