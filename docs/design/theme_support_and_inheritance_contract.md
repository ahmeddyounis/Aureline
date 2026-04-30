# Theme/contrast support row, import-mapping report, and extension/embedded UI appearance inheritance contract

This document freezes the **audited boundary** every first-party shell
surface, imported-theme reviewer, extension/webview author, screenshot-
diff packet, support exporter, and conformance gate reads when claiming
or inspecting visual-system support. It exists so theme, contrast,
density, and reduced-motion support become **inspectable rows** instead
of implied quality, so imported themes surface parity gaps explicitly,
and so extension or embedded-surface appearance posture can be diffed
mechanically against a canonical token export.

The contract is normative. Where it disagrees with the design-token
vocabulary, the appearance-session contract, the appearance import-
parity contract, the embedded boundary card contract, the theme-and-
visual-asset contract, the UI/UX spec, or the design-system style
guide it quotes, those documents win and this contract plus the
schemas and fixtures MUST be updated in the same change. Where this
document disagrees with a downstream surface's local support badge,
import banner, or extension parity claim, this document wins and the
surface is non-conforming.

The machine-readable schemas live at:

- [`/schemas/design/theme_support_row.schema.json`](../../schemas/design/theme_support_row.schema.json)
- [`/schemas/design/theme_import_mapping_report.schema.json`](../../schemas/design/theme_import_mapping_report.schema.json)
- [`/schemas/design/extension_ui_appearance_descriptor.schema.json`](../../schemas/design/extension_ui_appearance_descriptor.schema.json)

The canonical token-export linkage every record cites lives at:

- [`/schemas/design/token_export_manifest.schema.json`](../../schemas/design/token_export_manifest.schema.json)

Worked fixtures live under:

- [`/fixtures/design/theme_support_cases/`](../../fixtures/design/theme_support_cases/)

## Companion contracts

This contract reuses and composes with existing owners by reference
instead of re-minting their vocabulary:

- [`/docs/design/design_token_component_state_vocabulary.md`](./design_token_component_state_vocabulary.md)
  and
  [`/schemas/design/token_export_manifest.schema.json`](../../schemas/design/token_export_manifest.schema.json)
  own the `theme_class`, `accessibility_posture_class`, `density_class`,
  `trust_visual_state_class`, `semantic_status_class`, and surface-class
  vocabulary. This contract cites those values and never mints
  parallels.
- [`/docs/design/appearance_session_contract.md`](./appearance_session_contract.md)
  and
  [`/schemas/design/appearance_session.schema.json`](../../schemas/design/appearance_session.schema.json)
  own the live appearance session, follow-system policy, and revision
  events. Support rows cite session refs by id; this contract does not
  re-mint live-update vocabulary.
- [`/docs/ux/appearance_import_and_checkpoint_contract.md`](../ux/appearance_import_and_checkpoint_contract.md),
  [`/schemas/ux/appearance_checkpoint.schema.json`](../../schemas/ux/appearance_checkpoint.schema.json),
  and
  [`/schemas/ux/theme_import_report.schema.json`](../../schemas/ux/theme_import_report.schema.json)
  own the user-facing import-review and checkpoint workflow. The
  design-side import-mapping report in this contract is the **audited
  twin**: it cites the user-facing report by id and pins its mapping
  rows, fallback classes, and rollback path against the canonical
  token export.
- [`/docs/ux/theme_and_visual_asset_contract.md`](../ux/theme_and_visual_asset_contract.md)
  and
  [`/schemas/ux/theme_package_manifest.schema.json`](../../schemas/ux/theme_package_manifest.schema.json)
  own theme-package shape, distribution, signature, and mirrorability.
  Every support-row claim cites a theme-package manifest ref and a
  theme-package revision ref; the row never re-mints the package's
  fields.
- [`/docs/ux/embedded_surface_boundary_cards.md`](../ux/embedded_surface_boundary_cards.md)
  and
  [`/schemas/ux/embedded_boundary_card.schema.json`](../../schemas/ux/embedded_boundary_card.schema.json)
  own the host-rendered owner/origin chrome for embedded surfaces. The
  extension UI appearance descriptor cites a boundary-card ref by id;
  it does not re-derive trust posture.

## Why freeze this now

Without an audited boundary, theme, contrast, density, and reduced-
motion support drift the moment each surface invents its own claim:

- a shell surface ships a high-contrast theme that "mostly works" but
  no row records which trust cue depends on hue or which keyboard
  affordance becomes invisible under forced-colors;
- an imported VS Code or JetBrains theme renders in dark mode but the
  user cannot tell which syntax scopes were translated, which were
  unmapped, or what the rollback path is if the import drifts;
- an extension webview claims theme parity but silently falls back to
  fixed colors under high-contrast, leaving reviewers to diff
  screenshots to discover the gap;
- a screenshot-diff packet asserts "AA contrast" but cannot trace the
  claim back to a specific token row in the canonical export, so a
  later release can change the value without invalidating the claim;
- a marketplace account dashboard inherits host theme but its
  product-owned action toolbar uses a parallel density token, so the
  user sees compact rows next to comfortable rows with no recorded
  reason.

This contract forecloses these patterns by treating support, import
parity, and inheritance as three audited record kinds that all cite
one canonical token export, one theme-package manifest, and one
appearance-session contract.

## Scope

Frozen at this revision:

- one `theme_support_row_claim_record` per (surface_class,
  theme_package, claimed_mode) tuple, declaring explicit support state
  for dark, light, high-contrast dark, and high-contrast light, plus
  density-aware behavior, reduced-motion behavior, and forced-colors
  behavior on the claimed shell surface;
- one `theme_support_audit_event_record` per claim emission, refusal,
  evidence rotation, or downgrade on the support audit stream;
- one `theme_import_mapping_report_record` per imported translated
  theme, declaring source tool/version, target theme classes,
  per-slot mapping rows, syntax coverage, fallback token classes,
  unresolved/blocked counts, protected-cue honesty checks, and the
  rollback path back to the appearance checkpoint;
- one `theme_import_mapping_audit_event_record` per import emission,
  refusal, parity-claim downgrade, or rollback;
- one `extension_ui_appearance_descriptor_record` per extension or
  embedded-surface appearance claim, carrying inheritance rows for
  theme, focus, contrast, density, and reduced-motion (closed
  three-state vocabulary `inherits` / `partial` / `does_not_inherit`),
  per-mode theme/contrast support rows, protected-cue preservation,
  and the embedded-boundary-card ref;
- one `extension_ui_appearance_audit_event_record` per descriptor
  emission, refusal, gap disclosure, or downgrade.

## Out of scope

- Theme engine, settings UI, signal-watcher, screenshot-diff runner,
  or marketplace theming UI implementation. Those compose over this
  contract.
- Theme asset production. The fixtures here exercise the audited
  boundary; they do not seed final theme values, which live in
  `.t2/docs/Aureline_UX_Design_System_Style_Guide.md`.
- The full token export manifest body and the user-facing import
  workflow body. Those remain owned by the token-export-manifest and
  appearance-import-and-checkpoint contracts; this contract cites them
  by id only.
- Conformance-gate tooling. The records here are the input every later
  conformance gate diffs against; the runner itself is downstream.

## 1. Record boundary

Every record under this contract MUST resolve every field to exactly
one of the four boundaries below. Flattening them into one payload is
non-conforming.

| Boundary | What it carries | Where it lives |
|---|---|---|
| **Theme/contrast support claim** | surface class, claimed theme/contrast/density/reduced-motion support, forced-colors behavior, protected-cue preservation, evidence refs, theme-package ref, token-export ref | `theme_support_row_claim_record` |
| **Theme import-mapping report (audited)** | source tool/version, target theme classes, translated/unsupported/unresolved slot rows, syntax coverage, fallback token classes, rollback path, protected-cue honesty checks, theme-package ref, token-export ref | `theme_import_mapping_report_record` |
| **Extension/embedded UI appearance descriptor** | extension/surface refs, surface family, parity claim state, inheritance rows for theme/focus/contrast/density/reduced-motion, per-mode support rows, protected-cue preservation, embedded boundary ref, token-export ref | `extension_ui_appearance_descriptor_record` |
| **Audit events** | per-record id, audit-event id, denial reason, subject ref, policy-context stamp | `*_audit_event_record` (one per record family) |

Rules (frozen):

1. Every claim, report, and descriptor record MUST cite exactly one
   `token_export_manifest_ref`. The ref pins the canonical token-export
   manifest the row was minted against; later screenshot-diff packets
   and conformance audits compare implementation against that ref.
2. Every claim record MUST cite exactly one
   `theme_package_manifest_ref` and exactly one
   `theme_package_revision_ref`. A claim that does not pin the package
   revision is non-conforming.
3. Every report record MUST cite exactly one
   `appearance_checkpoint_ref` and exactly one rollback-path row. An
   imported theme without a recorded rollback path is non-conforming.
4. Every descriptor record MUST cite exactly five inheritance rows
   (theme, focus, contrast, density, reduced_motion). A descriptor
   that omits any axis is non-conforming.
5. Audit events MUST NOT carry raw token bytes, raw screenshots, raw
   URLs, raw asset payloads, or raw user content. Cross-record
   references travel by opaque id only.

## 2. Theme/contrast support row

The support row is the **audited claim** that a first-party shell
surface (or an embedded/extension surface that opts into host parity)
declares support for a specific theme class × contrast mode × density
class × accessibility posture combination on a specific theme package
revision.

Closed support-state vocabulary:

| Display label | Machine enum | Meaning |
|---|---|---|
| `Supported` | `supported` | The surface renders the claimed mode at parity quality on this theme package revision. Evidence refs MUST be present. |
| `Partial` | `partial` | The surface renders the mode but with at least one disclosed gap. The row MUST cite the gap rows; silent partial parity is non-conforming. |
| `Unsupported` | `unsupported` | The surface explicitly does not render the mode and falls back to a sibling mode under a recorded substitution rule. The row MUST cite the substitution rule. |
| `Not claimed` | `not_claimed` | The surface has not been reviewed for this mode on this revision. A reviewer MUST treat the row as a coverage gap, not as silent failure. |
| `Denied` | `denied` | A claim was refused (signature failure, schema lag, color-alone state violation, etc.). The denial event row carries the reason. |

Required record fields (frozen):

- `record_kind` — const `theme_support_row_claim_record`.
- `theme_support_schema_version` — integer schema version.
- `claim_id` — opaque, stable id for this claim.
- `surface_class` — closed surface-class enum, re-exported from the
  design-token export and extended with the embedded/extension
  surface-family classes.
- `theme_package_manifest_ref` — opaque ref to the theme-package
  manifest record this claim is bound to.
- `theme_package_revision_ref` — opaque ref to the theme-package
  revision (frozen for the lifetime of the claim).
- `token_export_manifest_ref` — opaque ref to the canonical token-
  export manifest the claim was minted against.
- `claimed_mode` — object resolving the four mode axes:
  - `theme_class` — closed theme-class enum.
  - `contrast_mode` — `contrast_standard`, `contrast_high`, or
    `contrast_forced_colors`.
  - `density_class` — closed density enum.
  - `accessibility_posture_class` — closed accessibility-posture enum.
- `support_state` — exactly one value from the closed support-state
  vocabulary.
- `dark_support_state`, `light_support_state`,
  `high_contrast_dark_support_state`,
  `high_contrast_light_support_state` — closed support-state values
  per theme class. Together these four columns ARE the audited support
  matrix; a claim that resolves any of them to `not_claimed` discloses
  a coverage gap.
- `density_aware_behavior` — object describing the density posture:
  - `supported_density_classes[]` — closed density enum, minItems 1.
  - `affects_information_architecture` — boolean, MUST be `false`.
  - `affects_focus_visibility` — boolean, MUST be `false`.
  - `affects_state_conveyance` — boolean, MUST be `false`.
  - `notes` — short reviewer note (optional).
- `reduced_motion_behavior` — object describing the accessibility-
  posture behavior the claim honours, listing
  `suppressed_motion_families[]`, `preserves_focus_visibility = true`,
  `preserves_state_conveyance = true`, and the
  `engagement_cue` short label.
- `forced_colors_behavior` — closed enum
  (`defers_to_system`, `applies_high_contrast_token_overrides`,
  `escalates_to_forced_colors_safe_subset`,
  `not_applicable_no_color_dependence`,
  `denied_color_alone_violation`).
- `protected_cue_preservation` — array (minItems 4) covering trust,
  policy_lock, severity, and source_integrity. Each row carries
  `non_color_cues[]` (minItems 1), `color_alone_prohibited = true`,
  `preserved_in_high_contrast = true`, and
  `preserved_in_forced_colors = true`.
- `token_overrides_namespace` — short label naming the namespace this
  claim's overrides ship under (e.g. `al.color`, `al.color.diff`).
- `import_export_portability_ref` — opaque ref to the theme-package
  manifest's portability record (signature state, mirrorability,
  permitted deployment profiles). Required so later support/export
  flows can reuse the audited row.
- `linked_appearance_session_refs[]` — opaque refs to the appearance-
  session records under which this row was last validated (optional;
  may be empty in seed fixtures).
- `evidence_refs[]` — opaque refs to screenshot diffs, keyboard
  journeys, assistive-tech checks, token-drift checks, or other
  evidence packets backing the claim. Required (minItems 1) for
  `supported`; optional for `not_claimed`.
- `gap_refs[]` — opaque refs to gap rows the partial / unsupported /
  denied claims cite. Required (minItems 1) when the claim is
  `partial`, `unsupported`, or `denied`.
- `policy_context` — managed-policy epoch and trust-state stamp.
- `redaction_class` — closed redaction-class enum.
- `minted_at` — producer-local monotonic timestamp.

Rules (frozen):

1. Every claim MUST resolve all four per-theme-class support columns
   (`dark_support_state`, `light_support_state`,
   `high_contrast_dark_support_state`,
   `high_contrast_light_support_state`). A claim that omits any
   column is non-conforming.
2. `support_state = supported` requires at least one
   `evidence_refs[]` entry. A `supported` claim with no evidence is
   non-conforming.
3. `support_state ∈ {partial, unsupported, denied}` requires at least
   one `gap_refs[]` entry. Silent partial / unsupported / denied
   claims are non-conforming.
4. `density_aware_behavior.affects_information_architecture`,
   `density_aware_behavior.affects_focus_visibility`, and
   `density_aware_behavior.affects_state_conveyance` are all frozen
   `false`. A surface that changes information architecture or focus
   visibility under density is non-conforming and MUST emit a
   `denied` claim with the corresponding denial reason.
5. `reduced_motion_behavior.preserves_focus_visibility` and
   `reduced_motion_behavior.preserves_state_conveyance` are frozen
   `true`. A claim that hides focus rings or strips state chrome under
   any reduced-motion posture is non-conforming.
6. `forced_colors_behavior = denied_color_alone_violation` requires
   the surface to emit a `denied` claim and a denial event citing
   `color_alone_conveyed_required_meaning`.
7. Protected-cue preservation MUST cover trust, policy_lock,
   severity, and source_integrity. A row missing any of the four is
   non-conforming.
8. Each claim is attributable to exactly one
   `theme_package_manifest_ref`, one `theme_package_revision_ref`,
   one `token_export_manifest_ref`, and one
   `import_export_portability_ref`. Multi-package claims MUST mint
   multiple rows.

## 3. Theme import-mapping report (audited)

The import-mapping report is the **audited boundary** for every
imported translated theme. It pins the source tool/version, the target
theme classes, the per-slot mapping rows, syntax coverage, fallback
token classes, unresolved/blocked counts, the protected-cue honesty
checks, and the rollback path back to the appearance checkpoint that
minted it.

Closed `mapping_state` vocabulary (per slot row):

| Machine enum | Meaning |
|---|---|
| `translated` | The source slot maps to a canonical token in the export with no fallback. |
| `substituted_fallback` | The slot resolved to a fallback token class because no exact target exists. The fallback class is recorded. |
| `unsupported` | The slot has no target and no safe fallback. The row renders inert; rendering with a hue-only stand-in is non-conforming. |
| `unresolved` | The slot was recognised but the import could not pick a target this revision. The row is held for review. |
| `blocked_honesty` | The slot would have resolved to a value that masks a protected cue. The row is denied; rendering proceeds with the canonical token. |
| `deprecated_replacement` | The slot maps to a deprecated token whose replacement is recorded. |

Closed `fallback_token_class` vocabulary: `semantic_neutral`,
`semantic_warning`, `semantic_danger`, `trust_visual_state`,
`syntax_default`, `syntax_comment`, `diff_default`,
`focus_ring_default`, `density_default`, `motion_reduced_default`,
`no_fallback_blocked`.

Closed `parity_claim_state` vocabulary: `not_claimed`,
`claimed_with_report`, `partial_claim_with_gaps`,
`denied_unresolved_or_blocked`.

Closed `rollback_path_class` vocabulary:
`restore_appearance_checkpoint`, `discard_preview`,
`reopen_import_review`, `manual_repair_required`,
`rollback_unavailable_denied`.

Required record fields (frozen):

- `record_kind` — const `theme_import_mapping_report_record`.
- `theme_import_mapping_schema_version` — integer schema version.
- `report_id` — opaque, stable id.
- `import_session_ref` — opaque id of the originating import session.
- `source_tool` — object pinning `source_ecosystem` (closed enum:
  `vscode`, `jetbrains`, `vim`, `emacs`, `zed`, `sublime`, `textmate`,
  `unknown`), `source_tool_name`, `source_tool_version`, and
  `source_theme_identifier`.
- `source_artifact_ref` — opaque ref to the imported artifact.
- `target_theme_package_ref` — nullable opaque ref to the resulting
  theme-package manifest record. Null only when the import has not
  yet minted a target package.
- `target_theme_classes[]` — closed theme-class enum, minItems 1.
- `token_export_manifest_ref` — opaque ref to the canonical token-
  export manifest the report was minted against. Required so the
  audited row can be traced back to the canonical token set.
- `translated_slots[]`, `unsupported_slots[]`,
  `unresolved_slots[]`, `blocked_slots[]` — arrays of slot rows.
- `mapping_summary` — counts of total source slots, translated,
  unsupported, unresolved, substituted-with-fallback, and
  blocked-honesty rows. Counts MUST sum to `total_source_slot_count`.
- `syntax_token_coverage` — object covering source-scope counts and a
  `coverage_percent` integer (0–100).
- `fallback_token_classes[]` — closed `fallback_token_class` enum.
- `protected_cue_honesty_checks[]` — array (minItems 4) covering
  trust, policy_lock, severity, and source_integrity.
- `appearance_checkpoint_ref` — opaque ref to the appearance
  checkpoint this import minted under.
- `rollback_path` — object resolving `rollback_path_class`,
  `checkpoint_ref`, `rollback_ref`, and `user_visible_action_id`.
- `parity_claim_state` — closed parity-claim enum.
- `import_outcome` — closed enum (`preview_ready`, `applied`,
  `applied_with_warnings`, `blocked`, `rolled_back`, `cancelled`,
  `policy_denied`, `review_required`).
- `linked_ux_import_report_ref` — nullable opaque ref to the user-
  facing report owned by the appearance import-and-checkpoint
  contract. The audited row pins the user-facing record by id; the
  audited row never re-mints the user-facing row's prose.
- `policy_context`, `redaction_class`, `minted_at`.

Rules (frozen):

1. The mapping summary counts MUST sum to
   `total_source_slot_count`. A summary that disagrees with its
   per-slot rows is non-conforming.
2. `parity_claim_state = claimed_with_report` requires
   `unresolved_mapping_count = 0` and `blocked_honesty_count = 0`.
3. `parity_claim_state = denied_unresolved_or_blocked` requires
   either `unresolved_mapping_count > 0` or
   `blocked_honesty_count > 0`.
4. Protected-cue honesty checks MUST cover trust, policy_lock,
   severity, and source_integrity. Each check carries
   `color_alone_prohibited = true`, `preserved_in_high_contrast = true`,
   and `preserved_in_forced_colors = true`.
5. Every report MUST cite a non-null `appearance_checkpoint_ref` and a
   `rollback_path` row. An import without a recorded rollback path is
   non-conforming.
6. Reports MUST NOT silently drop unknown source tokens. Unknown
   tokens MUST appear in `unresolved_slots[]` or `unsupported_slots[]`.

## 4. Extension/embedded UI appearance descriptor

The descriptor is the audited row for any extension, embedded webview,
service-dashboard, marketplace-account, docs/help embedded surface,
native extension panel, or terminal-decoration surface that claims
host appearance parity. It is the design-side twin of the user-facing
`extension_surface_appearance_record` owned by the appearance
import-and-checkpoint contract; the descriptor cites that record by
id and pins the audited inheritance and support rows against the
canonical token export.

Closed `surface_family` vocabulary (re-exported and extended):

| Machine enum | Meaning |
|---|---|
| `extension_hosted_surface` | A native extension panel rendered inside the host shell with extension-supplied content. |
| `embedded_webview_surface` | A webview-backed surface contributed by an extension. |
| `service_dashboard_surface` | A provider-owned service dashboard rendered embedded. |
| `docs_help_embedded_surface` | A product-owned docs/help pane rendered embedded. |
| `native_extension_panel` | A native extension panel that does not use a webview. |
| `terminal_decoration_surface` | A terminal decoration overlay contributed by an extension. |
| `marketplace_account_surface` | Marketplace and account dashboard surfaces. |

Closed inheritance vocabulary (per axis row): `inherits`, `partial`,
`does_not_inherit`.

Closed inheritance axis vocabulary: `theme`, `focus`, `contrast`,
`density`, `reduced_motion`.

Required record fields (frozen):

- `record_kind` — const `extension_ui_appearance_descriptor_record`.
- `extension_ui_appearance_schema_version` — integer schema version.
- `descriptor_id` — opaque, stable id.
- `extension_package_ref` — opaque ref to the contributing extension
  or product package.
- `surface_ref` — opaque ref to the rendered surface.
- `surface_family` — closed surface-family enum.
- `owner_label` — short privacy-safe owner label (e.g. extension
  publisher, service identity).
- `parity_claim_state` — closed enum (`no_parity_claim`,
  `claims_host_parity`, `partial_claim_with_gaps`, `denied_claim`).
- `inheritance_rows[]` — array (minItems 5) covering theme, focus,
  contrast, density, and reduced_motion. Each row carries
  `inheritance_state`, `claim_source_ref`, `known_gap_count`,
  `gap_refs[]`, and `user_visible_disclosure_required` (boolean).
- `theme_contrast_support_rows[]` — array (minItems 1) of audited
  support rows for the modes the descriptor claims; each row
  references a `theme_support_row_claim_record` by id so the
  descriptor never re-mints the support matrix.
- `protected_cue_preservation[]` — array (minItems 4) covering trust,
  policy_lock, severity, and source_integrity.
- `token_export_manifest_ref` — opaque ref to the canonical token-
  export manifest. Required so descriptors can be traced back to the
  canonical token set.
- `linked_theme_package_manifest_ref` — nullable opaque ref to the
  contributing extension's theme package, when present.
- `linked_ux_extension_appearance_record_ref` — nullable opaque ref
  to the user-facing extension appearance record owned by the
  appearance import-and-checkpoint contract.
- `embedded_boundary_card_ref` — nullable opaque ref to the host-
  rendered embedded boundary card. Required (non-null) for
  `surface_family ∈ {embedded_webview_surface,
  service_dashboard_surface, docs_help_embedded_surface,
  marketplace_account_surface}`.
- `appearance_session_ref` — nullable opaque ref to the appearance
  session in effect when the descriptor was minted.
- `policy_context`, `redaction_class`, `minted_at`.

Rules (frozen):

1. Every descriptor MUST publish exactly five inheritance rows with
   `axis ∈ {theme, focus, contrast, density, reduced_motion}`. A
   descriptor that omits any axis is non-conforming.
2. `inheritance_state ∈ {partial, does_not_inherit}` requires
   `known_gap_count ≥ 1` and `user_visible_disclosure_required = true`.
3. `parity_claim_state = claims_host_parity` requires every
   inheritance row to be `inherits`. A descriptor that claims host
   parity while a row reports `partial` or `does_not_inherit` is
   non-conforming.
4. Embedded surfaces (webview, service dashboard, marketplace account,
   docs/help embedded) MUST cite a non-null
   `embedded_boundary_card_ref`. Native extension panels and terminal
   decoration surfaces MAY cite null when no boundary card has been
   minted; the descriptor MUST then emit a `parity_claim_state` of
   `partial_claim_with_gaps` or `denied_claim`.
5. Protected-cue preservation MUST cover trust, policy_lock, severity,
   and source_integrity.
6. Every descriptor MUST cite the canonical
   `token_export_manifest_ref`. A descriptor that claims theme parity
   without pinning a token export ref is non-conforming.

## 5. Source-of-truth token export linkage

Every record under this contract names exactly one canonical
`token_export_manifest_ref`. The ref pins:

- the `design_token_schema_version` the row was minted against;
- the `manifest_id` of the design-token export the row references;
- via that manifest, the per-entity record ids for token families,
  component states, theme support rows, accessibility postures, layer
  orders, and scrim tokens the canonical vocabulary publishes.

This linkage exists so screenshot-diff packets, token-conformance
audits, and support evidence packets can answer one question
mechanically: **does the implementation still resolve the same token
set this row was minted against?** A claim that cannot be traced back
to a canonical token export is non-conforming.

Rules (frozen):

1. Every claim, report, and descriptor record cites exactly one
   `token_export_manifest_ref`.
2. The cited manifest MUST be the canonical
   `design_token_export_manifest_record`; copy-only shadow registries
   are non-conforming.
3. When the canonical manifest's `design_token_schema_version`
   advances, downstream surfaces re-mint their claims against the new
   manifest. A claim that pins a lagging version after the advance
   window MUST emit a `denied` event citing
   `design_token_schema_version_lagging`.
4. Multiple records MAY cite the same `token_export_manifest_ref`;
   the manifest itself does not change once minted.

## 6. Audit-event vocabulary

Every record family carries a sibling audit-event record:

| Audit-event id | Meaning |
|---|---|
| `theme_support_row_claim_emitted` | A claim was emitted. |
| `theme_support_row_claim_refused` | A claim was refused (color-alone violation, density semantics violation, missing evidence, schema lag). |
| `theme_support_row_claim_downgraded` | A previously `supported` claim downgraded to `partial`, `unsupported`, or `denied`. |
| `theme_support_row_claim_evidence_rotated` | The claim's evidence refs were rotated (new screenshot diff, new keyboard journey, new assistive-tech check). |
| `theme_import_mapping_report_emitted` | A report was emitted. |
| `theme_import_mapping_report_refused` | A report was refused (unknown token dropped, missing rollback path, missing checkpoint, blocked honesty check unresolved). |
| `theme_import_mapping_report_downgraded` | The parity claim was downgraded to `partial_claim_with_gaps` or `denied_unresolved_or_blocked`. |
| `theme_import_mapping_report_rolled_back` | The report's rollback path was executed. |
| `extension_ui_appearance_descriptor_emitted` | A descriptor was emitted. |
| `extension_ui_appearance_descriptor_refused` | A descriptor was refused (missing boundary card, color-alone violation, unresolved inheritance row). |
| `extension_ui_appearance_descriptor_gap_disclosed` | A previously unrecorded gap was added to a descriptor row. |
| `extension_ui_appearance_descriptor_downgraded` | A `claims_host_parity` descriptor downgraded. |

Closed denial-reason vocabulary (shared across the three audit
streams): `color_alone_conveyed_required_meaning`,
`density_changed_information_architecture`,
`density_changed_focus_visibility`,
`reduced_motion_stripped_state_conveyance`,
`reduced_motion_hid_focus_visibility`,
`forced_colors_unsafe_substitution`,
`protected_cue_color_only`, `unknown_token_dropped`,
`unresolved_mapping_hidden`, `rollback_path_missing`,
`appearance_checkpoint_missing`, `embedded_boundary_card_missing`,
`extension_inheritance_gap_hidden`,
`design_token_schema_version_lagging`,
`token_export_manifest_unresolved`,
`theme_package_manifest_unresolved`,
`shadow_registry_detected`,
`evidence_ref_missing`.

Audit events MUST NOT carry raw token bytes, raw screenshots, raw
URLs, raw asset payloads, or raw user content. Cross-record references
travel by opaque id only.

## 7. Acceptance summary

A surface satisfies this contract iff:

1. Every theme/contrast support claim resolves the four per-theme-
   class support columns, the density-aware behavior, the reduced-
   motion behavior, the forced-colors behavior, and the protected-cue
   preservation rows; cites one theme-package manifest ref, one
   theme-package revision ref, one canonical token-export manifest
   ref, and one import/export portability ref; and supplies evidence
   refs for `supported` claims and gap refs for `partial`,
   `unsupported`, or `denied` claims.
2. Every imported translated theme emits an audited mapping report
   that resolves source tool/version, target theme classes, per-slot
   mapping rows, syntax coverage, fallback token classes, the
   protected-cue honesty check matrix, the appearance checkpoint, and
   the rollback path; cites the canonical token-export manifest by id;
   and resolves `parity_claim_state` consistently with the
   unresolved/blocked counts.
3. Every extension or embedded surface emits an audited descriptor
   that publishes five inheritance rows, at least one audited support
   row reference, the protected-cue preservation matrix, and (for
   webview-backed surfaces) the embedded boundary card ref; and cites
   the canonical token-export manifest by id.
4. Audit events are emitted on every emission, refusal, downgrade,
   evidence rotation, gap disclosure, or rollback, with denial events
   citing one of the closed denial reasons. No event carries raw
   token bytes, raw screenshots, or raw user content.
