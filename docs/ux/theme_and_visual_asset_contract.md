# Theme-package, icon-slot, illustration-set, and motion-preset contract

This document freezes the cross-surface contract every Aureline
**theme package**, **icon slot**, **illustration slot**, and
**motion preset** resolves through before a component contract,
extension, or imported theme can widen the design-system surface.
The goal is a visual-asset surface that stays a **typed, addressable
overlay over stable design-token vocabulary** — stable theme-package
ids, stable slot ids, stable preset ids, stable token tokens — instead
of a screenshot-only handoff that breaks safety-critical state
conveyance, RTL behaviour, accessibility postures, or extension
parity the moment an asset pack ships.

The machine-readable schemas live at:

- [`/schemas/ux/theme_package_manifest.schema.json`](../../schemas/ux/theme_package_manifest.schema.json)
- [`/schemas/ux/icon_slot_map.schema.json`](../../schemas/ux/icon_slot_map.schema.json)
- [`/schemas/ux/motion_preset.schema.json`](../../schemas/ux/motion_preset.schema.json)

The companion fixtures live under:

- [`/fixtures/ux/theme_asset_cases/`](../../fixtures/ux/theme_asset_cases/)

This contract is normative for the projection, fallback, signature,
mirrorability, override, directionality, safety-critical metaphor
continuity, and reduced-motion posture of all theme packages, icon /
illustration slots, and motion presets. Where it disagrees with the
PRD, TAD, TDD, UI/UX spec, design-system style guide, or milestone
document, those sources win and this document plus its companion
schemas and fixtures update in the same change. Where a downstream
surface mints a parallel theme-package, slot-map, or motion-preset
vocabulary, this contract wins and the surface is non-conforming.

## Companion contracts this contract rides on

This contract does **not** re-mint vocabulary already frozen
upstream; it consumes it by reference:

- [`/docs/design/design_token_component_state_vocabulary.md`](../design/design_token_component_state_vocabulary.md)
  and
  [`/schemas/design/token_export_manifest.schema.json`](../../schemas/design/token_export_manifest.schema.json)
  — token-family, component-state, theme, accessibility-posture,
  layer / portal-order, scrim, density, icon-treatment,
  semantic-status, and trust-visual-state vocabularies. Every
  theme-package manifest, icon-slot map, and motion-preset record
  consumes those vocabularies by reference.
- [`/artifacts/design/theme_support_rows.yaml`](../../artifacts/design/theme_support_rows.yaml)
  — frozen `theme_support_row_record`s and
  `accessibility_posture_record`s. Theme packages declare which rows
  they support; motion presets declare which postures they cover.
- [`/docs/ux/component_contract_template.md`](./component_contract_template.md)
  and
  [`/schemas/design/component_contract.schema.json`](../../schemas/design/component_contract.schema.json)
  — the reusable component-contract packet whose theme-package hooks,
  visual-asset hooks, and motion bindings cite the records frozen
  here by ref instead of re-minting visual timing or fallback
  semantics locally.
- [`/docs/ux/appearance_import_and_checkpoint_contract.md`](./appearance_import_and_checkpoint_contract.md),
  [`/schemas/ux/appearance_checkpoint.schema.json`](../../schemas/ux/appearance_checkpoint.schema.json),
  and
  [`/schemas/ux/theme_import_report.schema.json`](../../schemas/ux/theme_import_report.schema.json)
  — appearance-session, single-checkpoint preview / rollback,
  imported-theme report, token-overlay report, and extension /
  embedded-surface inheritance records. Theme packages point at those
  records when they are previewed, imported, or claimed by extension
  surfaces.
- [`/docs/ux/localization_and_locale_pack_contract.md`](./localization_and_locale_pack_contract.md)
  — `bcp47_locale_tag` and locale-fallback vocabulary. Icon
  directionality (`mirror_in_rtl`) cooperates with the locale-pack
  contract; the locale contract owns the locale model and this
  contract owns the visual-asset model that rides on top.
- [`/artifacts/governance/deployment_profiles.yaml`](../../artifacts/governance/deployment_profiles.yaml)
  — `deployment_profile_id` vocabulary. Theme-package admissibility
  (built-in vs. extension vs. community vs. imported vs. air-gapped)
  resolves against deployment profiles mechanically.
- [`/schemas/governance/capability_lifecycle.schema.json`](../../schemas/governance/capability_lifecycle.schema.json)
  — `redaction_class` and `policy_context` shape for surface records.
- [`/schemas/docs/citation_anchor.schema.json`](../../schemas/docs/citation_anchor.schema.json)
  — `version_match_state`. Theme-package compatibility re-uses the
  citation contract's compatibility vocabulary verbatim.

## Who reads this contract

- **Component-contract authors** — to point at theme-package,
  icon-slot, illustration-slot, and motion-preset rows by ref instead
  of redefining their own visual timing or fallback semantics.
- **Theme-package authors** (first-party, extension, community,
  imported, air-gapped) — to declare version, supported token-schema
  range, density / contrast / motion coverage, deprecated-token
  visibility, signature posture, and safety-critical state
  preservation against one inspectable record shape.
- **Icon and illustration owners** — to publish slot ids whose
  metaphor, directionality, platform variants, safety-critical
  classification, and migration / supersession history are typed
  rather than tribal knowledge.
- **Motion-preset owners** — to publish named duration / easing
  bindings, semantic-transition classes, and per-posture reduced-motion
  fallbacks that preserve component-state semantics mechanically.
- **Extension authors** — to consume theme rows, slot rows, and
  motion presets without overriding host-owned safety-critical
  metaphors or hue-only-substituting suppressed motion.
- **Trust, accessibility, and screenshot-diff reviewers** — to
  inspect whether a package or asset pack uses deprecated, unmapped,
  or substituted tokens; whether RTL or platform variants cover the
  required surface; and whether reduced-motion fallbacks preserve
  state machines instead of collapsing them.

## Why this exists

Without this contract, theme / icon / illustration / motion drifts
the fastest:

- a theme package silently substitutes `restricted_workspace` into a
  hue-only difference and a reviewer cannot tell that the chip lost
  its border / icon cue;
- a community theme pack ships with a deprecated semantic token and
  the theme/debug surface renders "themed successfully" while
  important state copy quietly disappears;
- an icon pack swaps a safety-critical metaphor (e.g. the policy-lock
  glyph) for a near-miss alternative and the user no longer
  recognises the boundary;
- a directional chevron is shipped without an RTL mirror and the
  whole tour reads backwards in Hebrew or Arabic;
- a platform-variant icon is missing on Linux and the shell renders a
  blank slot instead of a declared coverage gap;
- an extension theme overrides the first-party high-contrast palette
  and the high-contrast posture stops meeting its 7:1 target;
- a motion preset has no reduced-motion fallback and a user with
  vestibular sensitivity cannot tell that a state changed;
- a "tour advance" preset substitutes its translation for a pure hue
  shift under reduced motion and the state machine collapses into one
  generic "something happened" posture;
- a critical-hot-path frame budget is committed and a panel-resize
  preset still animates, so key-to-paint regresses;
- an imported theme silently round-trips an unsupported token as if
  it had been mapped, and downstream tooling that keyed off the token
  silently breaks.

This contract closes those gaps by declaring one record per theme
package, one record per icon / illustration slot, one record per
motion preset, one closed denial-reason set the publisher MUST emit
when any of the above fails, and one explicit binding to the
design-token, component-state, and accessibility-posture truth model
so visual assets stay a **typed overlay over stable identity** rather
than an unwitnessed redefinition.

## 1. Record kinds

### 1.1 `theme_package_manifest_record`

One structured record per **(package_id, package_revision)**. Emitted
by the package owner (first-party theme pipeline, extension overlay
pipeline, community-pack ingest, imported-theme translator, or
air-gapped offline pack ingest). Every record carries:

- **identity** — `package_id`, `package_revision_ref`,
  `package_version_label`.
- **distribution** — `distribution_class` in `{built_in_with_product,
  extension_contributed, community_supplied, imported_translated,
  air_gapped_offline}`.
- **trust** — `signature_state` in `{signed_verified, signed_unverified,
  unsigned_explicit_acceptance, signature_failed_blocked,
  not_applicable_built_in}`, `mirrorability_class`,
  `permitted_deployment_profiles`.
- **coverage** — `supported_theme_classes`,
  `supported_density_classes`, `supported_accessibility_postures`.
- **schema range** — `supported_token_schema_range` declares the
  inclusive `[min, max]` `design_token_schema_version` the package
  supports; outside the range, the package resolves as
  `incompatible_drift_detected`.
- **build compatibility** — `compatibility_class` (re-exported from
  the citation contract's `version_match_state`),
  `compatibility_build_range`.
- **deprecation visibility** — `deprecated_token_summary`
  (`deprecated_token_count`, `substituted_token_count`,
  `unmapped_token_count`, `blocked_token_count`) plus the per-token
  `deprecated_token_disclosures[]`.
- **safety-critical preservation** —
  `safety_critical_visual_state_preservation[]` declaring which trust
  visual states the package MUST preserve (with required non-color
  cues, high-contrast preservation, and forced-colors preservation).
- **appearance change posture** — `appearance_change_posture` in
  `{live_apply, surface_reload_required, full_restart_required,
  not_applicable}`. Apply / revert MUST be atomic; half-updated
  appearance is forbidden.
- **import provenance** — `import_mapping_report_ref`, required for
  `imported_translated` distribution.
- **extension namespace** — `extension_namespace_refs[]`, required
  (minItems 1) for `extension_contributed` distribution and forbidden
  otherwise.
- **policy / redaction** — `policy_context`, `redaction_class`.
- **timestamps** — `minted_at`.

### 1.2 `icon_slot_map_record`

One structured record per **(slot_id, slot_revision)**. Emitted by
the icon / illustration registry owner. Names the slot kind
(`command_canonical`, `status_semantic`, `trust_state`, `file_type`,
`directional_chevron`, `directional_arrow`, `illustration_anchor`,
`illustration_empty_state`, `illustration_onboarding`), the default
icon-treatment class, the directionality class, the safety-critical
classification, the platform-variant coverage matrix, the extension
override boundary, the theme override boundary, and the migration
record (frozen, migrating-with-alias, superseded-with-redirect, or
withdrawn-with-replacement). Trust-state and command-canonical slots
carry stable refs to the trust visual state class and command id
respectively; status-semantic slots carry the semantic status class
they pair with.

### 1.3 `motion_preset_record`

One structured record per **(preset_id, preset_revision)**. Emitted
by the motion-preset registry owner. Names the preset kind
(`transition`, `entrance`, `exit`, `state_change`,
`progress_indicator`, `guided_overlay_step`, `focus_advance`), the
semantic transition class (e.g. `state_idle_to_active`,
`tour_advance`, `attention_redirect`), the named
`default_duration_tokens[]` and `default_easing_tokens[]` consumed
under `motion_standard`, the per-posture
`reduced_motion_fallbacks[]` (each pinning a substitution class plus
the non-motion state markers that remain visible), the
`component_state_compatibility[]` rows that pin which state machine
the preset is wired to, the `non_motion_state_markers[]` that the
slot always carries, and the override boundaries for extension and
theme contributors.

## 2. Why three records

The records ride together because they answer different but related
questions:

1. **The theme-package manifest** answers: *given a package, which
   tokens does it serve, which themes / densities / postures does it
   cover, which tokens did it deprecate or substitute, and how
   visible is that substitution to the user and the reviewer?*
2. **The icon-slot map** answers: *given a slot id a component
   contract or theme package points at, what is the canonical
   metaphor, how does it mirror under RTL, which platform variants
   ship, who can override it, and has its meaning migrated?*
3. **The motion preset** answers: *given a named animation,
   transition, or state change, what does it look like under the
   user's chosen accessibility posture, what non-motion cue keeps the
   state machine readable, and which extension / theme contributors
   may replace it?*

Component contracts cite the records frozen here so a downstream
packet can inherit visual behaviour mechanically — by pointing at one
named theme-support row, one named slot id, and one named motion
preset — instead of redefining its own visual timing or fallback
semantics in prose.

## 3. Schema-version lock-step

`theme_asset_schema_version` is currently **1**. The same integer
gates all three schemas; bumping it requires updating each schema
together. Adding a new vocabulary value (a new
`theme_package_distribution_class`, `icon_slot_kind_class`,
`semantic_transition_class`, etc.) is **additive-minor** and bumps
the version. Repurposing an existing value is **breaking** and
requires a new decision row on the launch decision register.

## 4. Theme-package distribution, signature, and mirrorability

`theme_package_distribution_class` is closed at five values:

| Value                      | Intent                                                                                                                                                                            |
|----------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `built_in_with_product`    | Ships with the product. `signature_state = not_applicable_built_in`. The four first-party theme classes (`dark_reference`, `light_parity`, `high_contrast_dark`, `high_contrast_light`) MUST resolve here. |
| `extension_contributed`    | Contributed by an extension. Carries one or more `extension_namespace_refs`; MUST NOT redefine first-party high-contrast tokens without explicit acceptance.                       |
| `community_supplied`       | Community pack. Resolves through the community-pack signing root once that lane lands; until then carries `unsigned_explicit_acceptance` plus a decision row.                      |
| `imported_translated`      | Translated from another ecosystem (VS Code theme, JetBrains palette, etc.). MUST cite an `import_mapping_report_ref` so unresolved slots stay visible.                              |
| `air_gapped_offline`       | Offline pack ingested through a fleet / air-gapped lane. Mirrorability narrows to `{mirror_allowed, mirror_with_attribution_required, air_gapped_only}`.                            |

`theme_package_signature_state` is closed at five values
(`signed_verified`, `signed_unverified`, `unsigned_explicit_acceptance`,
`signature_failed_blocked`, `not_applicable_built_in`).
`theme_package_mirrorability_class` is closed at five values
(`mirror_allowed`, `mirror_with_attribution_required`,
`mirror_forbidden`, `air_gapped_only`, `not_mirrorable_signed_blob`).

Rules (frozen):

1. **Built-in packs do not need signatures.** A package whose
   `distribution_class = built_in_with_product` MUST carry
   `signature_state = not_applicable_built_in`; any other state is
   non-conforming.
2. **Non-built-in packs MUST NOT claim built-in signature state.**
   Any package whose `distribution_class` is not
   `built_in_with_product` MUST carry one of the four other
   signature states.
3. **Air-gapped packs restrict mirrorability.** A pack whose
   `distribution_class = air_gapped_offline` MUST carry
   `mirrorability_class ∈ {mirror_allowed, mirror_with_attribution_required, air_gapped_only}`.
4. **Extension packs declare a namespace.** A pack whose
   `distribution_class = extension_contributed` MUST carry at least
   one `extension_namespace_refs` entry.
5. **Imported packs declare a mapping report.** A pack whose
   `distribution_class = imported_translated` MUST carry a non-null
   `import_mapping_report_ref` so unresolved slots stay visible
   (`denial_reason = theme_imported_pack_lacks_mapping_report`).
6. **Signature failure suppresses.** A pack whose
   `signature_state = signature_failed_blocked` MUST NOT render
   (`denial_reason = theme_signature_failed`).
7. **Compatibility drift suppresses.** A pack whose
   `compatibility_class = incompatible_drift_detected` MUST NOT
   render without an explicit accept-and-acknowledge route
   (`denial_reason = theme_compatibility_drift`).

## 5. Token coverage, deprecation, and substitution visibility

A theme-package manifest declares deprecation pressure as **inspectable
counts plus enumerated rows**, not as a single "themed successfully"
banner.

`theme_token_overlay_state` is closed at five values:

- `mapped_native` — the package serves the token natively. Most rows.
- `substituted_with_fallback` — the package serves the token through
  a documented fallback. The substitution MUST remain visible in the
  theme/debug surface (`visibility_required = true` on the disclosure
  row).
- `deprecated_with_replacement` — the package consumes a deprecated
  token and the replacement ref is named.
- `unmapped_inert` — the package leaves the token unmapped and the
  surface renders inert (no value) rather than redefining meaning.
- `unsupported_blocked` — the package's intended substitution is
  unsupported; the row will deny rather than render.

Rules (frozen):

1. **Counts and rows reconcile.** A non-zero
   `deprecated_token_summary.<count>` MUST have at least one matching
   row in `deprecated_token_disclosures[]`; a count without rows is
   non-conforming
   (`denial_reason = theme_deprecated_token_not_marked`).
2. **Substitution stays visible.** Every disclosure row MUST set
   `visibility_required = true` and MUST list at least one
   `visibility_surface_refs` entry (the theme/debug pane row, the
   support packet row, or the design-evidence packet row). Surfaces
   that bury substitution behind generic prose are non-conforming
   (`denial_reason = theme_substitution_not_disclosed`).
3. **Replacements survive round-trip.** A `deprecated_with_replacement`
   row MUST carry a non-null `replacement_token_ref`; an
   `unmapped_inert` or `unsupported_blocked` row MAY leave it null.

## 6. Density, contrast, posture coverage, and appearance-change posture

A theme-package manifest declares its design-system surface coverage
explicitly:

- `supported_theme_classes` MUST include at least one
  first-party theme class. First-party packages MUST cover all four
  (`dark_reference`, `light_parity`, `high_contrast_dark`,
  `high_contrast_light`); extension and community packages MAY narrow
  to a subset, with each missing theme a declared coverage gap rather
  than a silent fallback.
- `supported_density_classes` MUST include at least one density
  class; density may not change information architecture or focus
  visibility (`denial_reason = theme_density_changed_information_architecture`).
- `supported_accessibility_postures` MUST include `motion_standard`
  and `motion_reduced` at minimum. First-party packages cover the
  full set; the schema enforces the two-class minimum.
- `appearance_change_posture` is closed at four values
  (`live_apply`, `surface_reload_required`, `full_restart_required`,
  `not_applicable`). Apply and revert MUST be atomic; a half-updated
  appearance state is non-conforming
  (`denial_reason = theme_appearance_change_left_partial_state`).

`safety_critical_visual_state_preservation[]` MUST cover at least
`restricted_workspace` and `policy_locked`. Each row pins the required
non-color cues (`shape`, `border`, `icon`, `text`,
`iconography_metaphor`, `label_chip`), preservation under
high-contrast modes, and preservation under forced-colors. A package
that maps a safety-critical state to a hue-only difference is
non-conforming
(`denial_reason = theme_redefines_safety_critical_meaning`).

## 7. Icon-slot and illustration-slot rules

`icon_slot_kind_class` is closed at nine values:

| Value                          | Intent                                                                                                                                  |
|--------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------|
| `command_canonical`            | The canonical metaphor for one command. MUST carry a non-null `command_id_ref`. One metaphor per command (no parallel icons).          |
| `status_semantic`              | A status icon paired with one `semantic_status_class`. MUST carry `semantic_status_class_ref`.                                         |
| `trust_state`                  | A trust / boundary icon paired with one `trust_visual_state_class`. MUST carry `trust_visual_state_class_ref`. Host-owned only.        |
| `file_type`                    | A file-type metaphor. MAY carry richer treatment (`file_type_rich`) but MUST NOT impersonate shell chrome.                              |
| `directional_chevron`          | A directional chevron. MUST carry `directionality_class = mirror_in_rtl`.                                                              |
| `directional_arrow`            | A directional arrow. MUST carry `directionality_class = mirror_in_rtl`.                                                                |
| `illustration_anchor`          | An illustration used as a slot anchor (e.g. for tour cards or onboarding cards). Reserved for non-safety-critical surfaces.            |
| `illustration_empty_state`     | An empty-state illustration. Reserved for non-safety-critical surfaces.                                                                 |
| `illustration_onboarding`      | An onboarding illustration. Reserved for non-safety-critical surfaces.                                                                  |

`icon_directionality_class`: `mirror_in_rtl` (chevrons, arrows,
read-direction glyphs); `literal_no_mirror` (canonical command
metaphors, file-type metaphors, brand marks); `not_applicable`
(status / trust slots and most illustrations).

`safety_critical_class`: `safety_critical_metaphor_continuity_required`
(trust states, recovery banners, policy lock, restricted-workspace
glyphs) or `non_safety_critical`. Safety-critical slots MUST cite a
`migration_decision_row_ref` whenever their `migration_state_class`
moves out of `frozen`
(`denial_reason = icon_safety_critical_metaphor_drifted`).

`platform_variant_class` is closed at six values
(`desktop_macos`, `desktop_windows`, `desktop_linux_gtk`,
`desktop_linux_kde`, `web_browser`, `not_applicable_universal`).
Slots that ship one universal glyph emit exactly one row with
`not_applicable_universal`; slots with per-platform variants emit one
row per platform plus an optional universal fallback. Uncovered
variants are declared coverage gaps rather than silent fallbacks
(`denial_reason = icon_platform_variant_missing`).

`extension_override_boundary_class` is closed at four values:

| Value                                             | Intent                                                                                                          |
|---------------------------------------------------|-----------------------------------------------------------------------------------------------------------------|
| `host_owned_no_override`                          | The host owns the metaphor; extensions MUST NOT replace it.                                                      |
| `extension_may_replace_with_disclosure`           | Extensions MAY replace the metaphor only with a declared migration record.                                       |
| `extension_may_extend_only`                       | Extensions MAY add adjacent variants without replacing the canonical metaphor.                                   |
| `not_extension_consumable`                        | The slot is reserved entirely (e.g. trust-state slots backing security-critical chrome).                         |

`theme_override_boundary_class` is closed at three values
(`theme_may_recolor_only`, `theme_may_replace_glyph_with_disclosure`,
`theme_locked_to_first_party_glyph`). Trust-state slots MUST be
either `theme_may_recolor_only` or `theme_locked_to_first_party_glyph`.

`migration_state_class` is closed at four values (`frozen`,
`migrating_with_alias`, `superseded_with_redirect`,
`withdrawn_with_replacement`). The migration row carries
predecessor / successor slot refs and (for safety-critical slots) a
`migration_decision_row_ref`.

Rules (frozen):

1. **One metaphor per command.** A `command_canonical` slot MUST
   resolve to exactly one canonical metaphor. Parallel metaphors for
   the same canonical command are non-conforming
   (`denial_reason = icon_parallel_metaphor_for_canonical_command`).
2. **Action icons label.** Slots whose `accessible_text_required` is
   true MUST be paired by the consumer with a label or tooltip; a
   consumer that drops the label is non-conforming
   (`denial_reason = icon_action_slot_lacks_label_or_tooltip`).
3. **Trust slots are host-owned.** A slot whose
   `slot_kind_class = trust_state` MUST resolve to
   `extension_override_boundary_class ∈ {host_owned_no_override, not_extension_consumable}`
   and `theme_override_boundary_class ∈ {theme_may_recolor_only, theme_locked_to_first_party_glyph}`.
4. **Directional slots mirror.** `directional_chevron` and
   `directional_arrow` slots MUST set `directionality_class = mirror_in_rtl`.
5. **Illustrations stay outside safety-critical flows.**
   `illustration_*` slots MUST set
   `safety_critical_class = non_safety_critical`
   (`denial_reason = icon_illustration_in_safety_critical_flow`).

## 8. Motion-preset rules

`motion_preset_kind_class` is closed at seven values (`transition`,
`entrance`, `exit`, `state_change`, `progress_indicator`,
`guided_overlay_step`, `focus_advance`).

`semantic_transition_class` is closed at twelve values:
`state_idle_to_active`, `state_active_to_dismissed`, `surface_reveal`,
`surface_dismiss`, `value_update`, `progress_pulse`,
`attention_redirect`, `panel_resize`, `tour_advance`, `tour_retreat`,
`focus_ring_advance`, `focus_ring_suppress`. Component contracts cite
this class so downstream packets inherit visual behaviour
mechanically.

`reduced_motion_substitution_class` is closed at five values:

| Value                              | Intent                                                                                                                                            |
|------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------|
| `collapse_to_instant`              | Resolves to `motion.instant` under the listed posture.                                                                                            |
| `crossfade_only`                   | Keeps a short opacity crossfade with no translation / scale.                                                                                      |
| `essential_keep_simplified`        | Keeps the motion in a simplified form when state conveyance would otherwise be lost.                                                              |
| `suppress_entirely`                | Removes the motion (the slot may still mark state through non-motion cues).                                                                       |
| `non_motion_state_marker`          | Promotes the equivalent semantic-state marker into the slot when motion is suppressed (e.g. a chip, label, or icon takes over the state cue).     |

Named token vocabulary is closed:

- `motion_duration_token` ∈ `{motion.instant, motion.fast, motion.ui,
  motion.panel, motion.dialog}`.
- `motion_easing_token` ∈ `{ease.standard, ease.exit, ease.enter}`.

A duration or easing outside the closed set is non-conforming
(`denial_reason ∈ {motion_preset_duration_outside_token_set,
motion_preset_easing_outside_token_set}`).

Rules (frozen):

1. **Every preset declares a reduced-motion fallback.** A preset
   without a `motion_reduced` fallback row is non-conforming
   (`denial_reason = motion_preset_no_reduced_motion_fallback`).
2. **Critical hot path is always covered.** Every preset MUST also
   carry a `motion_critical_hot_path` fallback. Engine-internal
   posture engagement MUST suppress transient motion regardless of
   the user's chosen posture
   (`denial_reason = motion_preset_critical_hot_path_downgrade_refused`).
3. **No hue-only substitution.** When motion is suppressed, the
   substitution MUST NOT collapse into a hue-only change. Every
   preset declares `non_motion_state_markers[]` (shape, border, icon,
   text, label_chip, iconography_metaphor); a fallback that renders
   only a colour shift is non-conforming
   (`denial_reason = motion_preset_uses_hue_only_substitution`).
4. **State machines do not collapse.** A `state_change` preset MUST
   cite at least one `component_state_compatibility[]` row whose
   transition remains observable under `motion_reduced`. A reduced
   posture that collapses two distinct states (e.g. `degraded` and
   `policy_blocked`) into one generic posture is non-conforming
   (`denial_reason = motion_preset_collapses_state_machine`).
5. **Loops are reserved for progress indicators.** `progress_loop_permitted = true`
   is admissible only for `preset_kind_class = progress_indicator`.
   Any other kind that loops is non-conforming
   (`denial_reason = motion_preset_progress_loop_outside_permitted_kinds`).
6. **No layout shift during typing.** Presets whose semantic
   transition would shift layout under typing are non-conforming
   (`denial_reason = motion_preset_layout_shift_during_typing`).
7. **Safety-critical presets are host-owned.** `is_safety_critical = true`
   forces `extension_override_boundary_class ∈ {host_owned_no_override, not_extension_consumable}`
   and `theme_override_boundary_class ∈ {theme_may_recolor_only, theme_locked_to_first_party_motion}`.

## 9. Invariants

Every set of `theme_package_manifest_record`,
`icon_slot_map_record`, and `motion_preset_record` rows MUST be
reconcilable against the following const-true invariants:

1. `every_theme_package_declares_supported_theme_classes` — no
   manifest claims compatibility without naming at least one
   first-party theme class.
2. `safety_critical_visual_state_preservation_covers_restricted_and_policy_locked`
   — every theme-package manifest covers `restricted_workspace` and
   `policy_locked` in its preservation list.
3. `deprecated_token_counts_reconcile_with_disclosure_rows` — counts
   and per-row disclosures match.
4. `theme_substitution_visible_to_reviewer` — every disclosure row
   sets `visibility_required = true` and lists at least one
   visibility surface ref.
5. `safety_critical_icon_metaphor_continuity_preserved` — every
   safety-critical slot whose migration state moves out of `frozen`
   cites a migration decision row.
6. `directional_icons_mirror_in_rtl` — every `directional_chevron` /
   `directional_arrow` slot pins `mirror_in_rtl`.
7. `platform_variant_coverage_complete_or_declared_gap` — every slot
   either ships per-platform coverage or declares the universal
   variant; missing variants are explicit gaps.
8. `trust_state_slots_are_host_owned` — every `trust_state` slot is
   `host_owned_no_override` or `not_extension_consumable` and either
   `theme_may_recolor_only` or `theme_locked_to_first_party_glyph`.
9. `motion_preset_has_reduced_motion_and_critical_hot_path_fallbacks`
   — every preset carries fallbacks for both
   `motion_reduced` and `motion_critical_hot_path`.
10. `motion_preset_state_machine_preserved_under_reduced_motion` —
    every `state_change` preset's
    `component_state_compatibility[]` includes `motion_reduced`.
11. `motion_preset_no_hue_only_substitution` — every preset declares
    at least one non-motion state marker; hue-only substitutions are
    forbidden.
12. `signed_or_explicit_acceptance_required_before_theme_renders` —
    no theme package with `signature_failed_blocked` renders; a
    `signed_unverified` or `unsigned_explicit_acceptance` package
    renders only after the deployment-profile-permitted acceptance
    route runs.
13. `imported_theme_carries_mapping_report` — every
    `imported_translated` package cites an
    `import_mapping_report_ref`.

## 10. Denial reasons

The following denial reasons are reserved. A surface that would
otherwise emit non-conforming behaviour MUST deny with the matching
reason rather than silently fall back:

Theme package:

- `theme_signature_failed`
- `theme_compatibility_drift`
- `theme_redefines_safety_critical_meaning`
- `theme_substitution_not_disclosed`
- `theme_deprecated_token_not_marked`
- `theme_extension_overrides_first_party_high_contrast`
- `theme_color_alone_state_violation`
- `theme_density_changed_information_architecture`
- `theme_imported_pack_lacks_mapping_report`
- `theme_appearance_change_left_partial_state`
- `theme_asset_schema_version_lagging`

Icon / illustration slot:

- `icon_safety_critical_metaphor_drifted`
- `icon_directionality_unresolved`
- `icon_platform_variant_missing`
- `icon_extension_override_violates_boundary`
- `icon_theme_override_violates_boundary`
- `icon_replacement_lacks_migration_metadata`
- `icon_status_semantic_class_unresolved`
- `icon_trust_state_class_unresolved`
- `icon_action_slot_lacks_label_or_tooltip`
- `icon_parallel_metaphor_for_canonical_command`
- `icon_illustration_in_safety_critical_flow`
- `theme_asset_schema_version_lagging`

Motion preset:

- `motion_preset_no_reduced_motion_fallback`
- `motion_preset_collapses_state_machine`
- `motion_preset_uses_hue_only_substitution`
- `motion_preset_duration_outside_token_set`
- `motion_preset_easing_outside_token_set`
- `motion_preset_extension_replaces_safety_critical`
- `motion_preset_critical_hot_path_downgrade_refused`
- `motion_preset_progress_loop_outside_permitted_kinds`
- `motion_preset_layout_shift_during_typing`
- `motion_preset_state_marker_color_alone`
- `theme_asset_schema_version_lagging`

## 11. Acceptance mapping

| Acceptance clause                                                                                                                                                     | Resolved by                                                                                                                                                                                  |
|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Reviewers can inspect whether a theme or asset pack is using deprecated, unmapped, or substituted tokens rather than inferring it from visual glitches.                | §5 deprecation visibility, §1.1 manifest fields, invariants 3 / 4, denial reasons `theme_substitution_not_disclosed`, `theme_deprecated_token_not_marked`.                                   |
| Safety-critical icons or motion cues cannot change semantics without explicit migration metadata.                                                                     | §6 safety-critical preservation, §7 safety-critical class and migration record, §8 safety-critical preset rule, invariants 2 / 5 / 11 / 12, denial reasons `icon_safety_critical_metaphor_drifted`, `theme_redefines_safety_critical_meaning`, `motion_preset_extension_replaces_safety_critical`. |
| Fixtures cover at least: deprecated-token theme fallback, high-contrast theme row, RTL-aware icon swap, platform-variant icon set, and reduced-motion preset with equivalent semantic states. | `/fixtures/ux/theme_asset_cases/` (§13).                                                                                                                                                     |
| Theme and motion artifacts are precise enough that a downstream component contract can reference named token rows instead of redefining its own visual timing or fallback semantics. | §1, §3 schema-version lock-step, §8 named duration / easing tokens and semantic-transition class, component-contract template's `theme_package_hooks`, `visual_asset_hooks`, and `motion_bindings`. |

## 12. Adding a new vocabulary value

Adding a new `theme_package_distribution_class`,
`theme_package_signature_state`, `theme_package_mirrorability_class`,
`theme_token_overlay_state`, `appearance_change_class`,
`icon_slot_kind_class`, `icon_directionality_class`,
`safety_critical_class`, `platform_variant_class`,
`extension_override_boundary_class`, `theme_override_boundary_class`
(icon or motion variant), `migration_state_class`,
`motion_preset_kind_class`, `semantic_transition_class`,
`reduced_motion_substitution_class`, `motion_duration_token`,
`motion_easing_token`, `non_motion_state_marker_class`, or
`denial_reason` is **additive-minor** and bumps
`theme_asset_schema_version`. Repurposing an existing value is
**breaking** and requires a new decision row on the launch decision
register. A consumer surface that resolves a value it does not
recognize MUST deny with `theme_asset_schema_version_lagging` rather
than silently map to a default.

## 13. Worked examples

Fixtures under
[`/fixtures/ux/theme_asset_cases/`](../../fixtures/ux/theme_asset_cases/)
cover the acceptance set:

1. **Deprecated-token theme fallback** —
   `theme_package_with_deprecated_token_fallback.yaml`. A
   `theme_package_manifest_record` for a community theme that
   consumes one deprecated token, serves it through
   `substituted_with_fallback`, and pins the disclosure row to the
   theme/debug surface.
2. **High-contrast theme row** —
   `theme_package_high_contrast_dark_first_party.yaml`. A first-party
   `high_contrast_dark` package signed by the build, covering the
   full posture set, the four themes, and the safety-critical
   preservation list with no deprecated tokens.
3. **RTL-aware icon swap** — `icon_slot_directional_chevron_rtl.yaml`.
   An `icon_slot_map_record` for a directional chevron that
   `mirror_in_rtl` and ships one universal asset.
4. **Platform-variant icon set** —
   `icon_slot_command_canonical_per_platform.yaml`. An
   `icon_slot_map_record` for a `command_canonical` slot whose three
   desktop platforms ship distinct native metaphors plus a universal
   fallback.
5. **Reduced-motion preset with equivalent semantic states** —
   `motion_preset_state_change_with_reduced_fallback.yaml`. A
   `motion_preset_record` for a `state_change` preset wired to the
   `idle → restricted` transition; under `motion_reduced` the
   translation collapses to `non_motion_state_marker` while the
   restricted chip and border still convey the state.
6. **Safety-critical icon migration** —
   `icon_slot_safety_critical_policy_lock_migration.yaml`. An
   `icon_slot_map_record` that pins the policy-lock metaphor as
   `safety_critical_metaphor_continuity_required`,
   `host_owned_no_override`, and `migrating_with_alias` with a
   migration decision row.
7. **Imported theme with mapping report** —
   `theme_package_imported_translated_with_mapping_report.yaml`. An
   `imported_translated` `theme_package_manifest_record` whose
   `import_mapping_report_ref` is non-null and whose unmapped tokens
   are accounted for in the deprecated-token summary.

## 14. Cross-references

- Token, component-state, theme, accessibility-posture, layer, and
  scrim vocabulary:
  [`/docs/design/design_token_component_state_vocabulary.md`](../design/design_token_component_state_vocabulary.md)
  and
  [`/schemas/design/token_export_manifest.schema.json`](../../schemas/design/token_export_manifest.schema.json).
- Frozen theme-support rows and accessibility postures:
  [`/artifacts/design/theme_support_rows.yaml`](../../artifacts/design/theme_support_rows.yaml).
- Component-contract packet and how it cites theme / icon /
  illustration / motion hooks by ref:
  [`/docs/ux/component_contract_template.md`](./component_contract_template.md).
- Appearance import, token-overlay, checkpoint, rollback, and
  extension inheritance records:
  [`/docs/ux/appearance_import_and_checkpoint_contract.md`](./appearance_import_and_checkpoint_contract.md).
- Locale model and BCP-47 vocabulary:
  [`/docs/ux/localization_and_locale_pack_contract.md`](./localization_and_locale_pack_contract.md).
- Deployment profiles:
  [`/artifacts/governance/deployment_profiles.yaml`](../../artifacts/governance/deployment_profiles.yaml).
- Citation contract's `version_match_state`:
  [`/schemas/docs/citation_anchor.schema.json`](../../schemas/docs/citation_anchor.schema.json).

## 15. Out of scope at this revision

- **Theme marketplace implementation.** The
  `community_supplied` distribution class is reserved; the
  pack-signing trust root, community submission UX, and dispute
  process are owned by the pack-distribution lane and land later.
- **Final asset production.** Concrete theme palettes, icon sprites,
  illustrations, and animation files land in later milestones; this
  contract reserves the typed slot, manifest, and preset shape they
  resolve through.
- **Live theme-package conformance runner.** The schemas are precise
  enough for a future runner to diff canonical vocabulary against
  implementation or fixtures; the runner itself is a later task.
- **Final per-token contrast tables.** The design-system style guide
  owns the human-facing per-token presentation; this contract names
  the typed posture.
- **Full extension theming SDK.** The extension-override boundary
  vocabulary is reserved here; the extension SDK's runtime APIs land
  on the extension lane.
