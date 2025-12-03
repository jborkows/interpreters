use crate::{
    generate_vm_tests,
    vm::testing::setups::{run_vm_test, should_be_integer},
};

generate_vm_tests! {
    simple: (r#"
    let closureFactory = fn(a){ fn() { a;} };
    let closure = closureFactory(5);
    closure()
    "#, should_be_integer(5)),
}
