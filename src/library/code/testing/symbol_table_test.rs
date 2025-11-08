use crate::code::symbol_table::SymbolTable;

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

    let value = global.resolve("a").take().expect("Cannot find a in scope");
    assert_eq!("a", value.name);
    assert_eq!(0, value.index);
    let value = global.resolve("b").take().expect("Cannot find b in scope");
    assert_eq!("b", value.name);
    assert_eq!(1, value.index);
}
