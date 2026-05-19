# Interaction Transfer Beta Fixtures

This corpus is the replayable shell evidence for clipboard, drag/drop,
named-undo-group, back/forward, and reopen-history truth on the editor,
diff, review, result-grid, and provider-linked surfaces. It is generated from
the headless inspector in
`crates/aureline-shell/src/bin/aureline_shell_interaction_transfer.rs`.

## Files

- `packet.json` — complete beta packet.
- `clipboard_payloads.json` — clipboard payload-class rows, one per covered
  surface, declaring the default plain-text copy, rich/context variants,
  clipboard route posture, and sensitive-copy review when applicable.
- `drop_intents.json` — drop intent rows that advertise the verb (move, copy,
  attach, open, import, split, or blocked) and modifier meaning inline before
  drop completes.
- `undo_groups.json` — named undo-group attribution rows for multi-file
  replace, settings import, AI apply, extension refactor, and the no-undo
  preview/checkpoint posture for surfaces that cannot register undo.
- `back_forward.json` — workspace-scoped back/forward entries with timestamps
  and source labels.
- `reopen_history.json` — reopen-history entries that distinguish
  intentionally closed, back/forward navigation, crash recovery, disconnect
  recovery, and placeholder reopen sources.
- `support_export.json` — metadata-only support export referencing every
  record in the packet by id.
- `summary.json` — coverage summary computed by the validator.

## Regenerate

```sh
cargo run -q -p aureline-shell --bin aureline_shell_interaction_transfer -- packet > fixtures/ux/m3/clipboard_dragdrop_history/packet.json
cargo run -q -p aureline-shell --bin aureline_shell_interaction_transfer -- clipboard-payloads > fixtures/ux/m3/clipboard_dragdrop_history/clipboard_payloads.json
cargo run -q -p aureline-shell --bin aureline_shell_interaction_transfer -- drop-intents > fixtures/ux/m3/clipboard_dragdrop_history/drop_intents.json
cargo run -q -p aureline-shell --bin aureline_shell_interaction_transfer -- undo-groups > fixtures/ux/m3/clipboard_dragdrop_history/undo_groups.json
cargo run -q -p aureline-shell --bin aureline_shell_interaction_transfer -- back-forward > fixtures/ux/m3/clipboard_dragdrop_history/back_forward.json
cargo run -q -p aureline-shell --bin aureline_shell_interaction_transfer -- reopen-history > fixtures/ux/m3/clipboard_dragdrop_history/reopen_history.json
cargo run -q -p aureline-shell --bin aureline_shell_interaction_transfer -- support-export > fixtures/ux/m3/clipboard_dragdrop_history/support_export.json
cargo run -q -p aureline-shell --bin aureline_shell_interaction_transfer -- summary > fixtures/ux/m3/clipboard_dragdrop_history/summary.json
```

## Verify

```sh
cargo test -q -p aureline-shell --lib interaction_transfer
cargo test -q -p aureline-shell --test interaction_transfer_beta_fixtures
cargo run -q -p aureline-shell --bin aureline_shell_interaction_transfer -- validate
```
