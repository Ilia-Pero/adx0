<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">ad×0</h1>

<p align="center"><b>ad × 0 = 0</b></p>

<p align="center">Блокатор огласа за Windows, macOS, Linux и прегледаче</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/adx0/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/adx0?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · **Српски** · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md) · [Беларуская](README.be.md) · [Қазақша](README.kk.md)

<p align="center">
<img src="window-off.png" width="380" alt="Прозор програма: блокирање је искључено">
<img src="window-on.png" width="380" alt="Прозор програма: огласи су блокирани">
</p>

ad×0 уписује адресе сервера за огласе у системску датотеку hosts и завршава рад. Програми на рачунару после тога ништа не учитавају са тих адреса, а сам ad×0 нема покренут процес. Додатак за прегледач скрива огласе на страницама и уклања их из YouTube видео снимака.

## Инсталација

| Систем | Датотека | Како се инсталира |
| --- | --- | --- |
| Windows 10, 11 | [adx0-windows.exe](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-windows.exe) | Покрените датотеку. Ако Windows прикаже упозорење, изаберите „Више информација“, па „Ипак покрени“. |
| macOS 10.13+ | [adx0-macos.dmg](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-macos.dmg) | Превуците програм у „Апликације“ и отворите га. Када га macOS први пут одбије, отворите системска подешавања, „Приватност и безбедност“, и притисните „Ипак отвори“. |
| Ubuntu 24.04+ | [adx0-ubuntu.deb](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-ubuntu.deb) | `sudo apt install ./adx0-ubuntu.deb` |
| Firefox 128+ | [adx0-firefox.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-firefox.zip) | Отворите `about:debugging`, „Овај Firefox“, „Учитај привремени додатак“ и изаберите датотеку. После поновног покретања Firefox-а учитајте је опет. |
| Chrome, Edge, Opera, Yandex Browser | [adx0-chrome.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-chrome.zip) | Распакујте архиву у фасциклу коју нећете брисати. Отворите `chrome://extensions`, укључите режим за програмере, притисните „Учитај распаковано“ и изаберите фасциклу. |

Windows и macOS упозоравају јер датотеке немају плаћени потпис. GitHub их прави из овог репозиторијума, инсталира програме на чистим машинама и покреће их пре објављивања издања.

При деинсталацији се блокиране адресе уклањају из датотеке hosts. На macOS-у пре тога искључите блокирање.

## Програм

- Блокира око 10.000 адреса сервера за огласе и праћење. Листе бирате ви и можете да додате своје сајтове.
- Не дира ваше редове у датотеци hosts, прави њену копију пре прве измене и проверава датотеку после уписа.
- Ажурира листе највише једном дневно, убрзо после покретања система и без прозора. То може да се искључи.
- 14 језика.

Датотека hosts блокира целе адресе, па огласи са исте адресе као и садржај остају. Програми са проксијем или VPN-ом који сами траже адресе заобилазе је.

## Додатак за прегледач

<p align="center"><img src="popup.png" width="312" alt="Прозор додатка"></p>

- Блокира око 55.000 адреса и скрива огласе на страницама.
- Уклања распоред огласа из одговора YouTube-а.
- Дозвољава огласе на сајту једним дугметом.
- Ажурира листе једном дневно.

## Подаци и мрежа

Програм и додатак преузимају листе са њихових сајтова и не прикупљају податке. Програм пита GitHub за нову верзију када притиснете „Провери ажурирања“.

## Више

Листе и њихове лиценце, мерења, проверени системи и прављење програма описани су [на енглеском](../README.md).

Овај текст је машински превод који изворни говорници нису проверили. Исправке иду у фасциклу `docs`.

[MIT](../LICENSE)
