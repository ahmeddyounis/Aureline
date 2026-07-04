# M5 deployment/continuity component consumer contract (M05-833)

This contract is the **first-consumer adoption lane** over the frozen M5
deployment/continuity component matrix
([`docs/deployment/m5_deployment_continuity_component_matrix.md`](m5_deployment_continuity_component_matrix.md),
M05-828) and the M05-829..832 primitive resolvers. It proves the nine reusable
component families are genuine **primitives** — not one About page, one
diagnostics pane, or one admin-only dashboard — by adopting each across the
claimed M5 deployment consumer lanes without drifting operating-mode truth.

- **Rust module:**
  `crates/aureline-install/src/add_shared_about_update_diagnostics_admin_support_offboarding_and_browser_handoff_deployment_continuity_component_consumers/`
- **Boundary schema:**
  [`schemas/ui/m5-deployment-continuity-component-consumer.schema.json`](../../schemas/ui/m5-deployment-continuity-component-consumer.schema.json)
- **Checked support export:**
  [`artifacts/release/m5-deployment-continuity-component-consumer-proof/support_export.json`](../../artifacts/release/m5-deployment-continuity-component-consumer-proof/support_export.json)
  (plus `matrix.csv` and `report.md`)
- **Protected fixtures:**
  [`fixtures/ui/m5-deployment-continuity-component-consumers/`](../../fixtures/ui/m5-deployment-continuity-component-consumers/)

## Consumer groups

Every row is one consumer on one concrete surface. The five claimed consumer
groups (each covering the required adoption lanes) are:

| Group | Surfaces | Role |
| --- | --- | --- |
| `about_update` | `about_page`, `update_center` | About / update consumer |
| `diagnostics_support` | `diagnostics_pane`, `support_bundle_flow` | Diagnostics / support flow |
| `admin_offboarding` | `admin_fleet_dashboard`, `offboarding_uninstall_flow` | Admin / offboarding flow |
| `browser_handoff` | `browser_deep_link_handoff`, `handler_review_prompt` | Browser / deep-link or handler-review flow |
| `docs_help_release` | `help_center_docs`, `support_export_replay`, `release_proof_surface` | Docs / help + support-export lane (AC3) |

## Canonical family mapping

Each row points back to exactly one canonical family — the primitive schema plus
release-proof packet — via `canonical_schema_ref_for` / `canonical_packet_ref_for`.
The nine frozen families resolve to four sibling primitives:

| Family | Canonical primitive |
| --- | --- |
| `install_profile_card`, `side_by_side_import_sheet`, `rollout_ring_row` | M05-829 deployment-profile (`schemas/ui/m5-deployment-profile-primitive.schema.json`) |
| `deployment_summary_card`, `residual_dependency_row`, `control_plane_data_plane_status_strip` | M05-830 deployment-summary (`schemas/ui/m5-deployment-summary-primitive.schema.json`) |
| `mirror_offline_artifact_row`, `mode_change_review_sheet` | M05-831 mirror-transition (`schemas/ui/m5-mirror-transition-primitive.schema.json`) |
| `channel_association_review_row` | M05-832 handler-ownership (`schemas/ui/m5-handler-ownership-primitive.schema.json`) |

## Invariants (mirrors of the acceptance criteria)

- **AC1 — one canonical family, not per-surface prose.** Every row's
  `canonical_family_schema_ref` equals the family's canonical schema, references
  the family's release-proof packet, and sets
  `references_canonical_not_local_prose`. At least one family is adopted across
  two or more consumer groups (`families_reused_across_groups >= 1`), and all
  nine families plus all five groups are covered.
- **AC2 — label + state parity.** Every consumer preserves the identical
  controlled label families — `operating_mode`, `ownership_or_scope`,
  `provenance_freshness`, `residual_dependency`, `continuity_state` — and the
  identical degraded-state vocabulary. A narrower consumer (read-only,
  inspect-only, export-only, policy-blocked) is `disclosed_narrowed`, carries a
  reduced-capability banner whose `capability_state` matches its `authority_mode`
  with a non-generic label and concrete missing capabilities, and — when it punts
  elsewhere — a handoff note. A full-interactive consumer carries no banner. All
  labels remain copyable as text / JSON / Markdown; screenshot-only export is
  prohibited.
- **AC3 — docs / help + support export cite the primitives.** A `help_center_docs`
  consumer references the canonical families, and support-export / release-proof
  surfaces reconstruct the same operating-mode / residual-dependency truth from
  the shared packets rather than cloning local vocabulary.

The packet is metadata-only: raw config bytes, credentials, license keys, mirror
URLs, provider cursors, and raw device identifiers never cross this boundary.

## Regenerating the artifacts

The seeded builder
`seeded_m5_deployment_continuity_component_consumers_packet()` is the single
source of truth. The `checked_in_export_matches_seeded_builder` test asserts the
on-disk support export is byte-identical to it. To regenerate, run the builder's
`export_safe_json` / `render_matrix_csv` / `render_markdown_summary` and write the
result to the artifact and fixture directories.
