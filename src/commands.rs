// SPDX-License-Identifier: GPL-3.0-or-later

use crate::cli::{Cli, Command};

pub fn dispatch(cli: &Cli) -> Result<(), String> {
    let name = match &cli.command {
        Command::Install(_) => "install",
        Command::Remove(_) => "remove",
        Command::Upgrade(_) => "upgrade",
        Command::Refresh(_) => "refresh",
        Command::Search(_) => "search",
        Command::Info(_) => "info",
        Command::List(_) => "list",
        Command::Provides(_) => "provides",
        Command::Autoremove(_) => "autoremove",
        Command::Reinstall(_) => "reinstall",
        Command::Downgrade(_) => "downgrade",
        Command::History(_) => "history",
        Command::Group { .. }
        | Command::Grouplist(_)
        | Command::Groupinstall(_)
        | Command::Groupremove(_)
        | Command::Groupupdate(_) => "group",
        Command::Clean(_) => "clean",
        Command::Check(_) => "check",
        Command::Verify(_) => "verify",
        Command::Build(_) => "build",
        Command::Aur { .. } => "aur",
        Command::Animation { .. } => "animation",
        Command::Config { .. } => "config",
    };
    Err(format!("'{name}' is not implemented yet"))
}
