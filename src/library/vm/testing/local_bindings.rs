use crate::{
    generate_vm_tests,
    vm::testing::setups::{run_vm_test, should_be_integer},
};

generate_vm_tests! {
    one_variable: (r#"
    let fun = fn(){
        let one = 1;
        one
    }
    fun()
    "#, should_be_integer(1)),

    two_variables: (r#"
    let fun = fn(){
        let one = 1;
        let two = 2;
        one + two
    }
    fun()
    "#, should_be_integer(3)),
    two_scopes: (r#"
    let first = fn(){
        let one = 1;
        let two = 2;
        one + two
    }
    let second = fn(){
       let c = 3;
       let d = 4;
       c + d
    }
    first() + second()
    "#, should_be_integer(10)),
    two_scopes_same_variable:(r#"
    let first = fn(){
        let aaa = 1;
        aaa
    }
    let second = fn(){
        let aaa = 5;
        aaa
    }
    first() + second() + first()
    "#, should_be_integer(7)),
global_scope:(r#"
    let g = 4;
    let first = fn(){
        let aaa = 1;
        g - aaa
    }
    let second = fn(){
        let aaa = 2;
        g - aaa
    }
    first() + second() 
    "#, should_be_integer(5)),
nested:(r#"
    let g = 4;
    let first = fn(){
        let aaa = 1;
        g - fn(){
           let aaa = 2
           aaa
        }() - aaa
    }
    first()
    "#, should_be_integer(1)),
}
