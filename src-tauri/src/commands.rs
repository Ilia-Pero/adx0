//! Commands that the window of the program calls.

use std::collections::HashMap;
use std::time::UNIX_EPOCH;

use adx0_core::{Failure, Settings, SOURCES};
use adx0_net::Http;
use adx0_phrase::{Language, Phrase, LANGUAGES};
use serde::Serialize;
use tauri::{AppHandle, Manager};

use crate::guard;
use crate::home::Home;

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
    })
}

/// Downloads the chosen lists. Returns messages about those that could not be downloaded.
fn download(home: &Home) -> Result<Vec<String>, Failure> {
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
        .and_then(|home| download(&home))
        .map_err(|failure| failure.to_string())
}

/// Turns blocking on or off.
#[tauri::command(async)]
pub fn turn(app: AppHandle, on: bool) -> Result<(), String> {
    home(&app)
        .and_then(|home| guard::switch(&home, on))
        .map_err(|failure| failure.to_string())
}
