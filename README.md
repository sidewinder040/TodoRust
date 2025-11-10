# TodoRust

Lightweight command-line Todo application written in Rust. It stores a list of todos in memory while running and persists them to a JSON file when the application exits.

## Progress (what's implemented)

- Core data type `TodoItem` with a constructor and formatter.
- `TodoList` that holds a `Vec<TodoItem>` and provides `load(path)`, `save(path)`, and `add(item)` helpers.
- Persistence to JSON using `serde` + `serde_json`.
- Interactive command-line UI in `src/main.rs` with the following commands:
  - `list` / `l` — show saved todos
  - `add` / `a` — add a new todo (title is required)
  - `remove` / `r` — remove by 1-based index
  - `edit` / `e` — edit an existing todo by index
  - `quit` / `q` — save and exit
- Choose the todo JSON file via CLI argument or `TODO_FILE` environment variable (falls back to `todos.json`).
- Unit and integration tests added:
  - Library unit tests for `TodoItem` formatting.
  - Integration test `tests/persistence_tests.rs` that checks `TodoList::save/load` using a temporary file (`tempfile` crate).

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

When you quit the application (`q` or `quit`) the current todos are saved to the chosen JSON file.

## Example session

- Start the program: `cargo run`
- Add a todo: `a` then provide the Title and Description (Title cannot be empty)
- List todos: `l`
- Edit or remove using the `e` and `r` commands with 1-based indices
- Quit: `q` (saves to JSON file)

## Running tests

Run the full test suite with:

```bash
cargo test
```

Tests included:
- Library unit tests for `TodoItem` behavior.
- `tests/persistence_tests.rs` verifies save/load roundtrip using a temporary file.

## Files of interest

- `src/lib.rs` — library: `TodoItem`, `TodoList`, persistence helpers.
- `src/main.rs` — interactive CLI that uses the library.
- `tests/persistence_tests.rs` — integration test for persistence.
- `Cargo.toml` — declares dependencies (`serde`, `serde_json`) and dev-dependency `tempfile`.

## Next steps / suggestions

- Add more thorough CLI tests (spawn the binary and feed input) if you want full end-to-end automation.
- Add backup/atomic save (write to a temp file then rename) to make saves safer.
- Add search/filter, priorities, timestamps, or IDs for more complex workflows.

If you'd like, I can implement any of the above improvements next.
