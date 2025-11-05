use clap::{Parser, Subcommand};
use std::fs;
use memnav::{parse_map_file, analyze_memory, export_to_csv, export_to_json};

#[derive(Parser)]
#[command(name = "memnav")]
#[command(about = "Analyze .map files for per-symbol memory usage")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze a .map file
    Analyze {
        /// Input .map file path
        #[arg(short, long)]
        input: String,

        /// Optional CSV output path
        #[arg(short, long)]
        csv: Option<String>,

        /// Optional JSON output path
        #[arg(short, long)]
        json: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { input, csv, json } => {
            let content = fs::read_to_string(&input).expect("Failed to read map file");
            let mut symbols = memnav::parse_map_file(&content);
            let usage = memnav::analyze_memory(&mut symbols);

            if let Some(csv_path) = csv {
                export_to_csv(&usage, &csv_path).expect("Failed to write CSV");
                println!("✅ CSV report written to {}", csv_path);
            }
            if let Some(json_path) = json {
                export_to_json(&usage, &json_path).expect("Failed to write JSON");
                println!("✅ JSON report written to {}", json_path);
            }

            println!("Summary:\n  .text: {} bytes\n  .data: {} bytes\n  .bss: {} bytes",
                usage.total_text, usage.total_data, usage.total_bss);
        }
    }
}
