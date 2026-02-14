use crate::utils::{code_to_enum, Status};
use std::process::{exit, Command};

fn is_gitignored(file_name: &str) -> bool {
    let mut check_ignore = Command::new("git");
    check_ignore.arg("check-ignore").arg("-q").arg(file_name);
    check_ignore.status().map(|s| s.success()).unwrap_or(false)
}

pub fn besafe() {
    let mut gs = Command::new("git");
    gs.arg("status").arg("-s");

    let output_raw = gs.output().unwrap();
    let output = String::from_utf8_lossy(&output_raw.stdout);
    let files: Vec<String> = output
        .split("\n")
        .map(|i| i.to_owned())
        .filter(|i| !i.is_empty())
        .collect();

    for file in files.iter() {
        let mut file_split = file.split_whitespace();
        let code = file_split.next().expect("Failed to get file status!");
        let file_name = file_split.next().expect("Failed to get file info!");
        let status = code_to_enum(code);
        if file_name.starts_with(".env") && file_name != ".env.example" {
            let ignored = is_gitignored(file_name);
            match status {
                Status::Staged => {
                    println!("[besafe] {} file is staged! Unstage it and add it to .gitignore before committing.", file_name);
                    exit(1);
                },
                Status::Modified => {
                    println!("[besafe] {} file has changes and is not in .gitignore! Remove it and add it to .gitignore before committing.", file_name);
                    exit(1);
                },
                Status::StagedModified => {
                    println!("[besafe] {} file staged! Unstage it and add it to .gitignore before committing.", file_name);
                    exit(1);
                },
                Status::Untracked => {
                    if ignored {
                        // File is untracked but in .gitignore, safe to ignore
                        continue;
                    } else {
                        println!("[besafe] {} file is not in .gitignore! Consider adding it.", file_name);
                    }
                },
                Status::Unknown => println!("[besafe] {} file found in git history but status cannot be determined! Make sure it doesn't contain any sensitive data before committing", file_name),
            }
        }
    }
    println!("No unsafe .env problems found!");
}
