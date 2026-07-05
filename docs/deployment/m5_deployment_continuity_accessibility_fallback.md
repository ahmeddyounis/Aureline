# M5 deployment/continuity accessibility fallback and auto-narrowing (M05-834)

This contract is the keyboard, screen-reader, CLI/export, and auto-narrowing
capstone over the frozen M5 deployment/continuity component matrix
([`docs/deployment/m5_deployment_continuity_component_matrix.md`](m5_deployment_continuity_component_matrix.md))
and the M05-829..833 primitive and consumer lanes.

- **Rust module:**
  `crates/aureline-install/src/implement_keyboard_screen_reader_cli_export_parity_and_deployment_continuity_auto_narrowing/`
- **Boundary schema:**
  [`schemas/ui/m5-deployment-continuity-accessibility-fallback.schema.json`](../../schemas/ui/m5-deployment-continuity-accessibility-fallback.schema.json)
- **Checked support export:**
  [`artifacts/release/m5-deployment-continuity-accessibility-fallback-proof/support_export.json`](../../artifacts/release/m5-deployment-continuity-accessibility-fallback-proof/support_export.json)
  (plus `matrix.csv` and `report.md`)
- **Protected fixtures:**
  [`fixtures/ui/m5-deployment-continuity-accessibility-fallback/`](../../fixtures/ui/m5-deployment-continuity-accessibility-fallback/)

## Covered families

The packet certifies all nine reusable deployment/continuity families:

| Family | Primary weakening dimension |
| --- | --- |
| `install_profile_card` | `state_root_integrity` |
| `side_by_side_import_sheet` | `handler_ownership` |
| `rollout_ring_row` | `rollout_state` |
| `deployment_summary_card` | `control_plane_freshness` |
| `residual_dependency_row` | `residual_dependency` |
| `control_plane_data_plane_status_strip` | `control_plane_freshness` |
| `mirror_offline_artifact_row` | `mirror_verification` |
| `mode_change_review_sheet` | `state_root_integrity` |
| `channel_association_review_row` | `handler_ownership` |

## Invariants

- **Keyboard / screen-reader / CLI reach.** Each row exposes a non-visual path
  that reaches the same operating context, install/deployment ID, operating-mode
  label, rollout state, residual-dependency state, control/data-plane state, and
  mirror/offline state as the rich surface. The control-plane/data-plane status
  strip also binds its spatial layout to structured, list, textual, and CLI
  modalities.
- **Export parity.** Each row is copyable as text, JSON, and Markdown. Export
  summaries name typed fields and prohibit screenshot-only meaning, so support
  and admin exports reconstruct the same state shown in-product.
- **Honest auto-narrowing.** `partial` dimensions cap the effective claim at
  `review_required`, `stale` dimensions cap it at `local_cached_only`, and
  `unavailable` or `policy_blocked` dimensions cap it at `inspect_only`. A
  weakened lane must carry a `claim_narrow` block with the binding dimension,
  canonical downgrade trigger, precise label, and preserved canonical identity.
  An intact lane must not carry a spurious narrow block.
- **Cross-surface disclosure.** CLI-headless and support-export surfaces preserve
  the same labels as the desktop surface and disclose reduced interactions. Docs,
  help, release packets, and support/admin exports use the same narrowed state
  tokens, so claim publication and field triage stay aligned.

The packet is metadata-only: raw config bytes, credentials, license keys, mirror
URLs, provider cursors, and raw device identifiers never cross this boundary.

## Regenerating the artifacts

The seeded builder `seeded_m5_deployment_a11y_fallback_packet()` is the single
source of truth. The `on_disk_export_matches_builder` test asserts that the
checked support export is identical to the seeded packet. Regenerate the proof
artifacts from `export_safe_json`, `render_matrix_csv`, and
`render_markdown_summary`.
