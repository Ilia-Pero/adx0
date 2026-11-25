//! Commands that the window of the program calls.

use std::collections::HashMap;
use std::time::UNIX_EPOCH;

use adx0_core::{Failure, Settings, SOURCES};
use adx0_net::{Http, Latest};
use adx0_phrase::{Language, Phrase, LANGUAGES};
use adx0_system::{Native, System};
use serde::Serialize;
use tauri::{AppHandle, Manager};

use crate::guard;
use crate::home::Home;

/// The page of the program on GitHub.
const HOME: &str = env!("CARGO_PKG_REPOSITORY");

/// The installer for this system.
const INSTALLER: &str = if cfg!(target_os = "windows") {
    "adx0-windows.exe"
} else if cfg!(target_os = "macos") {
    "adx0-macos.dmg"
} else {
    "adx0-ubuntu.deb"
};

/// A list in the window.
#[derive(Serialize)]
struct Row {
    id: &'static str,
    title: &'static str,
    note: String,
    on: bool,
    /// When it was downloaded, in seconds since 1970. Zero if it is not downloaded yet.
    updated: u64,
}

/// A language in the window.
#[derive(Serialize)]
struct Tongue {
    code: &'static str,
    title: String,
}

/// Everything the window shows.
#[derive(Serialize)]
pub struct Report {
    /// How many addresses are blocked now. Zero if blocking is off.
    count: usize,
    rows: Vec<Row>,
    settings: Settings,
    /// The code of the current language.
    spoken: &'static str,
    languages: Vec<Tongue>,
    words: HashMap<String, String>,
    /// The version of the program.
    version: String,
}

/// The latest release.
#[derive(Serialize)]
pub struct Release {
    version: String,
    /// Whether it is newer than this program.
    newer: bool,
}

/// The folder of the program. In debug builds `ADX0_HOME` replaces it.
pub fn home(app: &AppHandle) -> Result<Home, Failure> {
    match guard::rehearsal("ADX0_HOME") {
        Some(root) => Ok(Home::new(root)),
        None => app
            .path()
            .app_config_dir()
            .map(Home::new)
            .map_err(|failure| Failure::Disk(failure.to_string())),
    }
}

fn report(home: &Home) -> Result<Report, Failure> {
    let settings = home.settings().read()?;
    Language::choose(&settings.language);
    let shelf = home.shelf();
    // The part that starts by itself has lists of its own, and they may be newer.
    let kept = guard::kept();
    Ok(Report {
        count: guard::hosts()?.read()?.count(),
        rows: SOURCES
            .iter()
            .map(|source| Row {
                id: source.id,
                title: source.title,
                note: Phrase::new(source.note).to_string(),
                on: settings.sources.iter().any(|id| id == source.id),
                updated: shelf
                    .moment(source.id)
                    .max(
                        kept.as_ref()
                            .and_then(|kept| kept.shelf().moment(source.id)),
                    )
                    .and_then(|moment| moment.duration_since(UNIX_EPOCH).ok())
                    .map_or(0, |age| age.as_secs()),
            })
            .collect(),
        settings,
        spoken: Language::current().code,
        languages: LANGUAGES
            .iter()
            .map(|language| Tongue {
                code: language.code,
                title: language.text("language"),
            })
            .collect(),
        words: Language::current().texts(),
        version: env!("CARGO_PKG_VERSION").to_owned(),
    })
}

fn released() -> Result<Release, Failure> {
    let latest = Latest::new(HOME).version()?;
    Ok(Release {
        version: latest.to_string(),
        newer: latest > env!("CARGO_PKG_VERSION").parse()?,
    })
}

/// Downloads the chosen lists. Returns messages about those that could not be downloaded.
fn renewed(home: &Home) -> Result<Vec<String>, Failure> {
    let settings = home.settings().read()?;
    let shelf = home.shelf();
    Ok(SOURCES
        .iter()
        .filter(|source| settings.sources.iter().any(|id| id == source.id))
        .filter_map(|source| shelf.refresh(source, &Http).err())
        .map(|failure| failure.to_string())
        .collect())
}

/// What to show in the window.
#[tauri::command(async)]
pub fn state(app: AppHandle) -> Result<Report, String> {
    home(&app)
        .and_then(|home| report(&home))
        .map_err(|failure| failure.to_string())
}

/// Remembers the settings.
#[tauri::command(async)]
pub fn save(app: AppHandle, settings: Settings) -> Result<(), String> {
    home(&app)
        .and_then(|home| home.settings().write(&settings))
        .map_err(|failure| failure.to_string())
}

/// Downloads the chosen lists.
#[tauri::command(async)]
pub fn refresh(app: AppHandle) -> Result<Vec<String>, String> {
    home(&app)
        .and_then(|home| renewed(&home))
        .map_err(|failure| failure.to_string())
}

/// Turns blocking on or off.
#[tauri::command(async)]
pub fn turn(app: AppHandle, on: bool) -> Result<(), String> {
    home(&app)
        .and_then(|home| guard::switch(&home, on))
        .map_err(|failure| failure.to_string())
}

/// Looks for a newer version. The program does this only when a person asks.
#[tauri::command(async)]
pub fn update() -> Result<Release, String> {
    released().map_err(|failure| failure.to_string())
}

/// Downloads the installer of the latest release with the browser.
#[tauri::command(async)]
pub fn download() -> Result<(), String> {
    browse(&Latest::new(HOME).file(INSTALLER))
}

/// Shows what is new in the latest release.
#[tauri::command(async)]
pub fn news() -> Result<(), String> {
    browse(&Latest::new(HOME).page())
}

fn browse(url: &str) -> Result<(), String> {
    Native.browse(url).map_err(|failure| failure.to_string())
}
