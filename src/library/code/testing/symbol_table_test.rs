use core::panic;

use crate::code::symbol_table::SymbolTable;

macro_rules! do_not_find_in {
    ($symbol_table:expr,$text:expr ) => {{
        let value = SymbolTable::resolve(&$symbol_table, $text);
        if let Some(v) = value {
            panic!("Should not find {v:?} in scope")
        }
    }};
}

macro_rules! find_in {
    ($symbol_table:expr, $expected:expr) => {{
        let value = SymbolTable::resolve(&$symbol_table, $expected.identifier)
            .take()
            .expect(&format!("Cannot find {:?} in local scope", $expected));
        assert_eq!($expected.identifier, value.name);
        assert_eq!($expected.index, value.index, "{:?}", value);
        assert_eq!($expected.level, value.level, "{:?}", value);
    }};
}

macro_rules! should_be_type {
    ($symbol_table:expr, $name:expr, $index:expr, $type:ident) => {
        match SymbolTable::resolve(&$symbol_table, $name) {
            Some(s) => match s.what_type() {
                crate::code::symbol_table::SymbolType::$type => {
                    assert_eq!(s.index, $index)
                }
                _ => panic!(
                    "Builtin was defined but with wrong type expected {} got {:?}",
                    stringify!($type),
                    s.what_type()
                ),
            },
            None => panic!("Builtin was not resolved in global scope"),
        }
    };
}

macro_rules! should_be_free {
    ($symbol_table:expr, $name:expr, $index:expr) => {{ should_be_type!($symbol_table, $name, $index, FREE) }};
}

macro_rules! should_be_function {
    ($symbol_table:expr, $name:expr, $index:expr) => {{ should_be_type!($symbol_table, $name, $index, FUNCTION) }};
}
macro_rules! should_not_be_free {
    ($symbol_table:expr, $name:expr) => {{
        match SymbolTable::resolve(&$symbol_table, $name) {
            Some(s) => match s.what_type() {
                crate::code::symbol_table::SymbolType::FREE => {
                    panic!("{} should not be find as free", $name)
                }
                _ => {}
            },
            None => {}
        }
    }};
}
#[test]
fn test_define() {
    let global = &SymbolTable::new_table();
    let value = SymbolTable::define(global, "a");
    assert_eq!("a", value.name);
    assert_eq!(0, value.index);
    let value = SymbolTable::define(global, "b");
    assert_eq!("b", value.name);
    assert_eq!(1, value.index);
}

#[test]
fn test_resolve() {
    let global = &SymbolTable::new_table();
    SymbolTable::define(global, "a");
    SymbolTable::define(global, "b");

    find_in!(
        global,
        Expected {
            index: 0,
            identifier: "a",
            level: 0,
        }
    );
    find_in!(
        global,
        Expected {
            index: 1,
            identifier: "b",
            level: 0,
        }
    );
}

#[test]
fn test_local_resolve() {
    let global = &SymbolTable::new_table();
    SymbolTable::define(global, "a");
    SymbolTable::define(global, "b");
    let local = &SymbolTable::enclosed(global);
    SymbolTable::define(local, "c");
    SymbolTable::define(local, "d");

    find_in!(
        local,
        Expected {
            index: 0,
            identifier: "a",
            level: 0,
        }
    );
    find_in!(
        local,
        Expected {
            index: 1,
            identifier: "b",
            level: 0,
        }
    );
    find_in!(
        local,
        Expected {
            index: 0,
            identifier: "c",
            level: 1,
        }
    );

    find_in!(
        local,
        Expected {
            index: 1,
            identifier: "d",
            level: 1,
        }
    );
}

#[test]
fn test_local_multiple_resolve() {
    let global = &SymbolTable::new_table();
    SymbolTable::define(global, "a");
    SymbolTable::define(global, "b");
    let first_one = &SymbolTable::enclosed(global);
    SymbolTable::define(&first_one, "c");
    SymbolTable::define(&first_one, "d");
    let second_one = &SymbolTable::enclosed(global);
    SymbolTable::define(&second_one, "e");
    SymbolTable::define(&second_one, "f");

    find_in!(
        first_one,
        Expected {
            index: 0,
            identifier: "a",
            level: 0,
        }
    );
    find_in!(
        first_one,
        Expected {
            index: 1,
            identifier: "b",
            level: 0,
        }
    );
    find_in!(
        first_one,
        Expected {
            index: 0,
            identifier: "c",
            level: 1,
        }
    );

    find_in!(
        first_one,
        Expected {
            index: 1,
            identifier: "d",
            level: 1,
        }
    );
    find_in!(
        second_one,
        Expected {
            index: 0,
            identifier: "a",
            level: 0,
        }
    );
    find_in!(
        second_one,
        Expected {
            index: 1,
            identifier: "b",
            level: 0,
        }
    );
    find_in!(
        second_one,
        Expected {
            index: 0,
            identifier: "e",
            level: 1,
        }
    );

    find_in!(
        second_one,
        Expected {
            index: 1,
            identifier: "f",
            level: 1,
        }
    );
    do_not_find_in!(first_one, "e");
    do_not_find_in!(first_one, "f");
    do_not_find_in!(second_one, "c");
    do_not_find_in!(second_one, "d");
}

#[test]
fn test_local_nested_resolve() {
    let global = &SymbolTable::new_table();
    SymbolTable::define(global, "a");
    SymbolTable::define(global, "b");
    let first_one = &SymbolTable::enclosed(global);
    SymbolTable::define(first_one, "c");
    SymbolTable::define(first_one, "d");
    let second_one = &SymbolTable::enclosed(first_one);
    SymbolTable::define(second_one, "e");
    SymbolTable::define(second_one, "f");

    find_in!(
        first_one,
        Expected {
            index: 0,
            identifier: "a",
            level: 0,
        }
    );
    find_in!(
        first_one,
        Expected {
            index: 1,
            identifier: "b",
            level: 0,
        }
    );
    find_in!(
        first_one,
        Expected {
            index: 0,
            identifier: "c",
            level: 1,
        }
    );

    find_in!(
        first_one,
        Expected {
            index: 1,
            identifier: "d",
            level: 1,
        }
    );
    find_in!(
        second_one,
        Expected {
            index: 0,
            identifier: "a",
            level: 0,
        }
    );
    find_in!(
        second_one,
        Expected {
            index: 1,
            identifier: "b",
            level: 0,
        }
    );

    // No longer true in this form -> "c" and "d" are "free variables" in context of second_one
    // find_in!(
    //     second_one,
    //     Expected {
    //         index: 0,
    //         identifier: "c",
    //         level: 1,
    //     }
    // );
    // find_in!(
    //     second_one,
    //     Expected {
    //         index: 1,
    //         identifier: "d",
    //         level: 1,
    //     }
    // );
    find_in!(
        second_one,
        Expected {
            index: 0,
            identifier: "e",
            level: 2,
        }
    );

    find_in!(
        second_one,
        Expected {
            index: 1,
            identifier: "f",
            level: 2,
        }
    );
    do_not_find_in!(first_one, "e");
    do_not_find_in!(first_one, "f");
}

#[test]
fn test_builtin() {
    let global = &SymbolTable::new_table();
    SymbolTable::define(global, "a");
    SymbolTable::define(global, "b");
    let local = &SymbolTable::enclosed(global);
    SymbolTable::define_builtin(local, "fun");
    match SymbolTable::resolve(global, "fun") {
        Some(s) => match s.what_type() {
            crate::code::symbol_table::SymbolType::BUILTIN => {}
            _ => panic!("Builtin was defined but with wrong type"),
        },
        None => panic!("Builtin was not resolved in global scope"),
    }
}

#[test]
fn test_free_variable() {
    /*
        let a = 1;
        let b = 2;
        let upper = fn(){
            let c = 3;
            let d = 4;
            a + b + c + d
            let inner = fn() {
                let e = 5;
                let f = 6;
                a + b + c + d + e + f
            }
        }
    *
    */
    let global = &SymbolTable::new_table();
    SymbolTable::define(global, "a");
    SymbolTable::define(global, "b");
    let upper = &SymbolTable::enclosed(global);
    SymbolTable::define(upper, "c");
    SymbolTable::define(upper, "d");
    let inner = &SymbolTable::enclosed(upper);
    SymbolTable::define(inner, "e");
    SymbolTable::define(inner, "f");

    #[rustfmt::skip]
    find_in!(global,Expected {index: 0,identifier: "a",level: 0});
    #[rustfmt::skip]
    find_in!(global,Expected {index: 1,identifier: "b",level: 0});
    #[rustfmt::skip]
    find_in!(upper,Expected {index: 0,identifier: "a",level: 0});
    #[rustfmt::skip]
    find_in!(upper,Expected {index: 1,identifier: "b",level: 0});
    #[rustfmt::skip]
    find_in!(inner,Expected {index: 0,identifier: "a",level: 0});
    #[rustfmt::skip]
    find_in!(inner,Expected {index: 1,identifier: "b",level: 0});

    #[rustfmt::skip]
    find_in!(upper,Expected {index: 0,identifier: "c",level: 1});
    #[rustfmt::skip]
    find_in!(upper,Expected {index: 1,identifier: "d",level: 1});

    #[rustfmt::skip]
    should_be_free!(inner, "c", 0);
    #[rustfmt::skip]
    should_be_free!(inner, "d", 1);
    should_not_be_free!(inner, "e");
}

#[test]
fn function_resolve_its_name() {
    let global = &SymbolTable::new_table();
    SymbolTable::define_function_name(global, "fun");
    should_be_function!(global, "fun", 0);
}

#[test]
fn shadowing_function() {
    let global = &SymbolTable::new_table();
    SymbolTable::define_function_name(global, "fun");
    SymbolTable::define(global, "fun");
    #[rustfmt::skip]
    find_in!(global,Expected {index: 0,identifier: "fun",level: 0});
}

#[derive(Debug)]
struct Expected<'a> {
    index: u16,
    identifier: &'a str,
    level: usize,
}
