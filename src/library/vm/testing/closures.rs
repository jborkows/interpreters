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
    more_complex: (r#"
    let global = 1000;
    let closureFactory = fn(a){ fn(b) { fn(c) { a + b + c +global } } };
    let closure = closureFactory(100)(10);
    closure(1)
    "#, should_be_integer(1111)),
    combination: (r#"
    let closureFactory = fn(a,b){ 
        let one = fn() { a; } 
        let two = fn() { b; } 
        fn() { one() + two() }
    };
    let closure = closureFactory(10,1);
    closure()
    "#, should_be_integer(11)),
    recursive: (r#"
    let countDown = fn(x){ 
        if (x == 0) {
            0
        } else {
            countDown(x-1);
        }
    };
    countDown(1)
    "#, should_be_integer(0)),
}
