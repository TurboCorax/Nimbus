use std::collections::HashMap;

pub struct SymTable {
    pub table: HashMap<String, Symbol>,
}

impl SymTable {
    pub fn new() -> Self {
        SymTable {
            table: HashMap::new(),
        }
    }
    
    pub fn insert(&mut self, symbol: Symbol){
        self.table.insert(symbol.name.clone(), symbol);
    }
}

struct Symbol{
    pub name: String,
    pub symbol_type: SymbolType,
}

enum SymbolType{
    Node,
    Component,
    Model,
    Subcircuit,
}