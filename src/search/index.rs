//use async_recursion::async_recursion;
use log::{debug, warn};
use std::{
    fs::{read_dir, File},
    io::Write,
    path::PathBuf,
};

//#[async_recursion]
pub fn index_folder(root: PathBuf, logfile: &mut File, padding: String) -> std::io::Result<()> {
    let entries = match read_dir(&root) {
        Ok(entries) => entries,
        Err(err) => match err.kind() {
            std::io::ErrorKind::PermissionDenied => {
                warn!("Skipping {}: Permission denied", root.display());
                return Ok(());
            }
            _ => {
                warn!("Skpping {}: {err}", root.display());
                return Ok(());
            }
        },
    };

    for entry in entries.map_while(Result::ok) {
        let path = entry.path();

        // symlink = skip
        if path.is_symlink() {
            warn!("Skipping {}: Link", path.display());
            continue;
        }

        // file = log
        if path.is_file() {
            logfile.write(
                format!(
                    "{}{}\n",
                    padding,
                    path.file_name().unwrap_or_default().to_string_lossy()
                )
                .as_bytes(),
            )?;
            continue;
        }

        // path = log + index
        if path.is_dir() {
            debug!("Entering {}", path.display());
            // write foldername { when entering new folder
            logfile.write(
                format!(
                    "{}{} {{\n",
                    padding,
                    path.file_name().unwrap_or_default().to_string_lossy()
                )
                .as_bytes(),
            )?;
            // enter folder and start indexing again
            index_folder(path, logfile, padding.clone() + "\t")?;
            // add closing } to folder
            logfile.write((padding.clone() + "}\n").as_bytes())?;
        }
    }
    Ok(())
}
