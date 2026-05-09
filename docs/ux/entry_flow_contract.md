# Entry-flow contract (Open / Clone / Import / Restore)

This page is the reviewer-facing entry point for Aureline’s **entry-flow state
machine** and the first shell consumer that renders distinct, reviewed flow
sheets for the primary entry verbs.

The contract goal is simple:

- **Verbs stay distinct.** `open`, `clone`, `import`, and `restore` remain
  separate user intents; the shell must not silently promote one verb into
  another because a target happens to look like a repo or archive.
- **Targets are disambiguated before commit.** If a target cannot be classified
  without user input (file vs folder, local vs remote, etc.), the flow fails
  closed and surfaces a disambiguation step rather than guessing.
- **Result mode is disclosed.** Every reviewed sheet names the resulting mode
  the action will materialize when committed.

## Canonical truth source

The canonical entry-flow resolver lives in:

- `crates/aureline-workspace/src/entry_flows/mod.rs`

It owns the stable vocabulary and fail-closed rules used by shell entry
surfaces.

## Fixtures (proof + failure drill)

The minimal fixture set that exercises distinct verbs and the ambiguity failure
drill lives in:

- `fixtures/workspace/entry_flow_cases/`

These cases cover:

- resolved Start Center intents (`open`, `clone`, `import`, `restore`);
- a failure drill where `open` receives a remote-repo specifier and must
  **suggest reroute** to `clone` without silently promoting; and
- a failure drill where a path-like specifier is ambiguous and must **deny with
  target_kind_unresolved**.

## Shell consumer

The first shell consumer renders reviewed flow sheets from Start Center and
commits them into command dispatch:

- `crates/aureline-shell/src/bootstrap/native_shell.rs`

