# Quality-engineering strategy seed

This document is the seed strategy for Aureline's quality-engineering
(QE) lane. It names the protected test lanes, the scenario coverage
they are responsible for, and the release-blocking posture each lane
carries, so protected paths are governed by declared test contracts
rather than by ad hoc per-change test choices.

The human-readable strategy lives here. The machine-readable contracts
live in:

- [`/artifacts/qe/test_lane_registry.yaml`](../../artifacts/qe/test_lane_registry.yaml)
  — every protected test lane with owner, scope, cadence, blocking
  level, artifact sink, scenario coverage, and incident-learning rule.
- [`/artifacts/qe/release_blocking_rules.yaml`](../../artifacts/qe/release_blocking_rules.yaml)
  — mechanical policy that turns severity class, protected-metric
  posture, incident class, sequence-packet failure, and waiver posture
  into merge or release verdicts.
- [`/artifacts/qe/quality_scenario_hooks.yaml`](../../artifacts/qe/quality_scenario_hooks.yaml)
  — scenario rows that bind protected user journeys, contract-family
  freeze exceptions, and risk-register entries to the lanes that must
  prove them.

If the narrative here disagrees with any of those files, the files are
authoritative for tooling and this document is updated in the same
change.

## Operating rule

Quality engineering is **part of architecture**, not a later add-on.
Every protected change — protected lane, public-truth surface,
release-eligible packet family, or public-facing claim — resolves
against this seed before a reviewer accepts it:

1. Which test lanes in the registry cover the change?
2. What is each lane's declared blocking posture for this severity and
   this protected-metric state?
3. Is the scenario coverage explicit enough that a reviewer can point
   at a row rather than reciting team knowledge?
4. If a lane is missing coverage, is the gap named on the scenario-
   hooks row and tied to a risk or freeze exception, or is the change
   widening silent coverage?

A change that cannot answer all four without a waiver MUST NOT widen
a public claim, promote to a stable-facing artifact family, or close a
protected freeze exception.

## Required test lanes

The registry declares at least one row per lane below. Adding a lane
is additive-minor; renaming or retiring a lane is breaking and opens a
decision row in `artifacts/governance/decision_index.yaml`.

| Lane id | Purpose |
|---|---|
| `property_and_model_tests` | Property-based and model-based tests for VFS, buffer, command identity, reactive-truth composition, and any protected packet family whose invariants can be expressed as properties rather than example cases. |
| `fixture_repo_integration` | End-to-end tests that drive protected flows against the reference-workspace fixtures and the protected benchmark corpus at a declared manifest revision. |
| `rendering_layout_goldens` | Deterministic renderer, text-stack, layout, and icon goldens captured under pinned display-class, DPI, and locale posture; drives layout and shaping regression review. |
| `accessibility_regression` | Screen-reader, keyboard, focus-order, reduced-motion, contrast, and IME regression using the accessibility task corpus and assistive-tech matrix. |
| `protocol_parser_fuzzing` | Structure-aware fuzzing for RPC transports, parser surfaces, settings-resolver input, and boundary schemas; targets crashes, panics, and contract violations. |
| `chaos_fault_injection` | Fault-injection suites for connectivity loss, deferred-intent reconciliation, credential-broker downgrade, trust-downgrade, remote-agent reconnect, and repair rollback. |
| `performance_ci` | Benchmark-lab and performance-CI runs tied to protected fitness functions, the corpus manifest, and the quarantine policy; gates regressions against declared thresholds. |
| `protected_sequence_packet_validation` | Validates protected-sequence packet families (reconnect sequences, browser-handoff sequences, trust-state transitions, remote-agent hello/heartbeat, repair rollback chronology) against their schemas and fixture seeds. |

These eight lanes are the floor. Protected cross-cutting areas named
in the registry route their scenarios through one or more of these
lanes; they do not invent a parallel lane vocabulary.

## Cross-cutting scenario coverage

The registry enumerates cross-cutting scenario rows for each of the
following areas. Each row cites the lanes that must exercise it, the
fixture or corpus the lane pulls from, the artifact sink where the
evidence lands, the incident-learning rule for a miss, and the
protected-metric or contract-family row the scenario guards.

- `provider_arbitration_truth`
- `diagnostics_code_action_convergence`
- `terminal_run_test_debug_replay_safety`
- `git_merge_history_edit_recovery`
- `connectivity_reconciliation`
- `interaction_integrity_conformance`
- `portability_export_integrity`
- `localization_locale_pack_parity`
- `theme_icon_motion_package_validation`
- `onboarding_voice_accessibility`
- `repair_rollback`

The scenario-hooks file is the canonical home for those rows; the
driver/SLO matrix and the future scenario library MUST cite scenario
row ids verbatim and MUST NOT reword lane intent. A scenario that
cannot point to at least one lane row is a validation failure.

## Blocking posture vocabulary

Every lane declares one of the following blocking postures per
severity class. Release-blocking rules compose lane postures; they
never silently override them.

| Blocking posture | Meaning |
|---|---|
| `merge_blocking` | Failure in this lane blocks merge on any change that touches the lane's protected scope. |
| `release_blocking` | Failure blocks promotion into the affected channel or artifact family, even if merge already landed. |
| `widening_blocking` | Failure does not block merge or a same-posture rebuild, but blocks widening a claim row, ring advancement, or a support-class promotion. |
| `dashboard_only_observation` | Failure surfaces on the shiproom and QE dashboards for observation; requires a named escalation before it becomes blocking for a subsequent change. |
| `seeded_not_enforcing` | Lane is declared but not yet producing enforced evidence; the registry row names the gap and the risk or freeze exception that covers it. |

No other posture values are allowed. A lane that reports a value
outside this list is non-conforming.

## Cadence vocabulary

| Cadence | Meaning |
|---|---|
| `each_change` | Runs on every PR that touches the lane's protected paths. |
| `pre_merge_required` | Runs before merge on protected changes; may be sampled on unrelated PRs. |
| `nightly` | Runs once per day on `main`; feeds the QE dashboard. |
| `pre_release_required` | Required in the release-evidence packet before a candidate widens. |
| `per_milestone` | Refreshed at milestone boundaries and on re-baseline events. |
| `on_incident_only` | Fires from incident or freeze-exception triggers; not scheduled. |

## Artifact-sink vocabulary

Every lane row names exactly one canonical artifact sink so evidence
is discoverable without reading narrative prose. The register uses
these sinks:

- `benchmark_lab` — `artifacts/benchmarks/` and `artifacts/bench/`.
- `release_evidence` — `artifacts/release/` and `schemas/release/`.
- `support_export` — `artifacts/support/` and `schemas/support/`.
- `governance_packets` — `artifacts/governance/`.
- `accessibility_review` — `artifacts/accessibility/`.
- `qe_dashboard` — `artifacts/qe/dashboard/` (sinks not yet seeded
  are allowed; the row MUST say so explicitly via
  `artifact_sink_posture: planned_not_yet_seeded`).

## Incident-learning rule

Every lane row names exactly one incident-learning rule that the
shiproom and the security-trust-review forum compose against when a
post-incident review lands. Rules are drawn from:

- `add_property_or_model_row` — the root cause is expressible as a
  property; the lane adds a property row and the scenario hook gains a
  failing fixture.
- `add_fixture_or_corpus_row` — the root cause is expressible as a
  fixture seed; the corpus manifest gains the fixture and the scenario
  hook cites it.
- `add_protected_metric_row` — the root cause is expressible as a
  fitness-function or protected-metric row; the benchmark council
  records the row.
- `add_golden_row` — the root cause is a rendering, layout, token, or
  localization regression; the goldens lane captures it.
- `add_fuzz_corpus_row` — the root cause is a parser or protocol crash
  or contract violation; the fuzz corpus gains the minimised input.
- `add_fault_injection_row` — the root cause is a chaos or fault path
  that was not previously exercised.
- `add_sequence_fixture_row` — the root cause is a sequence-packet
  mis-ordering, dropped transition, or silent replay.
- `tighten_existing_row` — an existing row failed to block; the rule
  is tightened (threshold, posture class, waiver scope) rather than a
  new row added.
- `open_decision_row` — the learning is ambiguous or cross-cutting and
  opens a decision-index row before any lane change lands.

The registry MUST include the rule per row. A change that closes an
incident ticket without a declared rule is non-conforming.

## Review forum

The QE strategy is reviewed by the architecture council, co-required
with the performance council on benchmark-lab changes, the security-
trust review on chaos and fuzzing changes, and the accessibility
review on accessibility-regression and goldens changes.

## Out of scope

The seed does not implement every lane or stand up full CI depth. It
names lanes, declares contracts, and reserves artifact sinks so that
implementation can land against one declared target rather than
inventing new lane vocabulary per feature. Milestone and task planning
metadata are never surfaced in lane ids, lane titles, or lane notes.
