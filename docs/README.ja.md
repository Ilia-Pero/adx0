<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">ad×0</h1>

<p align="center"><b>ad × 0 = 0</b></p>

<p align="center">常駐しない広告ブロッカー（Windows、macOS、Linux、Firefox）</p>

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

ad×0 は広告サーバーのアドレスをシステムの hosts ファイルに書き込み、終了します。すべてのプログラムで、これらのアドレスからの広告が読み込まれなくなり、メモリーやプロセッサーを使う常駐プロセスはありません。Firefox の拡張機能は hosts ファイルではできないことをします。ページの広告枠を隠し、YouTube の動画から広告を取り除きます。

## プログラム

- 選んだリストにある、広告サーバーとトラッカーのアドレス約 10,000 件をブロックします。
- hosts ファイルの自分の行を残し、最初の変更の前に控えを作り、書き込んだ内容を読み直して確かめます。
- 各リストの最後に正しく取得できた内容を保存するので、リストのサイトが止まっていてもブロックできます。
- リストの誤りで Google、Wikipedia、GitHub などのサイト（[一覧](../data/essential.json)）が閉じることはありません。ほかのサイトは「ブロックしない」で開けます。
- 14 の言語に対応しています。言語はウィンドウの上部と Windows のインストーラーで選べます。何も送信しません。リストを取得するのは、ボタンを押したときと、ブロックがオンの間のシステム起動時（多くても 1 日 1 回）だけです。起動時の取得はウィンドウなしで一瞬で終わり、オフにできます。

## Firefox の拡張機能

<p align="center"><img src="popup.png" width="312" alt="Firefox の拡張機能"></p>

- 広告のアドレスをブロックし、ページの広告枠を隠します。地域のリストはブラウザーの言語に合わせて選ばれます。
- YouTube の応答から広告の予定を取り除くので、プレーヤーには表示する広告がありません。
- リストを 1 日 1 回更新します。規則はブラウザー自身が適用し、その間バックグラウンド部分は読み込まれません。
- 同じ 14 の言語に対応しています。最初はブラウザーの言語で、拡張機能のウィンドウで変更できます。

## できないこと

hosts ファイルはアドレス全体を扱います。YouTube や SNS のように、内容と同じアドレスから届く広告は残ります。YouTube には拡張機能があります。自分でアドレスを調べるプロキシや VPN は hosts ファイルを通りません。

## 軽さ

2026-11-24、Windows Server 2022、リリースビルドで測定：

| 項目 | 結果 |
| --- | --- |
| ウィンドウを閉じているときのプロセス数 | 0 |
| ウィンドウを開いているときのプロセッサー時間 | 5 秒間で 0～31 ミリ秒 |
| 起動からウィンドウが出るまで | 4 回の測定で 0.06～0.6 秒 |
| プログラムの大きさ | 4.4 MB |
| hosts ファイルの増加 | アドレス 9,800 件で 198 KB |

## リスト

リストはこのリポジトリに含まれません。手元のプログラムがそれぞれのサイトから取得し、リストごとに独自のライセンスがあります。

| リスト | 使う場所 | ライセンス |
| --- | --- | --- |
| [Peter Lowe](https://pgl.yoyo.org/adservers/) | 両方 | [McRae GPL](https://pgl.yoyo.org/license/) |
| [AdAway](https://adaway.org/) | 両方 | CC BY 3.0 |
| [RU AdList](https://github.com/easylist/ruadlist) | 両方 | CC BY 3.0 |
| [Liste FR](https://github.com/easylist/listefr) | 両方 | CC BY-SA 3.0 |
| [EasyList China](https://github.com/easylist/easylistchina) | 両方 | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [StevenBlack](https://github.com/StevenBlack/hosts) | プログラム | MIT |
| [EasyList](https://easylist.to/) | 拡張機能 | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Germany](https://github.com/easylist/easylistgermany) | 拡張機能 | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Hebrew](https://github.com/easylist/EasyListHebrew) | 拡張機能 | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Italy](https://github.com/easylist/easylistitaly) | 拡張機能 | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Spanish](https://github.com/easylist/easylistspanish) | 拡張機能 | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [KoreanList](https://github.com/easylist/KoreanList) | 拡張機能 | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [AdGuard Japanese](https://github.com/AdguardTeam/AdguardFilters) | 拡張機能 | GPL 3.0 |
| [Greek AdBlock Filter](https://github.com/kargig/greek-adblockplus-filter) | 拡張機能 | CC BY-SA 4.0 |

## 状況

| システム | 状況 |
| --- | --- |
| Windows | Windows Server 2022 で動作します。実際のリストを使い、hosts ファイルの控えで確認しました。Windows 10 と 11 ではまだ確認していません。 |
| macOS 10.13+ | ビルドとテストは通り、GitHub のマシンでインストールと起動ができます。まだ誰も手で試していません。 |
| Ubuntu 24.04+ | ビルドとテストは通り、GitHub のマシンでインストールと起動ができます。まだ誰も手で試していません。 |
| Firefox 128+ | Firefox 160 で動作します。Mozilla の署名がまだないため、Firefox を再起動すると読み込み直す必要があります。 |

## インストール

| システム | ファイル | インストール方法 |
| --- | --- | --- |
| Windows 10 と 11 | [adx0-windows.exe](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-windows.exe) | 実行します。Windows が警告したら「詳細情報」、「実行」を選びます。 |
| macOS 10.13+ | [adx0-macos.dmg](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-macos.dmg) | プログラムを「アプリケーション」にドラッグして開きます。macOS に拒否されたら、システム設定の「プライバシーとセキュリティ」で「このまま開く」を押します。 |
| Ubuntu 24.04+ | [adx0-ubuntu.deb](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-ubuntu.deb) | `sudo apt install ./adx0-ubuntu.deb` |
| Firefox 128+ | [adx0-firefox.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-firefox.zip) | `about:debugging` を開き、「この Firefox」、「一時的なアドオンを読み込む」、このファイルを選びます。Firefox を再起動したら読み込み直します。 |

GitHub はこのリポジトリのコードからこれらのファイルをビルドし、プログラムをインストールして起動し、そのうえですべてを公開します。ファイルには有料の署名がないため、Windows と macOS は初回起動時に警告します。

自分でビルドすることもできます。Node.js 24 以降と Rust、Windows ではさらに Microsoft C++ Build Tools、Ubuntu ではさらに `libwebkit2gtk-4.1-dev librsvg2-dev` が必要です。

```sh
git clone https://github.com/Ilia-Pero/adx0
cd adx0
npm ci
npm run tauri build
```

インストーラーは `src-tauri/target/release/bundle` にできます。

アンインストールすると、ブロックしたアドレスは元に戻ります。macOS では、プログラムを削除する前にブロックをオフにしてください。

拡張機能：`extension` フォルダーで `npm ci && npm run build` を実行し、Firefox で `about:debugging` を開き、「この Firefox」、「一時的なアドオンを読み込む」、ファイル `extension/web/manifest.json` を選びます。

## 翻訳

翻訳は機械によるもので、母語話者による確認は受けていません。間違いを見つけたら `locales`、`extension/web/_locales` または `docs` を直してください。

## 参加とライセンス

ビルド、テスト、変更の方法：[CONTRIBUTING.md](../CONTRIBUTING.md)。ライセンスは [MIT](../LICENSE) です。
