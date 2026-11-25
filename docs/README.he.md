<div dir="rtl">

<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">ad×0</h1>

<p align="center"><b>ad × 0 = 0</b></p>

<p align="center">חוסם פרסומות שאינו רץ ברקע: עבור Windows‏, macOS‏, Linux ו־Firefox</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/adx0/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/adx0?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-12-0f766e" alt="12 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · **עברית** · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md)

<p align="center">
<img src="window-off.png" width="380" alt="חלון התוכנה: החסימה כבויה">
<img src="window-on.png" width="380" alt="חלון התוכנה: הפרסומות חסומות">
</p>

‏ad×0 כותבת את הכתובות של שרתי פרסום אל קובץ ה־hosts של המערכת ונסגרת. פרסומות מהכתובות האלה מפסיקות להיטען בכל התוכנות, ואין תהליך רקע שתופס זיכרון או מעבד. הרחבה ל־Firefox עושה את מה שקובץ ה־hosts אינו יכול: מסתירה בלוקים של פרסום בדפים ומסירה פרסומות מסרטוני YouTube.

## התוכנה

- חוסמת כ־10,000 כתובות של שרתי פרסום ומעקב מתוך הרשימות שתבחרו.
- שומרת על השורות שלכם בקובץ ה־hosts, משאירה עותק שלו לפני השינוי הראשון וקוראת שוב את מה שכתבה.
- שומרת את העותק התקין האחרון של כל רשימה, כך שהחסימה אינה תלויה בזמינות האתר של הרשימה.
- אינה מניחה לטעות ברשימה לסגור אתרים כמו Google‏, Wikipedia או GitHub: הנה [הרשימה המלאה](../data/essential.json). כל אתר אחר פותחים בשדה "לא לחסום".
- מדברת 12 שפות: את השפה בוחרים בראש החלון ובתוכנית ההתקנה ל־Windows. אינה שולחת דבר לשום מקום: רק מורידה את הרשימות, כשאתם לוחצים על הכפתור וכל עוד החסימה פועלת גם עם הפעלת המערכת, לכל היותר פעם ביום. זה קורה ללא חלון בתוך רגע, ואפשר לכבות את זה.

## ההרחבה ל־Firefox

<p align="center"><img src="popup.png" width="312" alt="ההרחבה ל־Firefox"></p>

- חוסמת כתובות פרסום ומסתירה בלוקים של פרסום בדפים. רשימות אזוריות נבחרות לפי שפות הדפדפן.
- מסירה את לוח הפרסומות מהתשובות של YouTube, כך שלנגן אין מה להציג.
- מעדכנת את הרשימות פעם ביום. הדפדפן עצמו אוכף את הכללים, וחלק הרקע אינו טעון בין לבין.
- מדברת את אותן 12 שפות: בהתחלה בשפת הדפדפן; שפה אחרת בוחרים בחלון ההרחבה.

## מה היא אינה יכולה

קובץ ה־hosts עובד עם כתובות שלמות. פרסומות שמגיעות מאותה כתובת כמו התוכן, כמו ב־YouTube או ברשתות חברתיות, נשארות; עבור YouTube יש הרחבה. פרוקסי או VPN שמאתרים כתובות בעצמם עוקפים את קובץ ה־hosts.

## כמה היא קלה

נמדד ב־24.11.2026 ב־Windows Server 2022, גרסה ממוטבת:

| מה | תוצאה |
| --- | --- |
| תהליכים כשהחלון סגור | 0 |
| מעבד כשהחלון פתוח | 0–31 אלפיות השנייה ב־5 שניות |
| מההפעלה ועד החלון | 0.06–0.6 שניות בארבע הפעלות |
| גודל התוכנה | 4.4 MB |
| גידול קובץ ה־hosts | 198 KB עבור 9,800 כתובות |

## רשימות

הרשימות אינן חלק ממאגר זה. העותק שלכם של התוכנה מוריד אותן מהאתרים שלהן, ולכל אחת רישיון משלה.

| רשימה | היכן | רישיון |
| --- | --- | --- |
| [Peter Lowe](https://pgl.yoyo.org/adservers/) | שתיהן | [McRae GPL](https://pgl.yoyo.org/license/) |
| [AdAway](https://adaway.org/) | שתיהן | CC BY 3.0 |
| [RU AdList](https://github.com/easylist/ruadlist) | שתיהן | CC BY 3.0 |
| [Liste FR](https://github.com/easylist/listefr) | שתיהן | CC BY-SA 3.0 |
| [EasyList China](https://github.com/easylist/easylistchina) | שתיהן | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [StevenBlack](https://github.com/StevenBlack/hosts) | תוכנה | MIT |
| [EasyList](https://easylist.to/) | הרחבה | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Germany](https://github.com/easylist/easylistgermany) | הרחבה | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Hebrew](https://github.com/easylist/EasyListHebrew) | הרחבה | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Italy](https://github.com/easylist/easylistitaly) | הרחבה | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Spanish](https://github.com/easylist/easylistspanish) | הרחבה | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [KoreanList](https://github.com/easylist/KoreanList) | הרחבה | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [AdGuard Japanese](https://github.com/AdguardTeam/AdguardFilters) | הרחבה | GPL 3.0 |
| [Greek AdBlock Filter](https://github.com/kargig/greek-adblockplus-filter) | הרחבה | CC BY-SA 4.0 |

## מצב

| מערכת | מצב |
| --- | --- |
| Windows | עובדת ב־Windows Server 2022: נבדקה עם רשימות אמיתיות על עותק של קובץ ה־hosts. עדיין לא נבדקה ב־Windows 10 ו־11. |
| macOS 10.13+ | נבנית ועוברת את הבדיקות; במחשב של GitHub היא מותקנת ומופעלת. איש עדיין לא ניסה אותה ביד. |
| Ubuntu 24.04+ | נבנית ועוברת את הבדיקות; במחשב של GitHub היא מותקנת ומופעלת. איש עדיין לא ניסה אותה ביד. |
| Firefox 128+ | עובדת ב־Firefox 160. Mozilla עדיין לא חתמה עליה, ולכן צריך לטעון אותה שוב אחרי הפעלה מחדש של Firefox. |

## התקנה

| מערכת | קובץ | איך מתקינים |
| --- | --- | --- |
| Windows 10 ו־11 | [adx0-windows.exe](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-windows.exe) | הפעילו אותו. אם Windows אינה מכירה את התוכנה: "מידע נוסף", "הפעל בכל זאת". |
| macOS 10.13+ | [adx0-macos.dmg](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-macos.dmg) | גררו את התוכנה ליישומים ופתחו אותה. כש־macOS מסרבת, פתחו את הגדרות המערכת, "פרטיות ואבטחה", ולחצו על "פתח בכל זאת". |
| Ubuntu 24.04+ | [adx0-ubuntu.deb](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-ubuntu.deb) | `sudo apt install ./adx0-ubuntu.deb` |
| Firefox 128+ | [adx0-firefox.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-firefox.zip) | פתחו את `about:debugging`, "This Firefox", "Load Temporary Add-on", ובחרו את הקובץ. אחרי הפעלה מחדש של Firefox טענו אותו שוב. |

GitHub בונה את הקבצים האלה מהקוד של מאגר זה, מתקין ומפעיל את התוכנות ורק אז מפרסם הכול. לקבצים אין חתימה בתשלום, ולכן Windows ו־macOS מזהירות בהפעלה הראשונה.

אפשר גם לבנות את התוכנה בעצמכם. דרושים Node.js 24 ומעלה ו־Rust, ב־Windows גם Microsoft C++ Build Tools, ב־Ubuntu גם `libwebkit2gtk-4.1-dev librsvg2-dev`.

</div>

```sh
git clone https://github.com/Ilia-Pero/adx0
cd adx0
npm ci
npm run tauri build
```

<div dir="rtl">

קובץ ההתקנה מופיע בתיקייה `src-tauri/target/release/bundle`.

הסרת ההתקנה פותחת מחדש את הכתובות החסומות. ב־macOS כבו את החסימה לפני מחיקת התוכנה.

ההרחבה: הריצו `npm ci && npm run build` בתיקייה `extension`, פתחו ב־Firefox את `about:debugging`, בחרו "This Firefox", "Load Temporary Add-on" ואת הקובץ `extension/web/manifest.json`.

## תרגומים

התרגומים נעשו במכונה ולא נבדקו על ידי דוברי השפה. אם ראיתם טעות, תקנו אותה בתיקייה `locales`, `extension/web/_locales` או `docs`.

## השתתפות ורישיון

איך לבנות, לבדוק ולשנות את התוכנה: [CONTRIBUTING.md](../CONTRIBUTING.md). הרישיון הוא [MIT](../LICENSE).

</div>
