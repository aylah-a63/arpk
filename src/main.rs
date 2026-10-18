// SPDX-License-Identifier: GPL-3.0-or-later

use std::process::ExitCode;

use arpk::cli::Cli;
use clap::Parser;

fn main() -> ExitCode {
    let cli = Cli::parse();
    if cli.global.verbose {
        eprintln!("{cli:#?}");
    }
    match arpk::commands::dispatch(&cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("arpk: {e}");
            ExitCode::FAILURE
        }
    }
}
