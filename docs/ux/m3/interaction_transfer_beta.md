# Interaction Transfer Beta Contract

This contract binds clipboard payload truth, drag/drop intent advertising,
named-undo-group attribution, workspace-scoped back/forward navigation, and
reopen-history fidelity into one shell packet for claimed beta surfaces. It
composes the existing alpha vocabulary in
[`crates/aureline-shell/src/transfer/`](../../../crates/aureline-shell/src/transfer)
and the cross-surface contracts the alpha vocabulary already cites:

- [`../clipboard_history_contract.md`](../clipboard_history_contract.md)
- [`../copy_export_representation_parity.md`](../copy_export_representation_parity.md)
- [`../cross_window_transfer_contract.md`](../cross_window_transfer_contract.md)
- [`../shell_close_reopen_contract.md`](../shell_close_reopen_contract.md)
- [`../restore_fidelity_classes.md`](../restore_fidelity_classes.md)
- [`../shell_interaction_safety_contract.md`](../shell_interaction_safety_contract.md)

The shell projection lives in
[`crates/aureline-shell/src/interaction_transfer/mod.rs`](../../../crates/aureline-shell/src/interaction_transfer/mod.rs).
It exports a packet under `shell:interaction_transfer_beta:v1`.

## Covered Surfaces

The beta packet covers five claimed surface classes:

| Surface | Examples |
|---|---|
| `editor` | Editor canvas, tab group, group split target, file tree |
| `diff` | Diff view, compare slot |
| `review` | Review queue, pull-request review, evidence attach bucket |
| `result_grid` | Search results, problems, log views, work-item boards, admin grids |
| `provider_linked` | Extension/marketplace listings, identity/provider rows, remote provider scopes |

## Contract Surface

The packet carries five record families plus a metadata-only support export:

- `clipboard_payload_class_record` — one row per source object that declares the
  default plain-text copy, any rich/context variants that diverge from raw or
  source truth, the clipboard route posture
  (`local_system`, `remote_bridge`, `named_register`, `policy_blocked`), and the
  sensitive-copy review when tokens, private paths, support links, or similar
  risky bodies are involved.
- `drop_intent_record` — one row per drop target that advertises the verb
  (`move`, `copy`, `attach`, `open`, `import`, `split`, or `blocked`), the
  modifier-key cue and its meaning, the destination scope, whether the drop is
  a broad workspace mutation requiring a checkpoint, and whether collision or
  overwrite review is part of the path. Verbs are announced to assistive tech.
- `undo_group_attribution_record` — one row per broad mutation (multi-file
  replace, settings import, AI apply, extension refactor, or other broad
  mutation). Rows that cannot register undo declare a `no_undo_posture`
  (preview-before-commit, checkpoint-before-commit, or refused).
- `back_forward_entry_record` — one row per workspace-scoped back/forward
  entry with a recorded timestamp and a source label.
- `reopen_history_entry_record` — one row per closed or recovered object that
  names the source class (intentional close, back/forward navigation, crash
  recovery, disconnect recovery, or placeholder reopen), the restored target
  identity, the closed-at and last-activity timestamps, and a continuity label
  that tells the user whether live authority truly survived.
- `interaction_transfer_support_export_record` — metadata-only export that
  references every record above by id and explicitly lists the omitted raw
  payload classes (`raw_clipboard_body`, `raw_file_body`, `raw_drop_payload`,
  `raw_private_path`, `raw_provider_token`).

## Acceptance Rules

The packet is green only when every one of these holds:

- Every covered surface (editor, diff, review, result_grid, provider_linked)
  declares at least one clipboard payload class with the default representation
  set to `plain_text`. Default copy must paste usefully into terminals,
  reviews, issue trackers, and support flows without hidden formatting traps
  (`paste_targets_neutral = true`).
- Rich, rendered, redacted, with-context, escaped, sanitized, and metadata-only
  copies are exposed as explicit non-default variants with labels and a
  `diverges_from_source_truth` flag when copying rendered or redacted form
  would differ from raw/source truth.
- The clipboard route posture is disclosed when it materially changes behavior
  (remote bridges always declare it material).
- Sensitive payloads (tokens, private paths, support links, and similarly
  risky bodies) declare a `label_first_preview` posture that defers the
  clipboard write until a preview is reviewed, or declare `write_blocked` so
  the clipboard write is refused entirely.
- Every drop target advertises the resulting verb and modifier meaning inline
  before drop completes. Blocked drops declare `blocked_before_commit = true`
  and a reason label; non-blocked verbs may not declare a block. Broad
  workspace mutations always create or verify a checkpoint before commit. The
  verb is announced to assistive tech.
- Broad mutations register a single reviewable undo entry with a meaningful
  source attribution label and command id rather than many opaque one-step
  undos. The packet must cover multi-file replace, settings import, AI apply,
  and extension refactor scopes.
- Surfaces that cannot register undo declare a no-undo posture
  (`preview_before_commit`, `checkpoint_before_commit`, or `refused`) and a
  label that names the posture. The packet must include at least one no-undo
  scope row with a preview or checkpoint posture.
- Back/forward entries exist in both directions and carry timestamps,
  workspace-scope labels, and source labels.
- Reopen-history entries distinguish intentional close, back/forward
  navigation, crash recovery, disconnect recovery, and placeholder reopen.
  Crash and disconnect recovery, and placeholder reopen, must forbid automatic
  rerun and must not claim live authority. Every covered surface appears at
  least once in the reopen-history rows.
- The support export references every record in the packet by id, declares
  the omitted raw payload classes, and never includes raw payload bodies.

## Fixture Corpus

Fixtures live under
[`/fixtures/ux/m3/clipboard_dragdrop_history/`](../../../fixtures/ux/m3/clipboard_dragdrop_history):

- `packet.json`
- `clipboard_payloads.json`
- `drop_intents.json`
- `undo_groups.json`
- `back_forward.json`
- `reopen_history.json`
- `support_export.json`
- `summary.json`

The corpus is regenerated from the headless inspector; see the corpus
[`README.md`](../../../fixtures/ux/m3/clipboard_dragdrop_history/README.md).

## Headless Inspector

```sh
cargo run -q -p aureline-shell --bin aureline_shell_interaction_transfer -- packet
cargo run -q -p aureline-shell --bin aureline_shell_interaction_transfer -- clipboard-payloads
cargo run -q -p aureline-shell --bin aureline_shell_interaction_transfer -- drop-intents
cargo run -q -p aureline-shell --bin aureline_shell_interaction_transfer -- undo-groups
cargo run -q -p aureline-shell --bin aureline_shell_interaction_transfer -- back-forward
cargo run -q -p aureline-shell --bin aureline_shell_interaction_transfer -- reopen-history
cargo run -q -p aureline-shell --bin aureline_shell_interaction_transfer -- support-export
cargo run -q -p aureline-shell --bin aureline_shell_interaction_transfer -- summary
cargo run -q -p aureline-shell --bin aureline_shell_interaction_transfer -- validate
```

## Verification

```sh
cargo test -q -p aureline-shell --lib interaction_transfer
cargo test -q -p aureline-shell --test interaction_transfer_beta_fixtures
cargo run -q -p aureline-shell --bin aureline_shell_interaction_transfer -- validate
```
