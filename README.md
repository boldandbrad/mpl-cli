# mpl

> **mpl**; short for **meeple** [`/mipel/`] - (*noun*) a board game player
> token.

`mpl` is a local board game collection management CLI tool built in rust and
powered by the BoardGameGeek public API.

Notice: `mpl` is currently in BETA. This means it may be missing key features and is generally unstable.
Please track the progress of features throughout this README with the following symbols:

| Symbol    | Status            |
| -         | -                 |
| ✅        | Implemented       |
| 🚧        | Work in Progress  |
| ❌        | Not Started       |

## Install

Via [Homebrew](https://brew.sh) (macOS/Linux) ❌

```sh
brew tap boldandbrad/tap
brew install mpl
```

Via [Scoop](https://scoop.sh) (Windows) ❌

```sh
scoop bucket add boldandbrad_scoop-bucket https://github.com/boldandbrad/scoop-bucket
scoop install boldandbrad_scoop-bucket/mpl
```

Via Cargo 🚧

```sh
cargo install mpl
```

## Commands

### Top Level 🚧

- `mpl add` 🚧 - add titles to a stash
- `mpl drop` 🚧 - drop titles from a stash
- `mpl titles` 🚧 - list titles in stashes
- `mpl move` ❌ - move a title to another stash
- `mpl update` ❌ - update local stash data

### Stashes 🚧

Manage local stashes.

- `mpl stash create` 🚧 - create new stashes
- `mpl stash delete` 🚧 - delete existing stashes
- `mpl stash list` 🚧 - list existing stashes
  - `-v`/`--verbose` ❌ - shows stash stats in table
- `mpl stash rename` ❌ - rename an existing stash
- `mpl stash info` ❌ - view stash details
- `mpl stash move` ❌ - move stash to another profile

### BoardGameGeek 🚧

BoardGameGeek specific actions.

- `mpl bgg search` 🚧 - search boardgamegeek for titles
- `mpl bgg info` 🚧 - view title details
- `mpl bgg open` ❌ - open boardgamegeek.com
  - Arg BGG_ID(s) - open titles on boardgamegeek.com
- `mpl bgg import` ❌ - import bgg collections
- `mpl bgg hot` ❌ - view bgg hotness list
- `mpl bgg preorders` ❌ - view active crowdfunding pre-orders list
- `mpl bgg new` ❌ - view new releases list
- `mpl bgg market search` ❌ - view current GeekMarket listings
- `mpl bgg market open` ❌ - open a GeekMarket listing

### Profiles 🚧

Manage user profiles.

- `mpl profile active` 🚧 - display current profile
- `mpl profile create` ❌ - create new profile
- `mpl profile delete` ❌ - delete existing profile
- `mpl profile list` 🚧 - list existing profiles
- `mpl profile rename` ❌ - rename existing profile
- `mpl profile switch` ❌ - switch active profile

### Config 🚧

Manage configurations.

> Profile level configs override global ones by default.

#### Flags

- `-g`/`--global` ❌ - action applies to global options. When not present, the action applies to the active profile options.
- `-F`/`--force` ❌ - used in combination with `-g`, changes default config value for all profiles with overwrite of profile values

#### Commands

- `mpl config active` ❌ - display current configuration
- `mpl config options` ❌ - show available config options
- `mpl config set` ❌ - set option value
- `mpl config unset` ❌ - revert option value to default
- `mpl config complete` 🚧 - setup shell tab completions

### Ratings ❌

> Needs more thought and design.

Manage personal title ratings.

- `mpl rating rate` ❌ - rate a title
- `mpl rating unrate` ❌ - unrate a title
- `mpl rating tiers` ❌ - list rated titles in tiers

### Plays ❌

> Needs more thought and design.

Log and manage title plays.

- `mpl play log`/`create` ❌ - log a new play
- `mpl play delete` ❌ - delete an existing play
- `mpl play list` ❌ - list all logged plays
  - Arg BGG_ID ❌ - list title logged plays
- `mpl play stats` ❌ - view title play stats

## Configs

- `update_on_change` - automatically pass `--update` to add/drop operations. Default `false`
- `default_stash` - the default stash to perform add/drop operations on. Default `default`

## Resources

- [Changelog](docs/changelog.md) - See a history of implemented features/changes.
- [Roadmap](https://github.com/boldandbrad/mpl-cli/milestones) - See a list of planned features and milestones.
- [FAQ](docs/faq.md) - Find answers to common questions.
- [Contributor Guide](docs/contributing.md) ❌ - Find out how to get involved.

## License

Copyright (c) 2023 Bradley Wojcik. Released under the MIT License. See
[LICENSE](LICENSE) for details.
