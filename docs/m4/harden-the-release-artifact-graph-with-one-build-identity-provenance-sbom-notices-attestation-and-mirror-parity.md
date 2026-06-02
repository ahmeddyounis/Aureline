# Harden the release artifact graph with one-build identity, provenance, SBOM, notices, attestation, and mirror parity

Workstream batch: B11 — Release engineering, certification, performance, accessibility, docs truth, and public proof.

Execution wave: W11
Readiness wave: R11
Milestone fit: Required M4 stable promotion and public-proof foundation
Delivery class: Governed artifact + release-control + certification lane
Track: Published proof, one-build identity, stable docs/help truth, certified archetypes, and operator-facing release readiness.

## Goal

Turn the release artifact graph into release-grade proof so that every marketed
stable build is backed by one exact identity across binaries, docs/help truth,
compatibility packets, symbols, SBOM/attestation data, and rollback manifests.
Any family that loses freshness, proof, or certification narrows automatically
instead of lingering as an unearned stable promise.

## What is in this artifact

The checked-in artifact at
`artifacts/release/harden_the_release_artifact_graph_with_one_build_identity_provenance_sbom_notices_attestation_and_mirror_parity.json`
publishes:

- One `ArtifactFamilyRow` per artifact family, carrying:
  - `family_kind` — one_build_identity, provenance, sbom, notices, attestation, or mirror_parity
  - `subject_ref` and `subject_summary` — what the row speaks for
  - `claim_ref` and `claim_label` — the stable claim manifest entry whose label is the ceiling
  - `proof_packet` — packet id, ref, freshness SLO, and evidence refs
  - `owner_signoff` — named owner and sign-off state
  - `family_state` — current, on_waiver, narrowed_stale, narrowed_missing, narrowed_unbacked, or narrowed_waiver_expired
  - `effective_label` — after narrowing automation
  - `active_gap_reasons` — reasons that narrowed the row

- `ArtifactFamilyRule` set covering all eight closed gap reasons:
  - `claim_label_narrowed`
  - `packet_missing`
  - `packet_freshness_breached`
  - `evidence_incomplete`
  - `artifact_unverified`
  - `waiver_expired`
  - `owner_signoff_missing`
  - `one_build_identity_mismatch`

- A `publication` block with the recomputed `hold`/`proceed` verdict.

- A `summary` block with counts that the typed model recomputes from the rows.

## Downgrade automation

An artifact family narrows automatically when any of the following is true:

| Condition | State | Action |
|---|---|---|
| Proof packet stale | `narrowed_stale` | Refresh the proof packet |
| Proof packet missing | `narrowed_missing` | Capture the proof packet |
| Evidence incomplete | `narrowed_unbacked` | Recapture evidence |
| Artifact unverified | `narrowed_unbacked` | Verify or complete the artifact |
| Waiver expired | `narrowed_waiver_expired` | Hold publication |
| Owner sign-off missing | `narrowed_unbacked` | Request owner sign-off |
| One-build identity mismatch | `narrowed_unbacked` | Reconcile build identity |
| Backing claim narrowed | `narrowed_*` | Inherit upstream ceiling |

## Current posture

At this revision four of six families are current or on-waiver, two are narrowed:

- **one-build identity** — current
- **provenance** — current
- **SBOM** — narrowed because the SBOM remains a structural placeholder (not SPDX or CycloneDX conformant)
- **notices** — current on an active waiver while reserved-import notice text is pending
- **attestation** — current
- **mirror parity** — narrowed because the mirror parity packet breached its freshness SLO on 2026-05-15

The publication gate holds because two blocking rules fire (packet freshness
breached for mirror parity, artifact unverified for SBOM). Promotion clears once
the mirror dry run is refreshed and the SBOM reaches standards conformance (or
those claims are formally narrowed).

## Verification

```
cargo test -p aureline-release
```

## Risks and follow-ups

- The SBOM is a structural placeholder; SPDX and CycloneDX conformance must be
  achieved before the SBOM family can reclaim Stable.
- The mirror parity packet expired on 2026-05-15 and must be refreshed before
  the family can reclaim Stable.
