<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">ad×0</h1>

<p align="center"><b>ad × 0 = 0</b></p>

<p align="center">Блакіроўшчык рэкламы, які не працуе ў фоне: для Windows, macOS, Linux і браўзераў</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/adx0/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/adx0?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md) · **Беларуская** · [Қазақша](README.kk.md)

<p align="center">
<img src="window-off.png" width="380" alt="Акно праграмы: блакіроўка выключана">
<img src="window-on.png" width="380" alt="Акно праграмы: рэклама закрыта">
</p>

ad×0 запісвае адрасы рэкламных сервераў у сістэмны файл hosts і завяршаецца. Рэклама з гэтых адрасоў перастае загружацца ва ўсіх праграмах, а фонавага працэсу няма: займаць памяць і працэсар няма чаму. Пашырэнне для браўзера робіць тое, чаго файл hosts не можа: хавае рэкламныя блокі на старонках і прыбірае рэкламу з ролікаў YouTube.

## Праграма

- Закрывае каля 10 тысяч адрасоў рэкламных сервераў і лічыльнікаў, а з рускім спісам каля 11,7 тысячы. Спісы выбіраеце вы.
- Захоўвае вашы радкі ў файле hosts, перад першай праўкай пакідае яго копію і перачытвае тое, што запісала.
- Захоўвае апошні ўдала спампаваны спіс, таму блакіроўка не залежыць ад таго, ці працуе сайт спіса.
- Не дае памылцы ў спісе закрыць такія сайты, як Google, Wikipedia або GitHub: вось [увесь пералік](../data/essential.json). Любы іншы сайт адкрываюць у полі «Не закрываць».
- Гаворыць на 14 мовах: мову выбіраюць уверсе акна і ва ўсталёўшчыку для Windows. Нікуды нічога не адпраўляе: толькі спампоўвае спісы, калі вы націскаеце кнопку, а пры ўключанай блакіроўцы яшчэ і пры ўваходзе ў сістэму, не часцей за раз у суткі. Другое адбываецца без акна і за імгненне, яго можна выключыць. Пра новую версію даведваецца, толькі калі вы націснеце кнопку.

## Пашырэнне для браўзера

Для Firefox і для браўзераў на Chromium: Chrome, Яндэкс Браўзер, Edge, Opera.

<p align="center"><img src="popup.png" width="312" alt="Пашырэнне для браўзера"></p>

- Закрывае рэкламныя адрасы і хавае рэкламныя блокі на старонках. Рэгіянальныя спісы выбіраюцца па мовах браўзера.
- Прыбірае расклад рэкламы з адказаў YouTube, і прайгравальніку няма чаго паказваць.
- Абнаўляе спісы раз у суткі. Правілы выконвае сам браўзер, фонавая частка паміж абнаўленнямі выгружана.
- Гаворыць на тых жа 14 мовах: спачатку на мове браўзера, іншую выбіраюць у акенцы пашырэння.

## Чаго ён не можа

Файл hosts закрывае адрасы цалкам. Рэклама, якая прыходзіць з таго ж адраса, што і змест, напрыклад у YouTube і ў сацыяльных сетках, застаецца; для YouTube ёсць пашырэнне. Проксі і VPN, якія самі знаходзяць адрасы, абыходзяць файл hosts.

## Наколькі ён лёгкі

Замеры 24.11.2026 на Windows Server 2022, аптымізаваная зборка:

| Што вымяралі | Вынік |
| --- | --- |
| Працэсаў пры закрытым акне | 0 |
| Працэсар пры адкрытым акне | 0–31 мс за 5 с |
| Ад запуску да акна | 0,06–0,6 с у чатырох запусках |
| Памер праграмы | 4,4 МБ |
| На колькі расце файл hosts | 198 КБ на 9 800 адрасоў, з рускім спісам 225 КБ на 11 700 |

## Спісы

Спісы не ўваходзяць у гэты рэпазіторый. Ваша копія праграмы спампоўвае іх з іх сайтаў, у кожнага свая ліцэнзія.

| Спіс | Дзе | Ліцэнзія |
| --- | --- | --- |
| [Peter Lowe](https://pgl.yoyo.org/adservers/) | абодва | [McRae GPL](https://pgl.yoyo.org/license/) |
| [AdAway](https://adaway.org/) | абодва | CC BY 3.0 |
| [RU AdList](https://github.com/easylist/ruadlist) | абодва | CC BY 3.0 |
| [Liste FR](https://github.com/easylist/listefr) | абодва | CC BY-SA 3.0 |
| [EasyList China](https://github.com/easylist/easylistchina) | абодва | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [StevenBlack](https://github.com/StevenBlack/hosts) | праграма | MIT |
| [EasyList](https://easylist.to/) | пашырэнне | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Germany](https://github.com/easylist/easylistgermany) | пашырэнне | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Hebrew](https://github.com/easylist/EasyListHebrew) | пашырэнне | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Italy](https://github.com/easylist/easylistitaly) | пашырэнне | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Spanish](https://github.com/easylist/easylistspanish) | пашырэнне | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [KoreanList](https://github.com/easylist/KoreanList) | пашырэнне | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [AdGuard Japanese](https://github.com/AdguardTeam/AdguardFilters) | пашырэнне | GPL 3.0 |
| [Greek AdBlock Filter](https://github.com/kargig/greek-adblockplus-filter) | пашырэнне | CC BY-SA 4.0 |

## Стан

| Сістэма | Што вядома |
| --- | --- |
| Windows | Працуе на Windows Server 2022: правераны з сапраўднымі спісамі на копіі файла hosts. На Windows 10 і 11 яшчэ не правяралі. |
| macOS 10.13+ | Збіраецца, тэсты праходзяць; на машыне GitHub усталёўваецца і запускаецца. Рукамі яго яшчэ ніхто не правяраў. |
| Ubuntu 24.04+ | Збіраецца, тэсты праходзяць; на машыне GitHub усталёўваецца і запускаецца. Рукамі яго яшчэ ніхто не правяраў. |
| Firefox 128+ | Працуе ў Firefox 161. Mozilla яго яшчэ не падпісала, таму пасля перазапуску Firefox яго загружаюць зноў. |
| Chrome, Яндэкс Браўзер, Edge, Opera | Працуе ў Edge 158, праверана без акна. Астатнія не правяралі, патрэбны Chromium 120 або навейшы. |

## Усталяванне

| Сістэма | Файл | Як усталяваць |
| --- | --- | --- |
| Windows 10 і 11 | [adx0-windows.exe](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-windows.exe) | Запусціце яго. Калі Windows не ведае праграму: «Падрабязней», «Выканаць у любым выпадку». |
| macOS 10.13+ | [adx0-macos.dmg](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-macos.dmg) | Перацягніце праграму ў Applications і адкрыйце. Калі macOS адмовіць, адкрыйце System Settings, Privacy & Security, і націсніце Open Anyway. |
| Ubuntu 24.04+ | [adx0-ubuntu.deb](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-ubuntu.deb) | `sudo apt install ./adx0-ubuntu.deb` |
| Firefox 128+ | [adx0-firefox.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-firefox.zip) | Адкрыйце `about:debugging`, This Firefox, Load Temporary Add-on і выберыце файл. Пасля перазапуску Firefox загрузіце яго зноў. |
| Chrome, Яндэкс Браўзер, Edge, Opera | [adx0-chrome.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-chrome.zip) | Распакуйце, адкрыйце `chrome://extensions`, уключыце Developer mode, націсніце Load unpacked і выберыце папку; папку не выдаляйце. Яндэкс Браўзер давярае толькі каталогам і можа выключыць пашырэнне. |

GitHub збірае гэтыя файлы з кода гэтага рэпазіторыя, усталёўвае і запускае праграмы і толькі потым усё публікуе. Платнага подпісу ў файлаў няма, таму пры першым запуску Windows і macOS папярэджваюць.

Праграму можна сабраць і самому. Спатрэбяцца Node.js 24 або навейшы і Rust, на Windows яшчэ Microsoft C++ Build Tools, на Ubuntu яшчэ `libwebkit2gtk-4.1-dev librsvg2-dev`.

```sh
git clone https://github.com/Ilia-Pero/adx0
cd adx0
npm ci
npm run tauri build
```

Усталёўшчык з’явіцца ў `src-tauri/target/release/bundle`.

Пры выдаленні праграма зноў адкрывае закрытыя адрасы. На macOS выключыце блакіроўку да выдалення.

Пашырэнне: выканайце `npm ci && npm run build` у папцы `extension`, адкрыйце ў Firefox `about:debugging`, выберыце This Firefox, Load Temporary Add-on і файл `extension/packed/firefox/manifest.json`. Для Chromium патрэбна папка `extension/packed/chrome`.

## Пераклады

Асноўныя тэксты рускія і англійскія. Астатнія пераклады машынныя, носьбіты мовы іх не правяралі. Калі бачыце памылку, выпраўце яе ў `locales`, у `extension/web/_locales` або ў `docs`.

## Удзел і ліцэнзія

Як сабраць, праверыць і змяніць праграму: [CONTRIBUTING.md](../CONTRIBUTING.md). Ліцэнзія [MIT](../LICENSE).
