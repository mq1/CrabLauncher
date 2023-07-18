<br>

<p align="center">
<img src="assets/logo.png" alt="CrabLauncher Logo" height="150">
</p>

<h1 align="center">CrabLauncher</h1>

<p align="center">
<a href="https://github.com/mq1/crab-launcher/releases/latest"><img alt="GitHub release (latest by date)" src="https://img.shields.io/github/v/release/mq1/crab-launcher"></a>
<a href="https://github.com/mq1/crab-launcher/blob/main/LICENSE"><img alt="License: GPL-3.0" src="https://img.shields.io/github/license/mq1/crab-launcher"></a>
</p>

<img alt="screenshot" src="screenshot.png">

-----

An unofficial WIP launcher for Minecraft 1.19+

As of now, this launcher is considered a technical preview and every release breaks everything from the previous install

**Table of Contents**

- [Installation](#installation)
- [FAQ](#faq)
- [Thanks](#thanks)
- [License](#license)

## Installation

### Package (recommended)

Just grab the [latest release](https://github.com/mq1/crab-launcher/releases/latest) for your platform

### From source

```sh
git clone https://github.com/mq1/crab-launcher.git
cd crab-launcher
cargo build --release
```

## FAQ

### Are offline/cracked accounts supported?

No.

However, you can build the launcher from source with the `offline-accounts` feature enabled to add offline account support (ONLY FOR TESTING PURPOSES)

## Thanks

- Héctor Ramón for [iced](https://github.com/iced-rs/iced)
- [Pictogrammers](https://pictogrammers.com) for Material Design Icons
- [Crafatar](https://crafatar.com/)

## License

CrabLauncher is distributed under the terms of the [GPL-3.0-only](https://spdx.org/licenses/GPL-3.0-only.html) license.
