<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">ad×0</h1>

<p align="center"><b>ad × 0 = 0</b></p>

<p align="center">Windows, macOS, Linux, 브라우저용 광고 차단기</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/adx0/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/adx0?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · [日本語](README.ja.md) · **한국어** · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md) · [Беларуская](README.be.md) · [Қазақша](README.kk.md)

<p align="center">
<img src="window-off.png" width="380" alt="프로그램 창: 차단 꺼짐">
<img src="window-on.png" width="380" alt="프로그램 창: 광고 차단 중">
</p>

ad×0은 광고 서버 주소를 시스템의 hosts 파일에 기록하고 종료합니다. 그 뒤 컴퓨터의 프로그램은 이 주소에서 아무것도 불러오지 않으며, ad×0 자체의 실행 중인 프로세스는 없습니다. 브라우저 확장 프로그램은 페이지의 광고 영역을 숨기고 YouTube 동영상의 광고를 없앱니다.

## 설치

| 시스템 | 파일 | 설치 방법 |
| --- | --- | --- |
| Windows 10, 11 | [adx0-windows.exe](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-windows.exe) | 파일을 실행합니다. Windows 경고가 나오면 “추가 정보”, “실행”을 차례로 선택합니다. |
| macOS 10.13+ | [adx0-macos.dmg](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-macos.dmg) | 프로그램을 응용 프로그램 폴더로 옮긴 뒤 엽니다. macOS가 처음 거부하면 시스템 설정의 “개인정보 보호 및 보안”에서 “그래도 열기”를 누릅니다. |
| Ubuntu 24.04+ | [adx0-ubuntu.deb](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-ubuntu.deb) | `sudo apt install ./adx0-ubuntu.deb` |
| Firefox 128+ | [adx0-firefox.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-firefox.zip) | `about:debugging`을 열고 “이 Firefox”, “임시 부가 기능 로드”에서 파일을 선택합니다. Firefox를 다시 시작하면 다시 로드해야 합니다. |
| Chrome, Edge, Opera, Yandex Browser | [adx0-chrome.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-chrome.zip) | 지우지 않을 폴더에 압축을 풉니다. `chrome://extensions`를 열고 개발자 모드를 켠 뒤 “압축해제된 확장 프로그램을 로드합니다”를 눌러 폴더를 선택합니다. |

파일에 유료 서명이 없어 Windows와 macOS가 경고를 표시합니다. GitHub가 이 저장소에서 파일을 만들고, 깨끗한 컴퓨터에 프로그램을 설치하고 실행한 뒤에 공개합니다.

제거하면 차단한 주소가 hosts 파일에서 삭제됩니다. macOS에서는 먼저 차단을 끄세요.

## 프로그램

- 광고 및 추적 서버 주소 약 10,000개를 차단합니다. 목록은 직접 고르고 사이트를 추가할 수 있습니다.
- hosts 파일의 사용자 줄은 그대로 두고, 처음 바꾸기 전에 사본을 만들며, 기록한 뒤 파일을 확인합니다.
- 시스템이 시작된 직후 창 없이, 하루 한 번 이하로 목록을 업데이트합니다. 끌 수 있습니다.
- 14개 언어.

hosts 파일은 주소 전체를 차단하므로 콘텐츠와 같은 주소에서 오는 광고는 남습니다. 주소를 직접 찾는 프록시나 VPN을 쓰는 프로그램은 hosts 파일을 우회합니다.

## 브라우저 확장 프로그램

<p align="center"><img src="popup.png" width="312" alt="확장 프로그램 창"></p>

- 주소 약 64,000개를 차단하고 페이지의 광고 영역을 숨깁니다.
- YouTube 응답에서 광고 일정을 제거합니다.
- 버튼 하나로 사이트의 광고를 허용합니다.
- 하루 한 번 목록을 업데이트합니다.

## 데이터와 네트워크

프로그램과 확장 프로그램은 목록을 각 사이트에서 내려받으며 데이터를 수집하지 않습니다. “업데이트 확인”을 누르면 프로그램이 GitHub에 새 버전을 문의합니다.

## 더 보기

목록과 라이선스, 측정 결과, 확인한 시스템, 빌드 방법은 [영어 문서](../README.md)에 있습니다.

이 글은 기계 번역이며 원어민의 검토를 받지 않았습니다. 수정은 `docs` 폴더에 해 주세요.

[MIT](../LICENSE)
