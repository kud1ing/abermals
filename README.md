# Abermals

ASCII/ANSI style retro graphics in Rust with SVG and WebAssembly.

<img width="545" alt="Screenshot" src="https://user-images.githubusercontent.com/391975/112140652-91c70900-8bd4-11eb-83bf-f1bec2399b9d.png">

Why SVG? Canvas looks often blurry.

The produced SVGs are not that wasteful, since symbols are reused via `<symbol>` and `<use>`.
