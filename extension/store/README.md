# The extension in catalogs

What to write and what to upload when the extension is published. The files come from a [release](https://github.com/Ilia-Pero/adx0/releases/latest). A release sends the extension to Firefox Add-ons by itself, with the texts of [firefox.json](firefox.json).

| Catalog | Browsers | File | Registration |
| --- | --- | --- | --- |
| [Firefox Add-ons](https://addons.mozilla.org/developers/) | Firefox | `adx0-firefox.zip` | a Mozilla account |
| [Microsoft Edge Add-ons](https://partner.microsoft.com/dashboard/microsoftedge) | Edge | `adx0-chrome.zip` | free, a Microsoft or GitHub account |
| [Opera Add-ons](https://addons.opera.com/developer/) | Opera, Yandex Browser | `adx0-chrome.zip` | an Opera account |
| [Chrome Web Store](https://chrome.google.com/webstore/devconsole) | Chrome, Yandex Browser, Edge, Opera | `adx0-chrome.zip` | a fee, once |

Pictures: `logo.png` is 300 × 300, `screen.png` is 1280 × 800.

## Name

ad×0

## Summary

Removes ads on pages and in YouTube videos. No load at rest.

## Description

ad × 0 = 0. Blocks about 64,000 addresses of ad servers and trackers, hides ad blocks on pages and takes the schedule of ads out of the answers of YouTube, so the player has nothing to show.

The rules are run by the browser itself, and the background part of the extension is unloaded between updates and pages, so there is no load at rest. The lists are downloaded from their own sites once a day: Peter Lowe, AdAway, EasyList, and a regional list chosen by the languages of the browser. The last good copy of each list is kept, so blocking does not depend on whether its site is up.

Ads can be allowed on any site with one button. A mistake in a list cannot close sites such as Google, Wikipedia or GitHub. The extension has no account, no telemetry and no server, and it lets no ads through for money. It speaks 14 languages. The code is open, under MIT: <https://github.com/Ilia-Pero/adx0>

## Описание

ad × 0 = 0. Закрывает около 64 тысяч адресов рекламных серверов и счётчиков, скрывает рекламные блоки на страницах и убирает расписание рекламы из ответов YouTube, поэтому проигрывателю нечего показывать.

Правила исполняет сам браузер, а фоновая часть расширения между обновлениями и страницами выгружена, поэтому в покое нагрузки нет. Списки скачиваются с их сайтов раз в сутки: Peter Lowe, AdAway, EasyList и региональный список по языкам браузера. Последняя удачная копия каждого списка хранится, поэтому блокировка не зависит от того, работает ли его сайт.

Рекламу на любом сайте можно открыть одной кнопкой. Ошибка в списке не закроет такие сайты, как Яндекс, Google или Госуслуги. У расширения нет учётной записи, телеметрии и сервера, рекламу за деньги оно не пропускает. Говорит на 14 языках. Код открыт, лицензия MIT: <https://github.com/Ilia-Pero/adx0>

## Single purpose

Blocking ads.

## Why the permissions

| Permission | What for |
| --- | --- |
| `declarativeNetRequest` | to give the browser the rules that block requests to ad servers |
| `<all_urls>` | ads are on every site; the addresses of lists differ too |
| `scripting` | to hide ad blocks on a page with a style sheet |
| `storage`, `unlimitedStorage` | to keep the lists and the sites where ads are allowed |
| `alarms` | to download the lists once a day |
| `activeTab` | to show in the popup which site it is about |

No data is collected. Only lists are downloaded, as data; no code is loaded from anywhere: everything is in the package.

## For reviewers

The scripts are bundled from TypeScript by esbuild and are not minified. To build the same files with Node.js 24 on any system:

```sh
git clone https://github.com/Ilia-Pero/adx0
cd adx0/extension
npm ci
npm run build
```

The packages are the folders `extension/packed/firefox` and `extension/packed/chrome`. The source is in `extension/src`.

`player.js` runs in the page of YouTube: it removes the fields with the schedule of ads from the answers the player gets. `skip.js` is the spare way: if an ad has started, it rewinds it to the end.
