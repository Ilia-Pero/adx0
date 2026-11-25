<div dir="rtl">

<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">ad×0</h1>

<p align="center"><b>ad × 0 = 0</b></p>

<p align="center">חוסם פרסומות עבור Windows‏, macOS‏, Linux ודפדפנים</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/adx0/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/adx0?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · **עברית** · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md) · [Беларуская](README.be.md) · [Қазақша](README.kk.md)

<p align="center">
<img src="window-off.png" width="380" alt="חלון התוכנה: החסימה כבויה">
<img src="window-on.png" width="380" alt="חלון התוכנה: הפרסומות חסומות">
</p>

התוכנה ad×0 כותבת את כתובות שרתי הפרסום אל קובץ hosts של המערכת ונסגרת. לאחר מכן תוכנות במחשב אינן טוענות דבר מהכתובות האלה, ול־ad×0 עצמה אין תהליך פעיל. תוסף לדפדפן מסתיר פרסומות בדפים ומסיר פרסומות מסרטוני YouTube.

## התקנה

| מערכת | קובץ | איך מתקינים |
| --- | --- | --- |
| Windows 10, 11 | [adx0-windows.exe](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-windows.exe) | הפעילו את הקובץ. אם Windows מציגה אזהרה, בחרו „מידע נוסף” ואז „הפעל בכל זאת”. |
| macOS 10.13+ | [adx0-macos.dmg](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-macos.dmg) | גררו את התוכנה לתיקיית „יישומים” ופתחו אותה. אחרי הסירוב הראשון של macOS פתחו את הגדרות המערכת, „פרטיות ואבטחה”, ולחצו על „פתח בכל זאת”. |
| Ubuntu 24.04+ | [adx0-ubuntu.deb](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-ubuntu.deb) | `sudo apt install ./adx0-ubuntu.deb` |
| Firefox 128+ | [adx0-firefox.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-firefox.zip) | פתחו את `about:debugging`, „Firefox זה”, „טעינת תוספת זמנית” ובחרו את הקובץ. אחרי הפעלה מחדש של Firefox טענו אותו שוב. |
| Chrome, Edge, Opera, Yandex Browser | [adx0-chrome.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-chrome.zip) | חלצו את הארכיון לתיקייה שתישאר במקומה. פתחו את `chrome://extensions`, הפעילו את מצב המפתחים, לחצו על „טעינת פריט unpacked” ובחרו את התיקייה. |

Windows ו־macOS מציגות אזהרה משום שלקבצים אין חתימה בתשלום. GitHub בונה אותם מהמאגר הזה, מתקין את התוכנות במחשבים נקיים ומפעיל אותן לפני פרסום גרסה.

בהסרת ההתקנה הכתובות החסומות נמחקות מקובץ hosts. ב־macOS כבו את החסימה לפני המחיקה.

## התוכנה

- חוסמת כ־10,000 כתובות של שרתי פרסום ומעקב. אתם בוחרים את הרשימות ויכולים להוסיף אתרים משלכם.
- אינה נוגעת בשורות שלכם בקובץ hosts, שומרת עותק שלו לפני השינוי הראשון ובודקת את הקובץ אחרי הכתיבה.
- מעדכנת את הרשימות לכל היותר פעם ביום, זמן קצר אחרי הפעלת המערכת וללא חלון. אפשר לכבות זאת.
- 14 שפות.

קובץ hosts חוסם כתובות שלמות, ולכן פרסומות שמגיעות מאותה כתובת כמו התוכן נשארות. תוכנות שפועלות דרך פרוקסי או VPN עם חיפוש כתובות משלהן עוקפות את הקובץ.

## תוסף לדפדפן

<p align="center"><img src="popup.png" width="312" alt="חלון התוסף"></p>

- חוסם כ־64,000 כתובות ומסתיר פרסומות בדפים.
- מסיר את לוח הפרסומות מהתשובות של YouTube.
- מתיר פרסומות באתר בלחיצת כפתור.
- מעדכן את הרשימות פעם ביום.

## נתונים ורשת

התוכנה והתוסף מורידים את הרשימות מהאתרים שלהן ואינם אוספים נתונים. התוכנה שואלת את GitHub על גרסה חדשה כשאתם לוחצים על „בדיקת עדכונים”.

## עוד

הרשימות והרישיונות שלהן, המדידות, המערכות שנבדקו ובניית התוכנה מתוארים [באנגלית](../README.md).

הטקסט תורגם במכונה ולא נבדק בידי דוברי השפה. תיקונים נעשים בתיקייה `docs`.

[MIT](../LICENSE)

</div>
