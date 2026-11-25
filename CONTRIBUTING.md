# Contributing

## Build

You need Node.js 24 or newer and Rust with clippy and rustfmt. On Windows, also the Microsoft C++ Build Tools. On Ubuntu, also `libwebkit2gtk-4.1-dev librsvg2-dev`.

```sh
npm ci
npm run build
cd extension
npm ci
npm run build
```

`npm run build` runs the linter, the type check, the tests and the bundle, and for the program also rustfmt, clippy and the tests of Rust. It has to pass without warnings.

| Command | What it does |
| --- | --- |
| `npm run dev` | opens the window in a browser; it downloads nothing and touches no files: <http://127.0.0.1:5392>; `?he` at the end chooses the language |
| `npm run tauri build` | builds the installer for this system |
| `npm run tauri build -- --debug --no-bundle` | builds the program for a try |

In a debug build two variables keep a try away from your system:

| Variable | What it replaces |
| --- | --- |
| `ADX0_HOSTS` | the hosts file |
| `ADX0_HOME` | the folder with the settings and the lists |

The compiler checks the code for other systems on any machine:

```sh
rustup target add x86_64-unknown-linux-gnu x86_64-apple-darwin
cargo clippy --manifest-path src-tauri/Cargo.toml -p adx0-system --target x86_64-unknown-linux-gnu
cargo clippy --manifest-path src-tauri/Cargo.toml -p adx0-system --target x86_64-apple-darwin
```

## Where things are

| Folder | What is in it |
| --- | --- |
| `src` | the window, TypeScript without frameworks |
| `src-tauri/src` | the program: commands of the window, and the part that works with the administrator's permission |
| `src-tauri/core` | lists, settings and the section of the hosts file |
| `src-tauri/net` | downloading lists |
| `src-tauri/system` | the hosts file, the permission and forgetting remembered addresses on each system |
| `src-tauri/phrase`, `locales` | texts in 14 languages |
| `data` | addresses shared by the program and the extension |
| `extension` | the browser extension |

## What to keep in mind

**Nothing runs in the background.** The program works while its window is open, and the browser unloads the background part of the extension.

**Lines of a person are sacred.** Outside its section ad×0 changes nothing in the hosts file, and it keeps the bytes as they are, in any encoding. If the section is damaged, it refuses to work.

**A list is not trusted.** Only names of sites are read from it, and only with the address `0.0.0.0` they get into the hosts file. In lists of the Adblock format only `||site^` is read: a name at the beginning of any other rule means "on this site", and reading it as "block this site" once closed YouTube in a test.

**macOS 10.13.** On High Sierra the system WebKit draws the window, so it is built for Safari 11: no `gap` in flex, no logical properties, no destructuring.

**Objects.** They are immutable, there is no `null`, and names are not made with "-er". A test has one assertion, and its name says what the program does. What is left undone is marked in the code with `@todo #issue` next to the place it concerns, as in [puzzle driven development](https://www.yegor256.com/2010/03/04/pdd.html). Such a comment is enough to take the work up. What is planned: [ROADMAP.md](ROADMAP.md).

## Translations

A language of the program is a file in `locales`, of the extension a folder in `extension/web/_locales`. To correct one, change its texts and leave the keys and the `{holes}` as they are: tests check that all languages have the same ones. The translated descriptions are `docs/README.*.md`.

## Releases

Set the version in `package.json`, `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`, `extension/package.json` and `extension/web/manifest.json`, write the changes and the date into `CHANGELOG.md`, and push a tag such as `0.2.0`. GitHub then builds the installers, installs each of them, starts the program, and only after that publishes the release. The extension goes to the catalog of Firefox after it, with the texts of `extension/store/firefox.json`, if the repository has the secrets `AMO_KEY` and `AMO_SECRET`. The workflow `catalog` sends a release that is already out.

## Licenses of dependencies

The code is under MIT. Dependencies are under MIT, Apache 2.0, BSD, ISC, Zlib, Unicode, BSL, MPL 2.0 and CDLA Permissive 2.0. The full list: `cargo tree --manifest-path src-tauri/Cargo.toml --format "{p} {l}"`. The installers carry the texts: `npm run licenses` collects them into `src-tauri/licenses.txt`.
