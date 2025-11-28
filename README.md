# TodoRust

Lightweight command-line Todo application written in Rust. It stores a list of todos in memory while running and persists them to a JSON file when the application exits.

## Features

- **Rich TodoItem Data Model**: Each todo includes:
  - `id` — Unique identifier (UUID v4)
  - `title` — Required title
  - `description` — Optional description (displayed in list view, indented and dimmed)
  - `status` — Pending, InProgress, or Completed (with color-coded output)
  - `priority` — Low, Medium, or High (optional)
  - `created_at` — Creation timestamp
  - `updated_at` — Last modification timestamp (optional)
  - `due_date` — Optional due date (YYYY-MM-DD format)
  - `tags` — Comma-separated tags for organization
  - `metadata` — Extensible key-value store for custom data

- **Full-Featured CLI** with color-coded output and intuitive prompts:
  - `list` / `l` — Show all todos with descriptions and metadata
  - `add` / `a` — Add a new todo with all optional fields
  - `edit` / `e` — Edit existing todos (preserves ID and creation timestamp)
  - `remove` / `r` — Remove todos by index
  - `quit` / `q` — Save and exit

- **Flexible Persistence**:
  - JSON format with pretty printing
  - Load/save helpers with error handling
  - Choose file via CLI argument or `TODO_FILE` environment variable (defaults to `todos.json`)

- **Color-Coded Output**:
  - Status badges with distinct colors (Pending: yellow, InProgress: cyan, Completed: green)
  - Priority indicators with colors
  - TTY-aware auto-detection (disable with `--no-color`, enable with `--color`)
  - Clear visual hierarchy with headers, separators, and dimmed secondary text

- **Comprehensive Test Suite** (15+ tests):
  - Unit tests for `TodoItem` and `Status`/`Priority` enums
  - Serialization/deserialization tests
  - Format output verification
  - `TodoList` add and persistence tests
  - Default value validation
  - Persistence roundtrip tests using temporary files

## Requirements

- Rust toolchain (tested with stable Rust).
- No external runtime dependencies — the app uses local file I/O.

## How to run

From the project root:

```bash
cargo run
```

This starts the interactive prompt and (by default) uses `todos.json` in the current working directory.

You can override the path used for persistence:

```bash
cargo run -- mytodos.json
# or
TODO_FILE=mytodos.json cargo run
```

Color behavior
----------------

The interactive UI uses colored output for improved readability (headers, prompts, success/warning/error messages, and per-item formatting).

- Default: colors are enabled when stdout is a TTY (interactive terminal).
- You can explicitly disable colors with `--no-color` or explicitly enable them with `--color`.
  These flags can be passed anywhere in the CLI args (for example `cargo run -- --no-color`).
- When colors are disabled the application sets the `NO_COLOR` environment variable so library code and downstream tools can detect the preference.

Examples:

```bash
# run with default (TTY detection)
cargo run

# force no colors
cargo run -- --no-color

# force colors even if stdout is not a TTY
cargo run -- --color
```

Note: when piping or redirecting output (for example `cargo run > out.txt`) colors will be disabled automatically.


When you quit the application (`q` or `quit`) the current todos are saved to the chosen JSON file.

## Example session

Here's a typical workflow:

```
$ cargo run

--------------------------------------------------
 TodoRust — 0 todos (file: todos.json) 
--------------------------------------------------
Commands:
  l, list  - List todos
  a, add   - Add a todo
  r, remove - Remove by index
  e, edit   - Edit by index
  q, quit  - Save and quit

Command: a
Title (required): Buy groceries
Description: Milk, bread, eggs
Priority (low/med/high) [blank for none]: med
Due date (YYYY-MM-DD) [blank for none]: 2025-12-25
Tags (comma separated) [blank for none]: shopping,urgent
Added.

Command: l
--------------------------------------------------
  1. Buy groceries [Medium] [2025-12-25] #shopping #urgent
      Milk, bread, eggs
--------------------------------------------------

Command: e
Enter index to edit (1-1):
> 1
Editing todo 1: Buy groceries [Medium] [2025-12-25] #shopping #urgent
Title (required) [Buy groceries]: 
Description [Milk, bread, eggs]: 
Priority (low/med/high) [med]: high
Due date (YYYY-MM-DD) [2025-12-25]: 
Tags (comma separated) [shopping,urgent]: 
Updated.

Command: q
Saved 1 todos to todos.json
```

## Running tests

Run the full test suite with:

```bash
cargo test
```

**Test Coverage** (15+ tests):
- **Unit tests** in `src/lib.rs`:
  - `TodoItem` creation and formatting
  - `Status` and `Priority` enum variants
  - Default field values

- **Integration tests** in `tests/todo_tests.rs`:
  - TodoItem field validation (all new fields)
  - Serialization/deserialization roundtrip
  - Format output verification
  - `TodoList` add and retrieval
  - Default value verification

- **Persistence tests** in `tests/persistence_tests.rs`:
  - Full save/load roundtrip with temporary files
  - JSON format validation

All tests pass with no warnings or errors.

## Files of interest

- `src/lib.rs` — Core library types:
  - `TodoItem` struct with all fields (id, status, priority, timestamps, tags, metadata)
  - `Status` enum (Pending, InProgress, Completed)
  - `Priority` enum (Low, Medium, High)
  - `TodoList` with load/save/add helpers
  - JSON serialization with serde
  
- `src/main.rs` — Interactive CLI:
  - Command loop for add, list, edit, remove, quit
  - Color-coded output with `owo-colors`
  - TTY detection with `atty`
  - Input prompts with defaults for editing
  - File selection via CLI args or `TODO_FILE` env var

- `tests/todo_tests.rs` — Integration test suite (12+ tests)
  
- `tests/persistence_tests.rs` — Persistence roundtrip test

- `Cargo.toml` — Dependency manifest:
  - `serde` + `serde_json` for JSON serialization
  - `chrono` for timestamp handling with serde support
  - `uuid` for unique identifiers
  - `owo-colors` for terminal styling
  - `atty` for TTY detection
  - `tempfile` (dev-dependency) for test support

## Migration and atomic save (recent changes)

- Automatic migration: on first run the app will automatically migrate a local `./todos.json` (if present) into the per-user data location when the per-user file does not already exist. This makes the todos available regardless of the directory you run the app from.
- Atomic save: saves are performed safely by writing to a temporary file in the same directory and then renaming into place. This reduces the chance of corrupt/partial files on crashes. The application removes the temp file after successful rename.

Behavior notes:
- Migration is automatic when using the default per-user storage path. If you specify a file via CLI or `TODO_FILE` the migration step is skipped.
- Migration is implemented as a move/rename when possible; when rename fails (cross-device) the code falls back to copy+remove.

## Next steps / Future improvements

The application is feature-complete for basic todo management. Potential enhancements include:

- **Search/Filter**: Add commands to search by title, tag, or priority
- **Sorting**: Sort todos by priority, due date, or status
- **Archiving**: Archive completed todos instead of deleting
- **Backup**: Implement atomic saves (write to temp, then rename)
- **Data validation**: Add constraints (e.g., max title length, valid dates)
- **Config file**: Support `.todorust.toml` for custom settings
- **Shell integration**: Bash/PowerShell completion scripts
- **Sync**: Optional cloud sync for multiple machines
- **Import/Export**: CSV or other format support
