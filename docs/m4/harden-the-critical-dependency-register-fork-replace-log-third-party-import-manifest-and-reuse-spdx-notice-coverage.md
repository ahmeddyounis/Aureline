# Harden the critical dependency register, fork/replace log, third-party import manifest, and REUSE/SPDX/notice coverage

Workstream batch: B11 — Release engineering, certification, performance, accessibility, docs truth, and public proof.

Execution wave: W11
Readiness wave: R11
Milestone fit: Required M4 stable promotion and public-proof foundation
Delivery class: Governed artifact + release-control + certification lane
Track: Published proof, one-build identity, stable docs/help truth, certified archetypes, and operator-facing release readiness.

## Goal

Turn the dependency and licensing governance lanes into release-grade proof so
that every marketed stable build is backed by a current critical dependency
register, fork/replace log, third-party import manifest, and REUSE/SPDX/notice
coverage. Any lane that loses freshness, proof, or certification narrows
automatically instead of lingering as an unearned stable promise.

## What is in this artifact

The checked-in artifact at
`artifacts/release/harden_the_critical_dependency_register_fork_replace_log_third_party_import_manifest_and_reuse_spdx_notice_coverage.json`
publishes:

- One `LaneRow` per governance lane, carrying:
  - `lane_kind` — critical_dependency_register, fork_replace_log, third_party_import_manifest, or reuse_spdx_notice_coverage
  - `subject_ref` and `subject_summary` — what the row speaks for
  - `claim_ref` and `claim_label` — the stable claim manifest entry whose label is the ceiling
  - `proof_packet` — packet id, ref, freshness SLO, and evidence refs
  - `owner_signoff` — named owner and sign-off state
  - `lane_state` — current, on_waiver, narrowed_stale, narrowed_missing, narrowed_unbacked, or narrowed_waiver_expired
  - `effective_label` — after narrowing automation
  - `active_gap_reasons` — reasons that narrowed the row

- `LaneRule` set covering all eleven closed gap reasons:
  - `claim_label_narrowed`
  - `packet_missing`
  - `packet_freshness_breached`
  - `evidence_incomplete`
  - `artifact_unverified`
  - `dependency_audit_gap`
  - `fork_divergence`
  - `import_mapping_failed`
  - `license_coverage_gap`
  - `waiver_expired`
  - `owner_signoff_missing`

- A `publication` block with the recomputed `hold`/`proceed` verdict.

- A `summary` block with counts that the typed model recomputes from the rows.

## Downgrade automation

A governance lane narrows automatically when any of the following is true:

| Condition | State | Action |
|---|---|---|
| Proof packet stale | `narrowed_stale` | Refresh the proof packet |
| Proof packet missing | `narrowed_missing` | Capture the proof packet |
| Evidence incomplete | `narrowed_unbacked` | Recapture evidence |
| Artifact unverified | `narrowed_unbacked` | Verify or complete the artifact |
| Dependency audit gap | `narrowed_unbacked` | Audit or remediate the dependency |
| Fork divergence | `narrowed_unbacked` | Reconcile fork or complete replacement |
| Import mapping failed | `narrowed_unbacked` | Remap import or resolve license conflict |
| License coverage gap | `narrowed_unbacked` | Complete REUSE/SPDX or notice coverage |
| Waiver expired | `narrowed_waiver_expired` | Hold publication |
| Owner sign-off missing | `narrowed_unbacked` | Request owner sign-off |
| Backing claim narrowed | `narrowed_*` | Inherit upstream ceiling |

## Current posture

At this revision six of eight lanes are current or on-waiver, two are narrowed:

- **critical dependency: upstream Rust** — current
- **critical dependency: platform/system** — current
- **fork/replace: forked crates** — current
- **fork/replace: replaced upstream** — narrowed because the proof packet breached its freshness SLO and the replacement migration is incomplete
- **third-party import: direct** — current
- **third-party import: transitive** — current on an active waiver while the final advisory sweep completes
- **REUSE/SPDX/notice: SPDX SBOM** — narrowed because the SBOM is a structural placeholder and has not reached full conformance
- **REUSE/SPDX/notice: human-readable notices** — current

The publication gate holds because two blocking rules fire (packet freshness
breached for replaced upstream, license coverage gap for SPDX SBOM). Promotion
clears once the fork/replace packet is refreshed and the SPDX SBOM reaches
standards conformance (or those claims are formally narrowed).

## Verification

Run the crate tests:

```bash
cd crates/aureline-release
cargo test harden_the_critical_dependency_register
```

The typed model validates the checked-in artifact, recomputes the summary and
publication verdict, and projects a redaction-safe export for downstream
surfaces.
