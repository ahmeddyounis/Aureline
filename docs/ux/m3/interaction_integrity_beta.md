# Interaction Integrity Beta Contract

This contract binds dense collection state, batch-review truth, responsive
identity cues, focus return, and support export into one shell packet for
claimed beta surfaces. It composes the existing collection contracts instead of
creating another table or selection model:

- [`../collection_view_contract.md`](../collection_view_contract.md)
- [`../selection_and_batch_action_contract.md`](../selection_and_batch_action_contract.md)
- [`../selection_and_scope_contract.md`](../selection_and_scope_contract.md)
- [`../dialog_sheet_contract.md`](../dialog_sheet_contract.md)
- [`../responsive_shell_rules.md`](../responsive_shell_rules.md)
- [`../tree_view_contract.md`](../tree_view_contract.md)

The shell projection lives in
[`crates/aureline-shell/src/interaction_integrity/mod.rs`](../../../crates/aureline-shell/src/interaction_integrity/mod.rs).
It exports a packet under `shell:interaction_integrity_beta:v1`.

## Contract Surface

The packet carries seven record sets:

- `object_interaction_state_record` - one shared state row for focused,
  active, selected, pending, disabled, filtered, hidden, and blocked objects.
- `batch_scope_truth_record` - a wrapper around the shared collection
  `BatchReviewSheet` that adds filtered, hidden, disabled, pending, and
  resulting target-id proof.
- `shell_identity_cue_record` - breadcrumb, status, tab, inspector, and
  terminal identity cues that survive resize, split, detach, and multi-window
  fallback.
- `focus_return_rule_record` - dialog, sheet, approval prompt, activity jump,
  and pane fallback return targets with screen-reader announcements.
- `collection_vocabulary_parity_record` - UI, keyboard-help, accessibility,
  and support-export token parity for launch-critical surfaces.
- `interaction_integrity_support_export_record` - metadata-only support export
  with selected scope, filtered scope, hidden scope, blocked scope, and
  resulting target ids.
- `interaction_integrity_replay_fixture_record` - replay drills that bind the
  checked-in fixture files to the packet.

## Acceptance Rules

Interaction integrity is conforming only when all of these are true:

- Focus, active/current item, selection, pending, disabled, filtered, hidden,
  and blocked states stay separate in every covered object-state row.
- Hidden or filtered selected objects are never included in a destructive,
  provider-owned, remote, publish-capable, or export-bearing target set.
- Batch reviews show included, excluded, blocked, filtered, hidden, disabled,
  pending, and resulting target ids before continuation.
- UI, keyboard help, accessibility narration, and support export expose the same
  state vocabulary.
- Breadcrumbs, status items, tabs, inspector headers, and terminal headers keep
  stable canonical object ids across resize, split, detach, and multi-window
  fallback.
- Dialogs, sheets, approval prompts, activity-center jumps, and pane fallback
  rules return focus to the logical origin or a named safe fallback.
- Support export is metadata-only and carries selected scope, filtered scope,
  hidden scope, blocked scope, reviewed batch scopes, and resulting target ids.

## Fixture Corpus

Fixtures live under
[`/fixtures/shell/m3/interaction_integrity/`](../../../fixtures/shell/m3/interaction_integrity/):

- `packet.json`
- `state_model.json`
- `batch_reviews.json`
- `identity_cues.json`
- `focus_return.json`
- `vocabulary.json`
- `support_export.json`
- `replay_fixtures.json`

## Headless Inspector

```sh
cargo run -q -p aureline-shell --bin aureline_shell_interaction_integrity -- packet
cargo run -q -p aureline-shell --bin aureline_shell_interaction_integrity -- state-model
cargo run -q -p aureline-shell --bin aureline_shell_interaction_integrity -- batch-reviews
cargo run -q -p aureline-shell --bin aureline_shell_interaction_integrity -- identity-cues
cargo run -q -p aureline-shell --bin aureline_shell_interaction_integrity -- focus-return
cargo run -q -p aureline-shell --bin aureline_shell_interaction_integrity -- vocabulary
cargo run -q -p aureline-shell --bin aureline_shell_interaction_integrity -- support-export
cargo run -q -p aureline-shell --bin aureline_shell_interaction_integrity -- replay-fixtures
cargo run -q -p aureline-shell --bin aureline_shell_interaction_integrity -- validate
```

## Verification

```sh
cargo test -q -p aureline-shell --lib interaction_integrity
cargo test -q -p aureline-shell --test interaction_integrity_beta_fixtures
cargo run -q -p aureline-shell --bin aureline_shell_interaction_integrity -- validate
```
