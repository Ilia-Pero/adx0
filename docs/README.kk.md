<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">ad×0</h1>

<p align="center"><b>ad × 0 = 0</b></p>

<p align="center">Фонда жұмыс істемейтін жарнама бұғаттағыш: Windows, macOS, Linux және Firefox үшін</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/adx0/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/adx0?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md) · [Беларуская](README.be.md) · **Қазақша**

<p align="center">
<img src="window-off.png" width="380" alt="Бағдарлама терезесі: бұғаттау өшірулі">
<img src="window-on.png" width="380" alt="Бағдарлама терезесі: жарнама бұғатталған">
</p>

ad×0 жарнама серверлерінің мекенжайларын жүйенің hosts файлына жазып, аяқталады. Бұл мекенжайлардағы жарнама барлық бағдарламада жүктелмейді, ал фондық процесс жоқ: жады мен процессорды алатын ештеңе жоқ. Firefox кеңейтуі hosts файлының қолынан келмейтінін істейді: беттердегі жарнама блоктарын жасырады және YouTube бейнелерінен жарнаманы алып тастайды.

## Бағдарлама

- Жарнама серверлері мен санауыштардың 10 мыңға жуық мекенжайын бұғаттайды, орыс тізімімен 11,7 мыңға жуық. Тізімдерді өзіңіз таңдайсыз.
- hosts файлындағы жолдарыңызды сақтайды, алғашқы өзгерту алдында көшірмесін қалдырады және жазғанын қайта оқиды.
- Соңғы сәтті жүктелген тізімді сақтайды, сондықтан бұғаттау тізім сайтының жұмыс істеуіне тәуелді емес.
- Тізімдегі қате Google, Wikipedia немесе GitHub сияқты сайттарды жаба алмайды: міне, [толық тізім](../data/essential.json). Кез келген басқа сайт «Бұғаттамау» өрісінде ашылады.
- 14 тілде сөйлейді: тіл терезенің жоғарғы жағында таңдалады, Windows орнатқышында қазақ тілі жоқ. Ешқайда ештеңе жібермейді: тек тізімдерді жүктейді, сіз түймені басқанда, ал бұғаттау қосулы болса, жүйе іске қосылғанда да, тәулігіне бір реттен жиі емес. Соңғысы терезесіз бір сәтте өтеді, оны өшіруге болады.

## Firefox кеңейтуі

<p align="center"><img src="popup.png" width="312" alt="Firefox кеңейтуі"></p>

- Жарнама мекенжайларын бұғаттайды және беттердегі жарнама блоктарын жасырады. Аймақтық тізімдер браузер тілдері бойынша таңдалады.
- YouTube жауаптарынан жарнама кестесін алып тастайды, сонда ойнатқышта көрсететін ештеңе қалмайды.
- Тізімдерді тәулігіне бір рет жаңартады. Ережелерді браузердің өзі орындайды, фондық бөлігі жаңартулар арасында жадта болмайды.
- Сол 14 тілде сөйлейді: алдымен браузер тілінде, басқасы кеңейту терезесінде таңдалады.

## Не істей алмайды

hosts файлы мекенжайды тұтас бұғаттайды. Мазмұнмен бір мекенжайдан келетін жарнама, мысалы YouTube пен әлеуметтік желілерде, қалады; YouTube үшін кеңейту бар. Мекенжайларды өздері табатын прокси мен VPN hosts файлын айналып өтеді.

## Қаншалықты жеңіл

2026 жылғы 24 қарашадағы өлшемдер, Windows Server 2022, оңтайландырылған жинақ:

| Не өлшенді | Нәтиже |
| --- | --- |
| Терезе жабық кездегі процестер | 0 |
| Терезе ашық кездегі процессор | 5 секундта 0–31 мс |
| Іске қосқаннан терезеге дейін | төрт рет іске қосқанда 0,06–0,6 с |
| Бағдарлама көлемі | 4,4 МБ |
| hosts файлы қаншаға өседі | 9 800 мекенжайға 198 КБ, орыс тізімімен 11 700 мекенжайға 225 КБ |

## Тізімдер

Тізімдер бұл репозиторийге кірмейді. Бағдарламаның сіздегі көшірмесі оларды өз сайттарынан жүктейді, әрқайсысының өз лицензиясы бар.

| Тізім | Қайда | Лицензия |
| --- | --- | --- |
| [Peter Lowe](https://pgl.yoyo.org/adservers/) | екеуі | [McRae GPL](https://pgl.yoyo.org/license/) |
| [AdAway](https://adaway.org/) | екеуі | CC BY 3.0 |
| [RU AdList](https://github.com/easylist/ruadlist) | екеуі | CC BY 3.0 |
| [Liste FR](https://github.com/easylist/listefr) | екеуі | CC BY-SA 3.0 |
| [EasyList China](https://github.com/easylist/easylistchina) | екеуі | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [StevenBlack](https://github.com/StevenBlack/hosts) | бағдарлама | MIT |
| [EasyList](https://easylist.to/) | кеңейту | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Germany](https://github.com/easylist/easylistgermany) | кеңейту | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Hebrew](https://github.com/easylist/EasyListHebrew) | кеңейту | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Italy](https://github.com/easylist/easylistitaly) | кеңейту | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Spanish](https://github.com/easylist/easylistspanish) | кеңейту | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [KoreanList](https://github.com/easylist/KoreanList) | кеңейту | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [AdGuard Japanese](https://github.com/AdguardTeam/AdguardFilters) | кеңейту | GPL 3.0 |
| [Greek AdBlock Filter](https://github.com/kargig/greek-adblockplus-filter) | кеңейту | CC BY-SA 4.0 |

## Күйі

| Жүйе | Не белгілі |
| --- | --- |
| Windows | Windows Server 2022 жүйесінде жұмыс істейді: hosts файлының көшірмесінде нақты тізімдермен тексерілді. Windows 10 және 11 жүйелерінде әлі тексерілген жоқ. |
| macOS 10.13+ | Жиналады, тесттерден өтеді; GitHub машинасында орнатылады және іске қосылады. Қолмен әлі ешкім тексерген жоқ. |
| Ubuntu 24.04+ | Жиналады, тесттерден өтеді; GitHub машинасында орнатылады және іске қосылады. Қолмен әлі ешкім тексерген жоқ. |
| Firefox 128+ | Firefox 160 нұсқасында жұмыс істейді. Mozilla оған әлі қол қойған жоқ, сондықтан Firefox қайта іске қосылғанда оны қайта жүктеу керек. |

## Орнату

| Жүйе | Файл | Қалай орнату керек |
| --- | --- | --- |
| Windows 10 және 11 | [adx0-windows.exe](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-windows.exe) | Іске қосыңыз. Windows бағдарламаны танымаса: More info, Run anyway. |
| macOS 10.13+ | [adx0-macos.dmg](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-macos.dmg) | Бағдарламаны Applications қалтасына сүйреп, ашыңыз. macOS бас тартса, System Settings, Privacy & Security бөлімін ашып, Open Anyway түймесін басыңыз. |
| Ubuntu 24.04+ | [adx0-ubuntu.deb](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-ubuntu.deb) | `sudo apt install ./adx0-ubuntu.deb` |
| Firefox 128+ | [adx0-firefox.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-firefox.zip) | `about:debugging` бетін ашып, This Firefox, Load Temporary Add-on тармағын таңдап, файлды көрсетіңіз. Firefox қайта іске қосылғанда оны қайта жүктеңіз. |

GitHub бұл файлдарды осы репозиторийдің кодынан жинайды, бағдарламаларды орнатып, іске қосады, содан кейін ғана бәрін жариялайды. Файлдарда ақылы қолтаңба жоқ, сондықтан алғаш іске қосқанда Windows пен macOS ескертеді.

Бағдарламаны өзіңіз де жинай аласыз. Node.js 24 немесе жаңарақ нұсқасы және Rust керек, Windows жүйесінде тағы Microsoft C++ Build Tools, Ubuntu жүйесінде тағы `libwebkit2gtk-4.1-dev librsvg2-dev`.

```sh
git clone https://github.com/Ilia-Pero/adx0
cd adx0
npm ci
npm run tauri build
```

Орнатқыш `src-tauri/target/release/bundle` қалтасында пайда болады.

Жойылғанда бағдарлама бұғатталған мекенжайларды қайта ашады. macOS жүйесінде жоймас бұрын бұғаттауды өшіріңіз.

Кеңейту: `extension` қалтасында `npm ci && npm run build` орындаңыз, Firefox браузерінде `about:debugging` бетін ашып, This Firefox, Load Temporary Add-on тармағын және `extension/web/manifest.json` файлын таңдаңыз.

## Аудармалар

Негізгі мәтіндер орыс және ағылшын тілдерінде. Қалған аудармалар машиналық, оларды тіл иелері тексерген жоқ. Қате көрсеңіз, `locales`, `extension/web/_locales` немесе `docs` ішінде түзетіңіз.

## Қатысу және лицензия

Бағдарламаны жинау, тексеру және өзгерту: [CONTRIBUTING.md](../CONTRIBUTING.md). Лицензиясы [MIT](../LICENSE).
