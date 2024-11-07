use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::io::{self, Write};


pub fn prompt_and_open_folder(path: &PathBuf) {
    println!("Do you want to open the folder at {:?}? (yes/no)", path);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    if input.trim().eq_ignore_ascii_case("yes") {
        open_folder(path);
    } else {
        println!("Folder will not be opened.");
    }
}

fn open_folder(path: &PathBuf) {
    let result = if cfg!(target_os = "windows") {
        Command::new("explorer").arg(path).status()
    } else if cfg!(target_os = "macos") {
        Command::new("open").arg(path).status()
    } else if cfg!(target_os = "linux") {
        Command::new("xdg-open").arg(path).status()
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Unsupported OS"))
    };

    match result {
        Ok(_) => println!("Folder opened successfully."),
        Err(e) => eprintln!("Error opening folder: {}", e),
    }
}

pub fn prompt_and_delete_file(file_path: &PathBuf) {
    println!("Are you sure you want to delete the file at {:?}? (yes/no)", file_path);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    if input.trim().eq_ignore_ascii_case("yes") {
        match fs::remove_file(file_path) {
            Ok(_) => println!("File deleted successfully."),
            Err(e) => eprintln!("Failed to delete file: {}", e),
        }
    } else {
        println!("File will not be deleted.");
    }
}
