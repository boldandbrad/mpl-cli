#![deny(missing_docs)]

use clap::{ArgAction, Parser, Subcommand, ValueHint};
use clap_complete::Shell;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(propagate_version = true)]
#[clap(arg_required_else_help = true)]
pub struct Mpl {
    #[clap(subcommand)]
    pub subcommand: SubCommands,

    #[clap(long, short)]
    #[clap(value_hint(ValueHint::FilePath))]
    /// Set the file to read the config from.
    /// This does not change the `cache` and `tmp` directories.
    /// You can also use the environment variable `CONFIG_FILE`.
    pub config_file: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum SubCommands {
    /// Add titles to a stash
    Add {
        /// The name of the stash to add to
        stash_name: String,
        /// The title BGG IDS to add
        bgg_ids: Vec<String>,
    },
    /// Drop titles from a stash
    Drop {
        /// The name of the stash to drop from
        stash_name: String,
        /// The title BGG IDS to drop
        bgg_ids: Vec<String>,
    },
    /// List titles in stashes
    Titles {},
    /// Perform BoardGameGeek actions
    Bgg {
        #[clap(subcommand)]
        subcommand: BggSubCommands,
    },
    /// Manage configurations
    Config {
        #[clap(subcommand)]
        subcommand: ConfigSubCommands,
    },
    /// Manage profiles
    Profile {
        #[clap(subcommand)]
        subcommand: ProfileSubCommands,
    },
    /// Manage stashes
    Stash {
        #[clap(subcommand)]
        subcommand: StashSubCommands,
    },
    /// Test command
    Test {},
}

#[derive(Subcommand)]
pub enum BggSubCommands {
    /// Retrieve item details
    Info {
        /// BoardGameGeek item id
        bgg_id: String,
    },
    /// Search BoardGameGeek items
    Search {
        /// BoardGameGeek search key. If searching multiple words, surround with quotes.
        query: String,
    },
}

#[derive(Subcommand)]
pub enum StashSubCommands {
    /// Create new stash(es)
    Create {
        /// The name(s) of the stash(es) to create
        stash_names: Vec<String>,
    },
    /// Delete existing stash(es)
    Delete {
        /// The name(s) of the stash(es) to delete
        stash_names: Vec<String>,
        #[clap(long, short = 'F')]
        #[clap(action = ArgAction::SetTrue)]
        /// Force delete without confirmation
        force: Option<bool>,
    },
    /// List all stashes
    List {},
}

#[derive(Subcommand)]
pub enum ConfigSubCommands {
    /// Print shell auto completions for the specified shell
    Complete {
        #[clap(value_enum)]
        /// The shell to generate auto completions for
        shell: Shell,
    },
}

#[derive(Subcommand)]
pub enum ProfileSubCommands {
    /// display active profile
    Active {},
}
