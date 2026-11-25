<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">ad×0</h1>

<p align="center"><b>ad × 0 = 0</b></p>

<p align="center">Un bloqueur de publicités qui ne tourne pas : pour Windows, macOS, Linux et Firefox</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/adx0/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/adx0?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · **Français** · [Deutsch](README.de.md) · [Italiano](README.it.md) · [Беларуская](README.be.md) · [Қазақша](README.kk.md)

<p align="center">
<img src="window-off.png" width="380" alt="La fenêtre du programme : le blocage est désactivé">
<img src="window-on.png" width="380" alt="La fenêtre du programme : les publicités sont bloquées">
</p>

ad×0 écrit les adresses des serveurs publicitaires dans le fichier hosts du système, puis se ferme. Les publicités de ces adresses ne se chargent plus dans aucun programme, et aucun processus en arrière-plan n’occupe la mémoire ou le processeur. Une extension pour Firefox fait ce que le fichier hosts ne peut pas faire : elle masque les blocs publicitaires des pages et retire les publicités des vidéos YouTube.

## Le programme

- Bloque environ 10 000 adresses de serveurs publicitaires et de traceurs, et environ 15 300 avec la liste française. Vous choisissez les listes.
- Conserve vos propres lignes du fichier hosts, en laisse une copie avant la première modification et relit ce qu’il a écrit.
- Conserve la dernière bonne copie de chaque liste : le blocage ne dépend pas de la disponibilité du site de la liste.
- Ne laisse pas une erreur de liste fermer des sites comme Google, Wikipedia ou GitHub : voici [la liste entière](../data/essential.json). Tout autre site s’ouvre avec le champ « Ne pas bloquer ».
- Parle 14 langues : la langue se choisit en haut de la fenêtre et dans l’installateur pour Windows. N’envoie rien : il télécharge seulement les listes, quand vous appuyez sur le bouton et, tant que le blocage est activé, au démarrage du système, une fois par jour au plus. Cela se fait sans fenêtre en un instant et peut être désactivé.

## L’extension pour Firefox

<p align="center"><img src="popup.png" width="312" alt="L’extension pour Firefox"></p>

- Bloque les adresses publicitaires et masque les blocs publicitaires des pages. Les listes régionales sont choisies selon les langues du navigateur.
- Retire le programme des publicités des réponses de YouTube : le lecteur n’a plus rien à montrer.
- Met les listes à jour une fois par jour. Le navigateur applique lui-même les règles, et la partie d’arrière-plan n’est pas chargée entre-temps.
- Parle les mêmes 14 langues : d’abord celle du navigateur ; une autre se choisit dans la fenêtre de l’extension.

## Ce qu’il ne peut pas faire

Le fichier hosts travaille avec des adresses entières. Les publicités venant de la même adresse que le contenu, comme sur YouTube ou les réseaux sociaux, restent ; pour YouTube, il y a l’extension. Un proxy ou un VPN qui cherche lui-même les adresses contourne le fichier hosts.

## Sa légèreté

Mesuré le 24.11.2026 sous Windows Server 2022, version optimisée :

| Mesure | Résultat |
| --- | --- |
| Processus quand la fenêtre est fermée | 0 |
| Processeur quand la fenêtre est ouverte | 0–31 ms en 5 s |
| Du lancement à la fenêtre | 0,06–0,6 s sur quatre essais |
| Taille du programme | 4,4 Mo |
| Croissance du fichier hosts | 198 Ko pour 9 800 adresses, 303 Ko pour 15 300 avec la liste française |

## Listes

Les listes ne font pas partie de ce dépôt. Votre copie du programme les télécharge depuis leurs sites, et chacune a sa propre licence.

| Liste | Où | Licence |
| --- | --- | --- |
| [Peter Lowe](https://pgl.yoyo.org/adservers/) | les deux | [McRae GPL](https://pgl.yoyo.org/license/) |
| [AdAway](https://adaway.org/) | les deux | CC BY 3.0 |
| [RU AdList](https://github.com/easylist/ruadlist) | les deux | CC BY 3.0 |
| [Liste FR](https://github.com/easylist/listefr) | les deux | CC BY-SA 3.0 |
| [EasyList China](https://github.com/easylist/easylistchina) | les deux | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [StevenBlack](https://github.com/StevenBlack/hosts) | programme | MIT |
| [EasyList](https://easylist.to/) | extension | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Germany](https://github.com/easylist/easylistgermany) | extension | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Hebrew](https://github.com/easylist/EasyListHebrew) | extension | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Italy](https://github.com/easylist/easylistitaly) | extension | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Spanish](https://github.com/easylist/easylistspanish) | extension | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [KoreanList](https://github.com/easylist/KoreanList) | extension | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [AdGuard Japanese](https://github.com/AdguardTeam/AdguardFilters) | extension | GPL 3.0 |
| [Greek AdBlock Filter](https://github.com/kargig/greek-adblockplus-filter) | extension | CC BY-SA 4.0 |

## État

| Système | État |
| --- | --- |
| Windows | Fonctionne sous Windows Server 2022 : vérifié avec de vraies listes sur une copie du fichier hosts. Pas encore vérifié sous Windows 10 et 11. |
| macOS 10.13+ | Se compile et passe les tests ; sur une machine de GitHub, il s’installe et démarre. Personne ne l’a encore essayé à la main. |
| Ubuntu 24.04+ | Se compile et passe les tests ; sur une machine de GitHub, il s’installe et démarre. Personne ne l’a encore essayé à la main. |
| Firefox 128+ | Fonctionne dans Firefox 160. Pas encore signée par Mozilla, il faut donc la recharger après un redémarrage de Firefox. |

## Installation

| Système | Fichier | Comment l’installer |
| --- | --- | --- |
| Windows 10 et 11 | [adx0-windows.exe](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-windows.exe) | Lancez-le. Si Windows ne connaît pas le programme : « Informations complémentaires », « Exécuter quand même ». |
| macOS 10.13+ | [adx0-macos.dmg](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-macos.dmg) | Glissez le programme dans Applications et ouvrez-le. Quand macOS refuse, allez dans Réglages Système, « Confidentialité et sécurité », et cliquez sur « Ouvrir quand même ». |
| Ubuntu 24.04+ | [adx0-ubuntu.deb](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-ubuntu.deb) | `sudo apt install ./adx0-ubuntu.deb` |
| Firefox 128+ | [adx0-firefox.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-firefox.zip) | Ouvrez `about:debugging`, « Ce Firefox », « Charger un module complémentaire temporaire », et choisissez le fichier. Après un redémarrage de Firefox, rechargez-le. |

GitHub compile ces fichiers à partir du code de ce dépôt, installe et lance les programmes, puis seulement publie le tout. Les fichiers n’ont pas de signature payante, Windows et macOS avertissent donc au premier lancement.

Vous pouvez aussi compiler le programme vous-même. Il faut Node.js 24 ou plus récent et Rust, sous Windows aussi Microsoft C++ Build Tools, sous Ubuntu aussi `libwebkit2gtk-4.1-dev librsvg2-dev`.

```sh
git clone https://github.com/Ilia-Pero/adx0
cd adx0
npm ci
npm run tauri build
```

L’installateur apparaît dans `src-tauri/target/release/bundle`.

La désinstallation rouvre les adresses bloquées. Sous macOS, désactivez le blocage avant de supprimer le programme.

L’extension : lancez `npm ci && npm run build` dans le dossier `extension`, ouvrez `about:debugging` dans Firefox, choisissez « Ce Firefox », « Charger un module complémentaire temporaire » et le fichier `extension/web/manifest.json`.

## Traductions

Les traductions sont automatiques et n’ont pas été relues par des locuteurs natifs. Si vous voyez une erreur, corrigez-la dans `locales`, dans `extension/web/_locales` ou dans `docs`.

## Participer et licence

Comment compiler, tester et modifier le programme : [CONTRIBUTING.md](../CONTRIBUTING.md). La licence est [MIT](../LICENSE).
