use std::fs;
use std::path::Path;

use clap::{Error, ErrorKind, Parser};
use uuid::Uuid;

/// File Randomizer. Randomizes the names of the files in a directory so that they are in a random order.
#[derive(Parser, Debug)]
struct Args {
    /// The path to the directory that you would like to randomize the files of
    path: String,

    /// This CLI operates in "dry-run" mode by default (for safety). Setting this flag to 'true' will actually rename the files.
    #[clap(short, long)]
    execute: bool,
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
        .filter_map(|entry| {
            let entry_path = entry.expect("Trouble reading entry item.").path();

            if !entry_path.is_file() {
                return None;
            }

            if entry_path
                .file_name()
                .expect("Failed to read file name")
                .to_str()
                .expect("Failed to convert file name to string")
                == ".DS_Store"
            {
                return None;
            }

            let new_file_name = Uuid::new_v4().to_string();
            let extension = entry_path
                .extension()
                .expect(format!("All files must have extensions: {:?}", entry_path).as_str())
                .to_str()
                .expect("Trouble converting extension to string")
                .to_string();

            let new_path = Path::new(&entry_path)
                .with_file_name(new_file_name)
                .with_extension(extension);

            return Some((entry_path, new_path));
        })
        // if all files CAN be renamed, then rename them
        .for_each(|(old_path, new_path)| {
            if args.execute {
                fs::rename(old_path, new_path).expect("Failed to rename file");
            } else {
                println!("{} -> {}", old_path.display(), new_path.display());
            }
        });
}
