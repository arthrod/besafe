use std::{
    env::current_exe,
    fs::{read_to_string, write},
    path::PathBuf,
};

const PRE_COMMIT_PATH: &str = ".git/hooks/pre-commit";

pub fn install_hook() {
    let path = PathBuf::from(PRE_COMMIT_PATH);
    let mut file = read_to_string(&path).unwrap_or(String::from(""));

    let current_path = current_exe()
        .unwrap_or(PathBuf::new())
        .as_path()
        .to_string_lossy()
        .to_string();

    // Convert Windows paths to Git Bash style (C:/path → /c/path)
    let current_path = if cfg!(windows) {
        let path = current_path.replace("\\", "/");
        if let Some(drive_colon) = path.find(":") {
            let (drive, rest) = path.split_at(drive_colon);
            let drive_lower = drive.to_lowercase();
            format!("/{}/{}", &drive_lower, &rest[1..])
        } else {
            path
        }
    } else {
        current_path
    };

    // Check if besafe is already installed
    if file.contains(&current_path) {
        println!("besafe is already installed in this repository!");
        return;
    }

    file.push_str(&format!("\n\n{}", current_path));

    if !file.starts_with("#!/") {
        file = String::from("#!/bin/sh\n\n") + &file;
    }

    write(path, file).expect("Failed to install pre-commit hook!");

    println!("Successfully installed pre-commit hook!");
}
