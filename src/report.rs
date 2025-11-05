use crate::analyzer::MemoryUsage;
use std::fs::File;
use std::io::Write;

pub fn export_to_csv(usage: &MemoryUsage, filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    writeln!(file, "Section,Symbol Name,Address,Size (bytes),Type")?;
    for sym in &usage.symbols {
        writeln!(
            file,
            "{},{},{:#X},{} ,{}",
            sym.section, sym.name, sym.address, sym.size, sym.symbol_type
        )?;
    }
    Ok(())
}

pub fn export_to_json(usage: &MemoryUsage, filename: &str) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(usage).unwrap();
    let mut file = File::create(filename)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
