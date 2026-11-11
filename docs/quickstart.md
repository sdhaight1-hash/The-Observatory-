# Evidence Engine 001 — Command-line quickstart

The first working slice is an offline Rust + SQLite library with a minimal CLI.
No AI service, network connection, API credentials, or browser is required.

Run from the repository root (Rust toolchain required):

```sh
cargo test
cargo run -- evidence.sqlite new "Why does this phenomenon occur?"
# prints the new investigation ID, e.g. 1
cargo run -- evidence.sqlite claim 1 hypothesis "Possible explanation"
# prints the claim ID, e.g. 1
cargo run -- evidence.sqlite source 1 "Field notes" "file:///notes.txt"
# prints the source ID, e.g. 1
cargo run -- evidence.sqlite link 1 1 "paragraph 3" supports "Initial observation"
cargo run -- evidence.sqlite report 1
cargo run -- evidence.sqlite revise 1 "file:///notes-revised.txt"
cargo run -- evidence.sqlite deps 1
# Create a consistent snapshot in a NEW file (will not overwrite)
cargo run -- evidence.sqlite backup evidence-backup.sqlite
```

All numeric IDs in this example assume a new, empty database.

## Data integrity

- Foreign keys enforce valid object references.
- Evidence links must join a source and claim from the same investigation.
- Evidence requires a nonempty locator and an explicit stance:
  `supports`, `contradicts`, or `context`.
- Evidence captures the source reference and version *at link time*.
  Changing the source later does not rewrite the earlier citation.
- Source reference updates record their history in `revisions`.
- `deps` shows claims that depend on a particular source.
- Reports reflect recorded assertions and **do not independently verify source passages**.

## Limitations / next work

This is a development prototype, not a finished investigation product.
- No graphical interface, bookmarks, RSS feeds, HTTP API, or MCP server yet.
- No imported document bytes or independently validated source passage offsets.
- Legacy databases from the earliest experimental branch are upgraded
  on open. Old evidence is explicitly marked `UNVERIFIED LEGACY CITATION`
  (source version 0): its historical reference cannot be reconstructed and
  must not be represented as the current source address.
- The SQLite schema is now versioned with `PRAGMA user_version`; migrations
  run in a transaction. Unknown future versions and partial legacy schemas
  fail instead of being silently altered.
- `backup` uses SQLite `VACUUM INTO` to produce a consistent, standalone
  snapshot of committed data (including committed WAL changes). It refuses to
  overwrite an existing destination. Keep backups outside synchronized folders
  while copying, and test restoring them before relying on them.
- Migration compatibility is currently limited to the initial experimental
  SQLite schemas. Corrupt or unsupported external schemas are not repaired.
- Deletion semantics, source immutability, and audit-log completeness still need formal validation.
- Reports are Markdown drafts, not reviewed research findings.

The project deliberately builds and tests the evidence foundation before
adding higher-level capture and visualization features.
