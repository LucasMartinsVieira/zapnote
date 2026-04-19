# CLAUDE.md

This file provides guidance to work with code in this repository.

## Commands

```bash
just test        # run all tests
just fmt         # format Rust + Nix files
just lint        # cargo check + fmt check + clippy + nixfmt check
just lint-all    # lint + msrv checks
just msrv        # check minimum supported Rust version (1.74.1)
```

Single test:
```bash
cargo test utils::tests::test_quarter_from_week
```

## Architecture

`zn` is a CLI note generator. User picks a template, provides a title; tool substitutes placeholders and opens the file in `$EDITOR`.

**Data flow:**
1. CLI parse (`clap`) → template name + note title
2. Config load from `~/.config/zapnote/zapnote.toml`
3. Case conversion on filename (`CaseStyle`: camel/kebab/pascal/snake/original)
4. Validate template exists, note doesn't already exist
5. Placeholder substitution in template content (`{{title}}`, legacy `{{date[:fmt]}}` / `{{time[:fmt]}}`, plus key/value forms like `{{date offset="-1 day" format="%Y-%m-%d"}}`)
6. Write to note folder, exec `$EDITOR`

**Two commands:**
- `zn note <template> <name...>` — create a regular note
- `zn journal <name> [--date <anchor>] [--offset <delta>]` — create a journal entry (uses `JournalConfig` for output format/folder)

**Key modules:**
- `src/main.rs` — entry point, config load, routes to `note.rs` or `journal.rs`
- `src/cli.rs` — clap CLI definitions
- `src/config.rs` — TOML config structs (`Config`, `GeneralConfig`, `JournalConfig`, `CaseStyle`)
- `src/note.rs` / `src/journal.rs` — command handlers
- `src/utils/mod.rs` — path helpers, `run_editor()`, duplicate-path checks, quarter helper
- `src/utils/date.rs` — shared date parsing, formatting, `%Q` replacement, and offset arithmetic for `--date`, `--offset`, and placeholders
- `src/utils/template.rs` — template file I/O (`check_template`, `load_template`, `insert_template_to_file`)
- `src/utils/placeholder.rs` — placeholder parsing via a shared template context; supports legacy placeholders and key/value attributes
- `src/utils/casing.rs` — case style conversion

**Placeholder format:** `{{date:%Y-Q%Q}}` → `2024-Q1`. `%Q` is a custom extension (not strftime); converts ISO week to quarter via `quarter_from_week()` on the effective reference date.

**Journal date anchors:** `--date` is format-independent and currently accepts `YYYY-MM-DD`, `YYYY-W01`/`YYYY-W1`, `YYYY-Q1`..`YYYY-Q4`, `YYYY-MM`, and `YYYY`. Partial anchors normalize to the first day of that period. `--offset` and placeholder offsets accept `day`, `week`, `quarter`, `month`, and `year` units.
