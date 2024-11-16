mod penguin;
mod savefile;

use penguin::PenguinApp;

fn main() -> Result<(), eframe::Error> {
    PenguinApp::run()
}
