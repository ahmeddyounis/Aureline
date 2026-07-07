# M5 failure-triage-panel / quarantine-review-sheet / environment-matrix-card primitive

This document is the contract reference for the reusable M5 **failure-triage panel**,
**quarantine/mute review sheet**, and **environment-matrix card** — three governed test-quality
components implemented as one triple primitive in the `aureline-runtime` crate
(`implement_failure_triage_panels_quarantine_review_sheets_and_environment_matrix_cards_with_assertion_diff_summaries_recent_attempts_env_build_runtime_deltas_owner_expiry_release_impact_and_rerun_debug_review_parity_across_claimed_m5_quality_surfaces`).

It narrows the last three of the seven families frozen by the
[test-explorer / watch / triage component matrix](m5_test_explorer_watch_triage_component_matrix.md)
— `failure_triage_panel`, `quarantine_review_sheet`, and `environment_matrix_card` — into three
resolvers plus a parity matrix, so a red test row stops leading straight to destructive
suppression without evidence, a quarantined or muted test stops disappearing into a hidden
filtered state, and an environment card stops implying safe equivalence across environments that
are not actually compatible.

## Why this exists

M5 cannot honestly claim rich test intelligence if a user has to infer whether a red mark can be
suppressed without first seeing evidence, what a mute/quarantine will hide from release, or
whether two environments are safely equivalent. This primitive makes triage evidence, suppression
truth, and environment compatibility explicit and identical across every claimed quality surface:
the test-explorer triage view, the editor inline triage, the notebook triage view, the run-panel
triage, and the quality report export.

## Failure-triage panel

`resolve_failure_triage_panel` takes one failure's category, triage disposition, result origin,
classifier confidence, recent attempt sequence, and whether it carries an assertion/diff summary
and environment/build/runtime deltas, and derives a **triage posture** one-to-one with the failure
category:

| Failure category | Triage posture |
| --- | --- |
| `assertion_failure` | `assertion_evidence_panel` |
| `runtime_error` | `runtime_evidence_panel` |
| `timeout` | `timeout_evidence_panel` |
| `environment_error` | `environment_evidence_panel` |
| `flaky_under_review` | `flaky_review_panel` |
| `unknown_failure` | `unclassified_evidence_panel` |

The recent attempt sequence must be non-empty — a panel always shows recent attempts. The panel
**provides evidence context** whenever it carries an assertion/diff summary, env/build/runtime
deltas, or a recent attempt sequence, and the `open_quarantine_review` action — the only route to
a destructive suppression — is offered **only once evidence context is present**. This is the
acceptance-criterion guarantee that a user never jumps from a red row straight to suppression
without evidence. `rerun_exact_selection` and `reveal_triage_evidence`/`export_triage` are always
offered; `open_debug_session` only for a live-local origin (an imported CI failure cannot be
attached to a local debugger).

## Quarantine/mute review sheet

`resolve_quarantine_review_sheet` takes one suppression's kind, scope, ownership, release impact,
expiry state, linked-artifacts flag, reason, and owner, and derives an honesty-first **review
posture**:

1. `expired_suppression` — the suppression has expired.
2. `unowned_suppression` — no accountable owner (unowned or owner-expired).
3. `hidden_release_suppression` — hidden from release gating.
4. `review_due_suppression` — a review is due.
5. `blocking_suppression` — still blocks release.
6. `governed_suppression` — owned, non-expired, disclosed (the only posture needing no attention).

The suppressed test **always stays visible** (never hidden behind a filter), the reason is always
preserved, the owner/expiry/release impact are always carried, and `restore_test` is **always**
offered. `renew_suppression` appears for an expired or review-due suppression; `reassign_owner`
for an unowned or owner-expired one; `open_linked_artifacts` when linked artifacts are present.
This is the acceptance-criterion guarantee that quarantined or muted tests stay visible with
owner/expiry/release-impact truth instead of disappearing into a hidden filtered state.

## Environment-matrix card

`resolve_environment_matrix_card` takes one card's target class, primary environment lane, and at
least two compared environment legs — each carrying its target/runtime/toolchain/build
compatibility class — and derives the **overall compatibility** as the worst axis across every
leg, mapped to a **card posture**:

| Overall (worst) compatibility | Card posture |
| --- | --- |
| `incompatible` | `incompatible_matrix` |
| `unverified` | `unverified_matrix` |
| `partially_compatible` | `mixed_matrix` |
| `fully_compatible` / `not_applicable` | `compatible_matrix` |

The card **never asserts safe equivalence**: `asserts_safe_equivalence` is always `false`, and any
non-compatible posture raises `warns_on_incompatibility`. `rerun_on_leg` is offered only when at
least one leg is runnable (fully or partially compatible on the target axis). This is the
acceptance-criterion guarantee that an environment card compares compatibility classes without
implying safe equivalence across incompatible environments.

## Parity matrix, invariants, and export

A single `M5QualityTriageStatusPacket` binds one row per claimed quality surface to the shared
triage/quarantine/environment anatomy, controlled vocabulary, postures, bounded actions, export
fields, and non-visual accessibility routes. Each row carries worked resolution cases proving all
three resolvers and four hard invariants (all `false`):

- `offers_suppression_without_evidence`
- `hides_owner_expiry_or_release_impact`
- `implies_safe_environment_equivalence`
- `drops_recent_attempts_or_deltas`

The packet validates full posture coverage (every triage/quarantine/environment posture and every
classifier confidence exercised), the AC coverage lints (evidence-gated open-review, hidden-yet-
visible suppression, incompatible-yet-non-equivalent environment), recent-attempt and restore-action
preservation, identity preservation, governance review, consumer projection, proof freshness, and
release/support parity. Raw log bodies, pasted paths, credentials, and private endpoints never cross
the export boundary; every reason, label, and identity is carried only as an opaque, export-safe
representation.

## Sources of truth

- Schemas: `schemas/ui/m5-test-failure-triage-panel.schema.json` (canonical packet),
  `schemas/ui/m5-test-quarantine-review-sheet.schema.json`,
  `schemas/ui/m5-test-environment-matrix-card.schema.json`.
- Bound contracts: `schemas/testing/test_quarantine_record.schema.json`,
  `schemas/testing/stability-verdicts-quarantines-and-release-visibility.schema.json`,
  `schemas/testing/session-plans-attempt-records-and-execution-lineage.schema.json`.
- Proof: `artifacts/release/m5-failure-triage-quarantine-environment-primitive-proof/`
  (`support_export.json`, `matrix.csv`) and
  `artifacts/design/m5-failure-triage-quarantine-environment-primitive.md`.
- Fixtures: `fixtures/ui/m5-failure-triage-quarantine-environment-primitive/`.

Regenerate every artifact with the headless emitter:

```sh
cargo run -q -p aureline-runtime --bin aureline_runtime_failure_triage_quarantine_environment_primitive -- support-export
cargo run -q -p aureline-runtime --bin aureline_runtime_failure_triage_quarantine_environment_primitive -- csv
cargo run -q -p aureline-runtime --bin aureline_runtime_failure_triage_quarantine_environment_primitive -- report
```
