use gui::window::SearchWindow;
use iced::{Application, Settings};

mod gui;
mod search;

const OUTPUT_PATH: &str = "scan.txt";
const MAX_RESULTS: usize = 1024;

fn main() -> iced::Result {
    pretty_env_logger::init();

    let path = std::path::Path::new(OUTPUT_PATH);
    /*
    if path.exists() {
        std::fs::remove_file(path).unwrap();
    }
    */
    if !path.exists() {
        let mut output = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
            .unwrap();

        let start = std::time::Instant::now();
        search::index::index_folder(
            std::path::Path::new(&std::env::args().collect::<Vec<String>>()[1]).into(),
            &mut output,
            String::new(),
        )
        .unwrap();
        println!(
            "Indexed in: {:?}",
            std::time::Instant::now().duration_since(start)
        );
    }
    SearchWindow::run(Settings::default())
}
