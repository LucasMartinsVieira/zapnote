<div align="center">
    <h1>Zapnote ⚡</h1>
    <p>Lightning-fast template-based note generator</p>
</div>

<img src="./.github/zapnote.webp"></img>

## About

Effective note-taking is essential for productivity and knowledge retention. For individuals with a terminal-based workflow, using a GUI application like [obsidian](https://obsidian.md) can be cumbersome for creating template-based notes. Zapnote adresses this by providing a lightning-fast, template-based note generation system that fits right into your command-line workflow.

## Instalation

The binary name for zapnote is `zn`.

### Cargo

You can install it using `cargo`.

`$ cargo install zapnote`

### Building

You can build yourself from source.

```sh
$ git clone https://github.com/LucasMartinsVieira/zapnote.git
$ cd zapnote
$ cargo build --release
$ ./target/release/zn --version # Copy the binary to your $PATH
```

## Usage

Zapnote has two commands for note creation: `note` (alias `n`) for regular notes, and `journal` (alias `j`) for journaling (not yet available).

```sh
$ zn note some_template some_file
```

## Configuration

To use zapnote, create a `zapnote.toml` config file in `$XDG_CONFIG_HOME/zapnote/` or `$HOME/.config/zapnote`. Run zapnote once to auto-generate or manually create it with the contents of [default-zapnote.toml](./resources/default-zapnote.toml).

```sh
$ zn n . .

# or

$ mkdir -p ~/.config/zapnote/ && touch ~/.config/zapnote/zapnote.toml
```
