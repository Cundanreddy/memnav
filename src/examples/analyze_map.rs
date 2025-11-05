use memnav::{parse_map_file, analyze_memory, export_to_csv};
use std::fs;

fn main() {
    let map_content = fs::read_to_string("app.map").expect("Failed to read map file");
    let mut symbols = parse_map_file(&map_content);
    let usage = analyze_memory(&mut symbols);
    export_to_csv(&usage, "memory_report.csv").unwrap();
    println!("Memory analysis complete!");
}
