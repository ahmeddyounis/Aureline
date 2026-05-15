# Claim-manifest contract

This document is the narrative companion for Aureline's public-truth
claim-manifest contract. It exists so release packets, docs/help truth,
support exports, migration notes, evaluation kits, and public-proof
artifacts all quote one claim row instead of re-authoring the same
statement in each channel.

Machine-readable companions:

- [`/schemas/governance/claim_manifest.schema.json`](../../schemas/governance/claim_manifest.schema.json)
  — boundary schema for the seeded `claim_manifest_packet` and reusable
  `claim_row` records.
- [`/artifacts/governance/claim_manifest_seed.yaml`](../../artifacts/governance/claim_manifest_seed.yaml)
  — seed packet carrying canonical claim rows for boundary truth,
  exact-build identity, benchmark publication, docs freshness,
  version-skew truth, and one launch-wedge/certified-archetype family.
- [`/artifacts/release/m3/claim_manifest.json`](../../artifacts/release/m3/claim_manifest.json)
  and
  [`/artifacts/release/m3/claim_manifest.md`](../../artifacts/release/m3/claim_manifest.md)
  — M3 governed claim manifest generated from the matrix at
  [`/artifacts/release/m3/claim_manifest_matrix.yaml`](../../artifacts/release/m3/claim_manifest_matrix.yaml)
  by [`/ci/check_m3_claim_manifest.py`](../../ci/check_m3_claim_manifest.py).
  The generator binds every claimed beta row, archetype, and canonical
  claim family to support class, lifecycle state, freshness badge, and
  provenance label so docs, Help/About, service health, support exports,
  and release packets read one row shape.
- [`/artifacts/governance/public_truth_parity_matrix.yaml`](../../artifacts/governance/public_truth_parity_matrix.yaml)
  — channel propagation matrix for docs, migration notes, Help/About,
  service health, support export, release packets, release notes,
  CLI/help, evaluation artifacts, marketplace-style discovery, and
  public-proof packets.
- [`/artifacts/governance/source_of_truth_map.yaml`](../../artifacts/governance/source_of_truth_map.yaml)
  and
  [`/docs/governance/drift_blocking_rules.md`](./drift_blocking_rules.md)
  — canonical owner-routing map and drift-blocking rules the claim rows
  participate in before they project onto downstream channels.

Related upstream contracts:

- [`/docs/release/release_artifact_graph.md`](../release/release_artifact_graph.md)
  — canonical release-artifact graph claim rows compose over.
- [`/docs/adr/0013-docs-help-service-health-truth.md`](../adr/0013-docs-help-service-health-truth.md)
  — docs/help/service-health truth-source vocabulary the claim rows
  quote instead of replacing.
- [`/artifacts/compat/qualification_matrix_seed.yaml`](../../artifacts/compat/qualification_matrix_seed.yaml)
  and
  [`/artifacts/compat/version_skew_register.yaml`](../../artifacts/compat/version_skew_register.yaml)
  — compatibility and version-skew rows the claim manifest extends by
  reference.
- [`/docs/build/exact_build_identity_model.md`](../build/exact_build_identity_model.md)
  — exact-build identity model claim-bearing publication cannot bypass.
- [`/docs/benchmarks/benchmark_publication_pack_template.md`](../benchmarks/benchmark_publication_pack_template.md)
  — public-proof packet template benchmark-facing claim rows route into.
- [`/docs/docs/reviewed_pack_and_late_copy_policy.md`](../docs/reviewed_pack_and_late_copy_policy.md)
  and
  [`/schemas/docs/late_copy_change_packet.schema.json`](../../schemas/docs/late_copy_change_packet.schema.json)
  — reviewed-pack binding and late-copy packet family used when
  post-string-freeze claim-bearing copy must narrow or correct a
  protected publication surface.

## Purpose

The claim manifest is the publication bridge between upstream evidence
and downstream copy. It does not replace the boundary manifest, the
compatibility matrix, exact-build identities, benchmark packets, or
docs/help truth records. It binds them together into one inspectable row
shape:

- one stable `claim_row_id`;
- one canonical copy block;
- canonical requirement ids;
- evidence packet links and downgrade rules;
- lifecycle/support-window posture;
- channel bindings for every protected publication surface; and
- known-limit, exclusion, launch-bundle, or certified-archetype refs.

If a downstream surface needs a public statement a reader could rely on,
that statement must be traceable back to a claim row.

## Row model

Every `claim_row` carries two separate posture fields:

- `declared_claim_posture`: the posture the row would have if all
  required evidence were current and fully scoped.
- `effective_claim_posture`: the posture surfaces must render right now.

That split is what lets the manifest fail closed. A row can declare a
claim-bearing future while still rendering `experimental`, `limited`,
`policy_disabled`, or `replacement_grade` today.

Every row also carries:

- `compatibility_row_refs` and `version_skew_register_refs` so packets
  and docs cite the same named boundary rows;
- `exact_build_identity_refs` when the claim depends on a coordinated
  artifact set;
- `known_limit_refs` and `exclusion_note_refs` so caveats remain part of
  the row rather than slide-footnote folklore; and
- `channel_bindings` that tell each publication channel which copy
  field, posture, and minimum truth state it must preserve.

## Channel propagation

The parity matrix treats every publication channel as a projection of
the same row, not as a new source of truth. That means:

- Docs, migration notes, Help/About, service health, support export,
  release packets, release notes, CLI/help, evaluation artifacts,
  marketplace-style discovery, and public-proof packets all point back
  to one `claim_row_id`.
- A channel may narrow further for space or local context, but it may
  not widen above `effective_claim_posture`.
- Service-health rows, release-note caveats, and CLI/help summaries must
  preserve the same downgrade reason the release packet or docs surface
  would report.
- If a channel must change claim-bearing copy after string freeze, it
  does so through a `late_copy_change_packet` bound to the current
  reviewed-pack version; silent channel-local rewrites are
  non-conforming.

## Downgrade rules

The seeded schema and parity matrix reserve explicit failure triggers:

- missing required evidence;
- stale evidence;
- evidence narrower than the claim text;
- active or expired test quarantine / mute debt that touches the claim
  row without an accepted waiver, stable-again closure, or explicit
  claim narrowing;
- docs version mismatch or freshness below floor;
- missing required known-limit notes;
- degraded upstream compatibility rows; and
- expired support windows.

When any trigger is active, the row downgrades. Downstream surfaces do
not get to decide whether the downgrade is user-visible; they only
decide how the already-downgraded row is presented within their space.

## Current seed scope

The current seed is intentionally narrow. It does not claim GA-ready
publication automation. It does freeze enough structure that later work
can extend one contract instead of minting new ones:

- open/local versus managed boundary truth;
- exact-build identity publication truth;
- benchmark-publication proof routing;
- docs freshness and version-match truth;
- compatibility/version-skew truth; and
- one launch-wedge/certified-archetype claim family.

That is enough for future generators, parity audits, and release packets
to fail closed when proof is stale, missing, or narrower than the copy.
