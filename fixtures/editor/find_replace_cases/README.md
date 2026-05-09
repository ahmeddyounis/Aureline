# Editor find/replace fixtures

These fixtures exercise the in-file lexical find/replace foundation in
`crates/aureline-editor/src/find_replace/` against real buffer snapshots. They
cover:

- literal match discovery with highlight overlays
- ASCII-only case-insensitive and whole-word options
- replace transactions that remain undoable as one buffer edit

The higher-level replace transaction contract (preview/apply packets and
multi-file scope) is defined in `docs/editor/refactor_and_replace_transaction_contract.md`.

