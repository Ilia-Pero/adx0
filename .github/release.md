
## Files

- `adx0-windows.exe`: Windows 10 and 11. If Windows does not know the program, choose “More info” and “Run anyway”.
- `adx0-macos.dmg`: macOS 10.13 and newer, Intel and Apple Silicon. Drag the program into Applications and open it. When macOS refuses, open System Settings, “Privacy & Security”, and press “Open Anyway”.
- `adx0-ubuntu.deb`: Ubuntu 24.04 and newer, 64-bit Intel and AMD. `sudo apt install ./adx0-ubuntu.deb`
- `adx0-firefox.zip`: the extension for Firefox 128 and newer. Open `about:debugging`, “This Firefox”, “Load Temporary Add-on”, and choose the file. Mozilla has not signed it, so load it again after Firefox is restarted.
- `adx0-chrome.zip`: the extension for Chrome, Yandex Browser, Edge and Opera, Chromium 120 and newer. Unpack it, open `chrome://extensions`, turn on “Developer mode”, press “Load unpacked” and choose the folder. Yandex Browser trusts only catalogs and may turn it off.

GitHub has built these files from the code of this tag, installed the three programs and started them. They have no paid signature, so Windows and macOS warn at the first start. Nobody has tried the program by hand on Windows 10 and 11, on a Mac or on Ubuntu yet: reports are welcome.
