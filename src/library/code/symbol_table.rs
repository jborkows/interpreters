use std::{collections::HashMap, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Symbol {
    pub name: String,
    pub index: u16,
    pub level: usize,
}

pub(crate) struct SymbolTable<'a> {
    store: HashMap<String, Rc<Symbol>>,
    counter: u16,
    level: usize,
    outer: Option<&'a SymbolTable<'a>>,
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> Self {
        SymbolTable {
            store: HashMap::new(),
            counter: 0,
            level: 0,
            outer: None,
        }
    }

    pub fn define(&mut self, name: &str) -> Rc<Symbol> {
        let symbol = Rc::new(Symbol {
            name: name.to_string(),
            index: self.counter,
            level: self.level,
        });
        self.store.insert(name.to_string(), symbol.clone());
        self.counter = self.counter + 1;
        symbol.clone()
    }

    pub fn enclosed(&'a self) -> Self {
        SymbolTable {
            store: HashMap::new(),
            counter: 0,
            level: self.level + 1,
            outer: Some(&self),
        }
    }

    pub(crate) fn resolve(&self, name: &str) -> Option<&Symbol> {
        let value = self.store.get(&name.to_string()).map(|x| x.as_ref());
        match value {
            Some(v) => Some(v),
            None => match self.outer {
                Some(o) => o.resolve(name),
                None => None,
            },
        }
    }
}
