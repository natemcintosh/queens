# Queens

This repository contains a Rust project designed to brute-force solve the LinkedIn Queens game.

The LinkedIn Queens game is a puzzle where you must place queens on a chessboard-like grid. The catch is that the board is divided into colored regions, and you must place exactly one queen in each of the regions. Like in chess, no two queens can attack each other, meaning they cannot share the same row or column.


## Running
1. Clone the repo
1. Build with `cargo build --release`
1. Get the help message with `./target/release/queens -h`
1. Enter a board at the command line, as described by the help message.

While there may be several million positions to check, I've never seen it take longer than 40ms (on my computer) to find the solution.
