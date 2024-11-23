mod penguin;
mod savefile;
mod views;
mod settings;

use penguin::PenguinApp;

fn main() -> Result<(), eframe::Error> {
    PenguinApp::run()
}
