# fsd_builder
Army builder for Full Spectrum Dominance
you can find more about the game on https://fsd-wargame.com/ .

## Installation
To get started, you need the following:
* WebAssembly target: install with `rustup target add wasm32-unknown-unknown`
* Trunk: install with `cargo install --locked trunk`. This will take _a while_.

## Serving
Trunk should be doing all the heavy lifting here. The index.html is the entry point (and for silly reasons has to be kept in the root directory), and a few directions are provided in the trunk.toml file. Other than that, calling `trunk build` and `trunk serve` (or, if you're feeling fancy, `trunk serve --open`) should set up the website on localhost:8080.
The intermediate files will be copied in a dist folder (which is in .gitignore) before serving.

## Features 
This army builder is a work in progress, and progressively multiple features will be included
* Creating lists for each faction (plain list of Units, Supports and Characters)
* Assigning Characters and Units' option
* Building a printable roster sheet for the faction
* Custom Units (?)


## License
This project is dual-licensed:

- The software code is licensed under the [MIT license](LICENSE-MIT).
- The unit profiles are proprietary and All Rights Reserved. 
