# Docs-pack truth — stable artifact

This is the human-readable narrative for the stable docs-pack truth lane that
hardens docs-pack manifests, mirror/offline truth, stale-example detection,
and citation-set export across help surfaces. The canonical truth source is
the checked-in packet at `artifacts/search/m4/docs_pack_truth_packet.json`;
later dashboards, docs, Help/About surfaces, and support exports should
ingest that file instead of cloning status text.

## What the artifact certifies

The artifact certifies that every claimed stable docs-pack row:

- **Pins a docs-pack manifest** with `pack_id`, `pack_revision_ref`, source
  class, source channel, signer identity, version range, refresh state,
  mirror lineage, pin state, local availability, publishable state, and
  manifest schema version. Identity is preserved even when content is
  unavailable locally (not_installed, unavailable_disclosed, quarantined).
- **Keeps the `nearby_version` / `stale_example` / `quarantined_pack` triad
  distinct.** A nearby-version finding must carry a `nearby_version_ref`; a
  quarantined-pack finding must reference an actually quarantined pack. The
  validator refuses to collapse these states into one warning.
- **Carries reviewable suppressions.** Every suppression records the actor,
  reason, expiry, evidence refs, source pack id, and source pack revision so
  the attribution survives export, mirror, and release-packet reuse.
- **Exports citation sets without raw pack bodies.** Citation sets reference
  cited files, symbols, docs anchors, the graph epoch, the locale, and the
  derivation tool / version. The packet refuses to certify if a citation set
  bundles raw pack bodies or admits raw URLs.
- **Reuses the closed enums on every consumer surface.** Source class, render
  mode, validation result class, stale-finding class, mirror state, pin
  state, and local availability are read verbatim by the docs-browser shell,
  help pane, onboarding tour, AI context inspector, CLI / headless emitter,
  support export, release proof index, mirror / offline console, citation
  drawer, and stale-example review surfaces.

## Consumer surfaces covered

The checked-in canonical packet binds projections to every required consumer
surface:

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

A projection that drops the pack-manifest schema, the mirror / offline
taxonomy, the render-mode taxonomy, the validation-result-class taxonomy,
the stale-finding-class taxonomy, the suppression attribution, or the
citation-set identity is auto-narrowed below stable.

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
