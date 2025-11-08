use std::{collections::HashMap, rc::Rc};

//TODO if needed

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SymbolScope {
    Global,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Symbol {
    pub name: String,
    pub index: u16,
}

pub(crate) struct SymbolTable {
    store: HashMap<String, Rc<Symbol>>,
    counter: u16,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            store: HashMap::new(),
            counter: 0,
        }
    }

    pub fn define(&mut self, name: &str) -> Rc<Symbol> {
        let symbol = Rc::new(Symbol {
            name: name.to_string(),
            index: self.counter,
        });
        self.store.insert(name.to_string(), symbol.clone());
        self.counter = self.counter + 1;
        symbol.clone()
    }

    pub(crate) fn resolve(&self, name: &str) -> Option<&Symbol> {
        self.store.get(&name.to_string()).map(|x| x.as_ref())
    }
}
