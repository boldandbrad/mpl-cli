*This is a reimagination and reimplementation of my other project
[meeple-cli](https://github.com/boldandbrad/meeple-cli).*

---

# mpl

[![Crates.io](https://img.shields.io/crates/v/mpl-cli)](https://crates.io/crates/mpl-cli)

> **mpl**; short for **meeple** [`/mipel/`] *noun* -  a board game player
> token. (E.g., <picture><source media="(prefers-color-scheme: dark)" srcset="assets/meeple-light.svg"><source media="(prefers-color-scheme: light)" srcset="assets/meeple-dark.svg"><img src="" style="vertical-align:top;" alt="meeple icon" width="24"></picture>)

**mpl** is a local board game collection management CLI tool built in rust and
powered by [BoardGameGeek](https://boardgamegeek.com) (BGG) public APIs.

**Jump to:**
[Features](#features) |
[Installation](#install) |
[Getting Started](#getting-started) |
[Command Reference](#cmd-ref) |
[Configuration](#config) |
[Resources](#resources) |
[Legal](#legal) |
[Documentation](https://boldandbrad.github.io/mpl/) 🔗

> [!WARNING]<br>
> **mpl** is currently in **ALPHA**. This means it is generally unstable and may
> be missing key features. Please track the progress of features throughout this
> README with the following symbols:
> | Symbol    | Status                    |
> | -         | -                         |
> | ✅        | Implemented - stable      |
> | ✳️        | Implemented - unstable    |
> | 🚧        | Work in Progress          |
> | ❌        | Not Started               |

## 🎥 Demo

> Coming soon.

## ✨ Features <a id="features"></a>

- Get started quickly with BGG user collection import ❌
- Discover new titles via BGG Hotness, new releases, active crowdfunding
  campaigns, and BGG search 🚧
- Grow your collection by searching and opening Geek Market listings ❌
- Flexible local stash maintainance and customization 🚧
- Powerful cross-stash search to find the best title for game night ❌
- Create and manage personal ratings ❌
- Log and view game playthroughs ❌
- Multiple user support with profiles 🚧
- Written in Rust 🦀 ✅

## 📦 Installation <a id="install"></a>

Install **mpl** with your favorite of the methods below, or read the
[docs](https://boldandbrad.github.io/mpl/) for more info. Then run
`mpl --version` to verify installation.

**Jump to:** [Homebrew](#install-homebrew) | [Scoop](#install-scoop) |
[Cargo](#install-cargo) | [Remote](#install-remote) |
[GitHub Release](#install-release) | [Source](#install-source)

### Install via [Homebrew](https://brew.sh) 🍺 (macOS/Linux) ❌ <a id="install-homebrew"></a>

1. Add tap:

    ```sh
    brew tap boldandbrad/tap
    ```

1. Install formula:

    ```sh
    brew install mpl-cli
    ```

### Install via [Scoop](https://scoop.sh) 🍦 (Windows) ❌ <a id="install-scoop"></a>

1. Add bucket:

    ```sh
    scoop bucket add boldandbrad_scoop-bucket https://github.com/boldandbrad/scoop-bucket
    ```

1. Install manifest:

    ```sh
    scoop install boldandbrad_scoop-bucket/mpl-cli
    ```

### Install via [Cargo](https://crates.io) 📦 ✅ <a id="install-cargo"></a>

> [!NOTE]
> In order to install pre-releases (alpha/beta/pre) you must run `cargo install`
> with `--version=<VERSION>`.

- Install crate:

  ```sh
  cargo install mpl-cli
  ```

### Install via remote install script 📜 ❌ <a id="install-remote"></a>

> [!NOTE]
> The [remote install script](scripts/install.sh) explains what it will do and
> prompts before doing so.

- Run script:

  ```sh
  curl -LSfs https://raw.githubusercontent.com/boldandbrad/mpl-cli/main/ci/install.sh | sh -s -- --git boldandbrad/mpl-cli
  ```

### Manual install from GitHub Release ⬇️ 🚧 <a id="install-release"></a>

1. Download the [latest GitHub Release](https://github.com/boldandbrad/mpl-cli/releases)
   for your platform
2. Extract contents and install to a location in your `$PATH`

### Manual install from source 👩‍💻 ✅ <a id="install-source"></a>

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Run `git clone https://github.com/boldandbrad/mpl-cli` and `cd mpl-cli`
3. Run `cargo install --path .`
4. Ensure `~/.cargo/bin` or `$CARGO_HOME/bin` is in your `$PATH`

## 🚀 Getting Started <a id="getting-started"></a>

Using **mpl** is as easy as:

```sh
mpl
```

To get you started, on first run `mpl` creates a default profile with the same
name as your user `$HOME` directory containing a stash called `collection`.

> [!TIP]
> If you prefer, you can rename these later with `mpl profile rename` and
> `mpl stash rename` respectively.

### Import BGG user collections 🚢

Initiate an import:

> [!TIP]
> Try it with `--dry-run` to *simulate* the import without actually making
> changes.

```sh
mpl bgg import --user=boldandbrad
```

**mpl** will guide you through the import process. When done, see your imported
stash(es):

```sh
mpl stash list --verbose
```

### Start fresh 🧼

**mpl** relies on BoardGameGeek item IDs to manage the titles you add to your
stashes. The easiest way to get these is by searching BoardGameGeek:

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

Run `mpl --help` or read the [docs](https://boldandbrad.github.io/mpl/) to
discover what to do next!

## 📖 Command Reference <a id="cmd-ref"></a>

> [!TIP]<br>
> You can discover **mpl** commands and options with `mpl --help`.

### Root 🚧

- `mpl add` ✳️ - add titles to a stash
- `mpl drop` 🚧 - drop titles from a stash
- `mpl list` 🚧 - list stashed titles
  - `--sort` ❌ - sort titles by provided value
  - `-g`/`--games-only` ❌ - list only games
  - `-e`/`--expansions-only` ❌ - list only expansions
  - `--group-expansions` ❌ - group expansions below their parent game
- `mpl move` ❌ - move titles to another stash
<!-- - `mpl update` ❌ - update local stash data -->

### Profiles 🚧

Manage user profiles.

- `mpl profile list` ✳️ - list all existing profiles
- `mpl profile active` ✅ - display the current profile
- `mpl profile switch` ✳️ - switch the active profile
- `mpl profile create` ✳️ - create a new profile
- `mpl profile delete` ✳️ - delete an existing profile
- `mpl profile rename` ❌ - rename an existing profile

### Stashes 🚧

Manage local stashes in the active profile.

#### Flags/Options

- `-p/--profile` - the profile to perform actions in (default: active profile)

#### Commands

- `mpl stash create` ✳️ - create new stashes
- `mpl stash delete` ✳️ - delete existing stashes
- `mpl stash list` ✳️ - list existing stashes
  - `-v`/`--verbose` ❌ - list stats for stashes
- `mpl stash rename` ❌ - rename an existing stash
- `mpl stash info` ❌ - view details of a stash
- `mpl stash move` ❌ - move stashes to another profile

### BoardGameGeek 🚧

Perform BoardGameGeek related actions.

- `mpl bgg search` ✳️ - search boardgamegeek for titles
  - Flag `--market` ❌ - search geek market listings
- `mpl bgg info` ✳️ - view title details
- `mpl bgg open` ❌ - open links in the web browser
  - Option `--title` - open a title's boardgamegeek page(s)
  - Option `--campaign` - open a title's crowdfunding campaign page
  - Option `--listing` - open a geek market listing
- `mpl bgg import` ❌ - import bgg user collections
  - Option `--dry-run` ❌ - simulate import without persisting changes
- `mpl bgg hotness` ✳️ - view bgg hotness list
- `mpl bgg campaigns` ❌ - list active crowdfunding campaigns
- `mpl bgg releases` ❌ - list recent title releases

### Config 🚧

Manage configurations.

> Profile level configs override global ones by default.

#### Flags/Options

- `-g`/`--global` ❌ - apply config actions to the global scope. When not
  present, the action applies to the active profile options.
- `--show-scope` ❌ - augment output with the scope
- `-F`/`--force` ❌ - used in combination with `-g`, changes default config
  value for all profiles with overwrite of profile values

#### Commands

- `mpl config list` ❌ - list all config options and their current values
  - `--name-only` ❌ - output only config option names
- `mpl config get` ❌ - get the current value of the given config option
  - `--default` ❌ - get the default value of the given config option
  - `--all-values` ❌ - get all valid values of the given config option
- `mpl config set` ❌ - set the value of the given config option
- `mpl config unset` ❌ - revert the value of the given option to its default
  - `-a`/`--all` ❌ - revert all option values to their default
- `mpl config complete` 🚧 - setup tab-completions for the given shell

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

## ⚙️ Configuration <a id="config"></a>

### Environment Variables

**mpl** respects the following env variables to modify default behavior. In
cases where multiple variables control the same behavior, **mpl** obeys the
left-most present variable.

- `MPL_CONFIG_HOME`/`XDG_CONFIG_HOME` ❌ - change where **mpl** configs are
  located. Default: `~/.config/mpl/`

### Config options

> Needs more thought and design.

These options can be managed with `mpl config`.

> Global configs are stored in `.mpl/config.toml`.
> Profile level configs are stored in `.mpl/<PROFILE>/config.toml`

- `update_on_change` ❌ - automatically pass `--update` to add/drop operations
  [Default `false`]
- `quiet_success` ❌ - force `--quiet` on all supported commands on success
  [Default `false`]
- `pretty_format` ❌ - format outputs with table borders and emojis [Default
  `true`]
- `default_stash_name` ❌ - the default name to use when creating new stashes
  [Default `collection`]

### Completions

**mpl** supports tab completions for `bash`, `zsh`, and `fish`. For
setup, run `mpl config completions <SHELL>`.

## 📚 Resources <a id="resources"></a>

- [Changelog](docs/CHANGELOG.md) - See a history of implemented
  features/changes.
- [Roadmap](https://github.com/boldandbrad/mpl-cli/milestones) - See a list of
  planned features and milestones.
- [FAQ](docs/faq.md) - Find answers to common questions.
- [Contributor Guide](docs/CONTRIBUTING.md) ❌ - Find out how to get involved.

## ⚖️ Legal <a id="legal"></a>

> [!NOTE]<br>
> Neither **mpl** nor its maintainers are affiliated with
> [BoardGameGeek](https://boardgamegeek.com).

Copyright (c) 2023 Bradley Wojcik. Released under the MIT License. See
[LICENSE](LICENSE) for details.
