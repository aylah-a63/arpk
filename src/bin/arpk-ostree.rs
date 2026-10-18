// SPDX-License-Identifier: GPL-3.0-or-later

use std::process::ExitCode;

use clap::{Parser, Subcommand};

/// Snapshot backend for `arpk history rollback` (btrfs root required)
#[derive(Debug, Parser)]
#[command(name = "arpk-ostree", version)]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Debug, Subcommand)]
enum Cmd {
    /// Show the booted snapshot and the next boot target
    Status,
    /// List snapshots with ids, dates and linked transaction ids
    List,
    /// Take a snapshot now
    Create {
        #[arg(long, value_name = "TEXT")]
        label: Option<String>,
    },
    /// Delete a snapshot (confirmation required)
    Delete { id: u64 },
    /// Set a snapshot as the next boot target
    Boot { id: u64 },
    /// Regenerate boot entries for all retained snapshots
    BootEntries,
    /// Delete old snapshots, keeping the last N
    Prune {
        #[arg(long, value_name = "N")]
        keep: Option<u32>,
    },
}

fn main() -> ExitCode {
    let _cli = Cli::parse();
    eprintln!("arpk-ostree: not implemented yet");
    ExitCode::FAILURE
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn command_tree_is_valid() {
        Cli::command().debug_assert();
    }

    #[test]
    fn boot_entries_is_kebab_case() {
        let c = Cli::try_parse_from(["arpk-ostree", "boot-entries"]).unwrap();
        assert!(matches!(c.command, Cmd::BootEntries));
    }
}
