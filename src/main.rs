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
    let cli_app = Mpl::parse();

    check_fs();
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
            BggSubCommands::Info { bgg_id } => {
                subcommand::bgg::info(bgg_id);
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
                subcommand::profile::active();
            }
        },

        // stash commands
        SubCommands::Stash { subcommand } => match subcommand {
            StashSubCommands::Create { stash_names } => {
                subcommand::stash::create(stash_names);
            }
            StashSubCommands::Delete { stash_names, force } => {
                subcommand::stash::delete(stash_names, force);
            }
            StashSubCommands::List {} => {
                subcommand::stash::list();
            }
        },
    }
}
