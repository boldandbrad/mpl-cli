#![deny(missing_docs)]

use clap::{ArgAction, Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(propagate_version = true)]
#[clap(arg_required_else_help = true)]
pub struct Mpl {
    #[clap(subcommand)]
    pub subcommand: SubCommands,
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
    /// Perform BoardGameGeek related actions
    Bgg {
        #[clap(subcommand)]
        subcommand: BggSubCommands,
    },
    /// Manage configurations
    Config {
        #[clap(subcommand)]
        subcommand: ConfigSubCommands,
    },
    /// Manage user profiles
    Profile {
        #[clap(subcommand)]
        subcommand: ProfileSubCommands,
    },
    /// Manage local stashes in the active profile
    Stash {
        #[clap(subcommand)]
        subcommand: StashSubCommands,
        #[clap(long, short = 'p')]
        /// The profile to perform actions in (Default: active profile)
        profile: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum BggSubCommands {
    /// List active crowdfunding campaigns
    Campaigns {},
    /// View BGG hotness list
    Hotness {},
    /// Import BGG user collections
    Import {},
    /// Retrieve item details
    Info {
        /// BoardGameGeek item id
        bgg_id: String,
    },
    /// Open links in the web browser
    Open {},
    /// List recent title releases
    Releases {},
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
    /// View details of a stash
    Info {},
    /// List existing stashes
    List {},
    /// Move stashes to another profile
    Move {},
    /// Rename an existing stash
    Rename {},
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
    /// Display the active profile
    Active {},
    /// Create new profiles
    Create {
        /// The name of the profile to create
        profile_name: String,
        #[clap(long)]
        #[clap(action = ArgAction::SetTrue)]
        /// Make the new profile active
        active: Option<bool>,
    },
    /// Delete existing profiles
    Delete {
        /// The name of the profile to delete
        profile_name: String,
    },
    /// List existing profiles
    List {},
    /// Rename an existing profile
    Rename {
        /// The name of the profile to rename
        profile_name: String,
        /// The new name of the profile
        new_name: String,
    },
    /// Switch the active profile
    Switch {
        /// The name of the profile to switch to
        profile_name: String,
    },
}
