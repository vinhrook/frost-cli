use chrono::Local;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

const HISTORY_FILE: &str = "history.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    pub timestamp: String,
    pub from_val: f64,
    pub from_unit: String,
    pub to_val: f64,
    pub to_unit: String,
}

pub fn save(from_val: f64, from_unit: &str, to_val: f64, to_unit: &str) {
    let mut history = load_history();

    let new_record = Record {
        timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        from_val,
        from_unit: from_unit.to_uppercase(),
        to_val,
        to_unit: to_unit.to_uppercase(),
    };

    history.push(new_record);

    if let Err(e) = write_history(&history) {
        eprintln!("{} Failed to save history: {}", "⚠️ Warning:".yellow(), e);
    }
}

pub fn show() {
    let history = load_history();

    if history.is_empty() {
        println!("{}", "📜 No conversion history recorded yet.".yellow());
        return;
    }

    println!("\n{}", "📜 Conversion History".cyan().bold());
    println!("{}", "==========================================".cyan());

    for (idx, record) in history.iter().enumerate() {
        println!(
            "{:2}. [{}] {:.2}°{} ➡️  {:.2}°{}",
            idx + 1,
            record.timestamp.dimmed(),
            record.from_val,
            record.from_unit.bold(),
            record.to_val,
            record.to_unit.bold()
        );
    }
    println!("{}", "==========================================".cyan());
    println!("Total logs: {}\n", history.len().to_string().green().bold());
}

fn load_history() -> Vec<Record> {
    if !Path::new(HISTORY_FILE).exists() {
        return Vec::new();
    }

    let mut file = match File::open(HISTORY_FILE) {
        Ok(file) => file,
        Err(_) => return Vec::new(),
    };

    let mut content = String::new();
    if file.read_to_string(&mut content).is_err() {
        return Vec::new();
    }

    serde_json::from_str(&content).unwrap_or_default()
}

fn write_history(records: &[Record]) -> Result<(), Box<dyn std::error::Error>> {
    let json_data = serde_json::to_string_pretty(records)?;
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(HISTORY_FILE)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}
