<h1 align="center">Rhino Setup</h1>

<!-- Social -->
<p align="center">
    <a href="https://discord.gg/uhdCz8vwV9"><img alt="join discord" src="https://img.shields.io/badge/Discord-%237289DA.svg?style=for-the-badge&logo=discord&logoColor=white"/></a>
    <a href="https://matrix.to/#/#rolling-rhino-remix:matrix.org"><img alt="join matrix" src="https://img.shields.io/badge/matrix-000000?style=for-the-badge&logo=Matrix&logoColor=white"/></a>
    <br/>
    <a href="https://ko-fi.com/rhinolinux"><img alt="donate" width="205" src="https://user-images.githubusercontent.com/58742515/225977527-99938108-f434-4bdc-8cd0-7648f5c06148.png"/></a>
    <br/>
    <a href="https://www.reddit.com/r/rhinolinux/"><img alt="join subreddit" src="https://img.shields.io/badge/Reddit-FF4500?style=for-the-badge&logo=reddit&logoColor=white"/></a>
    <a href="https://www.youtube.com/channel/UCLUw8_PTMXLMJ-Hz6_7LNVQ"><img alt="subscribe to youtube" src="https://img.shields.io/badge/YouTube-FF0000?style=for-the-badge&logo=youtube&logoColor=white"/></a>
</p>

<p align="center">Setup wizard for <a href="https://rhinolinux.org/">Rhino Linux</a></p>

## 🔱 Info

Setup wizard for [Rhino Linux](https://rollinglinux.org/) written in Rust. Inspired by [VanillaOS's setup wizard](https://github.com/Vanilla-OS/first-setup)

## 🌊 Features

+ Customize your theme
+ Manage your package managers
+ Configure crash reporting

## ⚙️ Building

Install the following dependencies:

* `libgtk-4-dev`
* `libadwaita-1-dev`
* `gettext`
* `desktop-file-utils`
* `rustc`
* `cargo`
* `meson`
* `ninja-build`

Run the following commands:

* `meson build` or `meson -D profile=development build`
* `ninja -C build install`

## 🗣️ Translation Status

<div align="center">
    <a href="https://hosted.weblate.org/engage/rhino-linux/">
        <img src="https://hosted.weblate.org/widgets/rhino-linux/-/rhino-setup/horizontal-auto.svg" alt="Translation status" />
    </a>
</div>

## 📜 License

<p align="center"><img alt="GPL-3.0-or-later" height="100" src="https://www.gnu.org/graphics/gplv3-or-later.svg" /></p>

```monospace
Copyright (C) 2022-present

This file is part of Rhino Setup.

Rhino Setup is free software: you can redistribute it and/or modify it under the
terms of the GNU General Public License as published by the Free Software
Foundation, either version 3 of the License, or (at your option) any later
version.

Rhino Setup is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with
Rhino Setup. If not, see <https://www.gnu.org/licenses/>.
```
