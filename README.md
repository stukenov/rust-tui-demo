# Rust TUI Demo Application

A simple terminal user interface (TUI) application built with Rust using the Ratatui library. This project demonstrates interactive terminal UI development with features like input handling, counter management, and message history.

## Features

- **Interactive Counter**: Increment, decrement, and reset a counter
- **Name Input**: Enter and update your name with a dedicated input mode
- **Message History**: View a scrollable history of all actions
- **Progress Gauge**: Visual progress bar displayed when counter is positive
- **Keyboard Navigation**: Full keyboard control with intuitive commands

## Screenshots

The application displays:
- A header with the application title
- User information panel showing name and current counter value
- Controls panel listing available keyboard commands
- Message history panel showing all actions taken
- Optional progress gauge overlay for counter values

## Prerequisites

- Rust 1.70 or higher
- A terminal emulator that supports ANSI escape codes

## Installation

1. Clone the repository:
```bash
git clone https://github.com/stukenov/rust-tui-demo.git
cd rust-tui-demo
```

2. Build the project:
```bash
cargo build --release
```

## Usage

Run the application:
```bash
cargo run
```

### Keyboard Commands

- `n` - Enter name input mode
- `+` - Increment counter
- `-` - Decrement counter
- `r` - Reset counter to zero
- `c` - Clear message history
- `q` - Quit application

**In input mode:**
- Type characters to enter your name
- `Enter` - Confirm name entry
- `Esc` - Cancel input mode
- `Backspace` - Delete last character

## Dependencies

- [ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI library
- [crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal manipulation

## Project Structure

```
.
├── src/
│   └── main.rs          # Main application code
├── Cargo.toml           # Project configuration
├── LICENSE              # MIT License
└── README.md            # This file
```

## Technical Details

The application uses:
- **Ratatui** for rendering the terminal UI with widgets (Block, Paragraph, List, Gauge)
- **Crossterm** for terminal control and event handling
- **Event-driven architecture** for responsive keyboard input
- **Layout system** with vertical constraints for organizing UI components

## License

MIT License - Copyright (c) 2025 Saken Tukenov

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## Acknowledgments

Built with [Ratatui](https://ratatui.rs/), a powerful Rust library for building terminal user interfaces.
