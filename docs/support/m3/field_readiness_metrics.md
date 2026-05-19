# M3 field-readiness metrics

This document is the reviewer-facing companion to the protected M3
field-readiness scorecards. It explains what the seeded scorecards
measure, how the closed vocabularies bound their rows, how the
stale-data downgrades fire, and how QE, support, and release review
consume the projection.

The implementation lives in
[`crates/aureline-support/src/field_readiness/mod.rs`](../../../crates/aureline-support/src/field_readiness/mod.rs)
and the protected drill harness is at
[`crates/aureline-support/tests/m3_field_readiness.rs`](../../../crates/aureline-support/tests/m3_field_readiness.rs).

## Three release-consumable scorecards from one corpus

The field-readiness projection turns the seeded M3 support-scenario
corpus at
[`fixtures/support/m3/scenario_corpus/`](../../../fixtures/support/m3/scenario_corpus/)
into three governed surfaces that release review, shiproom, and the
support-center can consume verbatim:

1. **Diagnosis-latency scorecard**
   [`artifacts/support/m3/diagnosis_latency_scorecard.md`](../../../artifacts/support/m3/diagnosis_latency_scorecard.md)
   — per-lane seeded budgets for time-to-first-actionable-finding,
   time-to-first-safe-repair-suggestion, and
   time-to-escalation-packet-completion, attributed per evidence-path
   class (`local_only`, `exported_to_support_packet`,
   `uploaded_to_vendor`).
2. **Exact-build availability report**
   [`artifacts/support/m3/exact_build_availability_report.md`](../../../artifacts/support/m3/exact_build_availability_report.md)
   — per-lane availability of exact-build identity and (where the
   evidence contract requires it) crash-symbolication report refs.
3. **Field-readiness dashboard**
   [`artifacts/support/m3/field_readiness_dashboard.json`](../../../artifacts/support/m3/field_readiness_dashboard.json)
   — per-lane current state, escalation-packet completeness,
   seeded false-safe-repair rate, claim-downgrade trigger coverage,
   and stale-data triggers; consumed by shiproom and release-evidence
   surfaces.

The scorecards reuse the alpha measurement window
([`artifacts/support/diagnosis_latency_scorecard_alpha.yaml`](../../../artifacts/support/diagnosis_latency_scorecard_alpha.yaml))
verbatim: timer starts at `support_scenario_started`, stops at
`first_actionable_result_packet_emitted`, with p90 target 600s, yellow
720s, red 900s. The escalation-packet window is doubled because the
export-support-packet step is the final drill step in every scenario.

## What this row owns

- The closed `EvidencePathClass` vocabulary
  (`local_only`, `exported_to_support_packet`, `uploaded_to_vendor`)
  and the `REQUIRED_EVIDENCE_PATH_CLASSES` set the projection refuses
  to ship without. Every lane row carries one budget per required
  class so latency and packet-completeness measures stay attributable
  to the user's chosen path.
- The closed `LatencyMeasurementState` vocabulary
  (`seeded_pending_live_measurement`, `stale_downgraded`,
  `live_green`, `live_yellow`, `live_red`). Live measurement is not
  yet wired; the seeded state remains
  `seeded_pending_live_measurement` until live timers replace the
  seeded values.
- The closed `StaleDataTrigger` vocabulary
  (`seeded_corpus_missing_lane`, `primary_fixture_missing`,
  `alpha_scorecard_missing`, `drill_report_older_than_corpus`,
  `symbolication_evidence_missing`,
  `claim_downgrade_rules_incomplete`) that flips a lane row to
  `stale_downgraded` and refuses release consumability.
- The `M3DiagnosisLatencyScorecard`, `ExactBuildAvailabilityReport`,
  and `FieldReadinessDashboard` projection types with build-time
  `include_str!` lineage through the M3 corpus, and the
  `is_release_consumable()` contract on each that the protected drill
  harness enforces.

## Acceptance and how this row meets it

- **Claimed supportability rows have one current, exportable
  scorecard instead of relying on anecdotal diagnosis success.** The
  projection emits one row per protected beta lane, sourced from the
  governed corpus rather than reviewer prose; the three scorecards
  are checked into the source tree at stable paths so shiproom and
  release evidence quote the same record set support cites.
- **Shiproom and release packets can see diagnosis-latency medians,
  exact-build availability percentages, and packet-completeness
  results without reading raw logs.** Every row carries seeded p50
  and p90 targets, exact-build and symbolication percentages, and
  escalation-packet completeness percentages with closed-vocabulary
  state tokens. Reviewers never read raw private material or
  freeform prose to determine release readiness.
- **Scorecards downgrade when the seeded scenario corpus or
  symbolication evidence is stale instead of preserving false green
  status.** When a lane is missing, a fixture is absent, the alpha
  scorecard cannot be read, the drill report is older than the
  corpus, or a scenario's claim-downgrade rules drop a required
  trigger, the scorecards flip the affected rows to
  `stale_downgraded` and refuse release consumability through the
  `is_release_consumable()` contract.

## Stale-data downgrades

The projection never preserves false-green status. Each
`StaleDataTrigger` token has a precise refusal posture:

- `seeded_corpus_missing_lane` — release candidate cannot promote
  past M3 until the missing lane's scenario is restored.
- `primary_fixture_missing` — drill cannot replay; row downgrades
  until the fixture is restored on disk.
- `alpha_scorecard_missing` — measurement window cannot be
  inherited; all rows downgrade until the alpha scorecard parses.
- `drill_report_older_than_corpus` — the corpus has moved since the
  drill-harness baseline was last regenerated; rerun the drill
  harness before quoting the scorecards.
- `symbolication_evidence_missing` — applies to lanes whose evidence
  quotes a crash dump or runtime-evidence packet (safe-mode,
  extension-bisect, runtime-replay).
- `claim_downgrade_rules_incomplete` — a scenario's
  `claim_downgrade_rules` dropped a required trigger; the corpus's
  own refuse-closed contract has fired and the scorecards reflect it.

## How to refresh

1. Run the protected drill:
   `cargo test -p aureline-support --test m3_field_readiness`.
2. The test recomputes the three scorecards from the corpus and the
   alpha scorecard, asserts the metadata-safe and release-consumable
   baseline, and compares the JSON dashboard against the checked-in
   baseline.
3. To refresh the checked-in JSON after an intentional projection
   change, set `AURELINE_UPDATE_BASELINE=1` and rerun the test.
4. When a beta lane is added, register it in
   `M3BetaLaneClass::REQUIRED_BETA_LANE_CLASSES`, seed a scenario
   fixture, update the drill-harness report, and refresh the three
   field-readiness artifacts in the same change so reviewer view
   stays in lockstep with the projection.

## Related contracts

- [Support-scenario corpus](support_scenario_corpus.md) — the seeded
  scenarios the projection consumes.
- [M3 drill-harness report](../../../artifacts/support/m3/drill_harness_report.md)
  — the per-lane coverage baseline this projection complements.
- [Alpha diagnosis-latency scorecard](../../../artifacts/support/diagnosis_latency_scorecard_alpha.yaml)
  — the measurement window and target budgets this projection
  inherits.
- [Recovery-ladder alpha](../recovery_ladder_alpha.md) — the parent
  recovery-ladder contract every M3 beta lane plugs into.

## Out of scope for this row

- Live wall-clock measurement of scenario latency. The corpus and
  alpha measurement contract define the window; live timers will
  replace the seeded `seeded_pending_live_measurement` state in a
  later row.
- Live symbolication coverage measurement on production crashes; the
  seeded availability percentage is bound to the scenario's evidence
  contract, not a live crash-loop signal.
- A generic org-wide observability dashboard outside the Aureline
  supportability lane. This row only owns the field-readiness
  scorecards for the protected M3 beta lanes.
