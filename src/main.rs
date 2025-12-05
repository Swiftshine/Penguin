#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod penguin;
mod savefile;
mod settings;
mod views;

use penguin::PenguinApp;

fn main() -> Result<(), eframe::Error> {
    PenguinApp::run()
}
