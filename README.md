<p align="center"><img src="src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">ad×0</h1>

<p align="center"><b>ad × 0 = 0</b></p>

<p align="center">An ad blocker that does not run: for Windows, macOS, Linux and Firefox</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<img src="https://img.shields.io/badge/languages-12-0f766e" alt="12 languages">
</p>

**English** · [Русский](docs/README.ru.md) · [Español](docs/README.es.md) · [Ελληνικά](docs/README.el.md) · [עברית](docs/README.he.md) · [简体中文](docs/README.zh.md) · [日本語](docs/README.ja.md) · [한국어](docs/README.ko.md) · [Српски](docs/README.sr.md) · [Français](docs/README.fr.md) · [Deutsch](docs/README.de.md) · [Italiano](docs/README.it.md)

<p align="center">
<img src="docs/window-off.png" width="380" alt="The window of the program: blocking is off">
<img src="docs/window-on.png" width="380" alt="The window of the program: ads are blocked">
</p>

ad×0 writes addresses of ad servers into the hosts file of the system and exits. Ads from these addresses stop loading in every program, and there is no background process: nothing to take memory or processor time. A Firefox extension does what the hosts file cannot: hides ad blocks on pages and takes ads out of YouTube videos.

## The program

- Blocks about 10,000 addresses of ad servers and trackers from the lists you choose.
- Keeps your own lines of the hosts file, leaves a copy of it before the first change, and reads back what it has written.
- Keeps the last good copy of each list, so blocking does not depend on whether the site of a list is up.
- Does not let a mistake in a list close sites such as Google, Wikipedia or GitHub: here is [the whole list](data/essential.json). Any other site is opened with the field “Do not block”.
- Speaks 12 languages. Sends nothing anywhere: it only downloads the lists when you press the button.

## The Firefox extension

<p align="center"><img src="docs/popup.png" width="312" alt="The Firefox extension"></p>

- Blocks ad addresses and hides ad blocks on pages. Regional lists are chosen by the languages of the browser.
- Takes the schedule of ads out of the answers of YouTube, so the player has nothing to show.
- Updates the lists once a day. The browser itself enforces the rules, and the background part is unloaded in between.

## What it cannot do

The hosts file works with whole addresses. Ads that come from the same address as the content, as in YouTube or in social networks, stay; for YouTube there is the extension. A proxy or a VPN that looks addresses up by itself bypasses the hosts file.

## How light it is

Measured on 2026-11-24 on Windows Server 2022, the release build:

| What | Result |
| --- | --- |
| Processes while the window is closed | 0 |
| Processor time while the window is open | 0–31 ms in 5 s |
| From the start to the window | 0.06–0.6 s in four runs |
| Size of the program | 4.4 MB |
| Growth of the hosts file | 198 KB for 9,800 addresses |

## Lists

The lists are not a part of this repository. Your copy of the program downloads them from their sites, and each has its own license.

| List | Where | License |
| --- | --- | --- |
| [Peter Lowe](https://pgl.yoyo.org/adservers/) | both | [McRae GPL](https://pgl.yoyo.org/license/) |
| [AdAway](https://adaway.org/) | both | CC BY 3.0 |
| [RU AdList](https://github.com/easylist/ruadlist) | both | CC BY 3.0 |
| [Liste FR](https://github.com/easylist/listefr) | both | CC BY-SA 3.0 |
| [EasyList China](https://github.com/easylist/easylistchina) | both | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [StevenBlack](https://github.com/StevenBlack/hosts) | program | MIT |
| [EasyList](https://easylist.to/) | extension | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Germany](https://github.com/easylist/easylistgermany) | extension | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Hebrew](https://github.com/easylist/EasyListHebrew) | extension | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Italy](https://github.com/easylist/easylistitaly) | extension | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Spanish](https://github.com/easylist/easylistspanish) | extension | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [KoreanList](https://github.com/easylist/KoreanList) | extension | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [AdGuard Japanese](https://github.com/AdguardTeam/AdguardFilters) | extension | GPL 3.0 |
| [Greek AdBlock Filter](https://github.com/kargig/greek-adblockplus-filter) | extension | CC BY-SA 4.0 |

## Status

| System | State |
| --- | --- |
| Windows | Works on Windows Server 2022: checked with real lists on a copy of the hosts file. Not checked on Windows 10 and 11 yet. |
| macOS 10.13+ | Builds, passes the tests. Nobody has run it yet. |
| Ubuntu 24.04+ | Builds, passes the tests. Nobody has run it yet. |
| Firefox 128+ | Works in Firefox 160. Not signed by Mozilla yet, so it has to be loaded again after Firefox is restarted. |

## Install

There are no ready-made downloads yet, so the program has to be built. You need Node.js 24 or newer, Rust, on Windows also the Microsoft C++ Build Tools, on Ubuntu also `libwebkit2gtk-4.1-dev librsvg2-dev`.

```sh
git clone https://github.com/Ilia-Pero/adx0
cd adx0
npm ci
npm run tauri build
```

The installer appears in `src-tauri/target/release/bundle`. It is not signed, so Windows and macOS will ask whether you trust it.

The uninstaller opens the blocked addresses again. On macOS, turn blocking off before you delete the program.

The extension: run `npm ci && npm run build` in the folder `extension`, open `about:debugging` in Firefox, choose “This Firefox”, “Load Temporary Add-on” and the file `extension/web/manifest.json`.

## Translations

The translations are machine-made and have not been checked by native speakers. If you see a mistake, please correct it in `locales`, in `extension/web/_locales` or in `docs`.

## Contributing and license

How to build, test and change the program: [CONTRIBUTING.md](CONTRIBUTING.md). The license is [MIT](LICENSE).
