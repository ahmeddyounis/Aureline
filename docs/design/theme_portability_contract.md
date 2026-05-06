# Theme portability record, provenance, and sync/export carry contract

This document freezes the **durable portability record** every theme
importer, migration-center path, profile export, managed-sync lane, and
support exporter uses to preserve appearance provenance and translation
truth across time.

The contract exists to prevent a common failure mode: an imported theme
is translated once, then flattened into an opaque profile blob. When
that happens, later sync/export/support flows cannot explain:

- where the theme came from (ecosystem/tool/version/artifact),
- what was translated versus substituted,
- what was unsupported or unresolved,
- which downgrade notes were emitted when a carrier could not preserve
  full fidelity.

The machine-readable schema lives at:

- [`/schemas/design/theme_portability_record.schema.json`](../../schemas/design/theme_portability_record.schema.json)

Worked fixtures live under:

- [`/fixtures/design/theme_portability_cases/`](../../fixtures/design/theme_portability_cases/)

This contract is normative. Where it disagrees with the PRD, TAD, TDD,
UI/UX spec, design-system style guide, or the upstream appearance/token
contracts it composes with, those sources win and this document plus the
schema and fixtures MUST be updated in the same change.

## Companion contracts

This contract composes with existing owners by reference instead of
re-minting their vocabulary:

- [`/docs/design/theme_support_and_inheritance_contract.md`](./theme_support_and_inheritance_contract.md),
  [`/schemas/design/theme_import_mapping_report.schema.json`](../../schemas/design/theme_import_mapping_report.schema.json),
  and
  [`/schemas/design/theme_support_row.schema.json`](../../schemas/design/theme_support_row.schema.json)
  own audited theme-support claims and the design-side import-mapping
  report (structured unresolved/unsupported slot evidence). This
  contract defines the **durable portability wrapper** those records
  must remain reachable through.
- [`/docs/ux/theme_and_visual_asset_contract.md`](../ux/theme_and_visual_asset_contract.md)
  and
  [`/schemas/ux/theme_package_manifest.schema.json`](../../schemas/ux/theme_package_manifest.schema.json)
  own user-facing package distribution, signature state, mirrorability,
  and permitted deployment profiles. This contract re-exports those
  fields so support/export flows can cite them without embedding the
  full theme manifest body.
- [`/docs/ux/appearance_import_and_checkpoint_contract.md`](../ux/appearance_import_and_checkpoint_contract.md)
  and
  [`/schemas/ux/theme_import_report.schema.json`](../../schemas/ux/theme_import_report.schema.json)
  own the user-facing import-review workflow and its report record. The
  portability record cites the user-facing report by id.
- [`/docs/state/profile_and_state_map.md`](../state/profile_and_state_map.md)
  and
  [`/docs/state/portable_state_package_contract.md`](../state/portable_state_package_contract.md)
  own the portability/sync posture for `themes_and_design_tokens`
  (`$AURELINE_CONFIG/themes/*`). This contract defines which theme
  provenance facts MUST survive those portability lanes.

## Scope

Frozen at this revision:

- One `theme_package_portability_record` per theme-package revision,
  capturing:
  - provenance (ecosystem/tool/version/artifact handle when imported),
  - translation coverage (translated counts, unresolved counts),
  - unsupported-slot and unresolved-slot notes suitable for export,
  - portability flags and parity-claim state,
  - explicit downgrade events when a carrier drops a fact that this
    contract normally requires it to carry.

## Out of scope

- Sync-service implementation, marketplace ingestion, or registry
  protocol design.
- Theme rendering, stylesheet generation, token resolution, or any UI
  implementation. This is an inspection/carry contract only.

## 1. Record boundary (do not flatten)

Theme portability is not one blob. The following record families remain
distinct and are linked by opaque ids:

| Record family | What it owns | Where it lives |
|---|---|---|
| **Theme package manifests** | package identity, supported modes, token-set handles, distribution/signature/mirrorability | design + UX manifest contracts |
| **Import-mapping report (audited)** | per-slot mapping rows, unresolved/unsupported evidence, protected-cue honesty checks | `theme_import_mapping_report_record` |
| **User-facing import report** | import-review disclosure and rollback UI linkage | `theme_import_report_record` |
| **Theme portability record** | durable wrapper tying provenance + translation summary + downgrade events together for sync/export/support | `theme_package_portability_record` |

Rules (frozen):

1. A surface MUST NOT replace the audited import-mapping report with
   free-form prose inside a portability record. Prose is a summary; the
   structured report remains the evidence.
2. A carrier MUST NOT flatten imported-theme provenance into an opaque
   profile blob. It MUST preserve (a) the portability record and (b) the
   referenced mapping/report ids, per the carry rules in §3.

## 2. Theme-package portability record

The portability record is the stable, export-safe description of a
theme package’s portability truth. It is safe to log, safe on RPC, safe
in portable profile packages, safe in managed-sync payloads, and safe in
support exports.

Required fields (frozen):

- `portability_ref` — stable id used by audited theme-support claims
  (`import_export_portability_ref`) and by export/support surfaces.
- `theme_package_manifest_ref`, `theme_package_revision_ref` —
  cross-links back to the design-side appearance manifest refs used by
  audited claims.
- `package_id`, `package_revision_ref`, `package_version_label` —
  canonical package identity.
- `provenance_class` — how the package entered the system
  (built-in/extension/community/imported/offline).
- `signature_state`, `mirrorability_class`, `permitted_deployment_profiles[]`
  — portability/trust posture required by later export surfaces.
- `import_provenance` and `import_mapping_report_ref` — import tool,
  version, and evidence linkage (nullable for non-imported packages).
- `translation_summary` — translated count, unresolved count, and
  portable unsupported/unresolved notes (nullable where not applicable).
- `parity_claim_state` — `claimed_with_report` parity is forbidden for
  imported packages unless `import_mapping_report_ref` is present.
- `portability_flags[]` — compact flags readers can project without
  re-parsing downstream reports.
- `downgrade_events[]` — carrier-emitted downgrade notes when facts are
  dropped in a portability lane that permits dropping.
- `policy_context`, `redaction_class`, `minted_at` — attribution and
  export-safe privacy posture.

## 3. Carry rules (sync, export, import, support)

This contract defines which appearance facts MUST be carried forward and
which MAY be dropped only with an explicit downgrade event.

### 3.1 Appearance fact classes

The portability schema classifies carryable facts by a closed
`appearance_fact_class` set (see schema). Carriers reason about carry
rules in terms of these fact classes rather than ad-hoc field lists.

### 3.2 Carry requirements by portability lane

1. **Portable profile export and managed sync (full fidelity).**
   `themes_and_design_tokens` is `portable` and `synced_opt_in`. A
   carrier that includes a theme package MUST carry:
   - the portability record,
   - the referenced audited import-mapping report (when non-null),
   - unsupported/unresolved slot notes (no summary-only substitution).
   Dropping any `appearance_fact_class` in these lanes is non-conforming.
2. **Support export (metadata-only).** Support exports MAY omit large
   per-slot detail *only* when:
   - aggregate translation counts remain present, and
   - a `downgrade_event` lists the dropped fact classes and provides a
     short downgrade note pointing at the full-fidelity portability
     lanes (profile export / sync).
   A support export that omits provenance fields (ecosystem/tool/version)
   is non-conforming.
3. **Migration-center and restore paths (durable truth).** Migration and
   repair flows MUST preserve portability records verbatim (unknown
   fields included) per the state map’s unknown-field posture for
   `themes_and_design_tokens`. A migration that drops unsupported-slot
   or unresolved-slot notes without a downgrade event is non-conforming.

## 4. Parity-claim honesty (imported themes)

Imported themes MUST NOT claim parity unless:

- `import_mapping_report_ref` is non-null, and
- `parity_claim_state` and the translation summary are consistent with
  the audited report (for example, `claimed_with_report` requires zero
  unresolved mappings and zero blocked honesty checks).

Where a carrier drops mapping detail (support exports only), the carrier
MUST NOT widen parity claims; it MAY narrow claims and MUST cite the
downgrade event.

## Fixture coverage

The fixture corpus covers:

- a built-in theme package portability record (no import lineage),
- an imported-translated theme portability record with unsupported-slot
  notes and an attached mapping report ref,
- a support-export projection that preserves provenance + counts while
  emitting a downgrade event for elided per-slot detail.

