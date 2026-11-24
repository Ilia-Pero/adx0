<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">ad×0</h1>

<p align="center"><b>ad × 0 = 0</b></p>

<p align="center">Un bloqueador de anuncios que no se queda en ejecución: para Windows, macOS, Linux y Firefox</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<img src="https://img.shields.io/badge/languages-12-0f766e" alt="12 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · **Español** · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md)

<p align="center">
<img src="window-off.png" width="380" alt="La ventana del programa: el bloqueo está desactivado">
<img src="window-on.png" width="380" alt="La ventana del programa: la publicidad está bloqueada">
</p>

ad×0 escribe las direcciones de los servidores de publicidad en el archivo hosts del sistema y se cierra. Los anuncios de esas direcciones dejan de cargarse en todos los programas y no hay ningún proceso en segundo plano que ocupe memoria o procesador. Una extensión para Firefox hace lo que el archivo hosts no puede: oculta los bloques de publicidad en las páginas y quita los anuncios de los vídeos de YouTube.

## El programa

- Bloquea unas 10 000 direcciones de servidores de publicidad y rastreadores de las listas que usted elija.
- Conserva sus propias líneas del archivo hosts, deja una copia antes del primer cambio y vuelve a leer lo que ha escrito.
- Guarda la última copia buena de cada lista, así el bloqueo no depende de que el sitio de la lista funcione.
- No deja que un error de una lista cierre sitios como Google, Wikipedia o GitHub: esta es [la lista completa](../data/essential.json). Cualquier otro sitio se abre con el campo «No bloquear».
- Habla 12 idiomas. No envía nada: solo descarga las listas cuando usted pulsa el botón.

## La extensión para Firefox

<p align="center"><img src="popup.png" width="312" alt="La extensión para Firefox"></p>

- Bloquea direcciones de publicidad y oculta bloques en las páginas. Las listas regionales se eligen según los idiomas del navegador.
- Quita el calendario de anuncios de las respuestas de YouTube, así el reproductor no tiene nada que mostrar.
- Actualiza las listas una vez al día. El propio navegador aplica las reglas, y la parte de fondo se descarga entre tanto.

## Qué no puede hacer

El archivo hosts trabaja con direcciones enteras. Los anuncios que vienen de la misma dirección que el contenido, como en YouTube o en las redes sociales, permanecen; para YouTube está la extensión. Un proxy o una VPN que busca las direcciones por su cuenta no pasa por el archivo hosts.

## Cuánto ocupa

Medido el 24.11.2026 en Windows Server 2022, compilación optimizada:

| Qué | Resultado |
| --- | --- |
| Procesos con la ventana cerrada | 0 |
| Procesador con la ventana abierta | 0–31 ms en 5 s |
| Del inicio a la ventana | 0,06–0,6 s en cuatro ejecuciones |
| Tamaño del programa | 4,4 MB |
| Crecimiento del archivo hosts | 198 KB para 9800 direcciones |

## Listas

Las listas no forman parte de este repositorio. Su copia del programa las descarga de sus sitios, y cada una tiene su propia licencia.

| Lista | Dónde | Licencia |
| --- | --- | --- |
| [Peter Lowe](https://pgl.yoyo.org/adservers/) | ambos | [McRae GPL](https://pgl.yoyo.org/license/) |
| [AdAway](https://adaway.org/) | ambos | CC BY 3.0 |
| [RU AdList](https://github.com/easylist/ruadlist) | ambos | CC BY 3.0 |
| [Liste FR](https://github.com/easylist/listefr) | ambos | CC BY-SA 3.0 |
| [EasyList China](https://github.com/easylist/easylistchina) | ambos | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [StevenBlack](https://github.com/StevenBlack/hosts) | programa | MIT |
| [EasyList](https://easylist.to/) | extensión | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Germany](https://github.com/easylist/easylistgermany) | extensión | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Hebrew](https://github.com/easylist/EasyListHebrew) | extensión | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Italy](https://github.com/easylist/easylistitaly) | extensión | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Spanish](https://github.com/easylist/easylistspanish) | extensión | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [KoreanList](https://github.com/easylist/KoreanList) | extensión | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [AdGuard Japanese](https://github.com/AdguardTeam/AdguardFilters) | extensión | GPL 3.0 |
| [Greek AdBlock Filter](https://github.com/kargig/greek-adblockplus-filter) | extensión | CC BY-SA 4.0 |

## Estado

| Sistema | Estado |
| --- | --- |
| Windows | Funciona en Windows Server 2022: probado con listas reales en una copia del archivo hosts. Aún no se ha probado en Windows 10 y 11. |
| macOS 10.13+ | Se compila y pasa las pruebas. Nadie lo ha ejecutado todavía. |
| Ubuntu 24.04+ | Se compila y pasa las pruebas. Nadie lo ha ejecutado todavía. |
| Firefox 128+ | Funciona en Firefox 160. Mozilla aún no la ha firmado, así que hay que cargarla de nuevo tras reiniciar Firefox. |

## Instalación

Aún no hay descargas listas, así que hay que compilar el programa. Hacen falta Node.js 24 o posterior y Rust; en Windows, también Microsoft C++ Build Tools; en Ubuntu, también `libwebkit2gtk-4.1-dev librsvg2-dev`.

```sh
git clone https://github.com/Ilia-Pero/adx0
cd adx0
npm ci
npm run tauri build
```

El instalador aparece en `src-tauri/target/release/bundle`. No está firmado, así que Windows y macOS preguntarán si confía en él.

El desinstalador vuelve a abrir las direcciones bloqueadas. En macOS, desactive el bloqueo antes de borrar el programa.

La extensión: ejecute `npm ci && npm run build` en la carpeta `extension`, abra `about:debugging` en Firefox, elija «Este Firefox», «Cargar complemento temporal» y el archivo `extension/web/manifest.json`.

## Traducciones

Las traducciones son automáticas y no las ha revisado ningún hablante nativo. Si ve un error, corríjalo en `locales`, en `extension/web/_locales` o en `docs`.

## Participar y licencia

Cómo compilar, probar y cambiar el programa: [CONTRIBUTING.md](../CONTRIBUTING.md). La licencia es [MIT](../LICENSE).
