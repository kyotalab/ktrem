# kterm (Zettelkasten for Terminal)

A terminal-based TUI for managing a [Luhmann-style](https://en.wikipedia.org/wiki/Zettelkasten) Zettelkasten, designed to work alongside [kasten](https://github.com/) — sharing the same workspace so you can switch seamlessly between desktop/mobile and terminal.

## Features

- **Two workspaces in one tool**
  - **Scratch**: quick, disposable notes for fleeting ideas
  - **Zettelkasten**: permanent notes organized with Luhmann-style IDs (`1`, `1a`, `1/1`, `1a/1`, ...)
- **Tree view** of your Zettelkasten with collapsible branches
- **Live preview** pane that renders the selected note's content
- **Backlinks**: automatically detects `[[id]]` wikilinks and shows which notes reference the current one
- **Promotion wizard**: browse the existing tree, pick a parent, assign an ID/title/tags, and promote a Scratch note (or create a new Zettelkasten note directly)
- **Tag editing** synced with `index.json`
- **Incremental search** across the current tab (content, title, and tags for Zettelkasten)
- **External editor integration** — opens notes in `$EDITOR` and reloads on return
- **Workspace switching** — detect and switch between multiple kasten workspaces without editing config files by hand

## Storage layout

kterm reads and writes the same directory structure as kasten:

```
kasten/
  cards/
    1.md
    1a.md
    1-1.md
  scratch/
    202506061430.md
  index.json
```

- Notes have **no frontmatter** — the title is taken from the first `# H1` heading.
- Filenames use hyphens (`1-1-2.md`); these are displayed as `1/1/2` in the tree.
- Metadata (status, tags, created/updated timestamps) for Zettelkasten notes lives in `index.json`.
- Scratch notes have no metadata — the filename timestamp (`YYYYMMDDHHMM.md`) is the only identifier.

## Installation

```bash
git clone https://github.com/kyotalob/kterm.git
cd kterm
cargo build --release
```

The binary will be at `target/release/kterm`.

## First run

On first launch, kterm looks for an existing kasten workspace (a directory containing both `cards/` and `index.json`) under `~/Documents` and `~/Library/Mobile Documents` (iCloud Drive).

- If exactly one candidate is found, you'll be asked to confirm it.
- If multiple candidates are found, you'll choose from a list.
- If none are found, you'll be prompted to enter the path manually.

The chosen path is saved to `~/.config/kterm/config.toml`.

## Keybindings

| Key                   | Action                                     | Notes                                                                  |
| --------------------- | ------------------------------------------ | ---------------------------------------------------------------------- |
| `j` / `k` / `↓` / `↑` | Move down / up                             |                                                                        |
| `h` / `l` / `←` / `→` | Collapse / expand                          | Zettelkasten tab only                                                  |
| `Tab`                 | Switch between Scratch / Zettelkasten tabs |                                                                        |
| `Enter`               | Open the selected note in `$EDITOR`        |                                                                        |
| `n`                   | New note                                   | Scratch: created immediately; Zettelkasten: opens the promotion wizard |
| `p`                   | Promote to Zettelkasten                    | Scratch tab only                                                       |
| `t`                   | Edit tags                                  | Zettelkasten tab only                                                  |
| `d`                   | Delete (with confirmation)                 |                                                                        |
| `/`                   | Incremental search                         | `Enter` to confirm, `Esc` to cancel                                    |
| `w`                   | Switch workspace                           |                                                                        |
| `q` / `Esc`           | Quit                                       | Normal mode only                                                       |

### Promotion wizard

1. Browse the existing Zettelkasten tree (top half) to decide where the new note belongs.
2. Fill in the ID, title, and tags (bottom half), using `Tab` to move between fields.
3. Press `Enter` to create the note in `cards/`, update `index.json`, and (if promoting from Scratch) delete the original Scratch file.

### Backlinks

Add `[[1a]]`-style links anywhere in a note's body. kterm builds a backlink index on startup and after every reload, and shows "Backlinks" below the preview for the currently selected note.

## Development

```
kterm/
  src/
    main.rs            # entry point, event loop, keybindings
    lib.rs
    app.rs             # application state
    model/
      note.rs          # Zettel, Scratch
      index.rs         # IndexJson, IndexEntry, CardStatus
      config.rs        # Config
    store/
      index.rs         # index.json read/write
      scratch.rs       # scratch/ file operations
      zettelkasten.rs  # cards/ file operations
    ui/
      layout.rs
      scratch.rs
      zettelkasten.rs
      preview.rs
      wizard.rs
      workspace_switch.rs
    search.rs
    config.rs           # config.toml + first-run setup
    error.rs
```

Run tests:

```bash
cargo test
```

Run lints:

```bash
cargo clippy
```

## License

MIT
