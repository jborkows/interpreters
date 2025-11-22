use crate::{
    generate_vm_tests,
    vm::testing::setups::{run_vm_test, should_be_error, should_be_integer},
};
fn argument_length_does_not_match(error_message: &str) -> Result<(), String> {
    if error_message.contains("Number of arguments does not match") {
        Result::Ok(())
    } else {
        let message = format!("Expecting error about number of arguments got {error_message}");
        Result::Err(message)
    }
}

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
   too_much_arguments:(
    "let add = fn(a,b){a + b}
    add(1,2,3)
    ", should_be_error(|_x|Result::Ok(()))
  ),
   too_much_arguments_with_global:(
    "
    let global = 111;
    let add = fn(a,b){a + b + global}
    add(1,2,3)
    ", should_be_error(argument_length_does_not_match)
  ),

   not_enough_arguments:(
    "
    let add = fn(a,b){a + b}
    add(1)
    ", should_be_error(argument_length_does_not_match)
  ),

}
