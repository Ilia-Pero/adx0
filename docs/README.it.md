<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">ad×0</h1>

<p align="center"><b>ad × 0 = 0</b></p>

<p align="center">Blocco della pubblicità per Windows, macOS, Linux e browser</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/adx0/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/adx0?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · **Italiano** · [Беларуская](README.be.md) · [Қазақша](README.kk.md)

<p align="center">
<img src="window-off.png" width="380" alt="Finestra del programma: il blocco è disattivato">
<img src="window-on.png" width="380" alt="Finestra del programma: la pubblicità è bloccata">
</p>

ad×0 scrive gli indirizzi dei server pubblicitari nel file hosts del sistema e si chiude. Da quel momento i programmi del computer non caricano nulla da quegli indirizzi, e ad×0 non ha processi attivi. Un’estensione nasconde i blocchi pubblicitari nelle pagine e toglie la pubblicità dai video di YouTube.

## Installazione

| Sistema | File | Come installare |
| --- | --- | --- |
| Windows 10, 11 | [adx0-windows.exe](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-windows.exe) | Avvia il file. Se Windows mostra un avviso, scegli “Ulteriori informazioni” e poi “Esegui comunque”. |
| macOS 10.13+ | [adx0-macos.dmg](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-macos.dmg) | Trascina il programma in Applicazioni e aprilo. Dopo il primo rifiuto di macOS apri Impostazioni di Sistema, “Privacy e sicurezza”, e premi “Apri comunque”. |
| Ubuntu 24.04+ | [adx0-ubuntu.deb](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-ubuntu.deb) | `sudo apt install ./adx0-ubuntu.deb` |
| Firefox 128+ | [adx0-firefox.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-firefox.zip) | Apri `about:debugging`, “Questo Firefox”, “Carica componente aggiuntivo temporaneo” e scegli il file. Dopo un riavvio di Firefox caricalo di nuovo. |
| Chrome, Edge, Opera, Yandex Browser | [adx0-chrome.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-chrome.zip) | Estrai l’archivio in una cartella da conservare. Apri `chrome://extensions`, attiva la modalità sviluppatore, premi “Carica estensione non pacchettizzata” e scegli la cartella. |

Windows e macOS avvisano perché i file non hanno una firma a pagamento. GitHub li compila da questo repository, installa i programmi su macchine pulite e li avvia prima di pubblicare una versione.

La disinstallazione toglie gli indirizzi bloccati dal file hosts. Su macOS disattiva prima il blocco.

## Programma

- Blocca circa 10.000 indirizzi di server pubblicitari e di tracciamento. Scegli tu gli elenchi e puoi aggiungere i tuoi siti.
- Non tocca le tue righe del file hosts, ne fa una copia prima della prima modifica e controlla il file dopo la scrittura.
- Aggiorna gli elenchi al massimo una volta al giorno, poco dopo l’avvio del sistema e senza finestra. Si può disattivare.
- 14 lingue.

Il file hosts blocca indirizzi interi, quindi resta la pubblicità che arriva dallo stesso indirizzo del contenuto. I programmi con proxy o VPN che cercano gli indirizzi da soli lo aggirano.

## Estensione per browser

<p align="center"><img src="popup.png" width="312" alt="Finestra dell’estensione"></p>

- Blocca circa 55.000 indirizzi e nasconde i blocchi pubblicitari nelle pagine.
- Toglie il programma degli annunci dalle risposte di YouTube.
- Consente la pubblicità su un sito con un pulsante.
- Aggiorna gli elenchi una volta al giorno.

## Dati e rete

Il programma e l’estensione scaricano gli elenchi dai loro siti e non raccolgono dati. Il programma chiede a GitHub una nuova versione quando premi “Cerca aggiornamenti”.

## Altre informazioni

Gli elenchi e le loro licenze, le misure, i sistemi verificati e la compilazione sono descritti [in inglese](../README.md).

Questo testo è una traduzione automatica non controllata da madrelingua. Le correzioni vanno nella cartella `docs`.

[MIT](../LICENSE)
