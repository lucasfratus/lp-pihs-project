---
author: Caetano V. Mantovani, Lucas Fratus, Vitor R. Machado
github: crowuete, lucasfratus, hito-boo
date: 2025-07-26
---
# THE WASM4 EXAM

THE WASM4 EXAM is an endless runner game inspired by the Google Chrome dinosaur game. It was developed as the final project for the Programming Languages and Hardware-Software Interface Programming courses.

The player must dodge obstacles like logs and meteors while collecting as many coins as possible. The character can jump and move sideways to survive. The goal is to collect as many coins as possible while surviving and beat your high-score!

A game written in Rust for the WASM-4 fantasy console.

## Controls
| Action        | Key/Button       |
|---------------|------------------|
| Move Left     | ← (Left Arrow)   |
| Move Right    | → (Right Arrow)  |
| Jump          | X                |
| Start/Restart | Z                |

## Building

Build the cart by running:
```
cargo build --release
```
Then run it with:
```
w4 run target/wasm32-unknown-unknown/release/cart.wasm
```

## License
This project is licensed under the MIT License.
