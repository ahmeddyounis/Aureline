# Fixtures: M5 mirror, private-registry, and side-load review

This directory contains fixture metadata for the `m5_mirror_and_sideload` packet.

The canonical full corpus is checked in at:

`artifacts/ecosystem/m5/m5-mirror-and-sideload.json`

## Coverage

- Eight review packets cover every marketed package kind — `first_party_framework_pack`,
  `docs_pack`, `local_model_pack`, `signed_recipe_pack`, `template_artifact`,
  `bridge_backed_package`, `side_loaded_package`, and `mirrored_registry_variant` — so one
  install-review surface is proven across all claimed M5 artifact families.
- Acquisition channel covers all five lanes — `public_registry`, `enterprise_mirror`,
  `private_registry`, `manual_import`, and `air_gapped_import` — and each packet
  reproduces the full public-registry install-review fact set (package identity,
  source class, compatibility, permission, runtime origin, lifecycle, activation
  budget, and rollback). No field is dropped because the source is mirrored, private,
  or side-loaded.
- The `first_party_framework_pack` (public registry) and `mirrored_registry_variant`
  (enterprise mirror) packets are the same underlying framework pack: they share
  publisher, signing root, and namespace, proving the mirror lane keeps every field
  while widening disclosure for a lagging mirror and a stripped attestation.
- Continuity facts cover publisher transfer (`transferred_verified`,
  `publisher_unknown`), signing-root continuity (`rotated_disclosed`, `unsigned`),
  namespace continuity (`renamed`, `orphaned`), maintenance state
  (`maintenance_reduced`, `orphaned`), mirror freshness (`mirror_lagging`,
  `mirror_unknown`), and provenance level (`signed_no_attestation`, `checksum_only`,
  `unverifiable`).
- Each packet's `review_disposition` and `continuity_signals` equal the values
  recomputed from its facts. The lane guardrails — `permission_expansion_unreviewed`,
  `compatibility_unsupported`, `rollback_incomplete`, `quarantined`,
  `signing_root_changed`, `namespace_discontinuous`, `unmaintained`,
  `publisher_discontinuous`, and `provenance_unverifiable` — each force a `blocked`
  disposition, so the air-gapped bridge-backed package and the manually-imported
  side-loaded package both block regardless of the offline lane.
- Every packet carries the full backing-ref set (provenance, permission manifest,
  compatibility, activation budget, rollback, publisher continuity, and support
  export), so manual-import and air-gapped reviews stay export-safe for support and
  audit.

## Policy filtering

Eight `policy_filters` exercise every `policy_dimension` — `publisher`,
`signing_root`, `runtime_origin`, `capability_class`, `network_class`,
`support_class`, `bridge_state`, and `activation_budget_band` — with `allow`,
`require_approval`, and `block` effects. Eight `policy_evaluations` apply the filter
set to the packets and recompute the gate decision as the stronger of the matched
filter effect and the packet's own review disposition, so policy can tighten
admission but never loosen it below the guardrail the review already requires.

## Validation

The typed consumer in `crates/aureline-ecosystem/src/m5_mirror_and_sideload/` embeds
the canonical packet and validates it: every closed vocabulary matches the canonical
enum order, every disposition and signal set is recomputed, every packet is
export-safe, every evaluation matches its recomputed gate decision, and the summary
counts match the records. The JSON Schema at
`schemas/ecosystem/m5-mirror-and-sideload.schema.json` validates the artifact shape.
