# Publish the signed M4 stable evidence pack plus the benchmark, compatibility, and migration launch bundle — proof packet

Reviewer-facing proof packet for the signed M4 stable evidence pack that
aggregates, signs, and publishes every evidence bundle required for M4 stable
promotion.

Canonical machine source (do not clone status text from this packet — ingest the
JSON):

- Artifact:
  [`/artifacts/release/publish_the_signed_m4_stable_evidence_pack_plus.json`](../publish_the_signed_m4_stable_evidence_pack_plus.json)
- Companion doc:
  [`/docs/m4/publish-the-signed-m4-stable-evidence-pack-plus.md`](../../../docs/m4/publish-the-signed-m4-stable-evidence-pack-plus.md)
- Typed consumer:
  `aureline_release::publish_the_signed_m4_stable_evidence_pack_plus`

## What this packet proves

1. **Every M4 evidence bundle is referenced in one signed pack.** The pack binds
   benchmark, compatibility, migration, accessibility, docs/help, security,
artifact graph, qualification, release promotion, and dependency bundles to
upstream artifacts so the exact build being promoted is always traceable.

2. **Each bundle carries an attestation or signature ref.** The pack records
   whether a bundle is signed, unsigned, or narrowed due to attestation gaps.
Unsigned bundles cannot back a Stable public claim.

3. **Downgrade automation narrows stale, unsigned, or unverified bundles before
   publication.** The `packet_freshness_breached`, `attestation_missing`,
`signature_invalid`, and `owner_signoff_missing` gap reasons fire blocking rules
that hold publication until the condition clears or the claim is formally
narrowed.

4. **The publication verdict is recomputed, not asserted.** The typed model and
   the CI gate both recompute the `hold`/`proceed` decision and the blocking
rule/row sets from the firing rules and fail on any drift.

## Current snapshot (as of 2026-06-03)

The checked-in pack holds publication. Of ten evidence bundle rows, seven are
signed current or on-waiver and three are narrowed below stable:

- **benchmark** — signed current, backed by current benchmark-lab traces and
  corpus metadata. All p50/p95 budgets are within published ceilings.
- **compatibility** — signed current, backed by current compatibility reports
  and deprecation packets within removal windows.
- **migration** — signed on an active waiver that covers the preview-platform
  migration guide gap until 2026-06-15.
- **accessibility** — signed current, backed by passing desktop platform
  conformance checks.
- **docs/help** — narrowed to beta because the proof packet breached its
  14-day freshness SLO on 2026-05-24 and the docs team has not yet re-signed.
- **security** — signed current, backed by current advisory templates and
  emergency-disable drills.
- **artifact graph** — narrowed to beta because the release engineering owner
  has not yet signed off, despite the packet being within SLO.
- **qualification** — signed current, backed by complete optional-surface
  qualification packets.
- **release promotion** — signed current, backed by canary and pilot evidence
  within soak windows.
- **dependency** — narrowed to preview because the bundle is unsigned and
  unattested; the REUSE/SPDX notice coverage audit is incomplete.

Three blocking rules fire (`packet_freshness_breached`, `attestation_missing`,
`owner_signoff_missing`) and hold the `m4:stable_promotion` gate. Promotion
clears once the docs/help packet is refreshed, the dependency bundle is attested
and signed, and the artifact graph owner signs off.

## Accessibility of this lane

The artifact and its companion doc are text/JSON artifacts: the doc renders as
headed sections and plain Markdown tables, and the machine source carries the
same truth so Help/About, the release center, support exports, docs, and
shiproom dashboards ingest one record per bundle rather than restating status
text.

## How to re-verify

```
cargo test -p aureline-release
```

This runs the typed contract tests that bind the model to the checked-in
artifact, including the negative fixture cases.
