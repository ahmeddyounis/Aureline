# Publish the signed M4 stable evidence pack plus the benchmark, compatibility, and migration launch bundle

**M04-185** | Generated: 2026-06-03

## Overview

This document defines the signed M4 stable evidence pack. It is the canonical
source for:

- The aggregation of all M4 evidence bundles into one signed, traceable pack.
- Signing and attestation status for each bundle (benchmark, compatibility,
  migration, accessibility, docs/help, security, artifact graph, qualification,
  release promotion, dependency).
- Downgrade automation that narrows any bundle whose proof packet goes stale,
  whose attestation is missing or invalid, or whose owner sign-off is absent.
- The publication verdict that shiproom and release tooling use to gate M4
  stable promotion.

## Pack identity

- `pack_id`: `signed_m4_stable_evidence_pack:release.m4`
- `record_kind`: `signed_m4_stable_evidence_pack`
- `schema_version`: `1`
- `as_of`: `2026-06-03`

## Lifecycle labels

The pack reuses the closed lifecycle-label vocabulary:

- `lts` — Long-term-support stable
- `stable` — Broad stable
- `beta` — Narrowed to beta
- `preview` — Narrowed to preview
- `withdrawn` — Claim withdrawn

## Bundle kinds

Ten kinds of evidence bundle are tracked:

1. **Benchmark** (`benchmark`) — public benchmark publication, hot-path
   performance budgets, benchmark-lab traces.
2. **Compatibility** (`compatibility`) — compatibility reports, deprecation
   packets, schema version windows, skew registers.
3. **Migration** (`migration`) — migration guides, known-limits publications,
   end-of-support notices, rollback sequences.
4. **Accessibility** (`accessibility`) — IME, grapheme, bidi, Unicode,
   high-contrast, zoom, density, pseudolocalization, RTL, desktop platform
   conformance.
5. **Docs/help** (`docs_help`) — docs browser truth, pack truth, Help/About,
   service health, semantic recall boundaries.
6. **Security** (`security`) — advisory, CVE, GHSA publication, emergency
   disable, mirror/offline drills.
7. **Artifact graph** (`artifact_graph`) — one-build identity, provenance,
   SBOM, notices, attestation, mirror parity.
8. **Qualification** (`qualification`) — optional-surface qualification packets
   and enforced narrower-than-stable labeling.
9. **Release promotion** (`release_promotion`) — release-center promotion
   evidence, canary/pilot controls, ring progression.
10. **Dependency** (`dependency`) — dependency register, fork/replace log,
    third-party import manifest, REUSE/SPDX/notice coverage.

## Bundle states

- `signed_current` — Signed, current, owner-signed.
- `signed_on_waiver` — Signed but held on an active waiver.
- `unsigned_unattested` — Missing attestation or signature.
- `narrowed_stale` — Proof packet breached freshness SLO.
- `narrowed_claim_narrowed` — Inherited from a narrowed claim.
- `narrowed_waiver_expired` — Waiver expired.
- `narrowed_owner_signoff_missing` — Missing owner sign-off.

## Gap reasons

- `claim_label_narrowed`
- `packet_missing`
- `packet_freshness_breached`
- `evidence_incomplete`
- `attestation_missing`
- `signature_invalid`
- `waiver_expired`
- `owner_signoff_missing`

## Bundle actions

- `hold_publication`
- `narrow_bundle_label`
- `refresh_packet`
- `recapture_evidence`
- `request_attestation`
- `request_owner_signoff`

## Checked-in artifact

The canonical JSON artifact is:

- `artifacts/release/publish_the_signed_m4_stable_evidence_pack_plus.json`

## Schema

The JSON Schema is:

- `schemas/release/publish-the-signed-m4-stable-evidence-pack-plus.schema.json`

## Verification

Run the protected tests in `crates/aureline-release/tests/` to validate the
checked-in artifact against the typed model.
