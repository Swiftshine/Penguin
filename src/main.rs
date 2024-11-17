mod penguin;
mod savefile;
mod views;

use penguin::PenguinApp;

fn main() -> Result<(), eframe::Error> {
    PenguinApp::run()
}
