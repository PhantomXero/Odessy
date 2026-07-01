mod app;
mod persistence;
mod ui;

fn main() {
    if let Err(err) = app::run() {
        eprintln!("Application error: {err}");
    }
}
