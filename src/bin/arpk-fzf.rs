// SPDX-License-Identifier: GPL-3.0-or-later

use std::process::ExitCode;

use clap::Parser;

/// Fuzzy package picker; runs `arpk install` or `arpk remove` on the selection
#[derive(Debug, Parser)]
#[command(name = "arpk-fzf", version)]
struct Cli {
    /// Pick from AUR packages only
    #[arg(long)]
    aur: bool,
    /// Limit to one repo
    #[arg(long, value_name = "NAME")]
    repo: Option<String>,
    /// Pick from installed packages
    #[arg(long)]
    installed: bool,
    /// Select installed packages to remove (implies --installed)
    #[arg(long)]
    remove: bool,
    /// Start fzf with this filter
    #[arg(long, value_name = "TEXT")]
    query: Option<String>,
    /// Hide the info pane
    #[arg(long)]
    no_preview: bool,
    /// Pass extra arguments to fzf
    #[arg(long, value_name = "STR")]
    fzf_args: Option<String>,
    /// Print the chosen packages and exit without running arpk
    #[arg(long)]
    dry_run: bool,
}

fn main() -> ExitCode {
    let _cli = Cli::parse();
    eprintln!("arpk-fzf: not implemented yet");
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
    fn query_has_no_short_flag() {
        assert!(Cli::try_parse_from(["arpk-fzf", "-q", "x"]).is_err());
        let c = Cli::try_parse_from(["arpk-fzf", "--query", "fire"]).unwrap();
        assert_eq!(c.query.as_deref(), Some("fire"));
    }
}
