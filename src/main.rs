use penguin::PenguinApp;
mod savefile;
mod penguin;

fn main() -> Result<(), eframe::Error> {
    PenguinApp::run()
}
