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
            .expect("Cannot find {expected_identifier} in local scope");
        assert_eq!($expected.identifier, value.name);
        assert_eq!($expected.index, value.index);
        assert_eq!($expected.level, value.level);
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

    find_in!(
        second_one,
        Expected {
            index: 0,
            identifier: "c",
            level: 1,
        }
    );
    find_in!(
        second_one,
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

struct Expected<'a> {
    index: u16,
    identifier: &'a str,
    level: usize,
}
