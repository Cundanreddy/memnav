
// src/main.rs
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use csv::Writer;

#[derive(Debug)]
struct Entry {
    section: String,
    address: Option<String>,
    size: Option<String>,
    symbol: Option<String>,
    source_file: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: memnav <map-file> [out.csv]");
        std::process::exit(2);
    }
    let map_path = &args[1];
    let out_path = if args.len() >= 3 {
        args[2].clone()
    } else {
        format!("{}-memnav.csv", Path::new(map_path).file_name().unwrap().to_string_lossy())
    };

    let f = File::open(map_path)?;
    let reader = BufReader::new(f);

    // Regexes:
    // 1) file path with .o (and possibly size fields): lines that contain a path ending with .o
    let re_file_line = Regex::new(r"(?P<path>[A-Za-z]:?[/\\][^\s]*?\.o\b|[^\s]*\.o\b|[^\s]*\.a\([^\)]+\))").unwrap();
    // 2) symbol line: optional 0xADDR, optional size, then symbol (name)
    // Typical lines: "0x60009210      0x95f D:/.../c0_audioprocess.o" or "0x600091d4                au_postprocess"
    let re_symbol_line = Regex::new(r"^\s*(?P<addr>0x[0-9a-fA-F]+)?\s*(?P<size>0x[0-9a-fA-F]+|\d+)?\s*(?P<sym>[A-Za-z_][\w\.\$@<>+-]*)\s*$").unwrap();
    // An alternative more permissive symbol name capture if needed:
    // let re_symbol_line = Regex::new(r"^\s*(?P<addr>0x[0-9a-fA-F]+)?\s*(?P<size>0x[0-9a-fA-F]+|\d+)?\s*(?P<sym>\S+)\s*$").unwrap();

    let mut current_section = String::new();
    let mut current_obj: Option<String> = None;
    let mut writer = Writer::from_path(&out_path)?;

    // Header
    writer.write_record(&["section", "address", "size", "symbol", "source_file"])?;

    // Keep a small lookback buffer for archive -> real-path mapping lines
    let mut prev_line: Option<String> = None;

    for raw in reader.lines() {
        let line = raw?;
        // If line looks like a section header: starts with "." and then section name
        if let Some(sec_caps) = line.trim_start().split_whitespace().next() {
            if sec_caps.starts_with('.') {
                // update current_section
                // Example: ".rodata         0x60000280      0x3b8"
                let sec_name = sec_caps.to_string();
                current_section = sec_name;
                // If this line also contains a file path (some lines do), capture it:
                if let Some(cap) = re_file_line.captures(&line) {
                    current_obj = Some(cap["path"].to_string());
                }
                prev_line = Some(line.clone());
                continue;
            }
        }

        // Check for a file path occurrence (object file or archive member)
        if let Some(cap) = re_file_line.captures(&line) {
            let path = cap["path"].to_string();
            // If it's an archive member like libfoo.a(member.o) we keep that literal,
            // but prefer a following real path if present (we'll handle mapping using prev_line or next lines heuristics).
            current_obj = Some(path);
            prev_line = Some(line.clone());
            continue;
        }

        // Check for symbol lines: address + symbol OR just symbol lines (indented)
        if let Some(sym_caps) = re_symbol_line.captures(&line) {
            // sym might be present even if no address; discard matches where sym looks like hex-only
            if let Some(sym) = sym_caps.name("sym") {
                // filter out section names captured as symbol incorrectly:
                let symbol = sym.as_str().to_string();
                // If symbol is a known section marker (rare), skip.
                if symbol.starts_with('.') {
                    prev_line = Some(line.clone());
                    continue;
                }

                let address = sym_caps.name("addr").map(|m| m.as_str().to_string());
                let size = sym_caps.name("size").map(|m| m.as_str().to_string());

                let entry = Entry {
                    section: current_section.clone(),
                    address,
                    size,
                    symbol: Some(symbol),
                    source_file: current_obj.clone(),
                };
                writer.serialize((&entry.section, &entry.address, &entry.size, &entry.symbol, &entry.source_file))?;
            }
            prev_line = Some(line.clone());
            continue;
        }

        prev_line = Some(line.clone());
    }

    writer.flush()?;
    println!("CSV written to {}", out_path);
    Ok(())
}
