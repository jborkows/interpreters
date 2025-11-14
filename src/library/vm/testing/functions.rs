use crate::{
    generate_vm_tests,
    vm::testing::setups::{run_vm_test, should_be_integer, should_be_null},
};

generate_vm_tests! {
    calling: (r#"
    let fun = fn(){5+10}
    fun()
    "#, should_be_integer(15)),
    call_in_serries:(r#"
       let a = fn() {1}
       let b = fn() { 2 + a()}
       let c = fn() { b() + 3}
       c()
    "#,should_be_integer(6)
    ),
    check_multiple_returns: (r#"
    let fun = fn(){return 10; return 15;}
    fun()
    "#, should_be_integer(10)),
    no_return_leads_to_null: (
    r#"
        let fun = fn() {};
        fun()
    "#,
        should_be_null()
    ),
    no_return_leads_to_null_no_matter_how_many: (
    r#"
        let a = fn() {};
        let b = fn() { a() };
        a()
        b()
    "#,
        should_be_null()
    ),
    higher_order_one: (
        r#"
        let base = fn() { 1 }
        let higher = fn() { base }
        higher()()
        "#,
        should_be_integer(1)
    ),
}
