## Rusty Minesweeper

![Ferris](/frontend/ferris.svg | width=110) ![WebAssembly](/frontend/WebAssembly_Logo.svg | width=55)

This is a simple minesweeper game written in Rust and brought to web with WebAssembly.

The game logic is written in Rust, compiled to WebAssembly and to Javascript bindings with `wasm-bindgen`. You can see the Rust code in [`src`](/src) directory and autogenerated WebAssembly code and Javascript bindings in [`pkg`](/pkg/README.md) directory.

The game is rendered with HTML mostly generated with simple vanilla Javascript code in [`/frontend/index.js`](/frontend//index.js) file and styled with CSS.

You can play the game [here](https://welf.github.io/rusty-minesweeper/).
