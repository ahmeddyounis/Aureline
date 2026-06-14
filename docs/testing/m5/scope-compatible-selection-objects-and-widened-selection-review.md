# M5 scope-compatible selection objects, portable selector parity, and widened-selection review

This document is the contract for the **selection objects** the M5 rerun and
triage lanes normalize onto. Where the durable test-item discovery contract lands
the discovered nodes and snapshots, this contract makes those durable targets
safely *selectable* and *re-runnable*: a selection is a durable product object —
not an ad hoc display-name match — that is portable across UI, CLI, AI, and
support packets only while it stays snapshot- and target-compatible.

A rerun stays trustworthy only if it either preserves the exact originating
selection or opens a widened-selection review before it executes. This contract
makes that guarantee structural.

## Source of truth

- Packet type: `PortableSelectionPacket`
  (`crates/aureline-runtime/src/scope_compatible_selection_objects_and_widened_selection_review/`).
- Boundary schema:
  `schemas/testing/scope-compatible-selection-objects-and-widened-selection-review.schema.json`.
- Checked support export:
  `artifacts/testing/m5/scope-compatible-selection-objects-and-widened-selection-review/support_export.json`.
- Markdown summary:
  `artifacts/testing/m5/scope-compatible-selection-objects-and-widened-selection-review.md`.
- Protected fixtures:
  `fixtures/testing/m5/scope-compatible-selection-objects-and-widened-selection-review/`.

Regenerate the canonical export and summary after any shape change:

```bash
cargo run -p aureline-runtime --example dump_scope_compatible_selection
cargo run -p aureline-runtime --example dump_scope_compatible_selection summary
```

## Selection objects

A `SelectionObject` ties a chosen set of `SelectionTarget`s to:

- the `SnapshotFingerprint` (snapshot id + non-display digest + consumer token) it
  was resolved against;
- an `ExpansionPolicy` (`pinned_exact`, `reresolve_within_snapshot`,
  `allow_widen_with_review`, or `frozen_imported_read_only`) that decides whether
  a re-resolution may widen, must stay pinned, or must fail closed;
- a `SelectionQuery` (include / exclude tokens and an optional `changed_since_ref`)
  so a re-resolution is reproducible.

Rerun, rerun-failed, CLI selectors, AI test plans, and support / export packets
all normalize onto the same object through a shared `SelectorChannel` (`ui`,
`cli`, `ai`, `support`) and `SelectionIntentKind` (`explicit_items`, `rerun_all`,
`rerun_failed`, `changed_since`, `snapshot_scoped`). The packet validation
requires the four channels and the `rerun_all`, `rerun_failed`, `changed_since`,
and `snapshot_scoped` intents to each be represented, so the parity is exercised,
not merely declared.

Identity rules every selection obeys (`SelectionObject::is_valid`):

- **A target's fingerprint is never its bare id.** Each `SelectionTarget` carries
  a `target_fingerprint_token` distinct from its `target_id`
  (`fingerprint_substitutes_identity`).
- **Templates stay distinct from invocations.** A target carries a
  `DurableTestNodeKind`; the packet requires both `parameterized_template` and
  `concrete_invocation` to appear (`template_collapsed_with_invocation`).
- **Imported selections are read-only.** Any selection whose snapshot consumer,
  expansion policy, or pinned target is imported must use the
  `frozen_imported_read_only` policy (`imported_rerun_as_local`).
- A `changed_since` selection carries a `changed_since_ref`; a `snapshot_scoped`
  selection carries at least one include query token.

## Compatibility assessment and widened-selection review

Before a rerun executes, `SelectionObject::assess_against` compares the selection
to the *current* discovery state and produces a
`SelectionCompatibilityAssessment` whose `TargetCompatibilityClass` is one of:

| class | meaning | review state |
| --- | --- | --- |
| `compatible` | snapshot + all target fingerprints match | `not_required` |
| `widened_needs_review` | re-resolution would add targets | review opens |
| `narrowed_needs_review` | re-resolution would drop targets | review opens |
| `snapshot_drifted` | snapshot fingerprint changed | review opens |
| `target_fingerprint_mismatch` | a target's fingerprint changed | review opens |
| `imported_not_rerunnable` | imported / provider-owned | `blocked` |

Only a `compatible` assessment dispatches without review
(`SelectionCompatibilityAssessment::dispatch_allowed`). A drifted, widened,
narrowed, or fingerprint-changed assessment stays `pending` — and so cannot
dispatch — until a reviewer records a decision: `approved_as_adjusted` runs the
re-resolved scope, while `rejected_keep_original` runs the original pinned scope.
An imported assessment is `blocked` and never re-dispatches as a local rerun.

Consistency (`SelectionCompatibilityAssessment::is_consistent`) binds the recorded
class, review state, and target deltas together: a widening or drift can never sit
behind a `not_required` state (`widening_hides_review`), and `preserves_origin` is
true only for a `compatible` selection or a `rejected_keep_original` decision.

## Reconstructability

Every assessment names its `selection_ref`, and the packet validation requires it
to resolve to a selection present in the export
(`assessment_selection_unresolved`); the recorded `snapshot_drifted` flag must
agree with the referenced selection's snapshot fingerprint
(`snapshot_drift_flag_mismatch`). Together with the pinned targets' fingerprint
tokens, support and release evidence can reconstruct the exact selection and
target fingerprint used for any rerun or triage path.

## Boundary discipline

The packet carries only typed class tokens, booleans, opaque ids, fingerprint
digests, and redaction-aware reviewable labels. Raw test source, raw provider
payloads, raw query bodies, provider cursors, credentials, and raw artifact bodies
never cross this boundary.
