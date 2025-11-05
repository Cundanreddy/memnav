use regex::Regex;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolInfo {
    pub section: String,
    pub name: String,
    pub address: u64,
    pub size: u64,
    pub symbol_type: String,
}

pub fn parse_map_file(content: &str) -> Vec<SymbolInfo> {
    let re_section = Regex::new(r"^\s*\.(\S+)\s+0x([0-9a-fA-F]+)\s+0x([0-9a-fA-F]+)").unwrap();
    let re_symbol = Regex::new(r"^\s+0x([0-9a-fA-F]+)\s+(\S+)").unwrap();

    let mut current_section = String::new();
    let mut symbols = vec![];

    for line in content.lines() {
        if let Some(cap) = re_section.captures(line) {
            current_section = format!(".{}", &cap[1]);
        } else if let Some(cap) = re_symbol.captures(line) {
            symbols.push(SymbolInfo {
                section: current_section.clone(),
                address: u64::from_str_radix(&cap[1], 16).unwrap_or(0),
                name: cap[2].to_string(),
                size: 0,
                symbol_type: if current_section == ".text" { "function".into() } else { "variable".into() },
            });
        }
    }
    symbols
}
