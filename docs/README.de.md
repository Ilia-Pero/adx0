<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">ad×0</h1>

<p align="center"><b>ad × 0 = 0</b></p>

<p align="center">Ein Werbeblocker, der nicht im Hintergrund läuft: für Windows, macOS, Linux und Firefox</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<img src="https://img.shields.io/badge/languages-12-0f766e" alt="12 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · **Deutsch** · [Italiano](README.it.md)

<p align="center">
<img src="window-off.png" width="380" alt="Das Fenster des Programms: Die Blockierung ist aus">
<img src="window-on.png" width="380" alt="Das Fenster des Programms: Werbung ist blockiert">
</p>

ad×0 schreibt Adressen von Werbeservern in die Datei hosts des Systems und beendet sich. Werbung von diesen Adressen wird in keinem Programm mehr geladen, und es gibt keinen Hintergrundprozess, der Speicher oder Prozessor belegt. Eine Erweiterung für Firefox tut, was die Datei hosts nicht kann: Sie blendet Werbeblöcke auf Seiten aus und entfernt Werbung aus YouTube-Videos.

## Das Programm

- Blockiert etwa 10.000 Adressen von Werbeservern und Trackern aus den Listen, die Sie wählen.
- Behält Ihre eigenen Zeilen in der Datei hosts, legt vor der ersten Änderung eine Kopie an und liest zurück, was es geschrieben hat.
- Bewahrt die letzte gute Kopie jeder Liste auf, sodass die Blockierung nicht davon abhängt, ob die Website der Liste erreichbar ist.
- Lässt nicht zu, dass ein Fehler in einer Liste Seiten wie Google, Wikipedia oder GitHub sperrt: hier ist [die ganze Liste](../data/essential.json). Jede andere Seite wird im Feld „Nicht blockieren“ geöffnet.
- Spricht 12 Sprachen. Sendet nichts: Es lädt nur die Listen herunter, wenn Sie auf die Schaltfläche klicken.

## Die Erweiterung für Firefox

<p align="center"><img src="popup.png" width="312" alt="Die Erweiterung für Firefox"></p>

- Blockiert Werbeadressen und blendet Werbeblöcke auf Seiten aus. Regionale Listen werden nach den Sprachen des Browsers gewählt.
- Entfernt den Werbeplan aus den Antworten von YouTube, sodass der Player nichts zu zeigen hat.
- Aktualisiert die Listen einmal am Tag. Der Browser selbst setzt die Regeln durch, der Hintergrundteil ist dazwischen nicht geladen.

## Was es nicht kann

Die Datei hosts arbeitet mit ganzen Adressen. Werbung, die von derselben Adresse wie der Inhalt kommt, etwa bei YouTube oder in sozialen Netzwerken, bleibt; für YouTube gibt es die Erweiterung. Ein Proxy oder VPN, der Adressen selbst sucht, umgeht die Datei hosts.

## Wie leicht es ist

Gemessen am 24.11.2026 unter Windows Server 2022, optimierter Build:

| Was | Ergebnis |
| --- | --- |
| Prozesse bei geschlossenem Fenster | 0 |
| Prozessor bei offenem Fenster | 0–31 ms in 5 s |
| Vom Start bis zum Fenster | 0,06–0,6 s in vier Läufen |
| Größe des Programms | 4,4 MB |
| Wachstum der Datei hosts | 198 KB für 9.800 Adressen |

## Listen

Die Listen gehören nicht zu diesem Repository. Ihre Kopie des Programms lädt sie von deren Websites, und jede hat ihre eigene Lizenz.

| Liste | Wo | Lizenz |
| --- | --- | --- |
| [Peter Lowe](https://pgl.yoyo.org/adservers/) | beide | [McRae GPL](https://pgl.yoyo.org/license/) |
| [AdAway](https://adaway.org/) | beide | CC BY 3.0 |
| [RU AdList](https://github.com/easylist/ruadlist) | beide | CC BY 3.0 |
| [Liste FR](https://github.com/easylist/listefr) | beide | CC BY-SA 3.0 |
| [EasyList China](https://github.com/easylist/easylistchina) | beide | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [StevenBlack](https://github.com/StevenBlack/hosts) | Programm | MIT |
| [EasyList](https://easylist.to/) | Erweiterung | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Germany](https://github.com/easylist/easylistgermany) | Erweiterung | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Hebrew](https://github.com/easylist/EasyListHebrew) | Erweiterung | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Italy](https://github.com/easylist/easylistitaly) | Erweiterung | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Spanish](https://github.com/easylist/easylistspanish) | Erweiterung | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [KoreanList](https://github.com/easylist/KoreanList) | Erweiterung | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [AdGuard Japanese](https://github.com/AdguardTeam/AdguardFilters) | Erweiterung | GPL 3.0 |
| [Greek AdBlock Filter](https://github.com/kargig/greek-adblockplus-filter) | Erweiterung | CC BY-SA 4.0 |

## Stand

| System | Stand |
| --- | --- |
| Windows | Funktioniert unter Windows Server 2022: mit echten Listen an einer Kopie der Datei hosts geprüft. Unter Windows 10 und 11 noch nicht geprüft. |
| macOS 10.13+ | Lässt sich bauen und besteht die Tests. Noch niemand hat es gestartet. |
| Ubuntu 24.04+ | Lässt sich bauen und besteht die Tests. Noch niemand hat es gestartet. |
| Firefox 128+ | Funktioniert in Firefox 160. Von Mozilla noch nicht signiert und muss nach einem Neustart von Firefox erneut geladen werden. |

## Installation

Fertige Dateien gibt es noch nicht, das Programm muss gebaut werden. Nötig sind Node.js 24 oder neuer und Rust, unter Windows auch die Microsoft C++ Build Tools, unter Ubuntu auch `libwebkit2gtk-4.1-dev librsvg2-dev`.

```sh
git clone https://github.com/Ilia-Pero/adx0
cd adx0
npm ci
npm run tauri build
```

Das Installationsprogramm erscheint in `src-tauri/target/release/bundle`. Es ist nicht signiert, deshalb fragen Windows und macOS, ob Sie ihm vertrauen.

Die Deinstallation gibt die blockierten Adressen wieder frei. Unter macOS schalten Sie die Blockierung aus, bevor Sie das Programm löschen.

Die Erweiterung: Führen Sie `npm ci && npm run build` im Ordner `extension` aus, öffnen Sie `about:debugging` in Firefox, wählen Sie „Dieser Firefox“, „Temporäres Add-on laden“ und die Datei `extension/web/manifest.json`.

## Übersetzungen

Die Übersetzungen sind maschinell und wurden nicht von Muttersprachlern geprüft. Wenn Sie einen Fehler sehen, berichtigen Sie ihn bitte in `locales`, in `extension/web/_locales` oder in `docs`.

## Mitmachen und Lizenz

So wird das Programm gebaut, geprüft und geändert: [CONTRIBUTING.md](../CONTRIBUTING.md). Die Lizenz ist [MIT](../LICENSE).
