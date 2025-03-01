//! # SQLite Application

use anyhow::{bail, Result};
use codecrafters_sqlite::constants::SELECT_PATTERN;
use codecrafters_sqlite::dot_cmd::{dot_dbinfo, dot_indexes, dot_tables};
use codecrafters_sqlite::sql::select;
use std::time::Instant;

fn main() -> Result<()> {
    // Parse arguments
    let args = std::env::args().collect::<Vec<_>>();
    match args.len() {
        0 | 1 => bail!("Missing <database path> and <command>"),
        2 => bail!("Missing <command>"),
        _ => {}
    }
    let db_file_path = &args[1];

    // Parse command and act accordingly
    let command = args[2].trim();

    // SQL commands
    if command.to_lowercase().starts_with(SELECT_PATTERN) {
        let start = Instant::now();
        let result = select(db_file_path, command)?;
        eprintln!("\nTook {:.3?} to complete.", start.elapsed());
        for item in result {
            println!("{}", item);
        }
    } else {
        // CLI (dot) commands
        match command.to_lowercase().as_str() {
            ".dbinfo" => {
                dot_dbinfo(db_file_path)?;
            }
            ".tables" => {
                let names = dot_tables(db_file_path)?;
                for name in names {
                    print!("{name} ");
                }
                println!();
            }
            ".index" | ".indexes" => {
                let names = dot_indexes(db_file_path)?;
                for name in names {
                    print!("{name} ");
                }
                println!();
            }
            _ => bail!("Missing or invalid command passed: {}", command),
        }
    }

    Ok(())
}
