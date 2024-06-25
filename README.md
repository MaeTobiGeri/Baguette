# Baguette Compiler

Baguette is made as a fun and educational Rust-based compiler that turns custom "baguette" and "croissant" language files into pixel art images.

## Features

- Compiles `.baguette` and `.croissant` files into pixel art
- Generates a PNG output file
- Displays the result in a window using the `pixels` crate
- Supports custom colors and repetition commands

## Language Syntax

The Baguette language supports the following commands:

- `Croissant` or `croissant`: Draws a black pixel
- `Baguette` or `baguette`: Draws a brown pixel
- `Patisserie(r,g,b,a)`: Sets the background color
- `Boulangerie(n,type)`: Repeats a command `n` times
- Custom colors: `Croissant(r,g,b,a)` or `Baguette(r,g,b,a)`

## Requirements

- Rust
- Dependencies (see `Cargo.toml`):
  - `pixels`
  - `winit`
  - `image`

## Building

To build the project, run:
cargo build --release

## Usage

To use the Baguette compiler, run:
./Baguette <File Path>

## Contributing

Contributions, ideas, and feedback are welcome! Feel free to open issues or submit pull requests to improve the project.
