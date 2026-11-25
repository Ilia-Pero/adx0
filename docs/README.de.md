<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">ad×0</h1>

<p align="center"><b>ad × 0 = 0</b></p>

<p align="center">Werbeblocker für Windows, macOS, Linux und Browser</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/adx0/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/adx0?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · **Deutsch** · [Italiano](README.it.md) · [Беларуская](README.be.md) · [Қазақша](README.kk.md)

<p align="center">
<img src="window-off.png" width="380" alt="Fenster des Programms: Blockieren ist aus">
<img src="window-on.png" width="380" alt="Fenster des Programms: Werbung ist blockiert">
</p>

ad×0 schreibt die Adressen von Werbeservern in die hosts-Datei des Systems und beendet sich. Danach laden Programme auf dem Computer nichts mehr von diesen Adressen, und ad×0 selbst hat keinen laufenden Prozess. Eine Browser-Erweiterung blendet Werbeblöcke auf Seiten aus und entfernt Werbung aus YouTube-Videos.

## Installation

| System | Datei | So wird installiert |
| --- | --- | --- |
| Windows 10, 11 | [adx0-windows.exe](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-windows.exe) | Datei starten. Wenn Windows warnt: „Weitere Informationen“, dann „Trotzdem ausführen“. |
| macOS 10.13+ | [adx0-macos.dmg](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-macos.dmg) | Programm in „Programme“ ziehen und öffnen. Nach der ersten Ablehnung in den Systemeinstellungen „Datenschutz & Sicherheit“ öffnen und „Dennoch öffnen“ wählen. |
| Ubuntu 24.04+ | [adx0-ubuntu.deb](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-ubuntu.deb) | `sudo apt install ./adx0-ubuntu.deb` |
| Firefox 128+ | [adx0-firefox.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-firefox.zip) | `about:debugging` öffnen, „Dieser Firefox“, „Temporäres Add-on laden“ und die Datei wählen. Nach einem Neustart von Firefox erneut laden. |
| Chrome, Edge, Opera, Yandex Browser | [adx0-chrome.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-chrome.zip) | Archiv in einen Ordner entpacken, der bestehen bleibt. `chrome://extensions` öffnen, „Entwicklermodus“ einschalten, „Entpackte Erweiterung laden“ und den Ordner wählen. |

Windows und macOS warnen, weil die Dateien keine bezahlte Signatur haben. GitHub baut sie aus diesem Repository, installiert die Programme auf sauberen Rechnern und startet sie, bevor eine Version veröffentlicht wird.

Bei der Deinstallation werden die blockierten Adressen aus der hosts-Datei entfernt. Unter macOS schalten Sie das Blockieren vorher aus.

## Programm

- Blockiert etwa 10.000 Adressen von Werbe- und Zählservern. Sie wählen die Listen und können eigene Websites hinzufügen.
- Lässt Ihre Zeilen in der hosts-Datei unverändert, kopiert die Datei vor der ersten Änderung und prüft sie nach dem Schreiben.
- Aktualisiert die Listen höchstens einmal am Tag, kurz nach dem Start des Systems und ohne Fenster. Das lässt sich ausschalten.
- 14 Sprachen.

Die hosts-Datei blockiert ganze Adressen, deshalb bleibt Werbung, die von derselben Adresse wie der Inhalt kommt. Programme mit Proxy oder VPN und eigener Namensauflösung umgehen die hosts-Datei.

## Browser-Erweiterung

<p align="center"><img src="popup.png" width="312" alt="Fenster der Erweiterung"></p>

- Blockiert etwa 64.000 Adressen und blendet Werbeblöcke auf Seiten aus.
- Entfernt den Werbeplan aus den Antworten von YouTube.
- Erlaubt Werbung auf einer Website mit einer Schaltfläche.
- Aktualisiert die Listen einmal am Tag.

## Daten und Netzwerk

Programm und Erweiterung laden die Listen von deren Websites und sammeln keine Daten. Nach einer neuen Version fragt das Programm bei GitHub, wenn Sie auf „Nach Updates suchen“ klicken.

## Mehr

Listen und ihre Lizenzen, Messwerte, geprüfte Systeme und der Bau des Programms sind [auf Englisch](../README.md) beschrieben.

Dieser Text wurde maschinell übersetzt und nicht von Muttersprachlern geprüft. Korrekturen gehören in den Ordner `docs`.

[MIT](../LICENSE)
