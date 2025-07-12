# Queens

This repository contains a Rust project designed to brute-force solve the LinkedIn Queens game.

The LinkedIn Queens game is a puzzle where you must place 8 queens on a chessboard-like grid. The catch is that the board is divided into colored regions, and you must place exactly one queen in each of the 8 regions. Like in chess, no two queens can attack each other, meaning they cannot share the same row or column.

## 8x8 Only
I created this repo believing that the board would always be 8x8, when in fact it is not. However, I hard-coded the size into a `u64`, and I'm currently too lazy to generalize it.
