<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">ad×0</h1>

<p align="center"><b>ad × 0 = 0</b></p>

<p align="center">Αποκλεισμός διαφημίσεων που δεν εκτελείται στο παρασκήνιο: για Windows, macOS, Linux και περιηγητές</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/adx0/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/adx0?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · **Ελληνικά** · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md) · [Беларуская](README.be.md) · [Қазақша](README.kk.md)

<p align="center">
<img src="window-off.png" width="380" alt="Το παράθυρο του προγράμματος: ο αποκλεισμός είναι ανενεργός">
<img src="window-on.png" width="380" alt="Το παράθυρο του προγράμματος: οι διαφημίσεις είναι αποκλεισμένες">
</p>

Το ad×0 γράφει διευθύνσεις διακομιστών διαφημίσεων στο αρχείο hosts του συστήματος και κλείνει. Οι διαφημίσεις από αυτές τις διευθύνσεις σταματούν να φορτώνονται σε όλα τα προγράμματα και δεν υπάρχει διεργασία στο παρασκήνιο που να πιάνει μνήμη ή επεξεργαστή. Μια επέκταση για τον περιηγητή κάνει ό,τι δεν μπορεί το αρχείο hosts: κρύβει τα διαφημιστικά μπλοκ στις σελίδες και αφαιρεί τις διαφημίσεις από τα βίντεο του YouTube.

## Το πρόγραμμα

- Αποκλείει περίπου 10.000 διευθύνσεις διακομιστών διαφημίσεων και ιχνηλατών από τις λίστες που επιλέγετε.
- Διατηρεί τις δικές σας γραμμές στο αρχείο hosts, αφήνει αντίγραφό του πριν από την πρώτη αλλαγή και ξαναδιαβάζει ό,τι έγραψε.
- Κρατά το τελευταίο καλό αντίγραφο κάθε λίστας, ώστε ο αποκλεισμός να μην εξαρτάται από τον ιστότοπο της λίστας.
- Δεν αφήνει ένα λάθος σε λίστα να κλείσει ιστότοπους όπως η Google, η Wikipedia ή το GitHub: εδώ είναι [όλος ο κατάλογος](../data/essential.json). Κάθε άλλος ιστότοπος ανοίγει από το πεδίο «Να μην αποκλείονται».
- Μιλά 14 γλώσσες: η γλώσσα επιλέγεται στο επάνω μέρος του παραθύρου και στο πρόγραμμα εγκατάστασης για Windows. Δεν στέλνει τίποτα πουθενά: μόνο κατεβάζει τις λίστες, όταν πατάτε το κουμπί και, όσο ο αποκλεισμός είναι ενεργός, κατά την εκκίνηση του συστήματος, το πολύ μία φορά την ημέρα. Το δεύτερο γίνεται χωρίς παράθυρο σε μια στιγμή και μπορείτε να το κλείσετε. Ελέγχει για νέα έκδοση μόνο όταν πατάτε το κουμπί.

## Η επέκταση για τον περιηγητή

Για τον Firefox και για περιηγητές που βασίζονται στο Chromium: Chrome, Yandex Browser, Edge, Opera.

<p align="center"><img src="popup.png" width="312" alt="Η επέκταση για τον περιηγητή"></p>

- Αποκλείει διαφημιστικές διευθύνσεις και κρύβει διαφημιστικά μπλοκ στις σελίδες. Οι τοπικές λίστες επιλέγονται από τις γλώσσες του προγράμματος περιήγησης.
- Αφαιρεί το πρόγραμμα διαφημίσεων από τις απαντήσεις του YouTube, ώστε ο αναπαραγωγός να μην έχει τι να δείξει.
- Ενημερώνει τις λίστες μία φορά την ημέρα. Τους κανόνες τους εφαρμόζει το ίδιο το πρόγραμμα περιήγησης, και το μέρος παρασκηνίου δεν είναι φορτωμένο ενδιάμεσα.
- Μιλά τις ίδιες 14 γλώσσες: στην αρχή τη γλώσσα του περιηγητή· άλλη επιλέγεται στο παράθυρο της επέκτασης.

## Τι δεν μπορεί να κάνει

Το αρχείο hosts δουλεύει με ολόκληρες διευθύνσεις. Οι διαφημίσεις που έρχονται από την ίδια διεύθυνση με το περιεχόμενο, όπως στο YouTube ή στα κοινωνικά δίκτυα, μένουν· για το YouTube υπάρχει η επέκταση. Ένας διακομιστής μεσολάβησης ή ένα VPN που βρίσκει μόνο του τις διευθύνσεις παρακάμπτει το αρχείο hosts.

## Πόσο ελαφρύ είναι

Μετρήθηκε στις 24.11.2026 σε Windows Server 2022, βελτιστοποιημένη έκδοση:

| Τι | Αποτέλεσμα |
| --- | --- |
| Διεργασίες με κλειστό παράθυρο | 0 |
| Επεξεργαστής με ανοιχτό παράθυρο | 0–31 ms σε 5 s |
| Από την εκκίνηση ως το παράθυρο | 0,06–0,6 s σε τέσσερις εκτελέσεις |
| Μέγεθος προγράμματος | 4,4 MB |
| Αύξηση του αρχείου hosts | 198 KB για 9.800 διευθύνσεις |

## Λίστες

Οι λίστες δεν είναι μέρος αυτού του αποθετηρίου. Το δικό σας αντίγραφο του προγράμματος τις κατεβάζει από τους ιστότοπούς τους και καθεμία έχει τη δική της άδεια.

| Λίστα | Πού | Άδεια |
| --- | --- | --- |
| [Peter Lowe](https://pgl.yoyo.org/adservers/) | και στα δύο | [McRae GPL](https://pgl.yoyo.org/license/) |
| [AdAway](https://adaway.org/) | και στα δύο | CC BY 3.0 |
| [RU AdList](https://github.com/easylist/ruadlist) | και στα δύο | CC BY 3.0 |
| [Liste FR](https://github.com/easylist/listefr) | και στα δύο | CC BY-SA 3.0 |
| [EasyList China](https://github.com/easylist/easylistchina) | και στα δύο | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [StevenBlack](https://github.com/StevenBlack/hosts) | πρόγραμμα | MIT |
| [EasyList](https://easylist.to/) | επέκταση | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Germany](https://github.com/easylist/easylistgermany) | επέκταση | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Hebrew](https://github.com/easylist/EasyListHebrew) | επέκταση | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Italy](https://github.com/easylist/easylistitaly) | επέκταση | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Spanish](https://github.com/easylist/easylistspanish) | επέκταση | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [KoreanList](https://github.com/easylist/KoreanList) | επέκταση | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [AdGuard Japanese](https://github.com/AdguardTeam/AdguardFilters) | επέκταση | GPL 3.0 |
| [Greek AdBlock Filter](https://github.com/kargig/greek-adblockplus-filter) | επέκταση | CC BY-SA 4.0 |

## Κατάσταση

| Σύστημα | Κατάσταση |
| --- | --- |
| Windows | Λειτουργεί σε Windows Server 2022: δοκιμάστηκε με πραγματικές λίστες σε αντίγραφο του αρχείου hosts. Δεν έχει δοκιμαστεί ακόμη σε Windows 10 και 11. |
| macOS 10.13+ | Μεταγλωττίζεται και περνά τις δοκιμές· σε μηχάνημα του GitHub εγκαθίσταται και εκκινεί. Κανείς δεν το έχει δοκιμάσει με το χέρι ακόμη. |
| Ubuntu 24.04+ | Μεταγλωττίζεται και περνά τις δοκιμές· σε μηχάνημα του GitHub εγκαθίσταται και εκκινεί. Κανείς δεν το έχει δοκιμάσει με το χέρι ακόμη. |
| Firefox 128+ | Λειτουργεί στον Firefox 161. Η Mozilla δεν την έχει υπογράψει ακόμη, οπότε πρέπει να φορτωθεί ξανά μετά την επανεκκίνηση του Firefox. |
| Chrome, Yandex Browser, Edge, Opera | Λειτουργεί στον Edge 158, δοκιμάστηκε χωρίς παράθυρο. Οι άλλοι δεν έχουν δοκιμαστεί· χρειάζεται Chromium 120 ή νεότερο. |

## Εγκατάσταση

| Σύστημα | Αρχείο | Πώς εγκαθίσταται |
| --- | --- | --- |
| Windows 10 και 11 | [adx0-windows.exe](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-windows.exe) | Εκτελέστε το. Αν τα Windows δεν γνωρίζουν το πρόγραμμα: «Περισσότερες πληροφορίες», «Εκτέλεση ούτως ή άλλως». |
| macOS 10.13+ | [adx0-macos.dmg](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-macos.dmg) | Σύρετε το πρόγραμμα στις Εφαρμογές και ανοίξτε το. Όταν το macOS αρνηθεί, πηγαίνετε στις Ρυθμίσεις συστήματος, «Απόρρητο και ασφάλεια», και πατήστε «Άνοιγμα οπωσδήποτε». |
| Ubuntu 24.04+ | [adx0-ubuntu.deb](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-ubuntu.deb) | `sudo apt install ./adx0-ubuntu.deb` |
| Firefox 128+ | [adx0-firefox.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-firefox.zip) | Ανοίξτε το `about:debugging`, «Αυτός ο Firefox», «Φόρτωση προσωρινού προσθέτου», και επιλέξτε το αρχείο. Μετά την επανεκκίνηση του Firefox φορτώστε το ξανά. |
| Chrome, Yandex Browser, Edge, Opera | [adx0-chrome.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-chrome.zip) | Αποσυμπιέστε το, ανοίξτε το `chrome://extensions`, ενεργοποιήστε το «Developer mode», πατήστε «Load unpacked» και επιλέξτε τον φάκελο· μην τον διαγράψετε. Ο Yandex Browser εμπιστεύεται μόνο καταλόγους και μπορεί να απενεργοποιήσει την επέκταση. |

Το GitHub μεταγλωττίζει αυτά τα αρχεία από τον κώδικα αυτού του αποθετηρίου, εγκαθιστά και εκκινεί τα προγράμματα και μόνο τότε τα δημοσιεύει όλα. Τα αρχεία δεν έχουν υπογραφή επί πληρωμή, γι’ αυτό τα Windows και το macOS προειδοποιούν στην πρώτη εκκίνηση.

Μπορείτε και να μεταγλωττίσετε το πρόγραμμα μόνοι σας. Χρειάζονται Node.js 24 ή νεότερο και Rust, στα Windows επίσης Microsoft C++ Build Tools, στο Ubuntu επίσης `libwebkit2gtk-4.1-dev librsvg2-dev`.

```sh
git clone https://github.com/Ilia-Pero/adx0
cd adx0
npm ci
npm run tauri build
```

Το πρόγραμμα εγκατάστασης εμφανίζεται στο `src-tauri/target/release/bundle`.

Η απεγκατάσταση ανοίγει ξανά τις αποκλεισμένες διευθύνσεις. Στο macOS απενεργοποιήστε τον αποκλεισμό πριν διαγράψετε το πρόγραμμα.

Η επέκταση: εκτελέστε `npm ci && npm run build` στον φάκελο `extension`, ανοίξτε το `about:debugging` στον Firefox, επιλέξτε «Αυτός ο Firefox», «Φόρτωση προσωρινού προσθέτου» και το αρχείο `extension/packed/firefox/manifest.json`. Για το Chromium, ο φάκελος είναι `extension/packed/chrome`.

## Μεταφράσεις

Οι μεταφράσεις είναι μηχανικές και δεν έχουν ελεγχθεί από φυσικούς ομιλητές. Αν δείτε λάθος, διορθώστε το στο `locales`, στο `extension/web/_locales` ή στο `docs`.

## Συμμετοχή και άδεια

Πώς να μεταγλωττίσετε, να δοκιμάσετε και να αλλάξετε το πρόγραμμα: [CONTRIBUTING.md](../CONTRIBUTING.md). Η άδεια είναι [MIT](../LICENSE).
