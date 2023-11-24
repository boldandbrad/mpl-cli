mod cli;
mod structs;
mod subcommand;
mod util;

use clap::{CommandFactory, Parser};
use cli::{
    BggSubCommands, ConfigSubCommands, Mpl, ProfileSubCommands, StashSubCommands, SubCommands,
};
use util::fs::check_fs;

fn main() {
    check_fs();
    let cli_app = Mpl::parse();
    match cli_app.subcommand {
        // TODO: remove test command
        SubCommands::Test {} => {
            subcommand::test::test();
        }

        // top level commands
        SubCommands::Add {
            stash_name,
            bgg_ids,
        } => {
            subcommand::add(stash_name, bgg_ids);
        }
        SubCommands::Drop {
            stash_name,
            bgg_ids,
        } => {
            subcommand::drop(stash_name, bgg_ids);
        }
        SubCommands::Titles {} => {
            subcommand::titles();
        }

        // bgg commands
        SubCommands::Bgg { subcommand } => match subcommand {
            BggSubCommands::Campaigns {} => {
                subcommand::bgg::campaigns();
            }
            BggSubCommands::Hotness {} => {
                subcommand::bgg::hotness();
            }
            BggSubCommands::Import {} => {
                subcommand::bgg::import();
            }
            BggSubCommands::Info { bgg_id } => {
                subcommand::bgg::info(bgg_id);
            }
            BggSubCommands::Open {} => {
                subcommand::bgg::open();
            }
            BggSubCommands::Releases {} => {
                subcommand::bgg::releases();
            }
            BggSubCommands::Search { query } => {
                subcommand::bgg::search(query);
            }
        },

        // config commands
        SubCommands::Config { subcommand } => match subcommand {
            ConfigSubCommands::Complete { shell } => {
                clap_complete::generate(shell, &mut Mpl::command(), "mpl", &mut std::io::stdout());
            }
        },

        // profile commands
        SubCommands::Profile { subcommand } => match subcommand {
            ProfileSubCommands::Active {} => {
                // TODO: implement
                subcommand::profile::active();
            }
            ProfileSubCommands::Create {
                profile_name,
                active,
            } => {
                subcommand::profile::create(profile_name, active);
            }
            ProfileSubCommands::Delete { profile_name } => {
                // TODO: implement
                subcommand::profile::delete(profile_name);
            }
            ProfileSubCommands::List {} => {
                // TODO: implement
                subcommand::profile::list();
            }
            ProfileSubCommands::Rename {
                profile_name,
                new_name,
            } => {
                // TODO: implement
                subcommand::profile::rename(profile_name, new_name);
            }
            ProfileSubCommands::Switch { profile_name } => {
                // TODO: implement
                subcommand::profile::switch(profile_name);
            }
        },

        // stash commands
        SubCommands::Stash {
            subcommand,
            profile,
        } => match subcommand {
            StashSubCommands::Create { stash_names } => {
                subcommand::stash::create(stash_names, profile);
            }
            StashSubCommands::Delete { stash_names, force } => {
                subcommand::stash::delete(stash_names, force);
            }
            StashSubCommands::Info {} => {
                subcommand::stash::info();
            }
            StashSubCommands::List {} => {
                subcommand::stash::list();
            }
            StashSubCommands::Move {} => {
                subcommand::stash::move_();
            }
            StashSubCommands::Rename {} => {
                subcommand::stash::rename();
            }
        },
    }
}
