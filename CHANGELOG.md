# Changes

## 0.4.1 (2026-11-25)

Fixed:

- The extension stopped updating its lists after Firefox was restarted.
- If the disk was full, the hosts file could be left half-written. Now its previous contents are restored.
- On Windows the lists were not updated while a laptop ran on battery power.
- Turning blocking on and off showed a console window for an instant on Windows.

Security:

- On macOS and Linux the part with administrator permission copied the lists of the user as they were. If a list had been replaced with a link to a protected file, the contents of that file became readable to everyone. Now only site names are copied.
- The background part of the extension accepts commands only from the window of the extension.

Changed:

- Russian and Belarusian texts.

## 0.4.0 (2026-11-25)

- “Check for updates” at the bottom of the window.
- The extension for Chrome, Edge, Opera and Yandex Browser.
- Dates and numbers in Belarusian and Kazakh on systems that do not support these languages.
- Better contrast of controls in both themes; scroll bars follow the dark theme.

## 0.3.0 (2026-11-25)

- Kazakh and Belarusian, 14 languages in total. The Windows installer does not support Kazakh; choose it at the top of the window.

## 0.2.0 (2026-11-25)

- While blocking is on, the lists are updated at most once a day after the system starts. The program does this without a window and exits. This can be turned off.

## 0.1.1 (2026-11-25)

- The publisher in Windows and Ubuntu is Ilia-Pero. In 0.1.0 it was “github” by mistake.

## 0.1.0 (2026-11-25)

- The program: lists of ad addresses in the hosts file, your own sites to block and to allow.
- The Firefox extension: blocked addresses, hidden ad blocks, YouTube without ads.
- Regional lists.
- 12 languages.
