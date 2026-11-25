# Publishing the extension

Texts and files for extension stores. The packages come from a [release](https://github.com/Ilia-Pero/adx0/releases/latest). Firefox Add-ons receives them from the release workflow together with the texts of [firefox.json](firefox.json).

| Store | Browsers | File | Registration |
| --- | --- | --- | --- |
| [Firefox Add-ons](https://addons.mozilla.org/developers/) | Firefox | `adx0-firefox.zip` | a Mozilla account |
| [Microsoft Edge Add-ons](https://partner.microsoft.com/dashboard/microsoftedge) | Edge | `adx0-chrome.zip` | free, a Microsoft or GitHub account |
| [Opera Add-ons](https://addons.opera.com/developer/) | Opera, Yandex Browser | `adx0-chrome.zip` | an Opera account |
| [Chrome Web Store](https://chrome.google.com/webstore/devconsole) | Chrome, Edge, Opera, Yandex Browser | `adx0-chrome.zip` | a one-time fee |

Images: `logo.png` is 300 × 300, `screen.png` is 1280 × 800.

## Name

ad×0

## Summary

Blocks ads on pages and in YouTube videos.

## Description

Blocks about 64,000 addresses of ad servers and trackers, hides ad blocks on pages and removes the schedule of ads from the answers of YouTube, so the player does not show them.

The browser applies the rules itself and unloads the background part of the extension when it is idle. The lists are downloaded from their own sites once a day: Peter Lowe, AdAway, EasyList and a regional list chosen by the languages of the browser. The last good copy of each list is kept, so blocking works while the site of a list is down.

Ads can be allowed on any site with one button. Sites such as Google, Wikipedia and GitHub stay available even if a list names them by mistake. The extension collects no data and makes no paid exceptions for ads. 14 languages. The source code is open under the MIT license.

<https://github.com/Ilia-Pero/adx0>

## Описание

Блокирует около 64 тысяч адресов рекламных серверов и счётчиков, скрывает рекламные блоки на страницах и убирает расписание рекламы из ответов YouTube, поэтому проигрыватель её не показывает.

Правила исполняет сам браузер, он же выгружает фоновую часть расширения, когда она не нужна. Списки скачиваются с их сайтов раз в сутки: Peter Lowe, AdAway, EasyList и региональный список по языкам браузера. Последняя удачная копия каждого списка хранится, поэтому блокировка работает, даже когда сайт списка недоступен.

На любом сайте рекламу можно разрешить одной кнопкой. Яндекс, Google, Госуслуги и другие крупные сайты остаются доступны, даже если список назовёт их по ошибке. Расширение не собирает данных и не делает платных исключений для рекламы. 14 языков. Исходный код открыт, лицензия MIT.

<https://github.com/Ilia-Pero/adx0>

## Single purpose

Blocking ads.

## Permissions

| Permission | Purpose |
| --- | --- |
| `declarativeNetRequest` | to give the browser the rules that block requests to ad servers |
| `<all_urls>` | to hide ad blocks on any site and to download the lists |
| `scripting` | to hide ad blocks on a page with a style sheet |
| `storage`, `unlimitedStorage` | to keep the lists and the sites where ads are allowed |
| `alarms` | to download the lists once a day |
| `activeTab` | to show in the window of the extension which site it is about |

The extension collects no data. It downloads lists as data; all its code is in the package.

## For reviewers

The scripts are bundled from TypeScript by esbuild and are not minified. To build the same files with Node.js 24 on any system:

```sh
git clone https://github.com/Ilia-Pero/adx0
cd adx0/extension
npm ci
npm run build
```

The packages are the folders `extension/packed/firefox` and `extension/packed/chrome`. The source is in `extension/src`.

`player.js` runs in the page of YouTube and removes the fields with the schedule of ads from the answers the player gets. `skip.js` is a fallback: if an ad has started, it rewinds the ad to the end.
