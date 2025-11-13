use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Symbol {
    pub name: String,
    pub index: u16,
    pub level: usize,
}

impl Symbol {
    pub(crate) fn is_global(&self) -> bool {
        self.level == 0
    }
}

pub(crate) struct SymbolTable {
    store: HashMap<String, Rc<Symbol>>,
    counter: u16,
    level: usize,
    outer: Option<Rc<RefCell<SymbolTable>>>,
}

impl SymbolTable {
    fn new() -> Self {
        SymbolTable {
            store: HashMap::new(),
            counter: 0,
            level: 0,
            outer: None,
        }
    }

    pub fn define(symbol_table: &Rc<RefCell<SymbolTable>>, name: &str) -> Rc<Symbol> {
        let symbol = Rc::new(Symbol {
            name: name.to_string(),
            index: symbol_table.borrow().counter,
            level: symbol_table.borrow().level,
        });
        symbol_table
            .borrow_mut()
            .store
            .insert(name.to_string(), symbol.clone());
        symbol_table.borrow_mut().counter += 1;
        symbol.clone()
    }

    pub fn is_enclosed(symbol_table: &Rc<RefCell<SymbolTable>>) -> bool {
        symbol_table.borrow().outer.is_some()
    }

    pub(crate) fn resolve(
        symbol_table: &Rc<RefCell<SymbolTable>>,
        name: &str,
    ) -> Option<Rc<Symbol>> {
        let borrowed = symbol_table.borrow();
        let value = borrowed.store.get(&name.to_string()).map(|x| x.as_ref());
        match value {
            Some(v) => Some(Rc::new(v.clone())),
            None => match symbol_table.borrow().outer.clone() {
                Some(o) => SymbolTable::resolve(&o, name),
                None => None,
            },
        }
    }
    pub fn new_table() -> Rc<RefCell<SymbolTable>> {
        let symbol = SymbolTable::new();
        Rc::new(RefCell::new(symbol))
    }

    pub fn outer(symbol_table: &Rc<RefCell<SymbolTable>>) -> Rc<RefCell<SymbolTable>> {
        symbol_table
            .borrow_mut()
            .outer
            .take()
            .expect("Outer was not defined")
    }

    pub fn enclosed(symbol_table: &Rc<RefCell<SymbolTable>>) -> Rc<RefCell<SymbolTable>> {
        let symbol = SymbolTable {
            store: HashMap::new(),
            counter: 0,
            level: symbol_table.borrow().level + 1,
            outer: Some(symbol_table.clone()),
        };
        Rc::new(RefCell::new(symbol))
    }
}
