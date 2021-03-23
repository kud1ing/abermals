# Abermals

ASCII/ANSI style retro graphics in Rust with SVG and WebAssembly.

<img width="545" alt="Screenshot" src="https://user-images.githubusercontent.com/391975/112140652-91c70900-8bd4-11eb-83bf-f1bec2399b9d.png">


## Instructions

* make sure that [Rust is installed](https://www.rust-lang.org/tools/install)
* make sure that [`wasm-pack` is installed](https://rustwasm.github.io/wasm-pack/installer/)
* execute `wasm-pack build --target web`
* open `index.html`


## Why SVG?

Canvas often looks blurry. See the [MegaZeux](https://www.digitalmzx.com) web player
or games made with [rot.js](https://github.com/ondras/rot.js/), e.g. [Funhouse](http://kevinw.github.io/funhouse/).

Abermals produces SVGs that are not too wasteful, since symbols are reused via `<symbol>` and `<use>`.
