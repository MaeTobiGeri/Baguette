# Baguette Compiler

Baguette was made as a fun side project. It's a rust-based compiler that turns custom "baguette" and "croissant" language files into pixel art images. Only tested on Linux should also work on Windows though.

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

First clone the repo using: 
git clone https://github.com/MaeTobiGeri/Baguette.git

Then cd into the Baguette folder using:
cd Baguette

and build the project by running:
cargo build --release

## Usage

To use the Baguette compiler, run:
./Baguette File Path

## Contributing

Contributions, ideas, and feedback are welcome! Feel free to open issues or submit pull requests to improve the project.
