use crate::{
    generate_vm_tests,
    vm::testing::setups::{run_vm_test, should_be_integer},
};

generate_vm_tests! {
    indentity: (r#"
    let identity = fn(a){a}
    identity(4)
    "#,should_be_integer(4)),
    addition: (r#"
    let add = fn(a,b){a+b}
    add(1,2)
    "#, should_be_integer(3)),
    addition_with_let: (r#"
    let add = fn(a,b){
      let c = a+b;
      c;
    }
    add(1,2)
    "#, should_be_integer(3)),
    complex_addition: (r#"
    let add = fn(a,b){
      let c = a+b;
      c;
    }
    add(1,2) + add(3,4)
    "#, should_be_integer(10)),

    complex_addition_wrapped: (r#"
    let add = fn(a,b){
      let c = a+b;
      c;
    }
    let wrapper = fn(){
      add(1,2) + add(3,4)
    }
    wrapper()
    "#, should_be_integer(10)),

    complex_addition_wrapped_with_global: (r#"
    let global = 100
    let add = fn(a,b){
      let c = a+b;
      c;
    }
    let wrapper = fn(){
      add(1,2) + add(3,4) + global
    }
    wrapper() + global
    "#, should_be_integer(210)),


}
