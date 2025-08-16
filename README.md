# Mini WASM Othello

A simple Othello (Reversi) game built with Rust and WebAssembly.

## 🎮 Play Online

**[Play the game here: https://rintarop.github.io/mini-wasm-othello/](https://rintarop.github.io/mini-wasm-othello/)**

## Overview

This project demonstrates the power of WebAssembly by implementing an Othello game where the core game logic is written in Rust and compiled to WebAssembly for browser execution. The game features an AI opponent and a responsive design that works on both desktop and mobile devices.

## Features

- Game logic implemented in Rust for performance and safety
- WebAssembly integration for seamless browser execution
- **Advanced AI opponent with 4 difficulty levels**:
  - **Easy**: Greedy algorithm (fast, beginner-friendly)
  - **Medium**: Minimax algorithm with 3-move lookahead
  - **Hard**: Alpha-beta pruning with 5-move lookahead
  - **Expert**: Alpha-beta pruning with 7-move lookahead + advanced evaluation
- **Dynamic AI difficulty selection** during gameplay
- Responsive design optimized for mobile devices
- Clean, modern UI with HTML5 Canvas rendering
- Player vs AI gameplay with switchable turn order

## Getting Started

### Prerequisites

Make sure you have the following tools installed:

1. **Rust** - Install from [https://rustup.rs/](https://rustup.rs/)
2. **wasm-pack** - Install from [https://rustwasm.github.io/wasm-pack/installer/](https://rustwasm.github.io/wasm-pack/installer/)
3. **Python 3** - For the development server

### Installation & Setup

1. Clone this repository:
   ```bash
   git clone https://github.com/rintarop/mini-wasm-othello.git
   cd mini-wasm-othello
   ```

2. Build the WebAssembly module:
   ```bash
   npm run build
   ```

3. Start the development server:
   ```bash
   npm run serve
   ```

4. Open your browser and navigate to `http://localhost:8000`

### Using VS Code Tasks

If you're using VS Code, you can use the predefined tasks:

- **Build WebAssembly**: `Ctrl/Cmd + Shift + P` → "Tasks: Run Task" → "Build WebAssembly"
- **Serve Development Server**: `Ctrl/Cmd + Shift + P` → "Tasks: Run Task" → "Serve Development Server"

## How to Play

1. The game starts with the black player (human player by default)
2. Click on an empty square to place your stone
3. You must place stones to capture opponent stones by flanking them
4. Players alternate turns
5. The game ends when no valid moves are available
6. The player with the most stones wins

### Game Controls

- **新しいゲーム (New Game)**: Start a fresh game
- **先攻・後攻を変更 (Switch Turn Order)**: Toggle between playing as black (first) or white (second)
- **AI難易度 (AI Difficulty)**: Select from 4 AI difficulty levels (Easy, Medium, Hard, Expert)

## Technology Stack

### Core Technologies
- **Rust** - Game logic implementation with memory safety and performance
- **WebAssembly (WASM)** - Compiling Rust to run in the browser
- **wasm-bindgen** - Rust and WebAssembly integration with JavaScript

### AI Algorithms
- **Greedy Algorithm** - Fast, position-based evaluation
- **Minimax Algorithm** - Game theory optimal play with lookahead
- **Alpha-Beta Pruning** - Optimized minimax with branch cutting
- **Advanced Evaluation Function** - Multi-factor board position evaluation

### Frontend
- **HTML5 Canvas** - Game board rendering and user interaction
- **JavaScript (ES6 Modules)** - WebAssembly integration and DOM manipulation
- **CSS3** - Responsive styling with mobile-first design
- **Flexbox** - Layout management

### Development Tools
- **wasm-pack** - Building and packaging WebAssembly modules
- **Python HTTP Server** - Development server for local testing

## Building for Production

To build the project for production deployment:

1. Build the WebAssembly module:
   ```bash
   wasm-pack build --target web --out-dir pkg --release
   ```

2. The generated files in the `pkg/` directory along with `index.html`, `styles.css`, and `script.js` can be deployed to any static web server.

## Project Structure

```
mini-wasm-othello/
├── src/
│   └── lib.rs              # Rust game logic and WebAssembly bindings
├── assets/                 # Static assets
│   ├── favicon.svg         # Main favicon
│   ├── favicon-16x16.svg   # Small favicon
│   ├── favicon-32x32.svg   # Standard favicon
│   └── apple-touch-icon.svg # iOS home screen icon
├── pkg/                    # Generated WebAssembly files
├── index.html             # Main HTML file
├── styles.css             # CSS styles and responsive design
├── script.js              # JavaScript game controller
├── Cargo.toml             # Rust dependencies and project config
└── README.md              # This file
```

## Browser Compatibility

This game works in all modern browsers that support:
- WebAssembly
- ES6 Modules
- HTML5 Canvas
- CSS3 Flexbox

## License

MIT License - see the LICENSE file for details.
