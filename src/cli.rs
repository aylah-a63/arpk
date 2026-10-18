// SPDX-License-Identifier: GPL-3.0-or-later

use std::path::PathBuf;

use clap::{ArgGroup, Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "arpk", version, about = "dnf-style package front end for Arch Linux")]
pub struct Cli {
    #[command(flatten)]
    pub global: GlobalOpts,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Args)]
pub struct GlobalOpts {
    /// Answer yes to prompts
    #[arg(short = 'y', long, visible_alias = "noconfirm", global = true, conflicts_with = "assumeno")]
    pub assumeyes: bool,
    /// Answer no to prompts (dry run)
    #[arg(long, global = true)]
    pub assumeno: bool,
    /// Minimal output
    #[arg(short, long, global = true, conflicts_with = "verbose")]
    pub quiet: bool,
    /// More output
    #[arg(short, long, global = true)]
    pub verbose: bool,
    /// Refresh sync databases first
    #[arg(long, global = true)]
    pub refresh: bool,
    /// Use only cached data, no network
    #[arg(short = 'C', long, global = true)]
    pub cacheonly: bool,
    /// Limit to one repository
    #[arg(long, global = true, value_name = "NAME")]
    pub repo: Option<String>,
    /// Enable a repo for this run
    #[arg(long, global = true, value_name = "NAME")]
    pub enablerepo: Vec<String>,
    /// Disable a repo for this run
    #[arg(long, global = true, value_name = "NAME")]
    pub disablerepo: Vec<String>,
    /// Never touch this package
    #[arg(long, global = true, value_name = "PKG")]
    pub exclude: Vec<String>,
    /// Restrict the operation to AUR packages
    #[arg(long, global = true, conflicts_with = "no_aur")]
    pub aur: bool,
    /// Ignore AUR entirely
    #[arg(long, global = true)]
    pub no_aur: bool,
    /// Set a config key for this run, e.g. downloads.parallel=4
    #[arg(long, global = true, value_name = "KEY=VALUE", value_parser = parse_setopt)]
    pub setopt: Vec<(String, String)>,
    /// Alternate database directory
    #[arg(long, global = true, value_name = "PATH")]
    pub dbpath: Option<PathBuf>,
    /// Alternate package cache
    #[arg(long, global = true, value_name = "PATH")]
    pub cachedir: Option<PathBuf>,
    /// Operate on another root filesystem
    #[arg(short = 'r', long, visible_alias = "root", global = true, value_name = "PATH")]
    pub installroot: Option<PathBuf>,
    /// Disable coloured output
    #[arg(long, global = true, conflicts_with = "color")]
    pub nocolor: bool,
    #[arg(long, global = true, value_name = "WHEN")]
    pub color: Option<Color>,
    /// Debug verbosity
    #[arg(long, global = true, value_name = "N")]
    pub debuglevel: Option<u8>,
    /// Concurrent repo downloads for this run
    #[arg(long, global = true, value_name = "N", value_parser = clap::value_parser!(u32).range(1..))]
    pub parallel: Option<u32>,
    /// Allow concurrent AUR builds for this run
    #[arg(long, global = true)]
    pub parallel_build: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Color {
    Auto,
    Always,
    Never,
}

fn parse_setopt(s: &str) -> Result<(String, String), String> {
    match s.split_once('=') {
        Some((k, v)) if !k.is_empty() => Ok((k.to_string(), v.to_string())),
        _ => Err(format!("expected KEY=VALUE, got '{s}'")),
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(visible_aliases = ["in", "i"])]
    Install(InstallArgs),
    #[command(visible_aliases = ["rm", "erase", "e"])]
    Remove(RemoveArgs),
    #[command(visible_aliases = ["up", "update", "u"])]
    Upgrade(UpgradeArgs),
    #[command(visible_aliases = ["makecache", "mc"])]
    Refresh(RefreshArgs),
    #[command(visible_aliases = ["s", "find"])]
    Search(SearchArgs),
    #[command(visible_alias = "if")]
    Info(InfoArgs),
    #[command(visible_alias = "ls")]
    List(ListArgs),
    #[command(visible_aliases = ["wp", "whatprovides", "prov"])]
    Provides(ProvidesArgs),
    /// Remove every orphaned dependency on the system
    #[command(visible_alias = "ar")]
    Autoremove(AutoremoveArgs),
    #[command(visible_alias = "ri")]
    Reinstall(ReinstallArgs),
    #[command(visible_alias = "dg")]
    Downgrade(DowngradeArgs),
    #[command(visible_aliases = ["hi", "hist"])]
    History(HistoryArgs),
    #[command(visible_alias = "grp")]
    Group {
        #[command(subcommand)]
        command: GroupCmd,
    },
    #[command(hide = true, alias = "groups")]
    Grouplist(GroupListArgs),
    #[command(hide = true)]
    Groupinstall(GroupInstallArgs),
    #[command(hide = true)]
    Groupremove(GroupNames),
    #[command(hide = true)]
    Groupupdate(GroupInstallArgs),
    #[command(visible_alias = "cl")]
    Clean(CleanArgs),
    #[command(visible_alias = "ck")]
    Check(CheckArgs),
    #[command(visible_alias = "vf")]
    Verify(VerifyArgs),
    #[command(visible_alias = "bd")]
    Build(BuildArgs),
    /// AUR operations
    Aur {
        #[command(subcommand)]
        command: AurCmd,
    },
    /// Choose the progress indicator
    Animation {
        #[command(subcommand)]
        command: AnimationCmd,
    },
    /// Read and write the .arpk configuration
    Config {
        #[command(subcommand)]
        command: ConfigCmd,
    },
}

#[derive(Debug, Args)]
pub struct InstallArgs {
    #[arg(required = true, value_name = "PKG")]
    pub packages: Vec<String>,
    /// Skip packages already at the requested version
    #[arg(long)]
    pub needed: bool,
    /// Mark as a dependency
    #[arg(long, conflicts_with = "asexplicit")]
    pub asdeps: bool,
    /// Mark as explicitly installed
    #[arg(long)]
    pub asexplicit: bool,
    /// Fetch packages, don't install
    #[arg(long)]
    pub downloadonly: bool,
    /// Allow overwriting matching files
    #[arg(long, value_name = "GLOB")]
    pub overwrite: Vec<String>,
    /// Skip a package during the upgrade
    #[arg(long, value_name = "PKG")]
    pub ignore: Vec<String>,
    /// Don't install weak dependencies
    #[arg(long)]
    pub no_weak_deps: bool,
    /// Install optional dependencies
    #[arg(long)]
    pub with_optional: bool,
    /// Skip packages with unresolved deps
    #[arg(long)]
    pub skip_broken: bool,
    /// Permit removing conflicting packages
    #[arg(long)]
    pub allowerasing: bool,
    /// Require the newest version or fail
    #[arg(long)]
    pub best: bool,
    /// Explicitly skip database sync (default)
    #[arg(long, conflicts_with = "refresh")]
    pub no_refresh: bool,
}

#[derive(Debug, Args)]
pub struct RemoveArgs {
    #[arg(required = true, value_name = "PKG")]
    pub packages: Vec<String>,
    /// Remove dependencies nothing else needs (-Rs)
    #[arg(long)]
    pub clean_deps: bool,
    /// Remove packages that depend on the target (-Rc)
    #[arg(long)]
    pub recursive: bool,
    /// Don't keep .pacsave config backups
    #[arg(long)]
    pub nosave: bool,
    /// Skip dependency checks (dangerous)
    #[arg(long)]
    pub nodeps: bool,
}

#[derive(Debug, Args)]
pub struct UpgradeArgs {
    /// Only security updates
    #[arg(long)]
    pub security: bool,
    /// Only bugfix updates
    #[arg(long)]
    pub bugfix: bool,
    /// Upgrade only to the minimal version that fixes the issue
    #[arg(long)]
    pub minimal: bool,
    /// Skip packages that break the transaction
    #[arg(long)]
    pub skip_broken: bool,
    /// Fetch only
    #[arg(long)]
    pub downloadonly: bool,
    /// Rebuild AUR packages even if unchanged
    #[arg(long)]
    pub rebuild_aur: bool,
    /// Show PKGBUILD diffs before rebuilding
    #[arg(long)]
    pub diff_aur: bool,
}

#[derive(Debug, Args)]
pub struct RefreshArgs {
    /// Force a full re-download (-Syy)
    #[arg(long)]
    pub force: bool,
    /// Refresh the files database (-Fy)
    #[arg(long)]
    pub files: bool,
}

#[derive(Debug, Args)]
pub struct SearchArgs {
    #[arg(required = true, value_name = "TERM")]
    pub terms: Vec<String>,
    /// Search installed packages only
    #[arg(long)]
    pub installed: bool,
    /// Match names only
    #[arg(long, conflicts_with = "all")]
    pub name: bool,
    /// Match names and descriptions
    #[arg(long)]
    pub all: bool,
}

#[derive(Debug, Args)]
pub struct InfoArgs {
    #[arg(required = true, value_name = "PKG")]
    pub packages: Vec<String>,
    /// Show local data
    #[arg(long)]
    pub installed: bool,
}

#[derive(Debug, Args)]
#[command(group(ArgGroup::new("selector").args([
    "installed", "available", "upgrades", "extras", "foreign",
    "native", "all", "explicit", "deps", "files",
])))]
pub struct ListArgs {
    /// Installed packages
    #[arg(long)]
    pub installed: bool,
    /// Packages available to install
    #[arg(long)]
    pub available: bool,
    /// Pending upgrades
    #[arg(long, visible_alias = "updates")]
    pub upgrades: bool,
    /// Installed but not in any enabled repo
    #[arg(long)]
    pub extras: bool,
    /// Installed from outside the repos (AUR)
    #[arg(long)]
    pub foreign: bool,
    /// Installed from the repos
    #[arg(long)]
    pub native: bool,
    /// Everything
    #[arg(long)]
    pub all: bool,
    /// Explicitly installed
    #[arg(long)]
    pub explicit: bool,
    /// Installed as dependencies
    #[arg(long)]
    pub deps: bool,
    /// Files owned by an installed package
    #[arg(long, value_name = "PKG")]
    pub files: Option<String>,
}

#[derive(Debug, Args)]
pub struct ProvidesArgs {
    #[arg(required = true, value_name = "TARGET")]
    pub targets: Vec<String>,
    /// Check local DB only
    #[arg(long, conflicts_with = "files")]
    pub installed: bool,
    /// Search the files database
    #[arg(long)]
    pub files: bool,
}

#[derive(Debug, Args)]
pub struct AutoremoveArgs {
    /// Show what would go, don't remove
    #[arg(long)]
    pub dry_run: bool,
    /// Don't keep .pacsave backups
    #[arg(long)]
    pub nosave: bool,
}

#[derive(Debug, Args)]
pub struct ReinstallArgs {
    #[arg(required = true, value_name = "PKG")]
    pub packages: Vec<String>,
    /// Use the cached package, don't fetch
    #[arg(long)]
    pub from_cache: bool,
    /// Allow file overwrites
    #[arg(long, value_name = "GLOB")]
    pub overwrite: Vec<String>,
}

#[derive(Debug, Args)]
pub struct DowngradeArgs {
    #[arg(required = true, value_name = "PKG=VERSION", value_parser = parse_pkg_version)]
    pub targets: Vec<PkgVersion>,
    /// Use only cached versions
    #[arg(long)]
    pub from_cache: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PkgVersion {
    pub name: String,
    pub version: String,
}

fn parse_pkg_version(s: &str) -> Result<PkgVersion, String> {
    match s.split_once('=') {
        Some((n, v)) if !n.is_empty() && !v.is_empty() => Ok(PkgVersion {
            name: n.to_string(),
            version: v.to_string(),
        }),
        _ => Err(format!("expected PKG=VERSION, got '{s}'")),
    }
}

#[derive(Debug, Args)]
pub struct HistoryArgs {
    /// Defaults to `list`
    #[command(subcommand)]
    pub command: Option<HistoryCmd>,
    /// Oldest first
    #[arg(long, global = true)]
    pub reverse: bool,
    /// Show only the most recent N
    #[arg(long, global = true, value_name = "N")]
    pub limit: Option<u32>,
}

#[derive(Debug, Subcommand)]
pub enum HistoryCmd {
    /// List transactions
    List,
    /// Show one transaction
    Info { id: u64 },
    /// Reverse one transaction
    Undo { id: u64 },
    /// Reapply one transaction
    Redo { id: u64 },
    /// Restore the state after transaction ID
    Rollback { id: u64 },
    /// Keep a transaction's cached packages from being pruned
    Pin { id: u64 },
    /// Remove the pin
    Unpin { id: u64 },
    /// List explicitly installed packages
    Userinstalled,
    /// Remove old cached packages, keeping the last N transactions
    Prune {
        #[arg(long, value_name = "N")]
        keep: Option<u32>,
    },
}

#[derive(Debug, Subcommand)]
pub enum GroupCmd {
    /// List groups
    List(GroupListArgs),
    /// Show a group's members
    Info { name: String },
    /// Install a group
    #[command(visible_alias = "in")]
    Install(GroupInstallArgs),
    /// Remove a group
    #[command(visible_alias = "rm")]
    Remove(GroupNames),
    /// Upgrade a group
    Upgrade(GroupInstallArgs),
}

#[derive(Debug, Args)]
pub struct GroupListArgs {
    /// Include hidden groups
    #[arg(long)]
    pub hidden: bool,
}

#[derive(Debug, Args)]
pub struct GroupNames {
    #[arg(required = true, value_name = "NAME")]
    pub names: Vec<String>,
}

#[derive(Debug, Args)]
pub struct GroupInstallArgs {
    #[command(flatten)]
    pub names: GroupNames,
    /// Include optional members
    #[arg(long)]
    pub with_optional: bool,
}

#[derive(Debug, Args)]
#[command(group(ArgGroup::new("what").required(true).multiple(true).args([
    "all", "packages", "metadata", "expire_cache", "aur_cache", "builds",
])))]
pub struct CleanArgs {
    /// Clear everything below
    #[arg(long)]
    pub all: bool,
    /// Clear the package cache
    #[arg(long)]
    pub packages: bool,
    /// Clear sync databases
    #[arg(long)]
    pub metadata: bool,
    /// Mark caches stale without deleting
    #[arg(long)]
    pub expire_cache: bool,
    /// Clear AUR clones and metadata
    #[arg(long)]
    pub aur_cache: bool,
    /// Remove old makepkg working directories
    #[arg(long)]
    pub builds: bool,
    /// Keep the most recent version of each package
    #[arg(long)]
    pub keep_current: bool,
}

#[derive(Debug, Args)]
pub struct CheckArgs {
    /// Run every check
    #[arg(long)]
    pub all: bool,
    /// Check for broken dependencies
    #[arg(long)]
    pub dependencies: bool,
    /// Check for duplicate packages
    #[arg(long)]
    pub duplicates: bool,
    /// Check database consistency (-Dk)
    #[arg(long)]
    pub database: bool,
}

#[derive(Debug, Args)]
pub struct VerifyArgs {
    #[arg(value_name = "PKG", conflicts_with = "all")]
    pub packages: Vec<String>,
    /// Verify every installed package
    #[arg(long)]
    pub all: bool,
    /// Compare checksums, not only metadata (-Qkk)
    #[arg(long)]
    pub deep: bool,
}

#[derive(Debug, Args)]
pub struct BuildArgs {
    /// Directory containing the PKGBUILD
    #[arg(default_value = ".")]
    pub path: PathBuf,
    /// Remove the work directory first
    #[arg(long)]
    pub clean: bool,
    /// Skip check()
    #[arg(long)]
    pub nocheck: bool,
    /// Build in a clean chroot
    #[arg(long)]
    pub chroot: bool,
    /// Don't remove build dependencies after
    #[arg(long)]
    pub keep_deps: bool,
    /// Sign the built package with GPGKEY
    #[arg(long)]
    pub sign: bool,
}

#[derive(Debug, Subcommand)]
pub enum AurCmd {
    /// Search the AUR
    Search {
        #[arg(required = true)]
        terms: Vec<String>,
    },
    /// Show AUR metadata
    Info {
        #[arg(required = true)]
        packages: Vec<String>,
    },
    /// Show the PKGBUILD and install scripts
    Review {
        package: String,
        /// Diff against the last reviewed commit
        #[arg(long, conflicts_with = "since")]
        diff: bool,
        /// Diff from a specific commit
        #[arg(long, value_name = "COMMIT")]
        since: Option<String>,
    },
    /// List AUR packages with newer upstream commits
    Outdated,
    /// Manage trusted maintainer keys
    Trust {
        #[command(subcommand)]
        command: TrustCmd,
    },
    /// Remove AUR caches
    Clean,
}

#[derive(Debug, Subcommand)]
pub enum TrustCmd {
    List,
    Add { key: String },
    Remove { key: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Animation {
    Braille,
    Minimal,
}

#[derive(Debug, Subcommand)]
pub enum AnimationCmd {
    /// List available styles
    List,
    /// Write ui.animation to ~/.arpk/config.toml
    Set { name: Animation },
    /// Play a style for a few seconds
    Preview { name: Option<Animation> },
}

#[derive(Debug, Subcommand)]
pub enum ConfigCmd {
    /// Show which config files are loaded, in precedence order
    Path,
    /// Show every effective key and where its value comes from
    List,
    /// Print one value
    Get { key: String },
    /// Write one value to ~/.arpk/config.toml
    Set { key: String, value: String },
    /// Remove a key so the default applies
    Unset { key: String },
    /// Check the config files for errors
    Validate,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    fn parse(args: &[&str]) -> Result<Cli, clap::Error> {
        Cli::try_parse_from(std::iter::once("arpk").chain(args.iter().copied()))
    }

    #[test]
    fn command_tree_is_valid() {
        Cli::command().debug_assert();
    }

    #[test]
    fn aliases_resolve() {
        for a in ["install", "in", "i"] {
            assert!(matches!(parse(&[a, "x"]).unwrap().command, Command::Install(_)), "{a}");
        }
        for a in ["remove", "rm", "erase", "e"] {
            assert!(matches!(parse(&[a, "x"]).unwrap().command, Command::Remove(_)), "{a}");
        }
        for a in ["upgrade", "up", "update", "u"] {
            assert!(matches!(parse(&[a]).unwrap().command, Command::Upgrade(_)), "{a}");
        }
        for a in ["refresh", "makecache", "mc"] {
            assert!(matches!(parse(&[a]).unwrap().command, Command::Refresh(_)), "{a}");
        }
        for a in ["search", "s", "find"] {
            assert!(matches!(parse(&[a, "x"]).unwrap().command, Command::Search(_)), "{a}");
        }
        for a in ["info", "if"] {
            assert!(matches!(parse(&[a, "x"]).unwrap().command, Command::Info(_)), "{a}");
        }
        for a in ["list", "ls"] {
            assert!(matches!(parse(&[a]).unwrap().command, Command::List(_)), "{a}");
        }
        for a in ["provides", "wp", "whatprovides", "prov"] {
            assert!(matches!(parse(&[a, "x"]).unwrap().command, Command::Provides(_)), "{a}");
        }
        for a in ["autoremove", "ar"] {
            assert!(matches!(parse(&[a]).unwrap().command, Command::Autoremove(_)), "{a}");
        }
        for a in ["reinstall", "ri"] {
            assert!(matches!(parse(&[a, "x"]).unwrap().command, Command::Reinstall(_)), "{a}");
        }
        for a in ["downgrade", "dg"] {
            assert!(matches!(parse(&[a, "x=1"]).unwrap().command, Command::Downgrade(_)), "{a}");
        }
        for a in ["history", "hi", "hist"] {
            assert!(matches!(parse(&[a]).unwrap().command, Command::History(_)), "{a}");
        }
        for a in ["group", "grp"] {
            assert!(matches!(parse(&[a, "list"]).unwrap().command, Command::Group { .. }), "{a}");
        }
        for a in ["clean", "cl"] {
            assert!(matches!(parse(&[a, "--all"]).unwrap().command, Command::Clean(_)), "{a}");
        }
        for a in ["check", "ck"] {
            assert!(matches!(parse(&[a]).unwrap().command, Command::Check(_)), "{a}");
        }
        for a in ["verify", "vf"] {
            assert!(matches!(parse(&[a]).unwrap().command, Command::Verify(_)), "{a}");
        }
        for a in ["build", "bd"] {
            assert!(matches!(parse(&[a]).unwrap().command, Command::Build(_)), "{a}");
        }
    }

    #[test]
    fn group_aliases() {
        let Command::Group { command: GroupCmd::Install(g) } = parse(&["grp", "in", "gnome"]).unwrap().command
        else {
            panic!("grp in")
        };
        assert_eq!(g.names.names, ["gnome"]);
        assert!(matches!(
            parse(&["grp", "rm", "gnome"]).unwrap().command,
            Command::Group { command: GroupCmd::Remove(_) }
        ));
        assert!(matches!(parse(&["groups"]).unwrap().command, Command::Grouplist(_)));
        assert!(matches!(parse(&["grouplist"]).unwrap().command, Command::Grouplist(_)));
        assert!(matches!(parse(&["groupinstall", "x"]).unwrap().command, Command::Groupinstall(_)));
        assert!(matches!(parse(&["groupremove", "x"]).unwrap().command, Command::Groupremove(_)));
        assert!(matches!(parse(&["groupupdate", "x"]).unwrap().command, Command::Groupupdate(_)));
    }

    #[test]
    fn conflicts_rejected() {
        assert!(parse(&["-y", "--assumeno", "upgrade"]).is_err());
        assert!(parse(&["--aur", "--no-aur", "upgrade"]).is_err());
        assert!(parse(&["-q", "-v", "upgrade"]).is_err());
        assert!(parse(&["aur", "review", "x", "--diff", "--since", "abc"]).is_err());
        assert!(parse(&["install", "x", "--refresh", "--no-refresh"]).is_err());
        assert!(parse(&["list", "--foreign", "--native"]).is_err());
        assert!(parse(&["install", "x", "--asdeps", "--asexplicit"]).is_err());
    }

    #[test]
    fn required_args() {
        assert!(parse(&["install"]).is_err());
        assert!(parse(&["clean"]).is_err());
        assert!(parse(&["history", "undo"]).is_err());
    }

    #[test]
    fn downgrade_syntax() {
        let Command::Downgrade(d) = parse(&["dg", "firefox=128.0-1"]).unwrap().command else {
            panic!()
        };
        assert_eq!(d.targets, [PkgVersion { name: "firefox".into(), version: "128.0-1".into() }]);
        assert!(parse(&["dg", "firefox"]).is_err());
        assert!(parse(&["dg", "=1"]).is_err());
        assert!(parse(&["dg", "firefox="]).is_err());
    }

    #[test]
    fn global_flag_spellings() {
        let c = parse(&["--setopt", "downloads.parallel=4", "--setopt=ui.color=never", "upgrade"]).unwrap();
        assert_eq!(
            c.global.setopt,
            [("downloads.parallel".into(), "4".into()), ("ui.color".into(), "never".into())]
        );
        assert!(parse(&["--setopt", "noequals", "upgrade"]).is_err());
        assert_eq!(parse(&["--root=/mnt", "up"]).unwrap().global.installroot, Some("/mnt".into()));
        assert_eq!(parse(&["-r", "/mnt", "up"]).unwrap().global.installroot, Some("/mnt".into()));
        assert!(parse(&["--noconfirm", "up"]).unwrap().global.assumeyes);
        // Global flags are accepted after the subcommand too.
        assert!(parse(&["install", "x", "-y"]).unwrap().global.assumeyes);
        assert!(parse(&["--parallel=0", "up"]).is_err());
    }

    #[test]
    fn list_updates_alias() {
        let Command::List(l) = parse(&["ls", "--updates"]).unwrap().command else { panic!() };
        assert!(l.upgrades);
        let Command::List(l) = parse(&["ls", "--files", "bash"]).unwrap().command else { panic!() };
        assert_eq!(l.files.as_deref(), Some("bash"));
    }

    #[test]
    fn history_flags_after_subcommand() {
        let Command::History(h) = parse(&["history", "list", "--limit", "5", "--reverse"]).unwrap().command
        else {
            panic!()
        };
        assert_eq!(h.limit, Some(5));
        assert!(h.reverse);
        assert!(matches!(h.command, Some(HistoryCmd::List)));
    }
}
