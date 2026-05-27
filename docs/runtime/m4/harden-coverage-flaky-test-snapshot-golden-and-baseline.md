# Harden coverage, flaky-test, snapshot/golden, and baseline-truth surfaces — M4 truth packet

This document is the reviewer-facing contract for the M4 stable
coverage / flaky-test / snapshot-golden / baseline-truth packet. The
cross-tool boundary schema lives at
[`schemas/runtime/harden_coverage_flaky_test_snapshot_golden_and_baseline_truth.schema.json`](../../../schemas/runtime/harden_coverage_flaky_test_snapshot_golden_and_baseline_truth.schema.json),
the canonical Rust contract at
[`crates/aureline-runtime/src/harden_coverage_flaky_test_snapshot_golden_and_baseline/`](../../../crates/aureline-runtime/src/harden_coverage_flaky_test_snapshot_golden_and_baseline/),
and the checked-in stable packet at
[`artifacts/runtime/m4/harden_coverage_flaky_test_snapshot_golden_and_baseline_truth_packet.json`](../../../artifacts/runtime/m4/harden_coverage_flaky_test_snapshot_golden_and_baseline_truth_packet.json).

The packet pins one boundary truth that the coverage surface, the
flaky-triage surface, the snapshot/golden surface, the baseline
surface, the release/support packet surface, the AI tool surface, the
CLI/headless inspector, the evidence export, the support export, the
release proof index, the Help/About proof card, and the conformance
dashboard all read. Surfaces MUST NOT mint local copies, hide muted /
quarantined tests, silently promote candidate AI tests into trusted
coverage proof, or collapse the test-source / coverage-impact
vocabularies; they project the packet verbatim.

## Lanes (closed vocabulary)

- `coverage_lane` — line / branch / file coverage and coverage-delta
  truth.
- `flaky_test_lane` — flaky-test triage truth (verdicts, retries,
  attempt history, quarantine/mute state).
- `snapshot_golden_lane` — snapshot / golden-file truth (recorded
  baselines, diff review, accept/reject/promote semantics).
- `baseline_truth_lane` — baseline-truth (baseline mutation,
  governance, AI candidacy chrome).

Adding or removing a lane is a vocabulary change that requires
bumping the schema and updating the Rust contract, the artifact, the
fixture corpus, and this document together.

## Row classes (closed vocabulary)

- `coverage_flaky_snapshot_baseline_quality` — the lane headline.
  Required at `launch_stable` for any lane that claims the M4 grade.
- `wedge_admission` — one row per wedge
  (`stability_verdict_separation`, `quarantine_mute_renewal_truth`,
  `ai_candidate_source_attribution`, `coverage_impact_truth`). All
  four required for any `launch_stable` lane.
- `stability_verdict_admission` — one row per stability verdict
  (`stable`, `flaky`, `failing`, `quarantined`, `muted`, `unknown`).
  All six required for any `launch_stable` lane.
- `quarantine_mute_state_admission` — one row per quarantine-mute
  state (`active`, `expiring_soon`, `expired_pending_renewal`,
  `removed`). All four required for any `launch_stable` lane.
- `test_source_admission` — one row per test-source class
  (`human_authored`, `candidate_ai_test`, `automated_baseline`,
  `imported_ci_evidence`). All four required for any `launch_stable`
  lane. Candidate / automated / imported sources MUST attest the
  session/attempt + review-checkpoint lineage that the
  candidate-lineage admissions cover.
- `coverage_impact_admission` — one row per coverage-impact class
  (`measured`, `estimated`, `stale`, `not_comparable`). All four
  required for any `launch_stable` lane.
- `candidate_lineage_admission` — one row per candidate-lineage class
  (`session_attempt_bound`, `review_checkpoint_bound`,
  `imported_ci_bound`). All three required for any `launch_stable`
  lane.
- `consumer_surface_binding` — one row per consumer surface
  (`coverage_surface`, `flaky_triage_surface`,
  `snapshot_golden_surface`, `baseline_surface`,
  `release_packet_surface`). All five required for any
  `launch_stable` lane. Each row MUST attest the stability-verdict,
  quarantine-mute-state, test-source, coverage-impact, and
  candidate-lineage vocabularies it is required to preserve.
- `lineage_admission` — binds the stable `execution_context_id` (or
  equivalent lineage object) into emitted coverage-quality envelopes
  and downstream consumer surfaces. Required for every
  `launch_stable` lane and MUST surface a non-empty
  `execution_context_id_binding`.
- `known_limit`, `downgrade_automation` — disclosed gap rows. Each
  must carry its disclosure ref.

## Support classes (closed vocabulary)

`launch_stable` is the M4 grade. `launch_stable_below`,
`beta_grade_only`, `preview_only`, and `unsupported` are the precise
narrowed labels; each narrowed row MUST surface a disclosure ref.
`support_unbound` never qualifies for stable promotion.

## Wedges (required per `launch_stable` lane)

Every lane claiming `launch_stable` MUST publish a `wedge_admission`
row for each of:

- `stability_verdict_separation` — the stability verdict (`stable`,
  `flaky`, `failing`, `unknown`) stays observable independently of
  the quarantine-mute state so reviewers can see "this is a flaky
  test that is currently quarantined" instead of one coarse
  status bit.
- `quarantine_mute_renewal_truth` — muted and quarantined tests
  remain visible, filterable, countable, and exportable. Quarantine
  and mute carry explicit `active` / `expiring_soon` /
  `expired_pending_renewal` / `removed` semantics and require
  renewal, expiry, or removal rather than indefinite hidden debt.
- `ai_candidate_source_attribution` — coverage / flaky / snapshot /
  baseline rows preserve a distinct `candidate_ai_test` source class
  so generated tests, baseline edits, ordinary human-authored tests,
  and imported CI evidence never collapse into the same promotion
  or coverage narrative. Candidate / automated / imported rows MUST
  attest the session/attempt + review-checkpoint lineage that the
  candidate-lineage admissions cover.
- `coverage_impact_truth` — coverage impact derived from AI-generated
  or sandbox-run candidate tests stays explicitly `measured`,
  `estimated`, `stale`, or `not_comparable` per target/environment
  family; a single passing run never silently upgrades a candidate
  into trusted stable coverage proof.

A missing wedge auto-narrows the lane below `launch_stable` with a
typed `missing_wedge_admission_coverage` finding.

## Stability verdicts (required per `launch_stable` lane)

| verdict token | meaning |
|---|---|
| `stable` | consistently passing within the verdict window. |
| `flaky` | known-flaky within the verdict window. |
| `failing` | currently failing within the verdict window. |
| `quarantined` | verdict-orthogonal: case is quarantined; the underlying verdict is still observable independently. |
| `muted` | verdict-orthogonal: case output is muted; the underlying verdict is still observable independently. |
| `unknown` | no signal yet. |

A missing verdict auto-narrows the lane below `launch_stable` with a
typed `missing_stability_verdict_coverage` finding.

## Quarantine-mute states (required per `launch_stable` lane)

| state token | meaning |
|---|---|
| `active` | quarantine or mute is in force; renewal window open. |
| `expiring_soon` | renewal window is within the documented warning horizon. |
| `expired_pending_renewal` | renewal has lapsed; the row MUST be renewed or removed before the lane can re-certify. |
| `removed` | quarantine or mute has been lifted; the case returns to ordinary stability-verdict accounting. |

A missing state auto-narrows the lane below `launch_stable` with a
typed `missing_quarantine_mute_state_coverage` finding.

## Test sources (required per `launch_stable` lane)

| source token | meaning |
|---|---|
| `human_authored` | ordinary human-authored tests, baselines, or coverage proof. |
| `candidate_ai_test` | AI-proposed test or AI-proposed baseline mutation in the candidate state. |
| `automated_baseline` | automated baseline change emitted by a reviewer-blessed automation (not promoted from a candidate). |
| `imported_ci_evidence` | coverage / flaky / snapshot / baseline signal imported from an external CI system. |

Candidate / automated / imported sources MUST attest the
session/attempt + review-checkpoint lineage their
`candidate_lineage_admission` rows cover (the row's
`attests_candidate_lineage_bound` flag is required). A missing or
unattested source auto-narrows the lane with a typed
`missing_test_source_coverage` or `candidate_source_not_lineage_bound`
finding.

## Coverage impact (required per `launch_stable` lane)

| impact token | meaning |
|---|---|
| `measured` | impact was measured against the same target/environment family as the trusted stable baseline. |
| `estimated` | impact was estimated; either the target/environment family differs or the sample size is too small for trusted measurement. |
| `stale` | impact is older than the documented freshness window. |
| `not_comparable` | impact cannot be compared (e.g. candidate suite changes the instrumentation contract). |

A missing impact auto-narrows the lane below `launch_stable` with a
typed `missing_coverage_impact_coverage` finding.

## Candidate lineage (required per `launch_stable` lane)

| lineage token | meaning |
|---|---|
| `session_attempt_bound` | candidate is bound to the same session/attempt id as the run that produced it. |
| `review_checkpoint_bound` | candidate is bound to a review checkpoint id (human or automated review). |
| `imported_ci_bound` | imported CI evidence is bound to a stable importer attestation id. |

A missing lineage auto-narrows the lane below `launch_stable` with a
typed `missing_candidate_lineage_coverage` finding.

## Consumer surfaces (required per `launch_stable` lane)

Every `launch_stable` lane MUST publish a `consumer_surface_binding`
row for each of:

- `coverage_surface` — MUST attest `attests_test_source_preserved`
  and `attests_coverage_impact_preserved`.
- `flaky_triage_surface` — MUST attest
  `attests_stability_verdict_preserved`,
  `attests_quarantine_mute_state_preserved`, and
  `attests_test_source_preserved`.
- `snapshot_golden_surface` — MUST attest
  `attests_test_source_preserved` and
  `attests_candidate_lineage_preserved`.
- `baseline_surface` — MUST attest
  `attests_test_source_preserved` and
  `attests_candidate_lineage_preserved`.
- `release_packet_surface` — MUST attest every vocabulary
  (`attests_stability_verdict_preserved`,
  `attests_quarantine_mute_state_preserved`,
  `attests_test_source_preserved`,
  `attests_coverage_impact_preserved`,
  `attests_candidate_lineage_preserved`).

A missing surface or missing attestation auto-narrows the lane below
`launch_stable` with a typed `missing_consumer_surface_coverage` or
`consumer_surface_missing_*_attestation` finding.

## Lineage and `execution_context_id`

A `lineage_admission` row MUST be present on every `launch_stable`
lane with a non-empty `execution_context_id_binding`. Coverage,
flaky-triage, snapshot/golden, and baseline-truth surfaces and their
support / release packets carry the same lineage id so a "why this
verdict?" or "why this baseline mutation?" question always resolves
to the same execution-context object.

## Consumer projections (required)

Every packet MUST carry a projection for each of:

- `coverage_surface`
- `flaky_triage_surface`
- `snapshot_golden_surface`
- `baseline_surface`
- `release_packet_surface`
- `ai_tool_surface`
- `cli_headless`
- `evidence_export`
- `support_export`
- `release_proof_index`
- `help_about`
- `conformance_dashboard`

Each projection MUST preserve the packet id and the thirteen
vocabularies verbatim (`preserves_lane_vocabulary`,
`preserves_row_class_vocabulary`,
`preserves_support_class_vocabulary`,
`preserves_wedge_vocabulary`,
`preserves_stability_verdict_vocabulary`,
`preserves_quarantine_mute_state_vocabulary`,
`preserves_test_source_vocabulary`,
`preserves_coverage_impact_vocabulary`,
`preserves_candidate_lineage_vocabulary`,
`preserves_consumer_surface_vocabulary`,
`preserves_known_limit_vocabulary`,
`preserves_downgrade_automation_vocabulary`,
`preserves_evidence_class_vocabulary`). A projection that collapses
any vocabulary auto-narrows the packet below `launch_stable`.

## Validator findings

The validator emits one or more findings (`info` / `warning` /
`blocker`) per gap. A `blocker` always demotes the packet to
`blocks_stable`; a `warning` demotes it to
`narrowed_below_stable`. The closed finding vocabulary covers
missing identity, missing lane coverage, missing wedge /
stability-verdict / quarantine-mute-state / test-source /
coverage-impact / candidate-lineage / consumer-surface coverage,
missing lineage admission, missing surface attestations, unbound
support / known-limit / downgrade-automation / evidence bindings,
missing or collapsed disclosure refs, candidate sources that fail
their session/attempt + review-checkpoint lineage attestation, raw
runtime material / secret / ambient authority leaks, missing or
drifted consumer projections, and promotion-state mismatch. See
[`mod.rs`](../../../crates/aureline-runtime/src/harden_coverage_flaky_test_snapshot_golden_and_baseline/mod.rs)
for the full list.

## Auto-narrowing

When any required row is missing or any binding is unbound, the
packet is demoted automatically with a typed finding kind. This is
the honesty contract: no lane silently inherits adjacent green
claims, candidate AI tests never quietly upgrade into trusted
stable coverage proof, and quarantined / muted tests never hide
indefinitely behind a coarse pass / fail bit.

## Anchors

- `auto_narrow_on_wedge_admission_gap`
- `auto_narrow_on_stability_verdict_gap`
- `auto_narrow_on_quarantine_mute_state_gap`
- `auto_narrow_on_test_source_gap`
- `auto_narrow_on_coverage_impact_gap`
- `auto_narrow_on_candidate_lineage_gap`
- `auto_narrow_on_consumer_surface_gap`
- `auto_narrow_on_stability_verdict_attestation_gap`
- `auto_narrow_on_quarantine_mute_attestation_gap`
- `auto_narrow_on_test_source_attestation_gap`
- `auto_narrow_on_coverage_impact_attestation_gap`
- `auto_narrow_on_candidate_lineage_attestation_gap`
- `auto_narrow_on_lineage_break`
- `auto_block_on_missing_evidence`

## See also

- Reviewer artifact:
  [`artifacts/runtime/m4/harden-coverage-flaky-test-snapshot-golden-and-baseline.md`](../../../artifacts/runtime/m4/harden-coverage-flaky-test-snapshot-golden-and-baseline.md)
- Generator:
  [`tools/regenerate_harden_coverage_flaky_test_snapshot_golden_and_baseline_truth_packet.py`](../../../tools/regenerate_harden_coverage_flaky_test_snapshot_golden_and_baseline_truth_packet.py)
- Companion test-explorer packet:
  [`docs/runtime/m4/stabilize-the-test-explorer-inline-results-watch-mode.md`](./stabilize-the-test-explorer-inline-results-watch-mode.md)
- Companion debug-fidelity packet:
  [`docs/runtime/m4/harden-breakpoint-call-stack-variables-watch-evaluate-and.md`](./harden-breakpoint-call-stack-variables-watch-evaluate-and.md)
