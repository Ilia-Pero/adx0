<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">ad×0</h1>

<p align="center"><b>ad × 0 = 0</b></p>

<p align="center">适用于 Windows、macOS、Linux 和浏览器的广告屏蔽工具</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<a href="https://github.com/Ilia-Pero/adx0/releases/latest"><img src="https://img.shields.io/github/v/release/Ilia-Pero/adx0?label=download&color=0f766e" alt="download"></a>
<img src="https://img.shields.io/badge/languages-14-0f766e" alt="14 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · **简体中文** · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md) · [Беларуская](README.be.md) · [Қазақша](README.kk.md)

<p align="center">
<img src="window-off.png" width="380" alt="程序窗口：屏蔽已关闭">
<img src="window-on.png" width="380" alt="程序窗口：广告已屏蔽">
</p>

ad×0 将广告服务器的地址写入系统的 hosts 文件，然后退出。此后电脑上的程序不再从这些地址加载任何内容，ad×0 本身也没有运行中的进程。浏览器扩展会隐藏网页上的广告块，并去除 YouTube 视频中的广告。

## 安装

| 系统 | 文件 | 安装方法 |
| --- | --- | --- |
| Windows 10, 11 | [adx0-windows.exe](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-windows.exe) | 运行文件。如果 Windows 显示警告，请选择“更多信息”，再选择“仍要运行”。 |
| macOS 10.13+ | [adx0-macos.dmg](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-macos.dmg) | 将程序拖到“应用程序”并打开。macOS 首次拒绝后，打开系统设置中的“隐私与安全性”，点击“仍要打开”。 |
| Ubuntu 24.04+ | [adx0-ubuntu.deb](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-ubuntu.deb) | `sudo apt install ./adx0-ubuntu.deb` |
| Firefox 128+ | [adx0-firefox.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-firefox.zip) | 打开 `about:debugging`，选择“此 Firefox”、“临时载入附加组件”，然后选择文件。重启 Firefox 后需要重新载入。 |
| Chrome, Edge, Opera, Yandex Browser | [adx0-chrome.zip](https://github.com/Ilia-Pero/adx0/releases/latest/download/adx0-chrome.zip) | 将压缩包解压到会保留的文件夹。打开 `chrome://extensions`，开启开发者模式，点击“加载已解压的扩展程序”并选择该文件夹。 |

这些文件没有付费签名，因此 Windows 和 macOS 会显示警告。GitHub 从本仓库构建文件，在干净的机器上安装并启动程序，之后才发布版本。

卸载程序时会从 hosts 文件中删除被屏蔽的地址。在 macOS 上，请先关闭屏蔽再删除程序。

## 程序

- 屏蔽约 10,000 个广告和跟踪服务器地址。列表由您选择，也可以添加自己的网站。
- 不改动您在 hosts 文件中的内容，首次修改前保留副本，写入后检查文件。
- 系统启动后不久在无窗口的情况下更新列表，每天最多一次。可以关闭。
- 14 种语言。

hosts 文件按整个地址屏蔽，因此与内容来自同一地址的广告会保留。通过代理或 VPN 自行解析地址的程序会绕过 hosts 文件。

## 浏览器扩展

<p align="center"><img src="popup.png" width="312" alt="扩展窗口"></p>

- 屏蔽约 64,000 个地址，并隐藏网页上的广告块。
- 从 YouTube 的响应中去除广告安排。
- 一键允许某个网站的广告。
- 每天更新一次列表。

## 数据与网络

程序和扩展从各列表的网站下载列表，不收集数据。点击“检查更新”时，程序会向 GitHub 查询新版本。

## 更多信息

列表及其许可证、测量数据、已测试的系统和构建方法见[英文文档](../README.md)。

本文为机器翻译，未经母语者校对。更正请提交到 `docs` 文件夹。

[MIT](../LICENSE)
