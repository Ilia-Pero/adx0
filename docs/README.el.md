<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">ad×0</h1>

<p align="center"><b>ad × 0 = 0</b></p>

<p align="center">Αποκλεισμός διαφημίσεων για Windows, macOS, Linux και προγράμματα περιήγησης</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/adx0/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/adx0?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · **Ελληνικά** · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md) · [Беларуская](README.be.md) · [Қазақша](README.kk.md)

<p align="center">
<img src="window-off.png" width="380" alt="Παράθυρο του προγράμματος: ο αποκλεισμός είναι ανενεργός">
<img src="window-on.png" width="380" alt="Παράθυρο του προγράμματος: οι διαφημίσεις είναι αποκλεισμένες">
</p>

Το ad×0 γράφει τις διευθύνσεις διαφημιστικών διακομιστών στο αρχείο hosts του συστήματος και τερματίζει. Μετά τα προγράμματα του υπολογιστή δεν φορτώνουν τίποτα από αυτές, και το ίδιο το ad×0 δεν έχει ενεργή διεργασία. Μια επέκταση κρύβει τις διαφημίσεις στις σελίδες και τις αφαιρεί από τα βίντεο του YouTube.

## Εγκατάσταση

| Σύστημα | Αρχείο | Πώς εγκαθίσταται |
| --- | --- | --- |
| Windows 10, 11 | [adx0-windows.exe](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-windows.exe) | Εκτελέστε το αρχείο. Αν τα Windows δείξουν προειδοποίηση, επιλέξτε «Περισσότερες πληροφορίες» και «Εκτέλεση ούτως ή άλλως». |
| macOS 10.13+ | [adx0-macos.dmg](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-macos.dmg) | Σύρετε το πρόγραμμα στις «Εφαρμογές» και ανοίξτε το. Μετά την πρώτη άρνηση του macOS ανοίξτε τις Ρυθμίσεις συστήματος, «Απόρρητο και ασφάλεια», και πατήστε «Άνοιγμα ούτως ή άλλως». |
| Ubuntu 24.04+ | [adx0-ubuntu.deb](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-ubuntu.deb) | `sudo apt install ./adx0-ubuntu.deb` |
| Firefox 128+ | [adx0-firefox.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-firefox.zip) | Ανοίξτε το `about:debugging`, «Αυτό το Firefox», «Φόρτωση προσωρινού προσθέτου» και επιλέξτε το αρχείο. Μετά από επανεκκίνηση του Firefox φορτώστε το ξανά. |
| Chrome, Edge, Opera, Yandex Browser | [adx0-chrome.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-chrome.zip) | Αποσυμπιέστε το αρχείο σε φάκελο που θα κρατήσετε. Ανοίξτε το `chrome://extensions`, ενεργοποιήστε τη λειτουργία προγραμματιστή, πατήστε «Φόρτωση μη συσκευασμένης» και επιλέξτε τον φάκελο. |

Τα Windows και το macOS προειδοποιούν επειδή τα αρχεία δεν έχουν πληρωμένη υπογραφή. Το GitHub τα φτιάχνει από αυτό το αποθετήριο, εγκαθιστά τα προγράμματα σε καθαρές μηχανές και τα εκκινεί πριν δημοσιευτεί η έκδοση.

Η απεγκατάσταση αφαιρεί τις αποκλεισμένες διευθύνσεις από το αρχείο hosts. Στο macOS απενεργοποιήστε πρώτα τον αποκλεισμό.

## Πρόγραμμα

- Αποκλείει περίπου 10.000 διευθύνσεις διαφημίσεων και ιχνηλάτησης. Εσείς επιλέγετε τις λίστες και μπορείτε να προσθέσετε δικούς σας ιστότοπους.
- Αφήνει τις γραμμές σας στο αρχείο hosts όπως είναι, κρατά αντίγραφο πριν από την πρώτη αλλαγή και ελέγχει το αρχείο μετά την εγγραφή.
- Ενημερώνει τις λίστες το πολύ μία φορά την ημέρα, λίγο μετά την εκκίνηση του συστήματος και χωρίς παράθυρο. Αυτό απενεργοποιείται.
- 14 γλώσσες.

Το αρχείο hosts αποκλείει ολόκληρες διευθύνσεις, γι’ αυτό μένουν οι διαφημίσεις που έρχονται από την ίδια διεύθυνση με το περιεχόμενο. Προγράμματα με proxy ή VPN που βρίσκουν μόνα τους τις διευθύνσεις το παρακάμπτουν.

## Επέκταση προγράμματος περιήγησης

<p align="center"><img src="popup.png" width="312" alt="Παράθυρο της επέκτασης"></p>

- Αποκλείει περίπου 64.000 διευθύνσεις και κρύβει διαφημίσεις στις σελίδες.
- Αφαιρεί το πρόγραμμα διαφημίσεων από τις απαντήσεις του YouTube.
- Επιτρέπει τις διαφημίσεις σε έναν ιστότοπο με ένα κουμπί.
- Ενημερώνει τις λίστες μία φορά την ημέρα.

## Δεδομένα και δίκτυο

Το πρόγραμμα και η επέκταση κατεβάζουν τις λίστες από τους ιστότοπούς τους και δεν συλλέγουν δεδομένα. Το πρόγραμμα ρωτά το GitHub για νέα έκδοση όταν πατάτε «Έλεγχος για ενημερώσεις».

## Περισσότερα

Οι λίστες και οι άδειές τους, οι μετρήσεις, τα ελεγμένα συστήματα και η μεταγλώττιση περιγράφονται [στα αγγλικά](../README.md).

Το κείμενο μεταφράστηκε από μηχανή και δεν το έχουν ελέγξει φυσικοί ομιλητές. Οι διορθώσεις γίνονται στον φάκελο `docs`.

[MIT](../LICENSE)
