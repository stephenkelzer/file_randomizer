use std::fs;
use std::path::Path;

use clap::{Error, ErrorKind, Parser};
use uuid::Uuid;

/// File Randomizer. Randomizes the names of the files in a directory so that they are in a random order.
#[derive(Parser, Debug)]
#[clap(author, about, long_about = None)]
struct Args {
    /// The path to the directory that you would like to randomize the files of
    path: String,
}

fn main() {
    let args = Args::parse();

    let path = Path::new(&args.path);

    if !path.is_dir() {
        Error::raw(ErrorKind::Io, "The path must be a directory.").exit();
    }

    path.read_dir()
        .expect("Trouble reading path")
        // first, validate that all files CAN be renamed
        .map(|entry| {
            let entry_path = entry.expect("Trouble reading entry item.").path();

            if !entry_path.is_file() {
                Error::raw(ErrorKind::Io, "Directory must only contain files.").exit();
            }

            let new_file_name = Uuid::new_v4().to_string();
            let extension = entry_path
                .extension()
                .expect("All files must have extensions")
                .to_str()
                .expect("Trouble converting extension to string")
                .to_string();

            let new_path = Path::new(&entry_path)
                .with_file_name(new_file_name)
                .with_extension(extension);

            return (entry_path, new_path);
        })
        // if all files CAN be renamed, then rename them
        .for_each(|(old_path, new_path)| {
            fs::rename(old_path, new_path).expect("Failed to rename file");
        });
}
