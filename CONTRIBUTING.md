# Contributing

## Build and test

You need Node.js 24 or later and Rust with clippy and rustfmt; on Windows also the Microsoft C++ Build Tools, on Ubuntu also `libwebkit2gtk-4.1-dev librsvg2-dev`.

```sh
npm ci
npm run build
cd extension
npm ci
npm run build
```

`npm run build` runs the linter, the type check, the tests and the bundler; for the program also rustfmt, clippy and the Rust tests. A pull request has to pass it without warnings.

| Command | What it does |
| --- | --- |
| `npm run dev` | opens the window of the program in a browser at <http://127.0.0.1:5392> (`?he` at the end selects a language); it downloads nothing and changes no files |
| `npm run tauri build` | builds the installer for your system |
| `npm run tauri build -- --debug --no-bundle` | builds a debug version without an installer |

In a debug build two variables protect your system from a test run:

| Variable | What it replaces |
| --- | --- |
| `ADX0_HOSTS` | the hosts file |
| `ADX0_HOME` | the folder with the settings and the lists |

Code for other systems can be checked on any machine:

```sh
rustup target add x86_64-unknown-linux-gnu x86_64-apple-darwin
cargo clippy --manifest-path src-tauri/Cargo.toml -p adx0-system --target x86_64-unknown-linux-gnu
cargo clippy --manifest-path src-tauri/Cargo.toml -p adx0-system --target x86_64-apple-darwin
```

## Structure

| Folder | Contents |
| --- | --- |
| `src` | the window, TypeScript without frameworks |
| `src-tauri/src` | the program: commands of the window and the part that runs with administrator permission |
| `src-tauri/core` | lists, settings and the section of the hosts file |
| `src-tauri/net` | downloading lists |
| `src-tauri/system` | everything that differs between systems: the path of the hosts file, administrator permission, scheduled updates, the DNS cache |
| `src-tauri/phrase`, `locales` | texts in 14 languages |
| `data` | addresses shared by the program and the extension |
| `extension` | the browser extension |

## Rules

**No background process.** The program runs while its window is open, and once a day for a moment to update the lists. The browser unloads the background part of the extension.

**The hosts file belongs to the user.** Outside its own section ad×0 changes nothing and keeps the bytes as they are, in any encoding. If the section is damaged, it refuses to write.

**Lists are untrusted data.** Only site names are read from them, and they enter the hosts file with the address `0.0.0.0`. From lists in the Adblock format only `||site^` is read. A name at the beginning of any other rule means “on this site”; when such names were once read as sites to block, YouTube stopped opening in a test.

**The administrator part is minimal.** It writes the hosts file and one folder of its own. Files from the folder of a user are read as data, and only site names are copied from them.

**macOS 10.13.** High Sierra draws the window with its own WebKit, so the code is built for Safari 11, which lacks `gap` in flex, logical properties and destructuring.

**Objects.** The code follows [Elegant Objects](https://www.elegantobjects.org/): objects are immutable, `null` is not used, names do not end in "-er". A test has one assertion, and its name describes behavior.

**Unfinished work** is marked with `@todo #issue` next to the code it concerns, as in [puzzle driven development](https://www.yegor256.com/2010/03/04/pdd.html). Plans are in [ROADMAP.md](ROADMAP.md).

## Translations

Each language of the program is a file in `locales`, of the extension a folder in `extension/web/_locales`. To correct one, change the texts and keep the keys and the `{placeholders}`; tests compare them with the other languages. Translated descriptions are in `docs/README.*.md`.

## Releases

1. Set the version in `package.json`, `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`, `extension/package.json` and `extension/web/manifest.json`.
2. Describe the changes in `CHANGELOG.md` under the version and the date.
3. Push a tag such as `0.2.0`.

GitHub builds the installers, installs each one, starts the program and then publishes the release. If the repository has the secrets `AMO_KEY` and `AMO_SECRET`, the release also sends the extension to Firefox Add-ons with the texts from `extension/store/firefox.json`. The workflow `catalog` does the same for a release that has already been published.

## Licenses of dependencies

The code is under MIT. Dependencies are under MIT, Apache 2.0, BSD, ISC, Zlib, Unicode, BSL, MPL 2.0 and CDLA Permissive 2.0; `cargo tree --manifest-path src-tauri/Cargo.toml --format "{p} {l}"` prints the full list. `npm run licenses` collects the texts into `src-tauri/licenses.txt`, which goes into the installers.
