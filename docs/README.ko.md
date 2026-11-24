<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">ad×0</h1>

<p align="center"><b>ad × 0 = 0</b></p>

<p align="center">백그라운드에서 돌지 않는 광고 차단기 (Windows, macOS, Linux, Firefox)</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<img src="https://img.shields.io/badge/languages-12-0f766e" alt="12 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · **한국어** · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md)

<p align="center">
<img src="window-off.png" width="380" alt="프로그램 창: 차단이 꺼져 있음">
<img src="window-on.png" width="380" alt="프로그램 창: 광고를 차단하고 있음">
</p>

ad×0은 광고 서버의 주소를 시스템의 hosts 파일에 쓰고 종료합니다. 모든 프로그램에서 이 주소들의 광고가 더 이상 로드되지 않으며, 메모리나 프로세서를 쓰는 백그라운드 프로세스가 없습니다. Firefox 확장 기능은 hosts 파일이 할 수 없는 일을 합니다. 페이지의 광고 영역을 숨기고 YouTube 동영상에서 광고를 없앱니다.

## 프로그램

- 고른 목록에 있는 광고 서버와 추적기의 주소 약 10,000개를 차단합니다.
- hosts 파일에 있던 내용을 그대로 두고, 처음 고치기 전에 사본을 남기며, 쓴 내용을 다시 읽어 확인합니다.
- 목록마다 마지막으로 제대로 받은 사본을 보관하므로 목록 사이트가 멈춰도 차단됩니다.
- 목록의 잘못으로 Google, Wikipedia, GitHub 같은 사이트([전체 목록](../data/essential.json))가 막히지 않습니다. 다른 사이트는 "차단하지 않음"에서 열 수 있습니다.
- 12개 언어를 지원합니다. 언어는 창 위쪽과 Windows 설치 프로그램에서 고릅니다. 아무 데도 보내지 않으며 단추를 누를 때 목록만 내려받습니다.

## Firefox 확장 기능

<p align="center"><img src="popup.png" width="312" alt="Firefox 확장 기능"></p>

- 광고 주소를 차단하고 페이지의 광고 영역을 숨깁니다. 지역 목록은 브라우저 언어에 따라 고릅니다.
- YouTube의 응답에서 광고 일정을 없애므로 플레이어가 보여 줄 광고가 없습니다.
- 목록을 하루에 한 번 업데이트합니다. 규칙은 브라우저가 직접 적용하고, 그동안 백그라운드 부분은 올라와 있지 않습니다.
- 같은 12개 언어를 지원합니다. 처음에는 브라우저 언어를 쓰며 확장 기능 창에서 바꿀 수 있습니다.

## 할 수 없는 일

hosts 파일은 주소 전체를 다룹니다. YouTube나 소셜 네트워크처럼 내용과 같은 주소에서 오는 광고는 남습니다. YouTube에는 확장 기능이 있습니다. 스스로 주소를 찾는 프록시나 VPN은 hosts 파일을 거치지 않습니다.

## 얼마나 가벼운가

2026-11-24, Windows Server 2022, 릴리스 빌드에서 측정:

| 항목 | 결과 |
| --- | --- |
| 창을 닫았을 때의 프로세스 수 | 0 |
| 창을 열었을 때의 프로세서 시간 | 5초 동안 0~31ms |
| 시작에서 창이 뜰 때까지 | 네 번 측정 0.06~0.6초 |
| 프로그램 크기 | 4.4MB |
| hosts 파일이 늘어나는 크기 | 주소 9,800개에 198KB |

## 목록

목록은 이 저장소에 들어 있지 않습니다. 여러분의 프로그램이 각 사이트에서 내려받으며 목록마다 라이선스가 다릅니다.

| 목록 | 쓰이는 곳 | 라이선스 |
| --- | --- | --- |
| [Peter Lowe](https://pgl.yoyo.org/adservers/) | 둘 다 | [McRae GPL](https://pgl.yoyo.org/license/) |
| [AdAway](https://adaway.org/) | 둘 다 | CC BY 3.0 |
| [RU AdList](https://github.com/easylist/ruadlist) | 둘 다 | CC BY 3.0 |
| [Liste FR](https://github.com/easylist/listefr) | 둘 다 | CC BY-SA 3.0 |
| [EasyList China](https://github.com/easylist/easylistchina) | 둘 다 | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [StevenBlack](https://github.com/StevenBlack/hosts) | 프로그램 | MIT |
| [EasyList](https://easylist.to/) | 확장 기능 | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Germany](https://github.com/easylist/easylistgermany) | 확장 기능 | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Hebrew](https://github.com/easylist/EasyListHebrew) | 확장 기능 | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Italy](https://github.com/easylist/easylistitaly) | 확장 기능 | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Spanish](https://github.com/easylist/easylistspanish) | 확장 기능 | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [KoreanList](https://github.com/easylist/KoreanList) | 확장 기능 | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [AdGuard Japanese](https://github.com/AdguardTeam/AdguardFilters) | 확장 기능 | GPL 3.0 |
| [Greek AdBlock Filter](https://github.com/kargig/greek-adblockplus-filter) | 확장 기능 | CC BY-SA 4.0 |

## 상태

| 시스템 | 상태 |
| --- | --- |
| Windows | Windows Server 2022에서 동작합니다. 실제 목록으로 hosts 파일의 사본에서 확인했습니다. Windows 10과 11에서는 아직 확인하지 않았습니다. |
| macOS 10.13+ | 빌드되고 테스트를 통과합니다. 아직 아무도 실행해 보지 않았습니다. |
| Ubuntu 24.04+ | 빌드되고 테스트를 통과합니다. 아직 아무도 실행해 보지 않았습니다. |
| Firefox 128+ | Firefox 160에서 동작합니다. 아직 Mozilla의 서명이 없어 Firefox를 다시 시작하면 다시 로드해야 합니다. |

## 설치

아직 내려받을 수 있는 파일이 없어 직접 빌드해야 합니다. Node.js 24 이상과 Rust가 필요하고, Windows에서는 Microsoft C++ Build Tools, Ubuntu에서는 `libwebkit2gtk-4.1-dev librsvg2-dev`도 필요합니다.

```sh
git clone https://github.com/Ilia-Pero/adx0
cd adx0
npm ci
npm run tauri build
```

설치 파일은 `src-tauri/target/release/bundle`에 생깁니다. 서명되지 않았으므로 Windows와 macOS가 신뢰할지 묻습니다.

제거하면 차단된 주소가 다시 열립니다. macOS에서는 프로그램을 지우기 전에 차단을 끄세요.

확장 기능: `extension` 폴더에서 `npm ci && npm run build`를 실행하고, Firefox에서 `about:debugging`을 열어 "현재 Firefox", "임시 부가 기능 로드", `extension/web/manifest.json` 파일을 고릅니다.

## 번역

번역은 기계 번역이며 원어민이 확인하지 않았습니다. 잘못된 곳을 보면 `locales`, `extension/web/_locales` 또는 `docs`에서 고쳐 주세요.

## 참여와 라이선스

빌드, 테스트, 수정 방법: [CONTRIBUTING.md](../CONTRIBUTING.md). 라이선스는 [MIT](../LICENSE)입니다.
