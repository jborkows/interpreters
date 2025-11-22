use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::object::BuiltInFunction;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SymbolType {
    GLOBAL,
    LOCAL,
    BUILTIN,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Symbol {
    pub name: String,
    pub index: u16,
    pub level: usize,
    pub symbol_type: SymbolType,
}

impl Symbol {
    pub fn what_type(&self) -> SymbolType {
        self.symbol_type.clone()
    }
}

pub(crate) struct SymbolTable {
    store: HashMap<String, Rc<Symbol>>,
    counter: u16,
    level: usize,
    outer: Option<Rc<RefCell<SymbolTable>>>,
    builtin_scope: Rc<RefCell<BuiltinScope>>,
}
struct BuiltinScope {
    store: HashMap<String, Rc<Symbol>>,
    counter: u16,
}

impl SymbolTable {
    fn new(builtin: BuiltinScope) -> Self {
        SymbolTable {
            store: HashMap::new(),
            counter: 0,
            level: 0,
            outer: None,
            builtin_scope: Rc::new(RefCell::new(builtin)),
        }
    }

    pub fn define(symbol_table: &Rc<RefCell<SymbolTable>>, name: &str) -> Rc<Symbol> {
        let symbol = Rc::new(Symbol {
            name: name.to_string(),
            index: symbol_table.borrow().counter,
            level: symbol_table.borrow().level,
            symbol_type: match symbol_table.borrow().level {
                0 => SymbolType::GLOBAL,
                _ => SymbolType::LOCAL,
            },
        });
        symbol_table
            .borrow_mut()
            .store
            .insert(name.to_string(), symbol.clone());
        symbol_table.borrow_mut().counter += 1;
        symbol.clone()
    }

    pub fn define_builtin(symbol_table: &Rc<RefCell<SymbolTable>>, name: &str) -> Rc<Symbol> {
        let builtin = symbol_table.borrow().builtin_scope.clone();
        let symbol = Rc::new(Symbol {
            name: name.to_string(),
            index: builtin.borrow().counter,
            level: 0,
            symbol_type: SymbolType::BUILTIN,
        });
        builtin
            .borrow_mut()
            .store
            .insert(name.to_string(), symbol.clone());
        builtin.borrow_mut().counter += 1;
        symbol.clone()
    }

    pub fn number_of_locals(symbol_table: &Rc<RefCell<SymbolTable>>) -> usize {
        symbol_table.borrow().store.len()
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
                None => match borrowed
                    .builtin_scope
                    .borrow()
                    .store
                    .get(&name.to_string())
                    .map(|x| x.as_ref())
                {
                    Some(v) => Some(Rc::new(v.clone())),
                    None => None,
                },
            },
        }
    }
    pub fn new_table() -> Rc<RefCell<SymbolTable>> {
        let builtin_store: HashMap<String, Rc<Symbol>> = BuiltInFunction::all()
            .into_iter()
            .map(|fun| {
                (
                    fun.to_string(),
                    Rc::new(Symbol {
                        name: fun.to_string(),
                        index: fun.index() as u16,
                        level: 0,
                        symbol_type: SymbolType::BUILTIN,
                    }),
                )
            })
            .collect();
        let size = builtin_store.len();

        let builtin = BuiltinScope {
            store: builtin_store,
            counter: size as u16,
        };
        let symbol = SymbolTable::new(builtin);
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
            builtin_scope: symbol_table.borrow().builtin_scope.clone(),
        };
        Rc::new(RefCell::new(symbol))
    }
}
