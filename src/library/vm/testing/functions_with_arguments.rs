use crate::{
    generate_vm_tests,
    vm::testing::setups::{run_vm_test, should_be_integer},
};

generate_vm_tests! {
    indentity: (r#"
    let identity = fn(a){a}
    identity(4)
    "#, should_be_integer(4)),
    addition: (r#"
    let add = fn(a,b){a+b}
    add(1,2)
    "#, should_be_integer(3)),
}
