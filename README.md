# Introduction
Astra is a WIP editor for FE: Engage game data. This covers:
* Core data like character, class, and item stats.
* Chapter data like spawn locations and scripts.
* Text data like cutscene dialogue.

**Supports FE: Engage 2.0+ ONLY.**

Note that Astra does **NOT** manage:
* Textures. Astra will *render* textures, but it is not a tool for editing them.
* 3D models, shaders, etc.
* Audio.

## Installation
Download the latest version from the releases page and extract.

Release binaries are only provided for windows. Mac and Linux users, please proceed to the next section if you want to compile it yourself.

## Building
Astra requires an up to date installation of [Rust](https://www.rust-lang.org/). You should also install [git](https://git-scm.com/).

1. Clone this repository from a terminal. This repository uses submodules, so you should include the recursive option ex. `git clone --recursive https://github.com/thane98/Astra`
2. Enter the project directory (`cd Astra`).
3. Build Astra in release mode (`cargo build --release`). Alternatively, run Astra directly using (`cargo run --release`)
4. After building, you can find the compiled binary under `target/release/astra.exe` for Windows or `target/release/astra` for Mac and Linux.

## Credits
* [Raytwo](https://github.com/DeathChaos25): Help at various stages + [Cobalt](https://github.com/Raytwo/Cobalt).
* [DeathChaos](https://github.com/DeathChaos25): Help at various stages.
* [AraragiHoozuki](https://github.com/AraragiHoozuki): Documentation for enums and bit flags which was incorporated into several editors.
* [Perfare](https://github.com/Perfare): [AssetStudio](https://github.com/Perfare/AssetStudio) which was instrumental to writing the low level component of this editor, [astra_formats](https://github.com/thane98/astra-formats).

## License
Astra uses the [MIT license](https://en.wikipedia.org/wiki/MIT_License). You may find a copy of it in this repository.
