<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">ad×0</h1>

<p align="center"><b>ad × 0 = 0</b></p>

<p align="center">Блокировщик рекламы, который не работает в фоне: для Windows, macOS, Linux и Firefox</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/adx0/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/adx0?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · **Русский** · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md) · [Беларуская](README.be.md) · [Қазақша](README.kk.md)

<p align="center">
<img src="window-off.png" width="380" alt="Окно программы: блокировка выключена">
<img src="window-on.png" width="380" alt="Окно программы: реклама закрыта">
</p>

ad×0 записывает адреса рекламных серверов в системный файл hosts и завершается. Реклама с этих адресов перестаёт загружаться во всех программах, а фонового процесса нет: занимать память и процессор нечему. Расширение для Firefox делает то, чего файл hosts не может: скрывает рекламные блоки на страницах и убирает рекламу из роликов YouTube.

## Программа

- Закрывает около 10 тысяч адресов рекламных серверов и счётчиков, а с русским списком около 11,7 тысячи. Списки выбираете вы.
- Сохраняет ваши строки в файле hosts, перед первой правкой оставляет его копию и перечитывает то, что записала.
- Хранит последний удачно скачанный список, поэтому блокировка не зависит от того, работает ли сайт списка.
- Не даёт ошибке в списке закрыть такие сайты, как Яндекс, Google или Госуслуги: вот [весь перечень](../data/essential.json). Любой другой сайт открывают в поле «Не закрывать».
- Говорит на 14 языках: язык выбирают вверху окна и в установщике для Windows. Никуда ничего не отправляет: только скачивает списки, когда вы нажимаете кнопку, а при включённой блокировке ещё и при входе в систему, не чаще раза в сутки. Второе происходит без окна и за мгновение, его можно выключить.

## Расширение для Firefox

<p align="center"><img src="popup.png" width="312" alt="Расширение для Firefox"></p>

- Закрывает рекламные адреса и скрывает рекламные блоки на страницах. Региональные списки выбираются по языкам браузера.
- Убирает расписание рекламы из ответов YouTube, и проигрывателю нечего показывать.
- Обновляет списки раз в сутки. Правила исполняет сам браузер, фоновая часть между обновлениями выгружена.
- Говорит на тех же 14 языках: сначала на языке браузера, другой выбирают в окошке расширения.

## Чего он не может

Файл hosts закрывает адреса целиком. Реклама, которая приходит с того же адреса, что и содержимое, например в YouTube и в социальных сетях, остаётся; для YouTube есть расширение. Прокси и VPN, которые сами находят адреса, обходят файл hosts.

## Насколько он лёгкий

Замеры 24.11.2026 на Windows Server 2022, оптимизированная сборка:

| Что измеряли | Результат |
| --- | --- |
| Процессов при закрытом окне | 0 |
| Процессор при открытом окне | 0–31 мс за 5 с |
| От запуска до окна | 0,06–0,6 с в четырёх запусках |
| Размер программы | 4,4 МБ |
| На сколько вырастает файл hosts | 198 КБ на 9 800 адресов, с русским списком 225 КБ на 11 700 |

## Списки

Списки не входят в этот репозиторий. Ваша копия программы скачивает их с их сайтов, у каждого своя лицензия.

| Список | Где | Лицензия |
| --- | --- | --- |
| [Peter Lowe](https://pgl.yoyo.org/adservers/) | оба | [McRae GPL](https://pgl.yoyo.org/license/) |
| [AdAway](https://adaway.org/) | оба | CC BY 3.0 |
| [RU AdList](https://github.com/easylist/ruadlist) | оба | CC BY 3.0 |
| [Liste FR](https://github.com/easylist/listefr) | оба | CC BY-SA 3.0 |
| [EasyList China](https://github.com/easylist/easylistchina) | оба | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [StevenBlack](https://github.com/StevenBlack/hosts) | программа | MIT |
| [EasyList](https://easylist.to/) | расширение | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Germany](https://github.com/easylist/easylistgermany) | расширение | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Hebrew](https://github.com/easylist/EasyListHebrew) | расширение | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Italy](https://github.com/easylist/easylistitaly) | расширение | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Spanish](https://github.com/easylist/easylistspanish) | расширение | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [KoreanList](https://github.com/easylist/KoreanList) | расширение | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [AdGuard Japanese](https://github.com/AdguardTeam/AdguardFilters) | расширение | GPL 3.0 |
| [Greek AdBlock Filter](https://github.com/kargig/greek-adblockplus-filter) | расширение | CC BY-SA 4.0 |

## Состояние

| Система | Что известно |
| --- | --- |
| Windows | Работает на Windows Server 2022: проверен с настоящими списками на копии файла hosts. На Windows 10 и 11 ещё не проверяли. |
| macOS 10.13+ | Собирается, тесты проходят; на машине GitHub устанавливается и запускается. Руками его ещё никто не проверял. |
| Ubuntu 24.04+ | Собирается, тесты проходят; на машине GitHub устанавливается и запускается. Руками его ещё никто не проверял. |
| Firefox 128+ | Работает в Firefox 160. Mozilla его ещё не подписала, поэтому после перезапуска Firefox его загружают заново. |

## Установка

| Система | Файл | Как установить |
| --- | --- | --- |
| Windows 10 и 11 | [adx0-windows.exe](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-windows.exe) | Запустите его. Если Windows не знает программу: «Подробнее», «Выполнить в любом случае». |
| macOS 10.13+ | [adx0-macos.dmg](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-macos.dmg) | Перетащите программу в «Программы» и откройте. Когда macOS откажет, откройте системные настройки, «Конфиденциальность и безопасность», и нажмите «Подтвердить вход». |
| Ubuntu 24.04+ | [adx0-ubuntu.deb](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-ubuntu.deb) | `sudo apt install ./adx0-ubuntu.deb` |
| Firefox 128+ | [adx0-firefox.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-firefox.zip) | Откройте `about:debugging`, «Этот Firefox», «Загрузить временное дополнение» и выберите файл. После перезапуска Firefox загрузите его снова. |

GitHub собирает эти файлы из кода этого репозитория, устанавливает и запускает программы и только потом всё публикует. Платной подписи у файлов нет, поэтому при первом запуске Windows и macOS предупреждают.

Программу можно собрать и самому. Понадобятся Node.js 24 или новее и Rust, на Windows ещё Microsoft C++ Build Tools, на Ubuntu ещё `libwebkit2gtk-4.1-dev librsvg2-dev`.

```sh
git clone https://github.com/Ilia-Pero/adx0
cd adx0
npm ci
npm run tauri build
```

Установщик появится в `src-tauri/target/release/bundle`.

При удалении программа снова открывает закрытые адреса. На macOS выключите блокировку до удаления.

Расширение: выполните `npm ci && npm run build` в папке `extension`, откройте в Firefox `about:debugging`, выберите «Этот Firefox», «Загрузить временное дополнение» и файл `extension/web/manifest.json`.

## Переводы

Русский и английский тексты основные. Остальные переводы машинные, носители языка их не проверяли. Если видите ошибку, исправьте её в `locales`, в `extension/web/_locales` или в `docs`.

## Участие и лицензия

Как собрать, проверить и изменить программу: [CONTRIBUTING.md](../CONTRIBUTING.md). Лицензия [MIT](../LICENSE).
