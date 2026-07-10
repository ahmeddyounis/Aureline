# M5 flaky-state-badge / retry-history-row primitive

This document is the contract reference for the reusable M5 **flaky-state badge** and
**retry-history row** — two governed test-intelligence components implemented as one twin
primitive in the `aureline-runtime` crate
(`implement_flaky_state_badges_and_retry_history_rows_with_controlled_verdict_vocabulary_classifier_confidence_retry_window_visibility_environment_drift_notes_and_rerun_or_open_logs_parity_across_claimed_m5_quality_surfaces`).

It narrows two of the seven families frozen by the
[test-intelligence component matrix](m5_test_intelligence_component_matrix.md) —
`flaky_state_badge` and `retry_history_row` — into two resolvers plus a parity matrix, so a
flaky verdict stays proportional to its evidence instead of folklore and a retry-history row
keeps enough context to explain why the same test passed here and failed there.

## Why this exists

A user should never trust a red "flaky" badge without knowing how confident the classifier is,
how large the evidence window was, where the verdict came from, and whether the test is muted or
quarantined. And a retry-history row should never hide why the same test diverged across
attempts, nor lose the path back to the raw attempt logs. This primitive makes each of those
states explicit and identical across every claimed quality consumer.

## Flaky-state badge

`resolve_flaky_state_badge` takes one badge's flaky classification, classifier confidence class,
classifier source, provenance class, mute state, retry-window size, observed-failure count, and
last outcome, and derives a **flaky posture** that is one-to-one with the flaky classification:

| Flaky classification | Flaky posture |
| --- | --- |
| `stable` | `stable_badge` |
| `suspected_flaky` | `suspected_flaky_badge` |
| `reproduced_flaky` | `reproduced_flaky_badge` |
| `stable_again` | `stable_again_badge` |
| `manually_muted` | `manually_muted_badge` |
| `unknown_flaky` | `unknown_flaky_badge` |

Because the map is one-to-one, a suspected verdict never borrows a reproduced posture. A
**reproduced-flaky verdict is only accepted when its retry window and observed-failure count meet
the required evidence threshold** (`REQUIRED_REPRODUCED_WINDOW` / `REQUIRED_REPRODUCED_FAILURES`)
and its confidence is not a single occurrence or insufficient data; otherwise resolution fails
with `ReproducedWithoutEvidenceWindow`. This is the acceptance-criterion guarantee: one
intermittent failure can never visually masquerade as reproduced flakiness. The retry window,
classifier source, confidence, last outcome, and mute status are always carried.

Actions: `reveal_flaky_details`, `open_retry_history`, `rerun_test`, and `export_flaky_badge` are
always offered; `mute_or_quarantine` whenever muting is not policy-blocked.

## Retry-history row

`resolve_retry_history_row` takes one row's last outcome, its recent outcomes in order, its retry
scope class, its attempt origin, its confidence class, its provenance class, and its environment
/ build / runtime delta flags, and derives a **retry posture** that is one-to-one with the last
attempt outcome:

| Retry outcome | Retry posture | Needs attention? |
| --- | --- | --- |
| `passed_first_try` | `passed_first_try_row` | no |
| `passed_on_retry` | `passed_on_retry_row` | yes |
| `failed_all_retries` | `failed_all_retries_row` | yes |
| `errored_attempt` | `errored_row` | yes |
| `skipped_attempt` | `skipped_row` | no |
| `aborted_attempt` | `aborted_row` | yes |

Because the map is one-to-one, a pass-on-retry never reads as a clean first-try pass and the row
never invents an alternate label. A **divergence** (a pass and a failure across the recent
outcomes, or a pass-on-retry) must carry an ordered sequence of at least two outcomes; otherwise
resolution fails with `DivergenceWithoutSequence`, so a row always preserves enough context to
explain divergent outcomes across local, remote, notebook, and imported-CI attempts. The
environment / build / runtime deltas, the attempt origin, and a durable path back to the raw
attempt logs (`has_log_continuity`) are always carried.

Actions: `reveal_retry_details`, `rerun_test`, `open_logs`, and `export_retry_history` are always
offered — the rerun-or-open-logs parity the acceptance criteria name.

## Parity matrix

`M5FlakyRetryComponentsPacket` binds one row per claimed quality consumer — the flaky dashboard,
the editor / test-tree flaky badge, the retry-history panel, the headless/CLI flaky-retry
surface, and the flaky-retry export — to the shared badge and row anatomy, vocabulary, postures,
actions, export fields, and non-visual accessibility routes, so the same flaky / retry grammar
holds across the dashboard, the editor, the retry panel, CI/headless, and support consumers with
identical vocabulary. Each row carries four hard invariants (all `false`):

- `labels_intermittent_as_confirmed_flaky`
- `hides_retry_window_or_classifier_source`
- `drops_env_build_runtime_delta_context`
- `invents_alternate_flaky_or_retry_state_label`

## Boundary

Raw test payloads, pasted paths, credentials, and private endpoints stay outside the export
boundary; every badge identity, test identity, and attempt-log ref is carried only as an opaque,
export-safe representation.

## Artifacts

- Canonical packet schema: `schemas/ui/m5-flaky-state-badge.schema.json`
- Retry-history-row companion schema: `schemas/ui/m5-retry-history-row.schema.json`
- Support export: `artifacts/release/m5-flaky-retry-primitive-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-flaky-retry-primitive-proof/matrix.csv`
- Design report: `artifacts/design/m5-flaky-retry-primitive.md`
- Narrowed fixtures: `fixtures/ui/m5-flaky-retry-primitive/`

All are minted from the seed builders by the `aureline_runtime_flaky_retry_primitive` headless
emitter; the checked-in support export is asserted equal to the seed builder in tests.
