<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">ad×0</h1>

<p align="center"><b>ad × 0 = 0</b></p>

<p align="center">Windows、macOS、Linux、ブラウザー用の広告ブロッカー</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/adx0/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/adx0?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · [简体中文](README.zh.md) · **日本語** · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md) · [Беларуская](README.be.md) · [Қазақша](README.kk.md)

<p align="center">
<img src="window-off.png" width="380" alt="プログラムのウィンドウ：ブロックはオフ">
<img src="window-on.png" width="380" alt="プログラムのウィンドウ：広告をブロック中">
</p>

ad×0 は広告サーバーのアドレスをシステムの hosts ファイルに書き込み、終了します。その後、コンピューターのプログラムはこれらのアドレスから何も読み込まず、ad×0 自身の動作中のプロセスもありません。ブラウザー拡張機能はページの広告枠を隠し、YouTube 動画の広告を取り除きます。

## インストール

| システム | ファイル | 方法 |
| --- | --- | --- |
| Windows 10, 11 | [adx0-windows.exe](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-windows.exe) | ファイルを実行します。Windows の警告が出たら「詳細情報」、「実行」の順に選びます。 |
| macOS 10.13+ | [adx0-macos.dmg](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-macos.dmg) | プログラムを「アプリケーション」に移して開きます。macOS に拒否されたら、システム設定の「プライバシーとセキュリティ」で「このまま開く」を押します。 |
| Ubuntu 24.04+ | [adx0-ubuntu.deb](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-ubuntu.deb) | `sudo apt install ./adx0-ubuntu.deb` |
| Firefox 128+ | [adx0-firefox.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-firefox.zip) | `about:debugging` を開き、「この Firefox」、「一時的なアドオンを読み込む」でファイルを選びます。Firefox を再起動したら、もう一度読み込みます。 |
| Chrome, Edge, Opera, Yandex Browser | [adx0-chrome.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-chrome.zip) | アーカイブを、削除しないフォルダーに展開します。`chrome://extensions` を開き、デベロッパーモードをオンにして「パッケージ化されていない拡張機能を読み込む」でフォルダーを選びます。 |

ファイルに有料の署名がないため、Windows と macOS は警告を表示します。GitHub がこのリポジトリからファイルを作成し、新しい環境にプログラムをインストールして起動した後に公開します。

アンインストールすると、ブロックしたアドレスが hosts ファイルから削除されます。macOS では、先にブロックをオフにしてください。

## プログラム

- 広告とトラッキングのサーバー約 10,000 アドレスをブロックします。リストは自分で選び、サイトを追加することもできます。
- hosts ファイルの自分の行はそのまま残し、最初の変更前にコピーを作り、書き込み後にファイルを確認します。
- システム起動後まもなく、ウィンドウなしで、多くても 1 日 1 回リストを更新します。オフにできます。
- 14 言語に対応。

hosts ファイルはアドレス全体をブロックするため、内容と同じアドレスから届く広告は残ります。独自にアドレスを調べるプロキシや VPN を使うプログラムには効きません。

## ブラウザー拡張機能

<p align="center"><img src="popup.png" width="312" alt="拡張機能のウィンドウ"></p>

- 約 55,000 アドレスをブロックし、ページの広告枠を隠します。
- YouTube の応答から広告の予定を取り除きます。
- ボタン 1 つでサイトの広告を許可できます。
- リストを 1 日 1 回更新します。

## データとネットワーク

プログラムと拡張機能はリストをそれぞれのサイトからダウンロードし、データを収集しません。プログラムは「更新を確認」を押したときに GitHub に新しい版を問い合わせます。

## 詳細

リストとライセンス、測定結果、確認済みのシステム、ビルド方法は[英語版](../README.md)にあります。

この文章は機械翻訳で、母語話者による確認を受けていません。修正は `docs` フォルダーにお願いします。

[MIT](../LICENSE)
