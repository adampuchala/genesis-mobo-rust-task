# Snake HTTP API - Rust Language Recruitment Task
Recruitment task for GenesisMobo

The project sources are two crates - `snake-core` and `snake-webapp`.

The `snake-core` crate is an implementation of snake game.

The `snake-webapp` crate is an implementation of snake game web api. 
The algorithm of next move selection is implemented in this crate.

## Prerequisities:
- Rust toolchain version 1.62.1
- Cargo
  
## Installation and run:
Execute `cargo run` command in project subdirectory `snake-webapp`.
The server is listening on `http://localhost:8080` by default.

## Configuration:
`actix-web` server configuration file location is `snake-webapp/Configuration.toml`.

## Tests:
- Web app: Execute `cargo test` command in project subdirectory `snake-webapp`.
- Snake core: Execute `cargo test` command in project subdirectory `snake-code`.

## Other info:
Development toolchain version: 
stable-x86_64-pc-windows-msvc (default)
rustc 1.62.1 (e092d0b6b 2022-07-16)
