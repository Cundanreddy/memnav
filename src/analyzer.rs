use crate::parser::SymbolInfo;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryUsage {
    pub total_text: u64,
    pub total_data: u64,
    pub total_bss: u64,
    pub symbols: Vec<SymbolInfo>,
}

pub fn analyze_memory(symbols: &mut [SymbolInfo]) -> MemoryUsage {
    for i in 0..symbols.len().saturating_sub(1) {
        let next_addr = symbols[i + 1].address;
        symbols[i].size = next_addr.saturating_sub(symbols[i].address);
    }

    let mut usage = MemoryUsage {
        total_text: 0,
        total_data: 0,
        total_bss: 0,
        symbols: symbols.to_vec(),
    };

    for sym in &usage.symbols {
        match sym.section.as_str() {
            ".text" => usage.total_text += sym.size,
            ".data" => usage.total_data += sym.size,
            ".bss" => usage.total_bss += sym.size,
            _ => {}
        }
    }

    usage
}
