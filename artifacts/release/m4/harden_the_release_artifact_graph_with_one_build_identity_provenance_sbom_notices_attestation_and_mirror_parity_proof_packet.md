# Harden the release artifact graph with one-build identity, provenance, SBOM, notices, attestation, and mirror parity — proof packet

Reviewer-facing proof packet for the release artifact graph lane that hardens
every marketed stable build with one exact identity across binaries, docs/help
truth, compatibility packets, symbols, SBOM/attestation data, and rollback
manifests.

Canonical machine source (do not clone status text from this packet — ingest the
JSON):

- Artifact:
  [`/artifacts/release/harden_the_release_artifact_graph_with_one_build_identity_provenance_sbom_notices_attestation_and_mirror_parity.json`](../harden_the_release_artifact_graph_with_one_build_identity_provenance_sbom_notices_attestation_and_mirror_parity.json)
- Companion doc:
  [`/docs/m4/harden-the-release-artifact-graph-with-one-build-identity-provenance-sbom-notices-attestation-and-mirror-parity.md`](../../../docs/m4/harden-the-release-artifact-graph-with-one-build-identity-provenance-sbom-notices-attestation-and-mirror-parity.md)
- Typed consumer:
  `aureline_release::harden_the_release_artifact_graph_with_one_build_identity_provenance_sbom_notices_attestation_and_mirror_parity`

## What this packet proves

1. **Every artifact family in the release graph is pinned to one build identity.**
   The one-build identity row binds the exact build digest to binaries, docs/help
packs, compatibility packets, symbols bundles, SBOM, attestation, and rollback
manifests. A mismatch is a blocking failure for Stable promotion.

2. **Each family ingests a proof packet with a freshness SLO.** Each row names
   the packet that grounds it, the SLO that protects it, and the evidence refs
that verify it. A missing or stale packet narrows the family automatically.

3. **Downgrade automation narrows stale, missing, or unverified families before
   publication.** The `packet_freshness_breached`, `packet_missing`, and
`artifact_unverified` gap reasons fire blocking rules that hold publication until
the condition clears or the claim is formally narrowed.

4. **The publication verdict is recomputed, not asserted.** The typed model and
   the CI gate both recompute the `hold`/`proceed` decision and the blocking
rule/row sets from the firing rules and fail on any drift.

## Current snapshot (as of 2026-06-02)

The checked-in artifact holds publication. Of six artifact family rows, four are
current or on-waiver and two are narrowed below stable:

- **one-build identity** — current, backed by a pinned digest that matches every
  artifact family.
- **provenance** — current, backed by a complete provenance capture seed.
- **SBOM** — narrowed to beta because the SBOM remains a structural placeholder,
  not SPDX or CycloneDX conformant.
- **notices** — current on an active waiver that covers pending reserved-import
  notice text until 2026-07-01.
- **attestation** — current, backed by signed attestations from the hardened
  signing boundary.
- **mirror parity** — narrowed to beta because the mirror parity packet breached
  its 30-day freshness SLO on 2026-05-15.

Both narrowed rows back claims still published Stable upstream, so their gap
reasons fire blocking rules and hold the
`harden_release_artifact_graph_publication` gate. Promotion clears once the
mirror dry run is refreshed and the SBOM reaches standards conformance (or those
public claims are formally narrowed).

## Accessibility of this lane

The artifact and its companion doc are text/JSON artifacts: the doc renders as
headed sections and plain Markdown tables, and the machine source carries the
same truth so Help/About, the release center, support exports, docs, and
shiproom dashboards ingest one record per family rather than restating status
text.

## How to re-verify

```
cargo test -p aureline-release
```

This runs the typed contract tests that bind the model to the checked-in
artifact, including the negative fixture cases.
