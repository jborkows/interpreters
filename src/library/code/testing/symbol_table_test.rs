use crate::code::symbol_table::SymbolTable;

macro_rules! do_not_find_in {
    ($symbol_table:expr,$text:expr ) => {{
        let value = $symbol_table.resolve($text);
        if let Some(v) = value {
            panic!("Should not find {v:?} in scope")
        }
    }};
}

macro_rules! find_in {
    ($symbol_table:expr, $expected:expr) => {{
        let value = $symbol_table
            .resolve($expected.identifier)
            .take()
            .expect("Cannot find {expected_identifier} in local scope");
        assert_eq!($expected.identifier, value.name);
        assert_eq!($expected.index, value.index);
        assert_eq!($expected.level, value.level);
    }};
}

#[test]
fn test_define() {
    let mut global = SymbolTable::new();
    let value = global.define("a");
    assert_eq!("a", value.name);
    assert_eq!(0, value.index);
    let value = global.define("b");
    assert_eq!("b", value.name);
    assert_eq!(1, value.index);
}

#[test]
fn test_resolve() {
    let mut global = SymbolTable::new();
    global.define("a");
    global.define("b");

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
    let mut global = SymbolTable::new();
    global.define("a");
    global.define("b");
    let mut local: SymbolTable = global.enclosed();
    local.define("c");
    local.define("d");

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
    let mut global = SymbolTable::new();
    global.define("a");
    global.define("b");
    let mut first_one: SymbolTable = global.enclosed();
    first_one.define("c");
    first_one.define("d");
    let mut second_one: SymbolTable = global.enclosed();
    second_one.define("e");
    second_one.define("f");

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
    let mut global = SymbolTable::new();
    global.define("a");
    global.define("b");
    let mut first_one: SymbolTable = global.enclosed();
    first_one.define("c");
    first_one.define("d");
    let mut second_one: SymbolTable = first_one.enclosed();
    second_one.define("e");
    second_one.define("f");

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
