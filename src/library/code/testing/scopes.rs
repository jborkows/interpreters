use crate::code::{OpCodes, compiler::Worker};

#[test]
fn compiler_scopes() {
    let mut compiler = Worker::new();
    assert_eq!(compiler.scope_index, 0);
    compiler.emit_op_code(OpCodes::Minus);
    compiler.enter_scope();

    assert_eq!(compiler.scope_index, 1);
    compiler.emit_op_code(OpCodes::Bang);
    let mut instructions = compiler
        .scopes
        .get(1)
        .take()
        .expect("Should be never scope")
        .clone();
    let instuction = instructions
        .last_instruction
        .take()
        .expect("Should be Bang");
    assert_eq!(OpCodes::Bang, instuction.opcode);
    compiler.leave_scope();
    assert_eq!(compiler.scope_index, 0);
    compiler.emit_op_code(OpCodes::Add);
    let main_scope = compiler
        .scopes
        .get(0)
        .take()
        .expect("Main scope has to be present");
    assert_eq!(2, main_scope.instructions.len()); //Minus + Add
    assert_eq!(
        OpCodes::Add,
        main_scope
            .last_instruction
            .clone()
            .map(|x| x.opcode)
            .take()
            .expect("Last instruction has to be present")
    );
    assert_eq!(
        OpCodes::Minus,
        main_scope
            .previous_instruction
            .clone()
            .map(|x| x.opcode)
            .take()
            .expect("Previous instruction has to be present")
    );
}
