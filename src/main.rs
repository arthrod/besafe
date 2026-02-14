use besafe::besafe;
use clap::Command;
use install::install_hook;

mod besafe;
mod install;
mod utils;

fn main() {
    let install = Command::new("install");

    let cli = Command::new("besafe")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("A simple Git pre-commit hook for preventing commiting of .env files!")
        .subcommand(install);

    let matches = cli.get_matches();

    match matches.subcommand_name() {
        Some("install") => install_hook(),
        None => {
            println!("[besafe] Checking...");
            besafe();
        }
        Some(_) => {}
    }
}
