# Interaction Integrity Beta Fixtures

This corpus is the replayable shell evidence for dense-collection
interaction integrity. It is generated from the headless inspector in
`crates/aureline-shell/src/bin/aureline_shell_interaction_integrity.rs`.

## Files

- `packet.json` - complete beta packet.
- `state_model.json` - focused, active, selected, pending, disabled,
  filtered, hidden, and blocked object state rows.
- `batch_reviews.json` - batch-scope truth records with included,
  excluded, blocked, filtered, hidden, disabled, pending, and resulting
  target ids.
- `identity_cues.json` - breadcrumb, status, tab, inspector, and terminal
  identity cues across resize, split, detach, and multi-window fallback.
- `focus_return.json` - dialog, sheet, approval prompt, activity jump, and
  pane fallback focus-return rules.
- `vocabulary.json` - UI, keyboard-help, accessibility, and support-export
  vocabulary parity rows.
- `support_export.json` - metadata-only support export for selected scope,
  filtered scope, hidden scope, blocked scope, and resulting target ids.
- `replay_fixtures.json` - drill rows that bind fixtures to replay classes.

## Regenerate

```sh
cargo run -q -p aureline-shell --bin aureline_shell_interaction_integrity -- packet > fixtures/shell/m3/interaction_integrity/packet.json
cargo run -q -p aureline-shell --bin aureline_shell_interaction_integrity -- state-model > fixtures/shell/m3/interaction_integrity/state_model.json
cargo run -q -p aureline-shell --bin aureline_shell_interaction_integrity -- batch-reviews > fixtures/shell/m3/interaction_integrity/batch_reviews.json
cargo run -q -p aureline-shell --bin aureline_shell_interaction_integrity -- identity-cues > fixtures/shell/m3/interaction_integrity/identity_cues.json
cargo run -q -p aureline-shell --bin aureline_shell_interaction_integrity -- focus-return > fixtures/shell/m3/interaction_integrity/focus_return.json
cargo run -q -p aureline-shell --bin aureline_shell_interaction_integrity -- vocabulary > fixtures/shell/m3/interaction_integrity/vocabulary.json
cargo run -q -p aureline-shell --bin aureline_shell_interaction_integrity -- support-export > fixtures/shell/m3/interaction_integrity/support_export.json
cargo run -q -p aureline-shell --bin aureline_shell_interaction_integrity -- replay-fixtures > fixtures/shell/m3/interaction_integrity/replay_fixtures.json
```

## Verify

```sh
cargo test -q -p aureline-shell --test interaction_integrity_beta_fixtures
cargo run -q -p aureline-shell --bin aureline_shell_interaction_integrity -- validate
```
