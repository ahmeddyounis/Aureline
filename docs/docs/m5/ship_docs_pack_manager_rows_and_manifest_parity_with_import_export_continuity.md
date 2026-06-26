# Docs-Pack Manager Rows And Manifest Parity With Import/Export Continuity

This document is the contract for the docs-pack manager: the rows/cards a person
uses to manage documentation packs, built on the canonical docs-pack manifest.
The manager keeps docs packs as versioned, mirrorable, exportable, policy-aware
product artifacts rather than hidden caches. The docs-browser manager, help-pane
manager, onboarding manager, settings docs-packs manager, air-gapped console, and
support export consume these rows directly rather than re-deriving
signer/channel/mirror/version state.

- Record kind: `docs_pack_manager_packet`
- Schema: [`schemas/docs/ship-docs-pack-manager-rows-and-manifest-parity-with-import-export-continuity.schema.json`](../../../schemas/docs/ship-docs-pack-manager-rows-and-manifest-parity-with-import-export-continuity.schema.json)
- Canonical support export: [`artifacts/docs/m5/ship_docs_pack_manager_rows_and_manifest_parity_with_import_export_continuity/support_export.json`](../../../artifacts/docs/m5/ship_docs_pack_manager_rows_and_manifest_parity_with_import_export_continuity/support_export.json)
- Summary artifact: [`artifacts/docs/m5/ship_docs_pack_manager_rows_and_manifest_parity_with_import_export_continuity.md`](../../../artifacts/docs/m5/ship_docs_pack_manager_rows_and_manifest_parity_with_import_export_continuity.md)
- Fixtures: [`fixtures/docs/m5/ship_docs_pack_manager_rows_and_manifest_parity_with_import_export_continuity/`](../../../fixtures/docs/m5/ship_docs_pack_manager_rows_and_manifest_parity_with_import_export_continuity/)
- Producer: `aureline_docs::current_stable_docs_pack_manager_export`

## Manifest parity

Each `DocsPackManagerRow` embeds the canonical `DocsPackManifest` owned by the
docs-pack truth packet, proving the manager reuses one manifest truth rather than
minting a parallel description. The manifest carries the pack id, signer, source
channel, version range, refresh state, mirror source, pin state, and schema
version. The row keeps each of these visible — the validator blocks promotion if a
row hides the signer, channel, mirror source, version range, refresh state, or
pin/offline posture.

## Manager row

On top of the manifest, a `DocsPackManagerRow` adds the manager layer:

| Field | Meaning |
| --- | --- |
| `row_id` | Stable id for the managed-pack row. |
| `lifecycle_flow` | One of `local_only`, `mirrored`, `managed`, or `air_gapped`; keeps mirror and offline flows first-class. |
| `pack_size_bytes` / `document_count` | Pack size and document count; absent (with disclosure) when the payload is unavailable locally. |
| `last_successful_refresh_at` / `last_refresh_attempt_at` | Refresh history surfaced on the row. |
| `actions` | The pin/unpin, refresh, remove, change-mirror-source, set-offline-availability, and export affordances, each with an availability and a disclosed reason when disabled. |
| `import_export_continuity` | The continuity block that preserves docs-pack identity and lifecycle state across import and export. |
| `shows_*` / `signature_state_visible` / `unavailable_payload_disclosed` | The visibility assertions the validator enforces. |
| `degraded_to_opaque_cache` / `browser_only_fallback_wording` | Must stay false; a mirror/offline pack never collapses into opaque cache or browser-only wording. |

## Manager actions

Every row exposes the manager actions a person needs: a pin or unpin affordance
(exactly one, consistent with the current pin state), plus refresh, remove,
change-mirror-source, set-offline-availability, and export. An action that is
disabled by policy, disabled because the payload is unavailable, or not applicable
to the pack must name a disclosed reason; an action that drops its reason blocks
promotion.

## Import/export continuity

A `DocsPackImportExportContinuity` block records the import provenance
(`freshly_installed`, `imported_bundle`, `mirrored_sync`, `air_gapped_sideload`,
or `operator_managed`), the optional import and export bundle refs, and a stable
continuity token carried verbatim across import and export. The block asserts that
both pack identity and lifecycle state are preserved on export, so a managed or
air-gapped pack never flattens into generic documentation cache metadata.

## Profile projections

A `DocsPackManagerProfileProjection` records, per claimed M5 manager surface, that
the manager truth is reused without drift. Each projection names the profile and
asserts that it preserves row identity, shows signer/channel/mirror source, shows
pin/offline/refresh posture, shows the version range, preserves the import/export
continuity and lifecycle state, supports JSON export, and excludes raw private
material and ambient authority. The packet requires a projection for every
profile:

- `docs_browser_manager`
- `help_pane_manager`
- `onboarding_manager`
- `settings_docs_packs_manager`
- `air_gapped_console`
- `support_export`

## Invariants

The packet's validator blocks promotion when any invariant fails:

- Every row keeps signer, channel, mirror source, version range, refresh state,
  and pin/offline posture visible.
- An unavailable payload or signature state is disclosed rather than hidden.
- A mirror, offline, or air-gapped row never degrades into opaque cache or
  browser-only fallback wording.
- Every required manager action is present and the pin/unpin posture is
  consistent; a disabled or not-applicable action names a disclosed reason.
- Import/export continuity preserves docs-pack identity and lifecycle state.
- A row's lifecycle flow stays consistent with its import provenance.
- All four lifecycle flows (`local_only`, `mirrored`, `managed`, `air_gapped`) are
  represented, and every required profile has a faithful projection.
- Raw document bodies, raw URLs, raw provider payloads, and credentials never
  cross the boundary.

## Consumers

The docs-browser manager, help-pane manager, onboarding manager, settings
docs-packs manager, air-gapped console, and support export consume the checked-in
packet directly. The support export preserves the exact packet identity and
lifecycle state without exporting raw private material or ambient authority.
