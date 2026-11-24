mod cargo_lock;
mod cargo_package;
mod command;
mod dependency;
mod download;
mod error;
mod installation;
mod project;
mod provenance;
mod publication;

use std::process::ExitCode;

fn main() -> ExitCode {
    match command::Command::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
