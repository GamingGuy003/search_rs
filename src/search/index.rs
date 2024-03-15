use std::{
    fs::{read_dir, File},
    io::Write,
    path::PathBuf,
};

pub fn index_folder(root: PathBuf, logfile: &mut File) -> std::io::Result<()> {
    let entries = match read_dir(&root) {
        Ok(entries) => entries,
        Err(err) => match err.kind() {
            std::io::ErrorKind::PermissionDenied => {
                println!("Skipping {}: Permission denied", root.display());
                return Ok(());
            }
            _ => {
                println!("Skpping {}: {err}", root.display());
                return Ok(());
            }
        },
    };
    for entry in entries.map_while(Result::ok) {
        if entry.path().is_symlink() {
            println!("Skipping {}: Link", entry.path().display());
            continue;
        }
        if entry.path().is_file() {
            logfile.write(format!("{}\n", entry.path().display()).as_bytes())?;
        } else {
            println!("Entering {}", entry.path().display());
            index_folder(entry.path(), logfile)?
        }
    }
    Ok(())
}
