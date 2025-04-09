### Issues Tracker

To report issues or propose new features for this repository, visit [our tracker](https://github.com/rhino-linux/tracker).

<h1 align="center">Rhino Setup</h1>

<!-- Social -->
<p align="center">
    <a href="https://discord.gg/uhdCz8vwV9"><img alt="join discord" src="https://img.shields.io/badge/Discord-%237289DA.svg?style=for-the-badge&logo=discord&logoColor=white"/></a>
    <a href="https://ko-fi.com/rhinolinux"> <img alt="donate" src="https://img.shields.io/badge/Kofi-72A5F2.svg?style=for-the-badge&logo=kofi&logoColor=white"/></a>
    <br/>
    <a href="https://opencollective.com/rhino-linux-and-pacstall"><img alt="donate" width="205" src="https://github.com/user-attachments/assets/8e2dda76-750b-418e-9fca-b232d07a6335"/></a>
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

### How you can help
* Work on translations into languages not finished yet by either editing the `po/<language_code>.po` file, making a new one by running `cp po/rhino-setup.pot po/<language_code>.po`, or using weblate (https://hosted.weblate.org/projects/rhino-linux/rhino-setup/). Once you have completed or partially completed a po file, make a PR and we will merge it! Our goal is to have as many languages translated as possible due to the amount of people who may not be fluent in English.

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
