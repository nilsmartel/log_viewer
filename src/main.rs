use colored::*;
use std::io;
use std::io::prelude::*;
use std::io::stdout;

fn main() {
    let stdin = io::stdin();
    let stdout = stdout();
    let mut out = stdout.lock();

    for line in stdin.lock().lines() {
        let line = line.expect("Reading line failed");
        let json = json::parse(&line);
        if let Ok(json) = json {
            let level = &json["level"];
            if level == "info" {
                write!(out, "{}\n", line.cyan());
            } else if level == "warn" {
                write!(out, "{}\n", line.yellow());
            } else if level == "danger" {
                write!(out, "{}\n", line.red());
            } else {
                write!(out, "{}\n", line);
            }

            continue;
        }

        write!(out, "{}\n", line.bright_black());
    }
}
