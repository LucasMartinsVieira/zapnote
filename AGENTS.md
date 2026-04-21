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
4. Validate template exists, resolve target path, and reuse existing notes when applicable
5. Placeholder substitution in template content (`{{title}}`, legacy `{{date[:fmt]}}` / `{{time[:fmt]}}`, plus key/value forms like `{{date offset="-1 day" format="%Y-%m-%d"}}`)
6. Write to note folder, print the resolved path when `--no-editor` is set, otherwise exec `$EDITOR`

**Primary commands:**
- `zn note <template> <name...>` — create a regular note
- `zn journal <name> [--date <anchor>] [--offset <delta>]` — create a journal entry (uses `JournalConfig` for output format/folder)
- `zn completion <shell>` — generate shell completions for `zn`
- `zn list templates [--json]` / `zn list journals [--json]` — list plugin-facing metadata

**Automation behavior:**
- `zn --no-editor note ...` and `zn --no-editor journal ...` print only the final resolved path on stdout
- `zn note` now mirrors journal behavior for duplicates by opening or returning the existing note instead of failing
- `src/cli.rs` pins the Clap command name to `zn`, which matters for generated completion scripts and `--version`

**Key modules:**
- `src/main.rs` — entry point, config load, routes to `note.rs` or `journal.rs`
- `src/cli.rs` — clap CLI definitions, command-name pinning, and dynamic completion candidates
- `src/config.rs` — TOML config structs (`Config`, `GeneralConfig`, `JournalConfig`, `CaseStyle`)
- `src/note.rs` / `src/journal.rs` — command handlers
- `src/utils/mod.rs` — path helpers, `run_editor()`, duplicate-path checks, quarter helper
- `src/utils/date.rs` — shared date parsing, formatting, `%Q` replacement, and offset arithmetic for `--date`, `--offset`, and placeholders
- `src/utils/template.rs` — template file I/O plus structured template/journal metadata for list commands and future editor integrations
- `src/utils/placeholder.rs` — placeholder parsing via a shared template context; supports legacy placeholders and key/value attributes
- `src/utils/casing.rs` — case style conversion
- `tests/cli.rs` — binary-level regression tests for path-printing, top-level Clap behavior, list output, and completion naming

**Placeholder format:** `{{date:%Y-Q%Q}}` → `2024-Q1`. `%Q` is a custom extension (not strftime); converts ISO week to quarter via `quarter_from_week()` on the effective reference date.

**Journal date anchors:** `--date` is format-independent and currently accepts `YYYY-MM-DD`, `YYYY-W01`/`YYYY-W1`, `YYYY-Q1`..`YYYY-Q4`, `YYYY-MM`, and `YYYY`. Partial anchors normalize to the first day of that period. `--offset` and placeholder offsets accept `day`, `week`, `quarter`, `month`, and `year` units.
