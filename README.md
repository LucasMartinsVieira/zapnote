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

Zapnote has two commands for note creation: `note` (alias `n`) for regular notes, and `journal` (alias `j`) for journal entries.

```sh
$ zn note some_template some_file
$ zn journal day
$ zn journal day --date 2026-04-19
$ zn journal week --date 2026-W1
$ zn journal day --offset -1 day
```

## Templates

Zapnote keeps the `{{placeholder}}` syntax and supports both the original format and the new key/value form.

```md
# {{date format="%A, %-d %B %Y"}}

<< [[{{date offset="-1 day" format="%Y-%m-%d"}}]] | [[{{date offset="+1 day" format="%Y-%m-%d"}}]] >>

Week: [[{{date format="%G-W%V"}}]]
Last Year: [[{{date offset="-1 year" format="%Y-%m-%d"}}]]
Next Year: [[{{date offset="+1 year" format="%Y-%m-%d"}}]]
```

Supported placeholders:

- `{{title}}`
- `{{date}}`
- `{{date:%Y-%m-%d}}`
- `{{date format="..."}}`
- `{{date offset="-1 day" format="..."}}`
- `{{time}}`
- `{{time:%H:%M}}`
- `{{time format="..."}}`

Journal `--date` input is format-independent. The journal config still controls the output filename, but `--date` can use any supported anchor shape:

- `%Y-%m-%d` -> `YYYY-MM-DD`
- `%G-W%V` -> `YYYY-W01` and `YYYY-W1`
- `%Y-Q%Q` -> `YYYY-Q1` through `YYYY-Q4`
- `%Y-%m` -> `YYYY-MM`
- `%Y` -> `YYYY`

`--offset` accepts `day`, `week`, `quarter`, `month`, and `year` units.

## Configuration

To use zapnote, create a `zapnote.toml` config file in `$XDG_CONFIG_HOME/zapnote/` or `$HOME/.config/zapnote`. Run zapnote once to auto-generate or manually create it with the contents of [default-zapnote.toml](./resources/default-zapnote.toml).

```sh
$ zn n . .

# or

$ mkdir -p ~/.config/zapnote/ && touch ~/.config/zapnote/zapnote.toml
```
