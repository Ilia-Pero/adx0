<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">ad×0</h1>

<p align="center"><b>ad × 0 = 0</b></p>

<p align="center">Αποκλεισμός διαφημίσεων που δεν εκτελείται στο παρασκήνιο: για Windows, macOS, Linux και Firefox</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<img src="https://img.shields.io/badge/languages-12-0f766e" alt="12 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · **Ελληνικά** · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md)

<p align="center">
<img src="window-off.png" width="380" alt="Το παράθυρο του προγράμματος: ο αποκλεισμός είναι ανενεργός">
<img src="window-on.png" width="380" alt="Το παράθυρο του προγράμματος: οι διαφημίσεις είναι αποκλεισμένες">
</p>

Το ad×0 γράφει διευθύνσεις διακομιστών διαφημίσεων στο αρχείο hosts του συστήματος και κλείνει. Οι διαφημίσεις από αυτές τις διευθύνσεις σταματούν να φορτώνονται σε όλα τα προγράμματα και δεν υπάρχει διεργασία στο παρασκήνιο που να πιάνει μνήμη ή επεξεργαστή. Μια επέκταση για τον Firefox κάνει ό,τι δεν μπορεί το αρχείο hosts: κρύβει τα διαφημιστικά μπλοκ στις σελίδες και αφαιρεί τις διαφημίσεις από τα βίντεο του YouTube.

## Το πρόγραμμα

- Αποκλείει περίπου 10.000 διευθύνσεις διακομιστών διαφημίσεων και ιχνηλατών από τις λίστες που επιλέγετε.
- Διατηρεί τις δικές σας γραμμές στο αρχείο hosts, αφήνει αντίγραφό του πριν από την πρώτη αλλαγή και ξαναδιαβάζει ό,τι έγραψε.
- Κρατά το τελευταίο καλό αντίγραφο κάθε λίστας, ώστε ο αποκλεισμός να μην εξαρτάται από τον ιστότοπο της λίστας.
- Δεν αφήνει ένα λάθος σε λίστα να κλείσει ιστότοπους όπως η Google, η Wikipedia ή το GitHub: εδώ είναι [όλος ο κατάλογος](../data/essential.json). Κάθε άλλος ιστότοπος ανοίγει από το πεδίο «Να μην αποκλείονται».
- Μιλά 12 γλώσσες. Δεν στέλνει τίποτα πουθενά: μόνο κατεβάζει τις λίστες όταν πατάτε το κουμπί.

## Η επέκταση για τον Firefox

<p align="center"><img src="popup.png" width="312" alt="Η επέκταση για τον Firefox"></p>

- Αποκλείει διαφημιστικές διευθύνσεις και κρύβει διαφημιστικά μπλοκ στις σελίδες. Οι τοπικές λίστες επιλέγονται από τις γλώσσες του προγράμματος περιήγησης.
- Αφαιρεί το πρόγραμμα διαφημίσεων από τις απαντήσεις του YouTube, ώστε ο αναπαραγωγός να μην έχει τι να δείξει.
- Ενημερώνει τις λίστες μία φορά την ημέρα. Τους κανόνες τους εφαρμόζει το ίδιο το πρόγραμμα περιήγησης, και το μέρος παρασκηνίου δεν είναι φορτωμένο ενδιάμεσα.

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
| macOS 10.13+ | Μεταγλωττίζεται και περνά τις δοκιμές. Δεν το έχει εκτελέσει κανείς ακόμη. |
| Ubuntu 24.04+ | Μεταγλωττίζεται και περνά τις δοκιμές. Δεν το έχει εκτελέσει κανείς ακόμη. |
| Firefox 128+ | Λειτουργεί στον Firefox 160. Η Mozilla δεν την έχει υπογράψει ακόμη, οπότε πρέπει να φορτωθεί ξανά μετά την επανεκκίνηση του Firefox. |

## Εγκατάσταση

Δεν υπάρχουν ακόμη έτοιμα αρχεία, οπότε το πρόγραμμα πρέπει να μεταγλωττιστεί. Χρειάζονται Node.js 24 ή νεότερο και Rust, στα Windows επίσης Microsoft C++ Build Tools, στο Ubuntu επίσης `libwebkit2gtk-4.1-dev librsvg2-dev`.

```sh
git clone https://github.com/Ilia-Pero/adx0
cd adx0
npm ci
npm run tauri build
```

Το πρόγραμμα εγκατάστασης εμφανίζεται στο `src-tauri/target/release/bundle`. Δεν είναι υπογεγραμμένο, οπότε τα Windows και το macOS θα ρωτήσουν αν το εμπιστεύεστε.

Η απεγκατάσταση ανοίγει ξανά τις αποκλεισμένες διευθύνσεις. Στο macOS απενεργοποιήστε τον αποκλεισμό πριν διαγράψετε το πρόγραμμα.

Η επέκταση: εκτελέστε `npm ci && npm run build` στον φάκελο `extension`, ανοίξτε το `about:debugging` στον Firefox, επιλέξτε «Αυτός ο Firefox», «Φόρτωση προσωρινού προσθέτου» και το αρχείο `extension/web/manifest.json`.

## Μεταφράσεις

Οι μεταφράσεις είναι μηχανικές και δεν έχουν ελεγχθεί από φυσικούς ομιλητές. Αν δείτε λάθος, διορθώστε το στο `locales`, στο `extension/web/_locales` ή στο `docs`.

## Συμμετοχή και άδεια

Πώς να μεταγλωττίσετε, να δοκιμάσετε και να αλλάξετε το πρόγραμμα: [CONTRIBUTING.md](../CONTRIBUTING.md). Η άδεια είναι [MIT](../LICENSE).
