<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">ad×0</h1>

<p align="center"><b>ad × 0 = 0</b></p>

<p align="center">Windows, macOS, Linux және браузерлерге арналған жарнама бұғаттағышы</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/adx0/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/adx0?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md) · [Беларуская](README.be.md) · **Қазақша**

<p align="center">
<img src="window-off.png" width="380" alt="Бағдарлама терезесі: бұғаттау өшірулі">
<img src="window-on.png" width="380" alt="Бағдарлама терезесі: жарнама бұғатталды">
</p>

ad×0 жарнама серверлерінің мекенжайларын жүйенің hosts файлына жазып, аяқталады. Содан кейін компьютердегі бағдарламалар бұл мекенжайлардан ештеңе жүктемейді, ал ad×0-дің өзінде жұмыс істеп тұрған процесс болмайды. Браузер кеңейтімі беттердегі жарнама блоктарын жасырып, YouTube бейнелерінен жарнаманы алып тастайды.

## Орнату

| Жүйе | Файл | Қалай орнатылады |
| --- | --- | --- |
| Windows 10, 11 | [adx0-windows.exe](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-windows.exe) | Файлды іске қосыңыз. Windows ескерту көрсетсе, «Толығырақ», содан кейін «Бәрібір іске қосу» дегенді таңдаңыз. |
| macOS 10.13+ | [adx0-macos.dmg](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-macos.dmg) | Бағдарламаны «Бағдарламалар» қалтасына сүйреп апарып, ашыңыз. macOS бірінші рет бас тартқан соң жүйе баптауларын, «Құпиялылық және қауіпсіздік» бөлімін ашып, «Бәрібір ашу» түймесін басыңыз. |
| Ubuntu 24.04+ | [adx0-ubuntu.deb](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-ubuntu.deb) | `sudo apt install ./adx0-ubuntu.deb` |
| Firefox 128+ | [adx0-firefox.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-firefox.zip) | `about:debugging` бетін, «Осы Firefox» бөлімін ашып, уақытша қосымшаны жүктеу түймесін басыңыз да, файлды таңдаңыз. Firefox қайта іске қосылған соң оны қайта жүктеңіз. |
| Chrome, Edge, Opera, Yandex Browser | [adx0-chrome.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-chrome.zip) | Мұрағатты өшірмейтін қалтаға шығарыңыз. `chrome://extensions` бетін ашып, әзірлеуші режимін қосыңыз, қапталмаған кеңейтімді жүктеу түймесін басып, қалтаны таңдаңыз. |

Файлдарда ақылы қолтаңба болмағандықтан, Windows пен macOS ескерту көрсетеді. GitHub файлдарды осы репозиторийден құрастырады, бағдарламаларды таза машиналарға орнатып, іске қосады, содан кейін ғана шығарылымды жариялайды.

Бағдарлама жойылғанда бұғатталған мекенжайлар hosts файлынан алынады. macOS-та жоймас бұрын бұғаттауды өшіріңіз.

## Бағдарлама

- Жарнама мен бақылау серверлерінің шамамен 10 мың мекенжайын бұғаттайды. Тізімдерді өзіңіз таңдайсыз, өз сайттарыңызды қосуға болады.
- hosts файлындағы жолдарыңызға тимейді, алғашқы өзгеріске дейін оның көшірмесін жасайды, жазғаннан кейін файлды тексереді.
- Тізімдерді тәулігіне бір реттен жиі емес, жүйе қосылғаннан кейін көп ұзамай, терезесіз жаңартады. Мұны өшіруге болады.
- 14 тіл.

hosts файлы мекенжайды тұтас бұғаттайды, сондықтан мазмұнмен бір мекенжайдан келетін жарнама қалады. Мекенжайды өздері іздейтін прокси немесе VPN арқылы жұмыс істейтін бағдарламалар hosts файлын айналып өтеді.

## Браузер кеңейтімі

<p align="center"><img src="popup.png" width="312" alt="Кеңейтім терезесі"></p>

- Шамамен 64 мың мекенжайды бұғаттап, беттердегі жарнама блоктарын жасырады.
- YouTube жауаптарынан жарнама кестесін алып тастайды.
- Сайттағы жарнамаға бір түймемен рұқсат береді.
- Тізімдерді тәулігіне бір рет жаңартады.

## Деректер және желі

Бағдарлама мен кеңейтім тізімдерді олардың сайттарынан жүктейді және дерек жинамайды. «Жаңартуларды тексеру» түймесін басқанда бағдарлама жаңа нұсқа туралы GitHub-тан сұрайды.

## Толығырақ

Тізімдер мен олардың лицензиялары, өлшемдер, тексерілген жүйелер және құрастыру [ағылшын тілінде](../README.md) сипатталған.

Бұл мәтінді машина аударды, тіл иелері тексерген жоқ. Түзетулер `docs` қалтасына енгізіледі.

[MIT](../LICENSE)
