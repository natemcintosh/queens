# Queens

This repository contains a Rust project designed to brute-force solve the LinkedIn Queens game.

The LinkedIn Queens game is a puzzle where you must place 8 queens on a chessboard-like grid. The catch is that the board is divided into colored regions, and you must place exactly one queen in each of the 8 regions. Like in chess, no two queens can attack each other, meaning they cannot share the same row or column.

## 8x8 Only
I created this repo believing that the board would always be 8x8, when in fact it is not. However, I hard-coded the size into a `u64`, and I'm currently too lazy to generalize it.

## Running
1. Clone the repo
1. Build with `cargo build --release`
1. Get the help message with `./target/release/queens -h`
1. Enter an 8x8 board at the command line, as described by the help message.

While there may be several million positions to check, I've never seen it take longer than 40ms (on my computer) to find the solution.
