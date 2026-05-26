# Docs-pack truth — contract

The docs-pack truth packet is the stable knowledge-plane contract that hardens
four launch-critical pillars across every claimed stable docs / help row:

1. **Docs-pack manifests.** Each pack carries `pack_id`, `pack_revision_ref`,
   signer identity, source channel, version range, refresh state, mirror
   source, pin state, and manifest schema version. Offline import/export,
   quarantine, refresh, and stale-example flows all consume the same manifest.
2. **Mirror / offline truth.** Mirror lineage, pin state, and local availability
   are pinned through closed enums; mirror continuity and offline expiration
   never collapse into one generic "offline" badge.
3. **Stale-example detection.** Findings keep the
   `nearby_version` / `stale_example` / `quarantined_pack` distinction visible
   and reviewable. Suppressions carry actor, reason, expiry, evidence refs,
   plus the source pack id and revision so attribution survives export,
   mirror, and release-packet reuse.
4. **Citation-set export.** Derived explanations carry a citation-set object
   with cited files, symbols, docs refs, graph epoch, locale, and derivation
   tool / version. Citation-set export works without bundling whole docs packs
   by default and stays available to AI evidence, onboarding/help, and
   support-export lanes.

The checked-in canonical form lives at
`artifacts/search/m4/docs_pack_truth_packet.json`; the implementation lives at
`crates/aureline-docs/src/docs_pack_truth_packet/mod.rs`. The schema lives at
`schemas/docs/docs_pack_truth_packet.schema.json`. The fixture corpus lives
under `fixtures/search/m4/docs_pack_truth_packet/`.

## What the packet certifies

The packet certifies that every claimed stable docs-pack row:

- **Pins a `DocsPackManifest`.** Pack id, revision, display label, source
  class, source channel, signing block (signer class, signature status,
  signing authority), version range, refresh state, mirror lineage, pin
  state, local availability, publishable state, blocking reasons, and
  manifest schema version are present. Identity (signer / channel /
  mirror-source) is preserved even when content is unavailable locally.
- **Keeps stale states distinct.** A row labelled `nearby_version` must carry
  a `nearby_version_ref`; a row labelled `quarantined_pack` must reference a
  pack whose `publishable_state` or `local_availability` is `quarantined`.
  Collapsing these states into a single warning blocks promotion.
- **Carries reviewable suppressions.** Every suppression names its actor,
  reason, expiry, and evidence refs, and pins the source pack id and revision
  the finding was raised against. Dropping any of these is a blocker.
- **Exports citation sets without raw pack bodies.** `raw_pack_bodies_excluded`
  and `raw_urls_excluded` are required true. Citation sets reference cited
  files, symbols, docs anchors, the graph epoch, the locale, and the
  derivation tool / version.
- **Reuses the closed enums on every consumer surface.** Source class, render
  mode, validation result class, stale-finding class, mirror state, pin
  state, and local availability are read verbatim by the docs-browser shell,
  help pane, onboarding tour, AI context inspector, CLI / headless emitter,
  support export, release proof index, mirror / offline console, citation
  drawer, and stale-example review surfaces.

## Consumer surfaces covered

| Surface | Why it ships |
|---|---|
| `docs_browser_shell` | In-product docs-browser row, table, and detail. |
| `help_pane` | In-product help pane and About card. |
| `onboarding_tour` | Onboarding / learning tour citation drawer. |
| `ai_context_inspector` | AI context inspector / context-picker projection. |
| `cli_headless` | Headless CLI emitter. |
| `support_export` | Support export bundle. |
| `release_proof_index` | Release proof index entry. |
| `mirror_offline_console` | Mirror / offline console (Help / About). |
| `citation_drawer` | Citation drawer for derived explanations. |
| `stale_example_review` | Stale-example review surface. |

A consumer projection that drops the pack-manifest schema, the
mirror / offline taxonomy, the pin / availability taxonomy, the render-mode
taxonomy, the validation-result-class taxonomy, the stale-finding-class
taxonomy, the suppression attribution, or the citation-set identity is
auto-narrowed below stable.

## Closed finding vocabulary

When the packet fails an invariant the validator emits one or more of:
`wrong_record_kind`, `wrong_schema_version`, `missing_packet_identity`,
`missing_pack_manifests`, `missing_citation_sets`,
`missing_stale_example_findings`, `pack_manifest_incomplete`,
`pack_identity_lost_when_offline`, `mirror_state_inconsistent`,
`publishable_state_inconsistent`, `refresh_disclosure_missing`,
`stale_state_collapsed`, `stale_finding_pack_ref_unpinned`,
`suppression_attribution_lost`, `suppression_lost_source_version`,
`citation_set_bundles_raw_pack`, `citation_set_identity_incomplete`,
`citation_set_pack_ref_unpinned`, `required_source_class_coverage_missing`,
`required_render_mode_coverage_missing`,
`required_local_availability_coverage_missing`, `missing_consumer_projection`,
`consumer_projection_drift`, `render_mode_vocabulary_dropped`,
`validation_result_class_dropped`, `stale_finding_class_dropped`,
`citation_set_identity_dropped`, `raw_boundary_material_present`,
`promotion_state_mismatch`.

The fixture corpus drills the most likely failure modes:

- `offline_pack_loses_signer_identity_blocks_stable.json` — a pack with local
  content unavailable drops its signing-authority ref; identity must survive
  the offline posture.
- `nearby_version_dropped_collapses_stale_state_blocks_stable.json` — a
  nearby-version finding drops its nearby-version ref and collapses into a
  generic stale warning.
- `citation_set_bundles_raw_pack_blocks_stable.json` — a citation-set export
  bundles raw pack bodies by default.
- `stale_suppression_loses_attribution_blocks_stable.json` — a stale-example
  suppression drops its actor attribution.
- `consumer_projection_drops_render_mode_blocks_stable.json` — a consumer
  projection sets `preserves_render_mode = false`, collapsing the
  rendered / syntax_checked / executed / mirrored / browser-handoff /
  not_rendered taxonomy into one generic success badge.
- `quarantined_finding_collapsed_blocks_stable.json` — a quarantined-pack
  finding now references a publishable pack.

## Coordination with docs-maintenance packets

The docs-pack contract reuses the docs-maintenance vocabulary: render mode,
validation result class, and stale-finding class line up field-for-field with
`DocsExampleValidationMode`, `DocsPreviewMode`, and `DocsFindingClass` in
`crates/aureline-docs/src/maintenance/mod.rs`. This ensures Help, onboarding,
AI citation, and release-proof surfaces read the same source / version,
freshness, and publish-boundary truth across both packets.

## How to regenerate the packet

The seed lives in code; the artifact and fixtures are regenerated with the
crate's headless emitter:

```sh
cargo run -q -p aureline-docs --bin aureline_docs_pack_truth_packet -- packet \
  > artifacts/search/m4/docs_pack_truth_packet.json

for case in baseline_stable \
            offline_pack_loses_signer_identity_blocks_stable \
            nearby_version_dropped_collapses_stale_state_blocks_stable \
            citation_set_bundles_raw_pack_blocks_stable \
            stale_suppression_loses_attribution_blocks_stable \
            consumer_projection_drops_render_mode_blocks_stable \
            quarantined_finding_collapsed_blocks_stable; do
  cargo run -q -p aureline-docs --bin aureline_docs_pack_truth_packet -- fixture "$case" \
    > "fixtures/search/m4/docs_pack_truth_packet/${case}.json"
done
```

The replay test `crates/aureline-docs/tests/docs_pack_truth_packet.rs`
ingests the artifact and every fixture; CI fails if the emitter, the artifact,
or a fixture drifts.
