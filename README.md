*This is a reimagination and reimplementation of my other project [meeple-cli](https://github.com/boldandbrad/meeple-cli).*

---

# mpl

<!-- TODO: add badges -->

> **mpl**; short for **meeple** [`/mipel/`] *noun* -  a board game player
> token. (E.g., <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round" style="vertical-align:top;"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M9 20h-5a1 1 0 0 1 -1 -1c0 -2 3.378 -4.907 4 -6c-1 0 -4 -.5 -4 -2c0 -2 4 -3.5 6 -4c0 -1.5 .5 -4 3 -4s3 2.5 3 4c2 .5 6 2 6 4c0 1.5 -3 2 -4 2c.622 1.093 4 4 4 6a1 1 0 0 1 -1 1h-5c-1 0 -2 -4 -3 -4s-2 4 -3 4z" /></svg>)

`mpl` is a local board game collection management CLI tool built in rust and
powered by [BoardGameGeek](https://boardgamegeek.com) (BGG) public APIs.

**Jump to:** [Features](#features) | [Installation](#install) | [Usage](#usage) | [Configuration](#config) | [Resources](#resources) | [Legal](#legal) | [Documentation](https://boldandbrad.github.io/mpl/) 🔗

> [!WARNING]<br>
> `mpl` is currently in **ALPHA**. This means it is generally unstable and may be missing key features.
> Please track the progress of features throughout this README with the following symbols:
> | Symbol    | Status            |
> | -         | -                 |
> | ✅        | Implemented       |
> | 🚧        | Work in Progress  |
> | ❌        | Not Started       |

## 🎥 Demo

> Coming soon.

## ✨ Features <a id="features"></a>

- Get started quickly with BGG user collection import
- Discover new titles via BGG Hotness, new releases, active crowdfunding campaigns, and BGG search
- Search and open Geek Market listings
- Add titles to local stashes
- Stats...
- Powerful cross-stash search to find the best title for game night
- Create and manage personal ratings
- Log and view game playthroughs
- Multiple user support with profiles

## 📦 Installation <a id="install"></a>

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

## 🚀 Usage <a id="usage"></a>

```sh
mpl
```

To get you started, on first run `mpl` creates a default profile with the same name as your user home directory (you can rename it later if you prefer with `mpl profile rename`), containing a stash called `collection`.

### 🚢 Import BGG user collections

```sh
mpl bgg import --user=boldandbrad
```

mpl will guide you through the import process. When done, see your imported collections:

```sh
mpl stash list --verbose
```

### 🧼 Start fresh

`mpl` relies on BoardGameGeek item IDs to manage the titles you add to your stashes. The easiest way to get these is by searching BoardGameGeek:

```sh
mpl bgg search "wingspan"
```

Copy an item ID from the output and use it in another command:

```sh
mpl add -s=collection 266192
```

You've added Wingspan 🦜 to the `collection` stash!

Now let's see what's in your collection:

```sh
mpl titles -s=collection
```

Run `mpl --help` to discover what else you can do with mpl!

### Commands

#### Top Level 🚧

- `mpl add` 🚧 - add titles to a stash
- `mpl drop` 🚧 - drop titles from a stash
- `mpl titles` 🚧 - list titles in stashes
- `mpl move` ❌ - move titles to another stash
- `mpl update` ❌ - update local stash data

#### Stashes 🚧

Manage local stashes in the active profile.

##### Flags/Options

- `-p/--profile` - the profile to perform actions in (default: active profile)

##### Commands

- `mpl stash create` 🚧 - create new stashes
- `mpl stash delete` 🚧 - delete existing stashes
- `mpl stash list` 🚧 - list existing stashes
  - `-v`/`--verbose` ❌ - list stats for stashes
- `mpl stash rename` ❌ - rename an existing stash
- `mpl stash info` ❌ - view details of a stash
- `mpl stash move` ❌ - move stashes to another profile

#### BoardGameGeek 🚧

BoardGameGeek specific actions.

- `mpl bgg search` 🚧 - search boardgamegeek for titles
  - Flag `--market` ❌ - search geek market listings
- `mpl bgg info` 🚧 - view title details
- `mpl bgg open` ❌ - open boardgamegeek.com
  - Option `--title` - open a title's boardgamegeek page(s)
  - Option `--campaign` - open a title's crowdfunding campaign page
  - Option `--listing` - open a geek market listing
- `mpl bgg import` ❌ - import bgg collections
- `mpl bgg hotness` ❌ - view bgg hotness list
- `mpl bgg campaigns` ❌ - list active crowdfunding campaigns
- `mpl bgg releases` ❌ - list recent releases

#### Profiles 🚧

Manage user profiles.

- `mpl profile active` 🚧 - display current profile
- `mpl profile create` ❌ - create new profile
- `mpl profile delete` ❌ - delete existing profile
- `mpl profile list` 🚧 - list existing profiles
- `mpl profile rename` ❌ - rename existing profile
- `mpl profile switch` ❌ - switch active profile

#### Config 🚧

Manage configurations.

> Profile level configs override global ones by default.

##### Flags/Options

- `-g`/`--global` ❌ - action applies to global options. When not present, the action applies to the active profile options.
- `-F`/`--force` ❌ - used in combination with `-g`, changes default config value for all profiles with overwrite of profile values

##### Commands

- `mpl config active` ❌ - display current configuration
- `mpl config options` ❌ - show available config options
- `mpl config set` ❌ - set option value
- `mpl config unset` ❌ - revert option value to default
- `mpl config complete` 🚧 - setup shell tab completions

#### Ratings ❌

> Needs more thought and design.

Manage personal title ratings.

- `mpl rating rate` ❌ - rate a title
- `mpl rating unrate` ❌ - unrate a title
- `mpl rating tiers` ❌ - list rated titles in tiers

#### Plays ❌

> Needs more thought and design.

Log and manage title plays.

- `mpl play log`/`create` ❌ - log a new play
- `mpl play delete` ❌ - delete an existing play
- `mpl play list` ❌ - list all logged plays
  - Arg BGG_ID ❌ - list title logged plays
- `mpl play stats` ❌ - view title play stats

## ⚙️ Configuration <a id="config"></a>

### Environment Variables

mpl respects the following env variables:

- `MPL_HOME`/`XDG_CONFIG_HOME` - change where `.mpl/` is stored. Default: `~/.mpl/`

### Config options

These options can be managed with `mpl config`.

> Global configs are stored in `.mpl/config.toml`.
> Profile level configs are stored in `.mpl/<PROFILE>/config.toml`

- `update_on_change` - automatically pass `--update` to add/drop operations. Default `false`
- `default_stash` - the default stash to perform add/drop operations on. Default `collection`

### Completions

> Coming soon.

## 📚 Resources <a id="resources"></a>

- [Changelog](docs/changelog.md) - See a history of implemented features/changes.
- [Roadmap](https://github.com/boldandbrad/mpl-cli/milestones) - See a list of planned features and milestones.
- [FAQ](docs/faq.md) - Find answers to common questions.
- [Contributor Guide](docs/contributing.md) ❌ - Find out how to get involved.

## ⚖️ Legal <a id="legal"></a>

> Disclaimer: Neither `mpl-cli` nor its maintainers are affiliated with
> [BoardGameGeek](https://boardgamegeek.com).

Copyright (c) 2023 Bradley Wojcik. Released under the MIT License. See
[LICENSE](LICENSE) for details.
