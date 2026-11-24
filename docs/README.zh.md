<p align="center"><img src="../src-tauri/icons/128x128.png" width="96" height="96" alt=""></p>

<h1 align="center">ad×0</h1>

<p align="center"><b>ad × 0 = 0</b></p>

<p align="center">不在后台运行的广告屏蔽工具，支持 Windows、macOS、Linux 和 Firefox</p>

<p align="center">
<a href="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml"><img src="https://github.com/Ilia-Pero/adx0/actions/workflows/build.yml/badge.svg" alt="build"></a>
<a href="../LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" alt="MIT"></a>
<img src="https://img.shields.io/badge/languages-12-0f766e" alt="12 languages">
</p>

[English](../README.md) · [Русский](README.ru.md) · [Español](README.es.md) · [Ελληνικά](README.el.md) · [עברית](README.he.md) · **简体中文** · [日本語](README.ja.md) · [한국어](README.ko.md) · [Српски](README.sr.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Italiano](README.it.md)

<p align="center">
<img src="window-off.png" width="380" alt="程序窗口：屏蔽已关闭">
<img src="window-on.png" width="380" alt="程序窗口：广告已屏蔽">
</p>

ad×0 把广告服务器的地址写入系统的 hosts 文件，然后退出。所有程序中来自这些地址的广告都不再加载，也没有后台进程占用内存或处理器。Firefox 扩展负责 hosts 文件做不到的事：隐藏网页上的广告块，并去掉 YouTube 视频中的广告。

## 程序

- 根据您选择的列表屏蔽约 10,000 个广告服务器和跟踪器的地址，加上中文列表约 15,400 个。
- 保留 hosts 文件中您自己的内容，首次修改前留下副本，并重新读取写入的内容。
- 保存每个列表最后一份有效副本，屏蔽不依赖列表网站是否可用。
- 列表中的错误不会关闭 Google、Wikipedia 或 GitHub 等网站（[完整清单](../data/essential.json)）。其他网站可在“不屏蔽”中放行。
- 支持 12 种语言，可在窗口顶部和 Windows 安装程序中选择。不向任何地方发送内容，只在您按下按钮时下载列表。

## Firefox 扩展

<p align="center"><img src="popup.png" width="312" alt="Firefox 扩展"></p>

- 屏蔽广告地址并隐藏网页上的广告块。区域列表按浏览器语言选择。
- 从 YouTube 的响应中去掉广告安排，播放器便没有广告可放。
- 每天更新一次列表。规则由浏览器自身执行，其间后台部分不会加载。
- 同样支持这 12 种语言：默认使用浏览器的语言，可在扩展的窗口中更换。

## 做不到的事

hosts 文件只能处理整个地址。与内容来自同一地址的广告（例如 YouTube 或社交网络）会保留；YouTube 可使用扩展。自行解析地址的代理或 VPN 会绕过 hosts 文件。

## 有多轻

2026-11-24 在 Windows Server 2022 上用发布版测得：

| 项目 | 结果 |
| --- | --- |
| 窗口关闭时的进程数 | 0 |
| 窗口打开时的处理器占用 | 5 秒内 0–31 毫秒 |
| 从启动到窗口出现 | 四次测得 0.06–0.6 秒 |
| 程序大小 | 4.4 MB |
| hosts 文件的增量 | 9,800 个地址为 198 KB，加上中文列表后 15,400 个地址为 284 KB |

## 列表

列表不属于本仓库。您的程序从各列表的网站下载它们，每个列表都有自己的许可证。

| 列表 | 用于 | 许可证 |
| --- | --- | --- |
| [Peter Lowe](https://pgl.yoyo.org/adservers/) | 两者 | [McRae GPL](https://pgl.yoyo.org/license/) |
| [AdAway](https://adaway.org/) | 两者 | CC BY 3.0 |
| [RU AdList](https://github.com/easylist/ruadlist) | 两者 | CC BY 3.0 |
| [Liste FR](https://github.com/easylist/listefr) | 两者 | CC BY-SA 3.0 |
| [EasyList China](https://github.com/easylist/easylistchina) | 两者 | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [StevenBlack](https://github.com/StevenBlack/hosts) | 程序 | MIT |
| [EasyList](https://easylist.to/) | 扩展 | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Germany](https://github.com/easylist/easylistgermany) | 扩展 | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Hebrew](https://github.com/easylist/EasyListHebrew) | 扩展 | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Italy](https://github.com/easylist/easylistitaly) | 扩展 | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [EasyList Spanish](https://github.com/easylist/easylistspanish) | 扩展 | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [KoreanList](https://github.com/easylist/KoreanList) | 扩展 | [GPL 3.0 / CC BY-SA 3.0](https://easylist.to/pages/licence.html) |
| [AdGuard Japanese](https://github.com/AdguardTeam/AdguardFilters) | 扩展 | GPL 3.0 |
| [Greek AdBlock Filter](https://github.com/kargig/greek-adblockplus-filter) | 扩展 | CC BY-SA 4.0 |

## 状态

| 系统 | 状态 |
| --- | --- |
| Windows | 在 Windows Server 2022 上可以使用：已用真实列表在 hosts 文件的副本上检查。尚未在 Windows 10 和 11 上检查。 |
| macOS 10.13+ | 可以构建并通过测试。还没有人运行过。 |
| Ubuntu 24.04+ | 可以构建并通过测试。还没有人运行过。 |
| Firefox 128+ | 在 Firefox 160 中可用。尚未经 Mozilla 签名，重启 Firefox 后需要重新加载。 |

## 安装

目前还没有现成的下载文件，需要自行构建。需要 Node.js 24 或更新版本和 Rust；Windows 还需要 Microsoft C++ Build Tools；Ubuntu 还需要 `libwebkit2gtk-4.1-dev librsvg2-dev`。

```sh
git clone https://github.com/Ilia-Pero/adx0
cd adx0
npm ci
npm run tauri build
```

安装程序位于 `src-tauri/target/release/bundle`。它没有签名，因此 Windows 和 macOS 会询问您是否信任它。

卸载程序会重新放行被屏蔽的地址。在 macOS 上，请在删除程序之前关闭屏蔽。

扩展：在 `extension` 文件夹中运行 `npm ci && npm run build`，在 Firefox 中打开 `about:debugging`，选择“此 Firefox”、“临时载入附加组件”和文件 `extension/web/manifest.json`。

## 翻译

译文由机器生成，未经母语者检查。如果发现错误，请在 `locales`、`extension/web/_locales` 或 `docs` 中修正。

## 参与和许可证

如何构建、测试和修改程序：[CONTRIBUTING.md](../CONTRIBUTING.md)。许可证为 [MIT](../LICENSE)。
