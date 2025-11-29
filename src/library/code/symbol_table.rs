use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{code::symbol_table, object::BuiltInFunction};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SymbolType {
    GLOBAL,
    LOCAL,
    BUILTIN,
    FREE,
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
    pub free_symbols: Vec<Rc<Symbol>>,
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
            free_symbols: vec![],
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

    fn define_free(symbol_table: &Rc<RefCell<SymbolTable>>, original: Rc<Symbol>) -> Rc<Symbol> {
        let mut table = symbol_table.borrow_mut();
        let number_of_free = table.free_symbols.len() as u16;
        let free_one = Rc::new(Symbol {
            name: original.name.clone(),
            index: number_of_free,
            level: 0,
            symbol_type: SymbolType::FREE,
        });
        table.free_symbols.push(free_one.clone());
        #[cfg(test)]
        println!(
            "Pushing {:?} at {number_of_free} current table {:?}",
            free_one.clone(),
            table.free_symbols
        );
        table.store.insert(original.name.clone(), free_one.clone());
        free_one
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
        let local_result = symbol_table.borrow().store.get(name).cloned();

        match local_result {
            Some(v) => Some(v),
            None => {
                let outer_ref = symbol_table.borrow().outer.clone();

                match outer_ref {
                    Some(o) => {
                        let resolved = SymbolTable::resolve(&o, name);
                        match resolved {
                            Some(ref v) => {
                                if matches!(v.what_type(), SymbolType::LOCAL) {
                                    Some(SymbolTable::define_free(symbol_table, v.clone()))
                                } else if matches!(v.what_type(), SymbolType::FREE) {
                                    Some(SymbolTable::define_free(symbol_table, v.clone()))
                                } else {
                                    resolved
                                }
                            }
                            None => None,
                        }
                    }
                    None => symbol_table
                        .borrow()
                        .builtin_scope
                        .borrow()
                        .store
                        .get(name)
                        .cloned(),
                }
            }
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
            free_symbols: vec![],
        };
        Rc::new(RefCell::new(symbol))
    }
}
