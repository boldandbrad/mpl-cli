mod cli;
mod structs;
mod subcommand;
mod util;

use anyhow::Result;
use clap::{CommandFactory, Parser};
use cli::{
    BggSubCommands, ConfigSubCommands, Mpl, ProfileSubCommands, StashSubCommands, SubCommands,
};
use util::fs::check_fs;

fn main() -> Result<()> {
    check_fs();
    let cli_app = Mpl::parse();
    match cli_app.subcommand {
        // root commands
        SubCommands::Add {
            stash_name,
            bgg_ids,
        } => subcommand::add(stash_name, bgg_ids),
        SubCommands::Drop {
            stash_name,
            bgg_ids,
        } => subcommand::drop(stash_name, bgg_ids),
        SubCommands::List {} => subcommand::list(),

        // bgg commands
        SubCommands::Bgg { subcommand } => match subcommand {
            BggSubCommands::Campaigns {} => subcommand::bgg::campaigns(),
            BggSubCommands::Hotness {} => subcommand::bgg::hotness(),
            BggSubCommands::Import {} => subcommand::bgg::import(),
            BggSubCommands::Info { bgg_id, verbose } => subcommand::bgg::info(bgg_id, verbose),
            BggSubCommands::Open {} => subcommand::bgg::open(),
            BggSubCommands::Releases {} => subcommand::bgg::releases(),
            BggSubCommands::Search { query } => subcommand::bgg::search(query),
        },

        // config commands
        SubCommands::Config { subcommand } => match subcommand {
            ConfigSubCommands::Complete { shell } => {
                clap_complete::generate(shell, &mut Mpl::command(), "mpl", &mut std::io::stdout());
                Ok(())
            }
            ConfigSubCommands::Get { option_name } => subcommand::config::get(option_name),
            ConfigSubCommands::List {} => subcommand::config::list(),
            ConfigSubCommands::Set {
                option_name,
                option_value,
            } => subcommand::config::set(option_name, option_value),
            ConfigSubCommands::Unset { option_name } => subcommand::config::unset(option_name),
        },

        // profile commands
        SubCommands::Profile { subcommand } => match subcommand {
            ProfileSubCommands::Active {} => subcommand::profile::active(),
            ProfileSubCommands::Create {
                profile_name,
                active,
            } => subcommand::profile::create(profile_name, active),
            ProfileSubCommands::Delete { profile_name } => {
                subcommand::profile::delete(profile_name)
            }
            ProfileSubCommands::List {} => subcommand::profile::list(),
            ProfileSubCommands::Rename {
                profile_name,
                new_name,
            } => subcommand::profile::rename(profile_name, new_name),
            ProfileSubCommands::Switch { profile_name } => {
                subcommand::profile::switch(profile_name)
            }
        },

        // stash commands
        SubCommands::Stash {
            subcommand,
            profile,
        } => match subcommand {
            StashSubCommands::Create { stash_names } => {
                subcommand::stash::create(stash_names, profile)
            }
            StashSubCommands::Delete { stash_names, force } => {
                subcommand::stash::delete(stash_names, force)
            }
            StashSubCommands::Info {} => subcommand::stash::info(),
            StashSubCommands::List {} => subcommand::stash::list(),
            StashSubCommands::Move {} => subcommand::stash::move_(),
            StashSubCommands::Rename {} => subcommand::stash::rename(),
        },
    }
}
