<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">ad×0</h1>

<p align="center"><b>ad × 0 = 0</b></p>

<p align="center">Bloqueur de publicités pour Windows, macOS, Linux et navigateurs</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/adx0/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/adx0?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · **Français** · [Deutsch](README.de.md) · [Italiano](README.it.md) · [Беларуская](README.be.md) · [Қазақша](README.kk.md)

<p align="center">
<img src="window-off.png" width="380" alt="Fenêtre du programme : le blocage est désactivé">
<img src="window-on.png" width="380" alt="Fenêtre du programme : les publicités sont bloquées">
</p>

ad×0 écrit les adresses des serveurs publicitaires dans le fichier hosts du système et se ferme. Les programmes de l’ordinateur ne chargent alors plus rien depuis ces adresses, et ad×0 n’a aucun processus actif. Une extension masque les blocs publicitaires sur les pages et retire la publicité des vidéos YouTube.

## Installation

| Système | Fichier | Comment installer |
| --- | --- | --- |
| Windows 10, 11 | [adx0-windows.exe](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-windows.exe) | Lancez le fichier. Si Windows affiche un avertissement, choisissez « Informations complémentaires », puis « Exécuter quand même ». |
| macOS 10.13+ | [adx0-macos.dmg](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-macos.dmg) | Glissez le programme dans Applications et ouvrez-le. Après le premier refus de macOS, ouvrez Réglages Système, « Confidentialité et sécurité », et cliquez sur « Ouvrir quand même ». |
| Ubuntu 24.04+ | [adx0-ubuntu.deb](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-ubuntu.deb) | `sudo apt install ./adx0-ubuntu.deb` |
| Firefox 128+ | [adx0-firefox.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-firefox.zip) | Ouvrez `about:debugging`, « Ce Firefox », « Charger un module complémentaire temporaire » et choisissez le fichier. Après un redémarrage de Firefox, chargez-le à nouveau. |
| Chrome, Edge, Opera, Yandex Browser | [adx0-chrome.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-chrome.zip) | Décompressez l’archive dans un dossier que vous garderez. Ouvrez `chrome://extensions`, activez le mode développeur, cliquez sur « Charger l’extension non empaquetée » et choisissez le dossier. |

Windows et macOS affichent un avertissement parce que les fichiers n’ont pas de signature payante. GitHub les compile à partir de ce dépôt, installe les programmes sur des machines vierges et les lance avant de publier une version.

La désinstallation retire les adresses bloquées du fichier hosts. Sous macOS, désactivez le blocage auparavant.

## Programme

- Bloque environ 10 000 adresses de serveurs publicitaires et de suivi. Vous choisissez les listes et pouvez ajouter vos propres sites.
- Laisse vos lignes du fichier hosts intactes, en fait une copie avant la première modification et vérifie le fichier après l’écriture.
- Met à jour les listes une fois par jour au plus, peu après le démarrage du système et sans fenêtre. Cela peut être désactivé.
- 14 langues.

Le fichier hosts bloque des adresses entières : la publicité qui vient de la même adresse que le contenu reste donc visible. Les programmes qui passent par un proxy ou un VPN avec leur propre résolution d’adresses le contournent.

## Extension de navigateur

<p align="center"><img src="popup.png" width="312" alt="Fenêtre de l’extension"></p>

- Bloque environ 64 000 adresses et masque les blocs publicitaires sur les pages.
- Retire le programme publicitaire des réponses de YouTube.
- Autorise la publicité sur un site avec un bouton.
- Met à jour les listes une fois par jour.

## Données et réseau

Le programme et l’extension téléchargent les listes depuis leurs sites et ne collectent aucune donnée. Le programme interroge GitHub sur une nouvelle version quand vous cliquez sur « Rechercher des mises à jour ».

## En savoir plus

Les listes et leurs licences, les mesures, les systèmes vérifiés et la compilation sont décrits [en anglais](../README.md).

Ce texte a été traduit par une machine et n’a pas été relu par des locuteurs natifs. Les corrections se font dans le dossier `docs`.

[MIT](../LICENSE)
