<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">ad×0</h1>

<p align="center"><b>ad × 0 = 0</b></p>

<p align="center">Un blocco della pubblicità che non resta in esecuzione: per Windows, macOS, Linux e Firefox</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/adx0/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/adx0?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-12-0f766e" alt="12 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · **Italiano**

<p align="center">
<img src="window-off.png" width="380" alt="La finestra del programma: il blocco è disattivato">
<img src="window-on.png" width="380" alt="La finestra del programma: la pubblicità è bloccata">
</p>

ad×0 scrive gli indirizzi dei server pubblicitari nel file hosts del sistema e si chiude. La pubblicità da questi indirizzi smette di caricarsi in tutti i programmi, e non c’è alcun processo in background che occupi memoria o processore. Un’estensione per Firefox fa ciò che il file hosts non può fare: nasconde i blocchi pubblicitari nelle pagine e toglie la pubblicità dai video di YouTube.

## Il programma

- Blocca circa 10.000 indirizzi di server pubblicitari e traccianti dalle liste che scegli.
- Conserva le tue righe del file hosts, ne lascia una copia prima della prima modifica e rilegge ciò che ha scritto.
- Conserva l’ultima copia valida di ogni lista, così il blocco non dipende dal sito della lista.
- Non permette a un errore in una lista di chiudere siti come Google, Wikipedia o GitHub: ecco [la lista completa](../data/essential.json). Ogni altro sito si apre con il campo «Non bloccare».
- Parla 12 lingue: la lingua si sceglie in alto nella finestra e nel programma di installazione per Windows. Non invia nulla: scarica soltanto le liste, quando premi il pulsante e, finché il blocco è attivo, all’avvio del sistema, al massimo una volta al giorno. Questo avviene senza finestra in un istante e si può disattivare.

## L’estensione per Firefox

<p align="center"><img src="popup.png" width="312" alt="L’estensione per Firefox"></p>

- Blocca gli indirizzi pubblicitari e nasconde i blocchi pubblicitari nelle pagine. Le liste regionali si scelgono in base alle lingue del browser.
- Toglie il programma della pubblicità dalle risposte di YouTube, così il lettore non ha nulla da mostrare.
- Aggiorna le liste una volta al giorno. È il browser ad applicare le regole, e la parte in background nel frattempo non è caricata.
- Parla le stesse 12 lingue: all’inizio quella del browser; un’altra si sceglie nella finestra dell’estensione.

## Che cosa non può fare

Il file hosts lavora con indirizzi interi. La pubblicità che arriva dallo stesso indirizzo del contenuto, come su YouTube o nei social network, resta; per YouTube c’è l’estensione. Un proxy o una VPN che cerca gli indirizzi da sé aggira il file hosts.

## Quanto è leggero

Misurato il 24.11.2026 su Windows Server 2022, build ottimizzata:

| Misura | Risultato |
| --- | --- |
| Processi con la finestra chiusa | 0 |
| Processore con la finestra aperta | 0–31 ms in 5 s |
| Dall’avvio alla finestra | 0,06–0,6 s in quattro prove |
| Dimensione del programma | 4,4 MB |
| Crescita del file hosts | 198 KB per 9.800 indirizzi |

## Liste

Le liste non fanno parte di questo repository. La tua copia del programma le scarica dai loro siti, e ognuna ha la propria licenza.

| Lista | Dove | Licenza |
| --- | --- | --- |
| [Peter Lowe](https://pgl.yoyo.org/adservers/) | entrambi | [McRae GPL](https://pgl.yoyo.org/license/) |
| [AdAway](https://adaway.org/) | entrambi | CC BY 3.0 |
| [RU AdList](https://github.com/easylist/ruadlist) | entrambi | CC BY 3.0 |
| [Liste FR](https://github.com/easylist/listefr) | entrambi | CC BY-SA 3.0 |
| [EasyList China](https://github.com/easylist/easylistchina) | entrambi | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [StevenBlack](https://github.com/StevenBlack/hosts) | programma | MIT |
| [EasyList](https://easylist.to/) | estensione | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Germany](https://github.com/easylist/easylistgermany) | estensione | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Hebrew](https://github.com/easylist/EasyListHebrew) | estensione | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Italy](https://github.com/easylist/easylistitaly) | estensione | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Spanish](https://github.com/easylist/easylistspanish) | estensione | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [KoreanList](https://github.com/easylist/KoreanList) | estensione | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [AdGuard Japanese](https://github.com/AdguardTeam/AdguardFilters) | estensione | GPL 3.0 |
| [Greek AdBlock Filter](https://github.com/kargig/greek-adblockplus-filter) | estensione | CC BY-SA 4.0 |

## Stato

| Sistema | Stato |
| --- | --- |
| Windows | Funziona su Windows Server 2022: verificato con liste reali su una copia del file hosts. Non ancora verificato su Windows 10 e 11. |
| macOS 10.13+ | Si compila e supera i test; su una macchina di GitHub si installa e si avvia. Nessuno l’ha ancora provato a mano. |
| Ubuntu 24.04+ | Si compila e supera i test; su una macchina di GitHub si installa e si avvia. Nessuno l’ha ancora provato a mano. |
| Firefox 128+ | Funziona in Firefox 160. Non ancora firmata da Mozilla, quindi va ricaricata dopo il riavvio di Firefox. |

## Installazione

| Sistema | File | Come si installa |
| --- | --- | --- |
| Windows 10 e 11 | [adx0-windows.exe](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-windows.exe) | Avvialo. Se Windows non conosce il programma: «Ulteriori informazioni», «Esegui comunque». |
| macOS 10.13+ | [adx0-macos.dmg](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-macos.dmg) | Trascina il programma in Applicazioni e aprilo. Quando macOS rifiuta, vai in Impostazioni di Sistema, «Privacy e sicurezza», e fai clic su «Apri comunque». |
| Ubuntu 24.04+ | [adx0-ubuntu.deb](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-ubuntu.deb) | `sudo apt install ./adx0-ubuntu.deb` |
| Firefox 128+ | [adx0-firefox.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-firefox.zip) | Apri `about:debugging`, «Questo Firefox», «Carica componente aggiuntivo temporaneo», e scegli il file. Dopo il riavvio di Firefox ricaricalo. |

GitHub compila questi file dal codice di questo repository, installa e avvia i programmi e solo dopo pubblica tutto. I file non hanno una firma a pagamento, quindi Windows e macOS avvisano al primo avvio.

Puoi anche compilare il programma da te. Servono Node.js 24 o successivo e Rust, su Windows anche Microsoft C++ Build Tools, su Ubuntu anche `libwebkit2gtk-4.1-dev librsvg2-dev`.

```sh
git clone https://github.com/Ilia-Pero/adx0
cd adx0
npm ci
npm run tauri build
```

L’installer compare in `src-tauri/target/release/bundle`.

La disinstallazione riapre gli indirizzi bloccati. Su macOS disattiva il blocco prima di eliminare il programma.

L’estensione: esegui `npm ci && npm run build` nella cartella `extension`, apri `about:debugging` in Firefox, scegli «Questo Firefox», «Carica componente aggiuntivo temporaneo» e il file `extension/web/manifest.json`.

## Traduzioni

Le traduzioni sono automatiche e non sono state controllate da madrelingua. Se vedi un errore, correggilo in `locales`, in `extension/web/_locales` o in `docs`.

## Partecipare e licenza

Come compilare, provare e modificare il programma: [CONTRIBUTING.md](../CONTRIBUTING.md). La licenza è [MIT](../LICENSE).
