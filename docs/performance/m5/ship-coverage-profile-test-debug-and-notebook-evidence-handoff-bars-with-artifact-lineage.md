# Ship Coverage, Profile, Test, Debug, and Notebook Evidence Handoff Bars with Artifact Lineage

This document is the reviewer-facing landing page for the M5 evidence-handoff bar
and artifact lineage lane.

## Scope

This lane governs how coverage, profiler, test, debug, and notebook surfaces:

- present evidence handoff bars that keep related evidence connected—showing
  originating run/test/build, artifact/build ID, commit or revision, capture
  source, and save/share scope—so users always know what they are looking at;
- carry artifact lineage that binds evidence to source run, build identity,
  environment fingerprint, capture mode, mapping quality, freshness, and
  provenance so attribution never gets lost;
- classify capture sources honestly as local live, remote live, imported,
  cached, provider-supplied, CI-provided, synthetic, sampled, instrumented,
  estimated, or partial;
- define save/share scope with visible redaction mode and destination class so
  export, share, upload, and attach-to-incident actions remain explicit and
  attributable.

## Canonical Artifacts

- **Implementation:** `crates/aureline-profiler/src/ship_coverage_profile_test_debug_and_notebook_evidence_handoff_bars_with_artifact_lineage/`
- **Packet:** `artifacts/perf/m5/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage.json`
- **Schema:** `schemas/perf/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage.schema.json`
- **Fixtures:** `fixtures/performance/m5/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage/`

## Surfaces

| Surface | Claim | Rationale |
|---|---|---|
| Coverage handoff bar | Stable | Shows originating run, build ID, commit, capture source, save/share scope, lineage state, and lineage detail. |
| Profile handoff bar | Stable | Shows originating run, build ID, commit, capture source, save/share scope, lineage state, and lineage detail. |
| Test handoff bar | Stable | Shows originating run, build ID, commit, capture source, save/share scope, lineage state, and lineage detail. |
| Debug handoff bar | Stable | Shows originating run, build ID, commit, capture source, save/share scope, lineage state, and lineage detail. |
| Notebook handoff bar | Stable | Shows originating run, build ID, commit, capture source, save/share scope, lineage state, and lineage detail. |
| Export review | Preview | Redaction-safe export flows for evidence handoff are still under qualification. |
| Support export | Preview | Support-bundle redaction for evidence handoff payloads is still under qualification. |

## Handoff-Bar Rows

Handoff-bar rows carry:

- `originating_run_ref` — the run, test, or build that produced the evidence;
- `artifact_build_id` — exact build identity at capture time;
- `commit_or_revision` — source control commit or revision;
- `capture_source_ref` — reference to a classified capture source;
- `save_share_scope_ref` — reference to a save/share scope;
- `lineage_ref` — reference to the deeper artifact lineage row;
- `lineage_state` — `exact_match`, `probable_mismatch`, `source_only`, `artifact_only`, `restricted_by_policy`, or `unavailable`.

Every handoff-bar row MUST show origin, build ID, commit, capture source,
save/share scope, lineage state, and lineage detail.

## Artifact Lineage Rows

Artifact lineage rows carry:

- `evidence_kind` — `coverage`, `profile`, `trace`, `memory_snapshot`, `test_result`, `debug_session`, `notebook_output`, or `replay_timeline`;
- `source_run_ref` — the originating run;
- `build_identity_ref` — build identity;
- `environment_fingerprint_ref` — normalized environment;
- `capture_mode_ref` — capture mode descriptor;
- `mapping_quality_ref` — mapping quality descriptor;
- `freshness` — `current`, `stale`, `expired`, `missing`, `imported`, or `unverified`.

Every artifact lineage row MUST show build identity, environment fingerprint,
capture mode, mapping quality, and freshness.

## Capture Source Rows

Capture source rows classify origin:

- `source_class` — `local_live`, `remote_live`, `imported`, `cached`, `provider_supplied`, `ci_provided`, `synthetic`, `sampled`, `instrumented`, `estimated`, or `partial`;
- `target_identity` — process, container, runtime, or provider identity;
- `trust_label` — trust classification;
- `data_class_label` — data sensitivity classification.

## Save/Share Scope Rows

Save/share scope rows define what can be done with evidence:

- `scope_kind` — `local_only`, `exportable`, `shareable`, `uploadable`, or `attach_to_incident`;
- MUST show redaction mode and destination class.

## Downgrade and Rollback

- Any surface that claims `stable` with an incomplete guard set is narrowed
  automatically by the validator.
- Handoff-bar rows MUST show origin, build ID, commit, capture source,
  save/share scope, and lineage state; missing truth labels trigger a validation
  violation.
- Artifact lineage rows MUST show build identity, environment fingerprint,
  capture mode, mapping quality, and freshness; missing truth labels trigger a
  validation violation.
- Save/share scope rows MUST show redaction mode and destination class; missing
  behavior triggers a validation violation.
- Cross-reference failures (unknown capture source, save/share scope, or lineage
  ref) trigger validation violations.

## Invariants

- Raw payload bytes, raw command lines, secrets, and ambient credentials do not
  cross this boundary.
- Every handoff bar points to exactly one capture source, one save/share scope,
  and one artifact lineage row.
- Capture source classifications must be honest; generic `profile available`
  badges are insufficient.
- Lineage claims narrow automatically when mapping fidelity, baseline
  comparability, or artifact identity are weak.
