<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">ad×0</h1>

<p align="center"><b>ad × 0 = 0</b></p>

<p align="center">Блокатор огласа који не ради у позадини: за Windows, macOS, Linux и Firefox</p>

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

ad×0 уписује адресе огласних сервера у системску датотеку hosts и затвара се. Огласи са тих адреса престају да се учитавају у свим програмима, а позадинског процеса који би заузимао меморију или процесор нема. Додатак за Firefox ради оно што датотека hosts не може: скрива огласне блокове на страницама и уклања огласе из YouTube снимака.

## Програм

- Блокира око 10.000 адреса огласних сервера и пратилаца са листа које изаберете.
- Чува ваше редове у датотеци hosts, пре прве измене оставља њену копију и поново чита оно што је уписао.
- Чува последњу исправну копију сваке листе, па блокирање не зависи од тога да ли сајт листе ради.
- Не дозвољава да грешка у листи затвори сајтове као што су Google, Wikipedia или GitHub: ево [целог списка](../data/essential.json). Сваки други сајт отвара се у пољу „Не блокирај“.
- Говори 14 језика: језик се бира на врху прозора и у инсталацији за Windows. Никуда ништа не шаље: само преузима листе, када притиснете дугме и, док је блокирање укључено, при покретању система, највише једном дневно. Ово друго се дешава без прозора за тренутак и може се искључити.

## Додатак за Firefox

<p align="center"><img src="popup.png" width="312" alt="Додатак за Firefox"></p>

- Блокира огласне адресе и скрива огласне блокове на страницама. Регионалне листе бирају се по језицима прегледача.
- Уклања распоред огласа из одговора YouTube-а, па плејер нема шта да прикаже.
- Ажурира листе једном дневно. Правила спроводи сам прегледач, а позадински део у међувремену није учитан.
- Говори истих 14 језика: на почетку језик прегледача, а други се бира у прозору додатка.

## Шта не може

Датотека hosts ради са целим адресама. Огласи који долазе са исте адресе као садржај, као на YouTube-у или на друштвеним мрежама, остају; за YouTube постоји додатак. Прокси или VPN који сам проналази адресе заобилази датотеку hosts.

## Колико је лаган

Измерено 24.11.2026. на Windows Server 2022, оптимизована верзија:

| Шта | Резултат |
| --- | --- |
| Процеса док је прозор затворен | 0 |
| Процесор док је прозор отворен | 0–31 ms за 5 s |
| Од покретања до прозора | 0,06–0,6 s у четири покретања |
| Величина програма | 4,4 MB |
| Колико порасте датотека hosts | 198 KB за 9.800 адреса |

## Листе

Листе нису део овог репозиторијума. Ваша копија програма преузима их са њихових сајтова и свака има своју лиценцу.

| Листа | Где | Лиценца |
| --- | --- | --- |
| [Peter Lowe](https://pgl.yoyo.org/adservers/) | оба | [McRae GPL](https://pgl.yoyo.org/license/) |
| [AdAway](https://adaway.org/) | оба | CC BY 3.0 |
| [RU AdList](https://github.com/easylist/ruadlist) | оба | CC BY 3.0 |
| [Liste FR](https://github.com/easylist/listefr) | оба | CC BY-SA 3.0 |
| [EasyList China](https://github.com/easylist/easylistchina) | оба | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [StevenBlack](https://github.com/StevenBlack/hosts) | програм | MIT |
| [EasyList](https://easylist.to/) | додатак | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Germany](https://github.com/easylist/easylistgermany) | додатак | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Hebrew](https://github.com/easylist/EasyListHebrew) | додатак | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Italy](https://github.com/easylist/easylistitaly) | додатак | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Spanish](https://github.com/easylist/easylistspanish) | додатак | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [KoreanList](https://github.com/easylist/KoreanList) | додатак | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [AdGuard Japanese](https://github.com/AdguardTeam/AdguardFilters) | додатак | GPL 3.0 |
| [Greek AdBlock Filter](https://github.com/kargig/greek-adblockplus-filter) | додатак | CC BY-SA 4.0 |

## Стање

| Систем | Стање |
| --- | --- |
| Windows | Ради на Windows Server 2022: проверено са правим листама на копији датотеке hosts. На Windows 10 и 11 још није проверено. |
| macOS 10.13+ | Гради се и пролази тестове; на машини GitHub-а се инсталира и покреће. Ручно га још нико није испробао. |
| Ubuntu 24.04+ | Гради се и пролази тестове; на машини GitHub-а се инсталира и покреће. Ручно га још нико није испробао. |
| Firefox 128+ | Ради у Firefox-у 160. Mozilla га још није потписала, па га после поновног покретања Firefox-а треба поново учитати. |

## Инсталација

| Систем | Датотека | Како се инсталира |
| --- | --- | --- |
| Windows 10 и 11 | [adx0-windows.exe](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-windows.exe) | Покрените је. Ако Windows не познаје програм: „Више информација“, „Ипак покрени“. |
| macOS 10.13+ | [adx0-macos.dmg](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-macos.dmg) | Превуците програм у Applications и отворите га. Кад macOS одбије, отворите System Settings, „Privacy & Security“, и притисните „Open Anyway“. |
| Ubuntu 24.04+ | [adx0-ubuntu.deb](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-ubuntu.deb) | `sudo apt install ./adx0-ubuntu.deb` |
| Firefox 128+ | [adx0-firefox.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-firefox.zip) | Отворите `about:debugging`, „Овај Firefox“, „Учитај привремени додатак“, и изаберите датотеку. После поновног покретања Firefox-а учитајте је поново. |

GitHub гради ове датотеке из кода овог репозиторијума, инсталира и покреће програме, па тек онда све објављује. Датотеке немају плаћени потпис, зато Windows и macOS упозоравају при првом покретању.

Програм можете изградити и сами. Потребни су Node.js 24 или новији и Rust, на Windows-у и Microsoft C++ Build Tools, на Ubuntu-у и `libwebkit2gtk-4.1-dev librsvg2-dev`.

```sh
git clone https://github.com/Ilia-Pero/adx0
cd adx0
npm ci
npm run tauri build
```

Инсталациони програм се појављује у `src-tauri/target/release/bundle`.

Деинсталација поново отвара блокиране адресе. На macOS-у искључите блокирање пре брисања програма.

Додатак: покрените `npm ci && npm run build` у фасцикли `extension`, отворите `about:debugging` у Firefox-у, изаберите „Овај Firefox“, „Учитај привремени додатак“ и датотеку `extension/web/manifest.json`.

## Преводи

Преводи су машински и изворни говорници их нису проверили. Ако видите грешку, исправите је у `locales`, у `extension/web/_locales` или у `docs`.

## Учешће и лиценца

Како изградити, проверити и мењати програм: [CONTRIBUTING.md](../CONTRIBUTING.md). Лиценца је [MIT](../LICENSE).
