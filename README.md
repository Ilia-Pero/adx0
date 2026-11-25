<p align="center"><img src="src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">ad×0</h1>

<p align="center"><b>ad × 0 = 0</b></p>

<p align="center">Ad blocker for Windows, macOS, Linux and browsers</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/adx0/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/adx0?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

**English** · [Русский](docs/README.ru.md) · [Español](docs/README.es.md) · [Ελληνικά](docs/README.el.md) · [עברית](docs/README.he.md) · [简体中文](docs/README.zh.md) · [日本語](docs/README.ja.md) · [한국어](docs/README.ko.md) · [Српски](docs/README.sr.md) · [Français](docs/README.fr.md) · [Deutsch](docs/README.de.md) · [Italiano](docs/README.it.md) · [Беларуская](docs/README.be.md) · [Қазақша](docs/README.kk.md)

<p align="center">
<img src="docs/window-off.png" width="380" alt="The window of the program: blocking is off">
<img src="docs/window-on.png" width="380" alt="The window of the program: ads are blocked">
</p>

ad×0 writes the addresses of ad servers into the hosts file of the system and exits. After that programs on the computer load nothing from those addresses, and ad×0 itself has no running process. A browser extension handles what the hosts file cannot: it hides ad blocks on pages and removes ads from YouTube videos.

## Install

| System | File | How to install |
| --- | --- | --- |
| Windows 10 and 11 | [adx0-windows.exe](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-windows.exe) | Run the file. If Windows shows a warning, choose “More info”, then “Run anyway”. |
| macOS 10.13 or later | [adx0-macos.dmg](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-macos.dmg) | Drag the program to Applications and open it. After macOS refuses for the first time, open System Settings, “Privacy & Security”, and press “Open Anyway”. |
| Ubuntu 24.04 or later | [adx0-ubuntu.deb](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-ubuntu.deb) | `sudo apt install ./adx0-ubuntu.deb` |
| Firefox 128 or later | [adx0-firefox.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-firefox.zip) | Open `about:debugging`, “This Firefox”, “Load Temporary Add-on”, and select the file. Firefox removes such an extension when it closes, so load it again after a restart. |
| Chrome, Edge, Opera, Yandex Browser | [adx0-chrome.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-chrome.zip) | Unpack the archive into a folder you will keep. Open `chrome://extensions`, turn on “Developer mode”, press “Load unpacked” and select the folder. Yandex Browser may turn off an extension that did not come from a store. |

Windows and macOS warn about the files because they are not signed with a paid certificate. GitHub builds them from this repository, installs the programs on clean machines and starts them before a release is published.

The uninstaller removes the blocked addresses from the hosts file. On macOS, turn blocking off before you delete the program.

## The program

- Blocks about 10,000 addresses of ad servers and trackers. You choose the lists they come from, and can add your own sites to block or to allow.
- Leaves your own lines of the hosts file as they are, makes a copy of the file before the first change and checks every write by reading the file again.
- Keeps the last good copy of each list, so blocking works while the site of a list is down.
- Skips the sites in [this file](data/essential.json), among them Google, Wikipedia and GitHub: finding one of them in a list of ads is a mistake of the list. You can still block them yourself.
- Updates the lists at most once a day, soon after the system starts and without a window. This can be turned off.
- Speaks 14 languages.

The hosts file works with whole addresses, so ads that come from the same address as the content stay: on YouTube, for example, and in social networks. Programs that use a proxy or a VPN with its own address lookup bypass the hosts file.

## Browser extension

<p align="center"><img src="docs/popup.png" width="312" alt="The window of the extension"></p>

- Blocks about 64,000 addresses and hides ad blocks on pages. Regional lists are chosen by the languages of the browser.
- Removes the schedule of ads from the answers of YouTube, so the player does not show them.
- Allows ads on a site with one button.
- Updates the lists once a day. The browser applies the rules itself and unloads the background part of the extension when it is idle.

The extension works in Firefox and in browsers based on Chromium.

## Privacy

The program and the extension download the lists from the sites named below and collect no data. The program also asks GitHub about new versions when you press “Check for updates”. There are no accounts and no paid exceptions for ads.

## Measurements

Windows Server 2022, release build, 2026-11-24.

| What was measured | Result |
| --- | --- |
| Processes while the window is closed | 0 |
| Processor time while the window is open | 0–31 ms in 5 s |
| From the start to the window | 0.06–0.6 s in four runs |
| Size of the program file | 4.4 MB |
| Growth of the hosts file | 198 KB for 9,800 addresses |

## Lists

The lists are not part of this repository. Your copy of the program downloads them from their own sites, and each list has its own license.

| List | Used by | License |
| --- | --- | --- |
| [Peter Lowe](https://pgl.yoyo.org/adservers/) | program and extension | [McRae GPL](https://pgl.yoyo.org/license/) |
| [AdAway](https://adaway.org/) | program and extension | CC BY 3.0 |
| [RU AdList](https://github.com/easylist/ruadlist) | program and extension | CC BY 3.0 |
| [Liste FR](https://github.com/easylist/listefr) | program and extension | CC BY-SA 3.0 |
| [EasyList China](https://github.com/easylist/easylistchina) | program and extension | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [StevenBlack](https://github.com/StevenBlack/hosts) | program | MIT |
| [EasyList](https://easylist.to/) | extension | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Germany](https://github.com/easylist/easylistgermany) | extension | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Hebrew](https://github.com/easylist/EasyListHebrew) | extension | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Italy](https://github.com/easylist/easylistitaly) | extension | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Spanish](https://github.com/easylist/easylistspanish) | extension | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [KoreanList](https://github.com/easylist/KoreanList) | extension | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [AdGuard Japanese](https://github.com/AdguardTeam/AdguardFilters) | extension | GPL 3.0 |
| [Greek AdBlock Filter](https://github.com/kargig/greek-adblockplus-filter) | extension | CC BY-SA 4.0 |

## Tested on

| System | What was checked |
| --- | --- |
| Windows Server 2022 | Works with real lists on a copy of the hosts file. |
| Windows Server, a GitHub machine | Installs, starts, blocks and unblocks an address in the hosts file. |
| Windows 10 and 11 | Not checked yet. |
| macOS and Ubuntu 24.04, GitHub machines | Builds, passes the tests, installs, starts, blocks and unblocks an address in the hosts file. Not checked by hand. |
| Ubuntu 26.04, a GitHub machine | Installs and starts. |
| Firefox 161, Edge 158 | The extension works. Chrome, Opera and Yandex Browser were not checked; they need Chromium 120 or later. |

If you run it on a system that has not been checked, please [write how it went](https://github.com/Ilia-Pero/adx0/issues).

## Build

You need Node.js 24 or later and Rust; on Windows also the Microsoft C++ Build Tools, on Ubuntu also `libwebkit2gtk-4.1-dev librsvg2-dev`.

```sh
git clone https://github.com/Ilia-Pero/adx0
cd adx0
npm ci
npm run tauri build
```

The installer appears in `src-tauri/target/release/bundle`. To build the extension, run `npm ci && npm run build` in the folder `extension`; the results are `extension/packed/firefox` and `extension/packed/chrome`. Tests, the structure of the code and the rules for changes are described in [CONTRIBUTING.md](CONTRIBUTING.md).

## Translations

English and Russian are the main languages. The other twelve are machine translations that native speakers have not checked. Corrections go to `locales`, `extension/web/_locales` and `docs`.

## License

[MIT](LICENSE)
