# M5 Rerun-Comparison-Sheet Primitive

Status: stable (M05-823, batch B96)

The reusable execution-lifecycle component matrix
([`m5-execution-lifecycle-component-matrix.schema.json`](../../schemas/ui/m5-execution-lifecycle-component-matrix.schema.json),
frozen in M05-820) *freezes* the run/attempt/input-request/artifact-publish/rerun/debug
component families as a governed contract. This primitive *narrows* one of those
families — `rerun_comparison_sheet` — into one working resolver with a real, tested
implementation, the sibling of the run/attempt-header primitive
([`m5_run_attempt_header_primitive.md`](m5_run_attempt_header_primitive.md), M05-821) and
the input-request / artifact-publish primitive
([`m5_input_request_artifact_publish_primitive.md`](m5_input_request_artifact_publish_primitive.md),
M05-822).

A single bounded **rerun review** — one prior run-and-attempt context the product is
about to re-execute — projects onto four surfaces that share one sheet identity, one
prior-run identity, and one prior-attempt identity:

- a **rerun comparison sheet** (`M5ResolvedRerunComparisonSheet`),
- a set of **changed-context rows** (`M5ResolvedRerunChangeRow`),
- a **CLI / headless line** (`M5ResolvedRerunCliLine`), and
- a **support-export projection** (`M5ResolvedRerunExport`).

The resolver is
`resolve_rerun_review(&M5RerunReviewInput) -> Result<M5ResolvedRerunReview, M5RerunReviewError>`
in
[`crates/aureline-runtime/src/implement_the_m5_rerun_comparison_sheet_and_retry_scope_review_primitive`](../../crates/aureline-runtime/src/implement_the_m5_rerun_comparison_sheet_and_retry_scope_review_primitive).
The boundary schema is
[`schemas/ui/m5-rerun-comparison-sheet.schema.json`](../../schemas/ui/m5-rerun-comparison-sheet.schema.json).

## Distinct reviewed rerun actions

`M5RerunMode` names the three distinct reviewed actions: `rerun_exactly`,
`rerun_with_current_context`, and `retry_failed_step_only`. Each mode pins a
`M5RerunContext` (reused from the frozen matrix) and a `M5RetryScope`:

| mode | context | retry scope |
| --- | --- | --- |
| `rerun_exactly` | `exact_replay` | `whole_run` |
| `rerun_with_current_context` | `current_context` / `modified_selection` / `modified_environment` | `whole_run` / `selected_subset` |
| `retry_failed_step_only` | any that differs from exact | `failed_step_only` |

A `retry_failed_step_only` action is offered only when the prior run `failed` or was
`partially_complete` (`retry_failed_step_not_applicable` is otherwise rejected). A mode
that disagrees with its context or scope is rejected
(`rerun_mode_context_mismatch`, `retry_scope_inconsistent_with_mode`).

**AC1 — rerun controls no longer present as one generic action when inputs, targets, or
authority have changed.** When any dimension changed (or the side-effect class
escalates), the sheet must keep `rerun_exactly` and `rerun_with_current_context` distinct
reviewed actions in `available_modes`; collapsing them to one control is rejected
(`distinct_rerun_actions_collapsed`). When nothing has changed, the modes are
semantically equivalent (`modes_semantically_equivalent`) and may be offered as one
action — the publish exact-replay row demonstrates this.

## Changed-context review before dispatch

Each changed-context row diffs one `M5RerunChangeDimension` — `input`, `target`,
`runtime`, `profile`, `approval_authority`, or `side_effect_class` — with a
`M5RerunChangeState` of `unchanged`, `changed`, `unknown`, or `not_applicable`. A
`changed` dimension must name both its `before_label` and `after_label`
(`changed_dimension_missing_delta`); an `unknown` dimension is one the product cannot
confirm is unchanged, so it is reviewed rather than assumed exact. Every row is shown
before dispatch.

**AC2 — users can review the changed execution context before retrying request, task,
notebook, debug, preview, or publish lanes.** The resolver enumerates every changed /
unknown dimension with a deterministic `change_summary` and a one-line sheet summary, all
shown before the rerun leaves the shell.

Side effects are classed by `M5SideEffectClass` (`none`, `read_only`, `local_write`,
`external_write`, `irreversible`, ranked by escalation). A rerun whose side-effect class
escalates beyond the prior attempt must disclose a reviewable `side_effect_class` change
row (`side_effect_escalation_not_disclosed` is otherwise rejected) — the AI-mediated
row demonstrates a `read_only` → `external_write` escalation.

## Prior-attempt lineage and export

**AC3 — support and export artifacts preserve the reviewed rerun mode and the
changed-input summary.** The sheet cites the prior attempt (`prior_attempt_ref`), the
new attempt (`new_attempt_ordinal`, always after the prior ordinal), and the
`difference_reason` the product believes distinguishes them. The support-export
projection carries the reviewed `rerun_mode`, `retry_scope`, `rerun_context`, the
enumerated `changed_dimensions`, the one-line `change_summary`, the `side_effect_class`,
and the `difference_reason`; the mandatory `M5RerunExportField` subset
(`prior_run_id`, `prior_attempt_id`, `rerun_mode`, `changed_dimensions`,
`change_summary`, `difference_reason`) must appear on every row.

## Redaction

Raw command bytes, secret values, credentials, provider cursors, and raw diff payloads
never cross this boundary. The resolver carries only opaque refs, typed class tokens,
booleans, and redacted labels; the packet's `validate()` re-scans the export-safe JSON
for forbidden material.

## Checked-in proof

The canonical packet is built by `seeded_m5_rerun_review_packet()` and emitted by the
`dump_m5_rerun_comparison_sheet_primitive` example. Regenerate:

```sh
cargo run -p aureline-runtime --example dump_m5_rerun_comparison_sheet_primitive -- support \
  > artifacts/release/m5-rerun-comparison-sheet-primitive-proof/support_export.json
cargo run -p aureline-runtime --example dump_m5_rerun_comparison_sheet_primitive -- csv \
  > artifacts/release/m5-rerun-comparison-sheet-primitive-proof/matrix.csv
cargo run -p aureline-runtime --example dump_m5_rerun_comparison_sheet_primitive -- summary \
  > artifacts/release/m5-rerun-comparison-sheet-primitive-proof/report.md
```

`current_stable_m5_rerun_review_export()` reads the checked-in support export via
`include_str!` and re-validates it; `checked_support_export_matches_builder` asserts it
stays byte-aligned with the in-crate builder.
