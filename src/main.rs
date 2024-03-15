use std::{fs::OpenOptions, path::Path};

use gui::window::SearchWindow;
use iced::{Application, Settings};
use search::index;

mod gui;
mod search;

fn main() -> iced::Result {
    let mut output = OpenOptions::new()
        .append(true)
        .create(true)
        .open("scan.txt")
        .unwrap();
    index::index_folder(Path::new("F:\\SteamLibrary").into(), &mut output).unwrap();
    SearchWindow::run(Settings::default())
}
