# M5 constrained-file-state, canonical-source-relation, and write-target-review — operations

This note is the human-readable companion to the frozen **M5 constrained-file-state matrix**
(`schemas/program/m5-constrained-file-state-matrix.schema.json`). The authoritative gate is the Rust
validator in `crates/aureline-ui/src/m5_constrained_file_state_matrix/`; the checked-in support export
(`artifacts/support/m5-constrained-object-state/support_export.json`) is minted only by the headless
emitter and must always match the seed builder.

## What this batch (B150) freezes

One canonical model for how Aureline classifies a *current* object as **read-only, generated, policy-locked,
managed, projection, or captured snapshot**, and how every write-capable consumer explains the safe next
step. The matrix is the shared, reusable contract that later B150 rows (state consumers, canonical-source
plumbing, write-review sheets, actor-parity mutation gates, certification) build on. This row builds no new
consumer surface itself — it standardizes the shared consumer truth.

### Constrained object classes and their canonical domain schema

| Object class | Write disposition | Canonical domain schema |
| --- | --- | --- |
| `read_only` | `read_only_blocked` | `m5-constrained-file-state.schema.json` |
| `generated` | `regenerate_only` | `m5-canonical-source-relation.schema.json` |
| `policy_locked` | `approval_gated` | `m5-constrained-file-state.schema.json` |
| `managed` | `approval_gated` | `m5-write-target-review.schema.json` |
| `projection` | `detach_required` | `m5-canonical-source-relation.schema.json` |
| `captured_snapshot` | `restore_only` | `m5-write-target-review.schema.json` |

`write_disposition` makes a write-constrained object mechanically distinct from an ordinary
`directly_writable` object (`M5ConstrainedFileStateWriteDisposition::is_write_constrained`).

### Required visible state (per class)

Every covered class must carry all seven visible-state fields: **state badge, reason, canonical source or
live target, exact write target, allowed actions, blocked actions, and export / retain notes**. The matrix
makes **write target, canonical source, and nearest safe action mechanically distinct** for every class.

### Shared consumer surfaces that must agree

`tab_chrome`, `breadcrumb_trail`, `status_bar`, `command_palette`, `editor_banner`, `diff_review_header`,
`write_review_sheet`, `ai_automation_path`, and `support_export_packet`. No claimed M5 consumer may lack a
controlled vocabulary for blocked-write truth.

### Hard invariants (all `false` on every row)

1. one constrained-state class hides another when both materially affect behavior;
2. generated / managed / projection / archived objects silently fall back to a lossy direct write;
3. AI / automation / import / repair flows get a hidden bypass around the constrained-state rules;
4. the canonical source, exact write target, preserved-versus-lost sync, or recovery / regenerate path is
   left unstated;
5. a constrained object is presented as directly writable, or the recovery / regenerate path is hidden.

## Emitter

```text
cargo run -p aureline-ui --example dump_m5_constrained_file_state_matrix -- support-export
cargo run -p aureline-ui --example dump_m5_constrained_file_state_matrix -- report
cargo run -p aureline-ui --example dump_m5_constrained_file_state_matrix -- csv
cargo run -p aureline-ui --example dump_m5_constrained_file_state_matrix -- dashboard
cargo run -p aureline-ui --example dump_m5_constrained_file_state_matrix -- fixture-managed-beta-narrowed
cargo run -p aureline-ui --example dump_m5_constrained_file_state_matrix -- fixture-projection-preview-narrowed
cargo run -p aureline-ui --example dump_m5_constrained_file_state_matrix -- validate
```

Checked-in outputs: the support export and matrix CSV under
`artifacts/support/m5-constrained-object-state/`, the Markdown report at
`artifacts/program/m5-constrained-file-state-matrix.md`, the health dashboard at
`dashboards/m5-constrained-object-health.json`, and the narrowed fixtures under
`fixtures/editor/m5-constrained-object-states/`. Raw secret values and private endpoints stay outside the
export boundary.
