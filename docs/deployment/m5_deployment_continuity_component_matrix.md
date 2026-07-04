# M5 Deployment/Continuity Component Matrix

Status: **Frozen** (M05-828, batch B97)

This document is the contract for the reusable **deployment/continuity component**
family. Where earlier M5 rows froze install topology, deployment-profile truth,
control-plane/data-plane continuity, side-by-side channels, portable-state packages, and
restore-fidelity systems, this lane freezes the reusable *cards, rows, sheets, and
strips* users and admins actually rely on to understand operating mode, rollout state,
mirror freshness, residual dependency, and local-safe continuity before acting.

One canonical packet — the
[`DeploymentContinuityComponentMatrix`](../../crates/aureline-install/src/freeze_the_m5_deployment_continuity_component_matrix/mod.rs)
— defines every reusable primitive, its state vocabulary, its required labels, and its
export / assistive parity expectations, so later M5 rows reference one canonical
component family instead of restating install / about / admin truth in feature-local
prose.

## Boundary artifacts

| Artifact | Path |
| --- | --- |
| Boundary schema | `schemas/ui/m5-deployment-continuity-component-matrix.schema.json` |
| Contract doc (this file) | `docs/deployment/m5_deployment_continuity_component_matrix.md` |
| Canonical support export (`include_str!`) | `artifacts/release/m5-deployment-continuity-component-proof/support_export.json` |
| Release matrix CSV | `artifacts/release/m5-deployment-continuity-component-proof/matrix.csv` |
| Design matrix summary | `artifacts/design/m5-deployment-continuity-component-matrix.md` |
| Protected fixtures | `fixtures/ui/m5-deployment-continuity-components/` |

The support export is the one source of truth shared by the Rust builder
(`seeded_deployment_continuity_component_matrix()`) and the on-disk JSON. The
`checked_support_export_matches_builder` test keeps them byte-aligned.

## Component families

Every reusable primitive later M5 rows reference by name. The matrix must define all nine.

| Family | Token | Freezes |
| --- | --- | --- |
| Install-profile card | `install_profile_card` | install mode, channel, updater owner, durable state roots |
| Side-by-side import sheet | `side_by_side_import_sheet` | handler ownership free of last-writer-wins capture, isolation |
| Rollout-ring row | `rollout_ring_row` | rollout ring and promotion state, rollback path |
| Deployment summary card | `deployment_summary_card` | operating mode, tenant / region, both planes visible |
| Residual-dependency row | `residual_dependency_row` | remaining vendor dependency, kept explicit |
| Control-plane/data-plane status strip | `control_plane_data_plane_status_strip` | the two planes stay distinct; impairment never masked as local failure |
| Mirror/offline artifact row | `mirror_offline_artifact_row` | mirror source, freshness, signature; stale never shown as current |
| Mode-change review sheet | `mode_change_review_sheet` | cache reuse / rollback reviewed before a durable boundary change |
| Channel-association review row | `channel_association_review_row` | current owner disclosed, reviewed before apply, no capture |

## Shared state vocabularies

Components bind to the same install-mode, provenance / freshness, client-scope, and
degraded-state language used elsewhere in Aureline rather than bespoke installer / admin
chrome.

- **Operating mode** (`deployment_mode`): `desktop`, `managed`, `self_hosted`,
  `portable`, `air_gapped`.
- **Provenance / freshness** (`truth_mode`): `live`, `mirrored`, `cached_offline`,
  `imported`, `provider_reported`. Only `live` is a current first-party source.
- **Required labels**: `identity`, `operating_mode`, `ownership_or_scope`,
  `freshness_class`, `continuity_state`, `keyboard_route`. The mandatory subset —
  `identity`, `operating_mode`, `freshness_class`, `keyboard_route` — appears on every
  row.
- **Downgrade triggers**: `control_plane_impaired`, `mirror_stale`, `offline_cache_only`,
  `signature_unverified`, `rollout_paused`, `handler_ownership_contested`,
  `state_root_unavailable`, `residual_vendor_dependency`, `provenance_incomplete`.

## Honesty rules (per row)

1. **Operating mode, ownership, and state roots stay explicit.** An install-profile card
   or deployment summary card never hides install mode, channel, updater owner, tenant /
   region, or durable state roots.
2. **No last-writer-wins handler capture.** Side-by-side import sheets and
   channel-association review rows keep handler ownership inspectable and never silently
   capture a default handler.
3. **Control-plane impairment never masquerades as local-runtime failure.** The status
   strip keeps the two planes distinct.
4. **Mirror / offline freshness is never shown as current.** A mirror/offline artifact
   row discloses freshness and signature truth; stale mirrored / cached content never
   reads as a live source.
5. **Self-hosted claims never omit residual vendor dependency.** A residual-dependency
   row keeps any remaining vendor dependency explicit.
6. **Mode switches / cache reuse / rollback are reviewed before durable boundary
   changes** — a mode-change review sheet shows the consequences before, never after.

Descriptors that carry a mode or freshness class must disclose the *same* class the row
records: an install-profile card's `install_mode` and a deployment summary card's
`operating_mode` match the row's `deployment_mode`, and a mirror/offline artifact row's
`freshness` matches the row's `truth_mode`. A row that disagrees is rejected as a
`descriptor_row_mismatch`.

## Export / assistive parity

Every row is `export_safe` and `assistive_ready`. Raw config bytes, credentials, license
keys, mirror URLs, provider cursors, and raw device identifiers never cross this
boundary; the packet carries only typed class tokens, opaque refs, booleans, and redacted
labels, so support and diagnostics exports can reconstruct exactly what a component would
have shown without leaking source or live payloads.

## Consumers

Product (About / install / update), docs / help, diagnostics, admin, support export, and
release-control surfaces all ingest these component rows instead of cloning chrome. Later
M5 rows reference one canonical component family instead of restating install /
deployment truth in feature-local prose.

## Guardrails

This lane hardens shared UI contracts layered on top of already-claimed
install/deployment/continuity systems. It does not widen into new installers, new rollout
engines, new managed services, or new mirror protocols, and it does not re-architect
packaging backends, sync services, or control-plane APIs.
