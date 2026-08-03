# Ultimate Tic Tac Toe

A web-based Ultimate Tic Tac Toe game built with Leptos and Rust, compiled to WebAssembly.

## What is Ultimate Tic Tac Toe?

Ultimate Tic Tac Toe is a more complex variant of the classic game. The game consists of 9 small tic-tac-toe boards arranged in a 3x3 grid. To win, you need to win three small boards in a row (horizontally, vertically, or diagonally).

The twist: **Your move determines which board your opponent must play on next!** If you play in the top-right cell of a small board, your opponent must play in the top-right small board.

## Setup

### Prerequisites

1. Install Rust: https://rustup.rs/
2. Add the WASM target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
3. Install Trunk (build tool for Rust WASM apps):
   ```bash
   cargo install trunk
   ```

## Running the Game

From the project directory, run:

```bash
trunk serve --open
```

This will:
- Compile your Rust code to WebAssembly
- Start a local development server
- Open the game in your browser (usually at http://127.0.0.1:8080)

## Building for Production

To create an optimized build:

```bash
   trunk build --release
```

The output will be in the `dist/` directory, which you can deploy to any static web host.

## Game Rules

1. Players take turns placing X or O in small boards
2. Your move determines which small board your opponent plays next
3. If you send your opponent to a board that's already won or full, they can play anywhere
4. Win three small boards in a row to win the game!

## Features

- 🎮 Full Ultimate Tic Tac Toe gameplay
- 🎨 Beautiful gradient UI with animations
- 🟢 Visual indicators for playable boards
- 📱 Responsive design
- ⚡ Fast WASM performance
- 🔄 New game button to reset

Enjoy the game!
