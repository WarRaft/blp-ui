#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::run::run;

mod run;
mod ui;
mod ext;

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
