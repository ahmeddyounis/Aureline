# Fixtures: M5 ecosystem certification, qualification automation, and downgrade paths

This directory contains fixture metadata for the `m5_ecosystem_certification` packet.

The canonical full corpus is checked in at:

`artifacts/ecosystem/m5/m5-ecosystem-certification.json`

## Coverage

- Nine certification entries cover every marketed package kind and every distribution
  source class — `first_party`, `partner`, `community`, `bridge_backed`, `side_loaded`,
  `mirrored_registry`, and `private_registry` — and every qualification disposition —
  `qualified`, `conditionally_qualified`, `downgraded`, and `disqualified`.
- Every entry carries one lane-evidence record for each of the eight ecosystem drills —
  marketplace information, install review, lifecycle state, compatibility label,
  permission manifest, activation budget, mirror/private-registry parity, and
  rollback/quarantine — so qualification is decided over the full drill set, not a subset.
- The guardrail is proven both ways. `qualified` entries (the first-party framework pack
  and the signed recipe pack) name an owner, link supported profiles and a certified
  conformance scorecard, and publish a support claim. `disqualified` entries withdraw
  their claim and name a requalification path: the template artifact for stale evidence
  and an uncertified scorecard, the bridge-backed package for a failed compatibility lane,
  the side-loaded package for a missing owner/profiles/registry-lane/conformance, and the
  mirrored variant for a retest-pending scorecard and lapsed mirror freshness.
- Source-class capping is proven independently of evidence. The private-registry framework
  pack clears every drill with a certified scorecard yet is held to `best_effort_supported`
  because its source class structurally caps the claim — it cannot inherit the
  public-registry full claim. The mirrored variant carries the same `source_class_capped`
  signal on top of its disqualifying signals.
- Each entry's `qualification_signals`, `qualification_disposition`,
  `effective_support_class`, and `downgrade_path` equal the values recomputed from its
  facts. The downgrade path always names where a narrowed claim landed and the opaque
  requalification ref to restore it.

## Validation

The typed model in the `aureline-ecosystem` crate
(`m5_ecosystem_certification::M5EcosystemCertification::validate`) is canonical: it parses
the embedded packet, recomputes every entry's signals, disposition, effective support
class, and downgrade path, asserts each entry runs every drill lane, and asserts the
summary counts. A test cross-checks each entry's `conformance_certified` flag against the
linked conformance scorecard in `artifacts/ecosystem/m5/m5-conformance-and-validators.json`,
so the aggregation cannot drift from its source. The JSON Schema at
`schemas/ecosystem/m5-ecosystem-certification.schema.json` validates the artifact's shape
and closed vocabularies.
