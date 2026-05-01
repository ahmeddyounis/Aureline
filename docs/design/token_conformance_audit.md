# Design-token conformance audit, drift fail gate, and source-of-truth export contract

This document is the **token-conformance audit format** the design
system, shell, docs / help, trust, onboarding, durable-attention,
embedded-surface, and notification lanes share so launch-critical
visual-system vocabulary cannot fork silently.

The vocabulary it audits is frozen in:

- [`/docs/design/design_token_component_state_vocabulary.md`](./design_token_component_state_vocabulary.md)
  — the closed token-family, component-state, theme, accessibility-
  posture, layer / portal-order, scrim, semantic-status, and trust-
  visual-state vocabularies.
- [`/schemas/design/token_export_manifest.schema.json`](../../schemas/design/token_export_manifest.schema.json)
  — the boundary schema for `design_token_export_manifest_record`,
  `token_family_record`, `component_state_record`,
  `theme_support_row_record`, `accessibility_posture_record`,
  `layer_order_record`, `scrim_token_record`, and
  `token_export_audit_event_record`.
- [`/artifacts/design/theme_support_rows.yaml`](../../artifacts/design/theme_support_rows.yaml)
  and
  [`/artifacts/design/layer_and_scrim_tokens.yaml`](../../artifacts/design/layer_and_scrim_tokens.yaml)
  — the four first-party theme rows, the five accessibility postures,
  the seven layer / portal tokens, and the four scrim tokens.

The fail-gate rules this audit emits are encoded in:

- [`/artifacts/design/token_drift_rules.yaml`](../../artifacts/design/token_drift_rules.yaml)
  — closed audit-class, drift-class, severity, allowed-inheritance-
  gap, and gate-state vocabularies plus the per-surface block / warn
  / pass policy.

Worked manifest, denial, and gap fixtures live under:

- [`/fixtures/design/token_export_cases/`](../../fixtures/design/token_export_cases/)
  — emitted-manifest, refused-manifest, allowed-inheritance-gap, and
  prohibited-first-party-fork rows that conform to the boundary
  schema and resolve to the rule ids in `token_drift_rules.yaml`.

If this document and the vocabulary doc disagree, the vocabulary
doc wins and this document MUST be updated in the same change. If
this document and `token_drift_rules.yaml` disagree, the YAML wins
for tooling and this document MUST be updated.

## Why this audit exists

A visual system is only as enforceable as its ability to detect
drift. Without a mechanically diffable export, parallel token
families, alias mints, hard-coded z-index values, and color-alone
state indicators creep into shell surfaces and never get caught
until a screenshot review notices a difference no reviewer can
name precisely.

The audit makes drift mechanically detectable:

1. Every consuming surface — shell, components, trust, onboarding,
   notification, embedded-surface, docs / help, durable-attention —
   resolves a `design_token_export_manifest_record` and reads the
   per-entity records by id. No surface re-derives token-family,
   component-state, theme, posture, layer, or scrim values locally.
2. The audit diffs canonical token names, state names, theme rows,
   posture rows, layer ordinals, and scrim alphas against
   implementation manifests and fixture rows. Names are bytes; the
   diff is exact and fail-closed.
3. Allowed extension and embedded-surface inheritance gaps are
   declared explicitly, by class. Every other gap on a launch-
   critical first-party surface is a token-drift violation that
   blocks merge and release.
4. Reviewer prose ("looks off") is replaced with `audit_event_id`
   and `denial_reason` values from the export schema. Downstream
   screenshot, accessibility, and appearance reviews cite canonical
   names instead of screenshots alone.

## Audit format (one row per audited surface)

The audit is a flat row set. Each row pins one consuming surface
against the canonical export and records what the surface
actually consumes. Reviewers, conformance tooling, and release-
evidence packets read the same row shape.

Required fields on every audit row:

| Field                         | Resolves to                                                                                                  |
|-------------------------------|--------------------------------------------------------------------------------------------------------------|
| `audit_row_id`                | Opaque, stable id for this row.                                                                              |
| `audited_surface_class`       | One of the closed `audited_surface_class` values below.                                                      |
| `surface_owner_role_class`    | `first_party_shell`, `first_party_component`, `first_party_docs_help`, `first_party_trust`, `first_party_onboarding`, `first_party_notification`, `first_party_durable_attention`, `extension_contributed_surface`, or `embedded_surface_contributed`. |
| `manifest_ref`                | `design_token_export_manifest_record.manifest_id` the surface resolved against.                              |
| `running_build_identity_ref`  | The same `running_build_identity_ref` the manifest was minted against (ADR-0013). A row whose surface ran against a different build identity is denied. |
| `consumed_token_family_refs`  | `family_id` values the surface consumes. MUST be a subset of `design_token_export_manifest_record.token_family_ids`. |
| `consumed_component_state_refs` | `state_id` values the surface renders.                                                                     |
| `consumed_theme_row_refs`     | `theme_row_id` values the surface supports.                                                                  |
| `consumed_accessibility_posture_refs` | `posture_id` values the surface honors.                                                              |
| `consumed_layer_order_refs`   | `layer_id` values the surface stacks against.                                                                |
| `consumed_scrim_refs`         | `scrim_id` values the surface uses.                                                                          |
| `gap_records`                 | Zero or more declared inheritance gaps. Each gap pins one `allowed_inheritance_gap_class` value (see below). |
| `findings`                    | Zero or more `audit_finding` records keyed by `audit_finding_class` (see below).                             |
| `gate_state_class`            | Resolved gate state for this row: `pass`, `pass_with_disclosed_gap`, `warn`, `block`, or `block_release`.    |
| `policy_context`              | Re-exported policy-epoch / trust / execution-context packet.                                                 |
| `redaction_class`             | Re-exported redaction class. Tokens, names, and ids cross the boundary; raw bytes never do.                  |
| `minted_at`                   | ISO 8601 monotonic timestamp.                                                                                 |

The closed `audited_surface_class` vocabulary is the same enum the
schema's `applies_to_surfaces` field carries, plus an audit-only
`embedded_surface` slot used by extensions and managed embeds:

- `shell_chrome`
- `editor_canvas`
- `terminal_canvas`
- `review_and_diff_canvas`
- `palette_and_search_canvas`
- `install_update_attach_canvas`
- `ai_apply_canvas`
- `collaboration_canvas`
- `provider_bearing_canvas`
- `docs_help_service_health_canvas`
- `support_export_canvas`
- `onboarding_surface`
- `notification_surface`
- `inspector_surface`
- `embedded_surface`

Surface authors MUST NOT mint additional surface classes. A
surface that does not match any class above is non-conforming and
the audit row resolves to `block`.

## Source-of-truth export contract

The export contract is the schema of record; this section is the
plain-language summary so audit rows and fixtures cite the same
fields.

A first-party manifest emission MUST publish exactly one
`design_token_export_manifest_record` per build with:

- `manifest_id` — opaque, stable id; reused only for additive-minor
  re-emission of the same manifest.
- `source_of_truth_ref` — pinned at one
  `source_of_truth_class` value drawn from the closed eight-class
  vocabulary in the schema (`design_system_manifest`,
  `theme_support_manifest`, `layer_scrim_manifest`,
  `component_state_manifest`, `accessibility_posture_manifest`,
  `icon_registry`, `semantic_status_registry`,
  `trust_visual_state_registry`). A copy-only shadow registry that
  does not point at one of these eight classes is denied with
  `source_of_truth_unresolved` or `shadow_registry_detected`.
- `token_family_ids` — exactly one `token_family_record` id per
  frozen `token_family_class`. The seed publishes 25 ids; a
  manifest with fewer than 25 families is provisional and cannot be
  consumed as authoritative.
- `component_state_ids` — one `component_state_record` id per
  frozen `component_state_class` (22 at M0).
- `theme_support_row_ids` — one `theme_support_row_record` id per
  frozen `theme_class` (4 at M0).
- `accessibility_posture_ids` — one `accessibility_posture_record`
  id per frozen `accessibility_posture_class` (5 at M0).
- `layer_order_ids` — one `layer_order_record` id per frozen
  `layer_order_class` (7 at M0).
- `scrim_token_ids` — one `scrim_token_record` id per frozen
  `scrim_class` (4 at M0).
- `running_build_identity_ref` — opaque ref to the build identity
  re-exported from ADR-0013.
- `policy_context` — re-exported policy-epoch / trust / execution
  context packet (ADR-0008 / ADR-0009 / ADR-0001).
- `redaction_class` — re-exported redaction class (ADR-0011);
  `metadata_safe_default` is the usual value. Raw asset bytes,
  rendered screenshots, and user content never cross.

A manifest that omits any required `*_ids` axis is refused with
the matching schema-level denial reason
(`token_family_class_unresolved`,
`component_state_class_unresolved`, `theme_class_unresolved`, etc.)
and emits a `design_token_manifest_refused`
`token_export_audit_event_record`.

## Per-entity record contract

Every per-entity record references the manifest by id and carries
its own `source_of_truth_ref`, `policy_context`, `redaction_class`,
and `minted_at`. A per-entity record that disagrees with the
manifest's `source_of_truth_ref` class without an explicit
`source_of_truth_class` of its own is denied with
`source_of_truth_unresolved`.

Token families publish:

- `family_id`, `token_family_class`, `token_namespace`,
  `token_count`, `example_token_names`.
- The namespace is part of the contract: `al.color.*`, `space.*`,
  `type.*`, `z.*`, `motion.*`, etc. A token published outside its
  family namespace is a drift violation.

Component states publish:

- `state_id`, `component_state_class`, `applies_to_surfaces`,
  `paired_with_semantic_status`, `not_color_alone` (frozen true).
- Audit rows that bind a surface to a `state_id` outside that
  state's `applies_to_surfaces` list are denied with
  `component_state_class_unresolved`.

Theme rows publish:

- `theme_row_id`, `theme_class`, `is_reference_theme`,
  `is_first_party` (frozen true), `is_color_alone_prohibited`
  (frozen true), `token_overrides_namespace`, contrast targets,
  focus-ring stroke, `forced_colors_friendly`,
  `overlay_scrim_default`, `example_semantic_tokens`.
- A theme row whose `is_first_party` is not true, or whose
  `is_color_alone_prohibited` is not true, is non-conforming.

Accessibility postures publish:

- `posture_id`, `accessibility_posture_class`,
  `suppresses_motion_families`, `preserves_focus_visibility`
  (frozen true), `preserves_state_conveyance` (frozen true),
  `engagement_cue`, `allowed_duration_tokens`.
- A surface that escalates to a more restrictive posture but later
  silently relaxes back to a less restrictive posture before the
  runtime signal clears is denied with
  `accessibility_posture_silent_downgrade`.

Layer-order tokens publish:

- `layer_id`, `layer_order_class`, `ordinal`,
  `is_extension_consumable`, `dialog_and_critical_outrank` (frozen
  true), `toast_never_blocks_dialog` (frozen true).
- A surface that emits a hard-coded z-index instead of binding the
  layer token is denied with `layer_order_hard_coded`.

Scrim tokens publish:

- `scrim_id`, `scrim_class`, `alpha`,
  `never_sole_state_indicator` (frozen true),
  `preserves_focus_visibility` (frozen true), `paired_with_layer`.
- A surface that uses scrim alpha as the only indicator of a
  disabled, blocked, loading, or held state is denied with
  `scrim_used_as_sole_state_indicator`.

## Drift fail-gate vocabulary

The audit emits one or more `audit_finding` records per row. Each
finding pins exactly one `audit_finding_class` from the closed
set:

| `audit_finding_class`                       | Severity   | Routes to                                                   |
|---------------------------------------------|------------|-------------------------------------------------------------|
| `manifest_resolved_clean`                   | `pass`     | Pass row.                                                   |
| `extension_inherited_partial_token_set`     | `pass_with_disclosed_gap` | Allowed inheritance gap (declared `gap_record`). |
| `embedded_surface_partial_inheritance`      | `pass_with_disclosed_gap` | Allowed inheritance gap (declared `gap_record`). |
| `seed_subset_published`                     | `warn`     | Provisional manifest; cannot be consumed as authoritative.  |
| `token_family_class_unresolved`             | `block`    | Schema denial; manifest refused.                            |
| `token_family_repurposed_without_decision_row` | `block` | Breaking change; requires a new decision row and schema bump. |
| `component_state_class_unresolved`          | `block`    | Schema denial.                                              |
| `component_state_repurposed_without_decision_row` | `block` | Breaking change.                                          |
| `theme_class_unresolved`                    | `block`    | Schema denial; theme row missing.                           |
| `accessibility_posture_silent_downgrade`    | `block`    | Posture relaxed without runtime signal clear.               |
| `layer_order_hard_coded`                    | `block`    | Hard-coded z-index at boundary.                             |
| `scrim_used_as_sole_state_indicator`        | `block`    | Scrim alpha is the only state cue.                          |
| `brand_gold_on_restricted_state`            | `block`    | Brand gold is not a state colour.                           |
| `brand_gold_on_policy_locked_state`         | `block`    | Brand gold is not a state colour.                           |
| `density_changed_information_architecture`  | `block`    | Density rules violated.                                     |
| `density_changed_focus_visibility`          | `block`    | Density rules violated.                                     |
| `color_alone_conveyed_required_meaning`     | `block`    | Colour-alone state conveyance.                              |
| `icon_without_label_or_tooltip`             | `block`    | Action icon missing label.                                  |
| `parallel_icon_metaphor`                    | `block`    | Two metaphors for the same canonical command.               |
| `source_of_truth_unresolved`                | `block`    | Per-entity row points at no canonical source.               |
| `shadow_registry_detected`                  | `block`    | Copy-only registry minted parallel ids.                     |
| `design_token_schema_version_lagging`       | `block_release` | Manifest pinned to an older `design_token_schema_version` than the running build. |

`block` findings prevent merge of the surface change; the surface
is non-conforming and the manifest emission, where applicable, is
refused. `block_release` findings additionally prevent the train
that contains the surface from cutting until the lag is resolved.

## Allowed-inheritance-gap classes

Audit rows MAY declare `gap_record` entries that explicitly disclose
inheritance gaps the audit treats as `pass_with_disclosed_gap`
rather than as drift. Each gap pins exactly one
`allowed_inheritance_gap_class`:

- `extension_inherits_first_party_palette`
  — an extension-contributed surface inherits the first-party
  semantic palette unchanged and does not contribute its own theme
  package. The gap declares `inherits_token_family_refs` covering
  the inherited families.
- `extension_partial_high_contrast_inheritance`
  — an extension-contributed surface ships only the dark and light
  reference theme rows and inherits high-contrast support from the
  host. Audit rows resolve high-contrast support to the host's
  theme rows and do not require the extension to publish parallel
  theme rows.
- `embedded_surface_inherits_outer_chrome`
  — an embedded surface (browser webview, host-managed embed)
  inherits the outer shell's theme, posture, layer, and scrim
  tokens because the embed cannot mint its own canvas. The gap
  records the embedding host's `manifest_ref` so the audit can
  follow the inheritance chain.
- `embedded_surface_inert_placeholder_for_unmapped_role`
  — an embedded surface uses the host's `inert_placeholder` token
  for a semantic role it cannot map (e.g. a managed-embed cannot
  resolve `al.color.diff.added` because it has no diff context).
  The gap records the unmapped role so reviewers can confirm the
  fall-back is visually quiet rather than misleading.
- `docs_help_service_health_inherits_shell_palette`
  — service-health and docs / help canvases inherit the shell's
  semantic-status palette unchanged. The gap is allowed at M0
  because these surfaces predate the docs-system theme package.
- `notification_surface_inherits_durable_attention_palette`
  — notification surfaces inherit the durable-attention palette
  rather than minting their own; the gap declares the inherited
  palette.

Rules (frozen):

1. **Inheritance gaps are explicit.** A gap that is not declared on
   the audit row is drift, not inheritance. The audit treats the
   row as `block` and the surface as non-conforming.
2. **Gaps pin a single class.** A surface declaring two
   inheritance reasons in one entry is non-conforming; declare two
   `gap_record` entries instead.
3. **First-party launch-critical surfaces have no inheritance
   gap.** `first_party_shell`, `first_party_component`,
   `first_party_trust`, `first_party_onboarding`,
   `first_party_notification`, and `first_party_durable_attention`
   surfaces MUST resolve every consumed axis through their own
   manifest. The only allowed `gap_record` for these owners is
   `docs_help_service_health_inherits_shell_palette` on the docs /
   help canvas, which is owned by the docs / help lane.
4. **Gaps never widen the export.** A gap that introduces a new
   token family, state class, theme row, posture, layer, or scrim
   beyond the frozen vocabulary is denied. Adding a value to the
   vocabulary requires a `design_token_schema_version` bump and a
   decision row.
5. **Gaps re-export by id, not by value.** A gap that copies token
   bytes locally instead of resolving the inherited
   `manifest_ref` and `family_id` / `state_id` / etc. is treated
   as a parallel mint and denied with `shadow_registry_detected`.

## Mechanical evaluation

Token-drift evaluation runs without human interpretation. The
inputs are:

- one or more `design_token_export_manifest_record` payloads;
- the per-entity `token_family_record`,
  `component_state_record`, `theme_support_row_record`,
  `accessibility_posture_record`, `layer_order_record`, and
  `scrim_token_record` rows the manifest names;
- the per-surface audit rows above; and
- the rule rows in
  [`/artifacts/design/token_drift_rules.yaml`](../../artifacts/design/token_drift_rules.yaml).

The evaluator:

1. Resolves each surface's `manifest_ref` and refuses the row if
   the manifest is missing, refused, or pinned to a different
   `running_build_identity_ref`.
2. Diffs `consumed_*_refs` against the manifest's `*_ids` arrays.
   Any consumed ref the manifest does not publish is a drift
   finding (`token_family_class_unresolved` and friends).
3. Walks each per-entity record and asserts the frozen rules
   (namespace prefix, `not_color_alone = true`,
   `is_first_party = true`, `dialog_and_critical_outrank = true`,
   `never_sole_state_indicator = true`, etc.). A record that does
   not satisfy a frozen rule is a drift finding routed by
   denial-reason class.
4. Resolves declared `gap_records`. Each gap pinned to a closed
   `allowed_inheritance_gap_class` downgrades the matching finding
   to `pass_with_disclosed_gap`. A gap that does not match any
   closed class is treated as drift.
5. Picks the row's `gate_state_class` from the highest severity
   finding still standing after gap resolution
   (`block_release > block > warn > pass_with_disclosed_gap > pass`).

Results are emitted as `token_export_audit_event_record` rows on
the design-token-vocabulary audit stream
(`design_token_conformance_diff_emitted` or
`design_token_conformance_diff_refused`).

## Audit row examples (cross-link)

Worked manifest, denial, and gap rows live under
[`/fixtures/design/token_export_cases/`](../../fixtures/design/token_export_cases/).
The seed includes:

- a clean shell-chrome first-party manifest emission;
- a refused manifest with a missing token-family axis;
- an allowed extension partial-inheritance row that resolves to
  `pass_with_disclosed_gap`;
- an allowed embedded-surface `inert_placeholder` row that resolves
  to `pass_with_disclosed_gap`;
- a blocked first-party token fork (parallel `gold-v2` family);
- a blocked color-alone state indicator on a notification row;
- a blocked hard-coded z-index on a palette canvas; and
- a blocked density change that affected information architecture.

Each fixture pins `audit_row_id`, the resolved
`audit_finding_class`, the resolved `gate_state_class`, and the
`token_drift_rules.yaml` rule id that produced the verdict.

## Out of scope

This document **reserves** the audit format and the fail-gate
vocabulary. It does **not** implement the runtime token consumer or
a visual-diff tool. Specifically out of scope until a superseding
decision row opens:

- The design-system Rust crate that publishes the manifest from
  the implementation. The schema is the cross-tool boundary; the
  Rust types remain the schema of record once the crate lands.
- A pixel-diff or screenshot-diff runner. The audit operates on
  ids and small value packets; pixel comparison is a later lane.
- Per-surface adoption order. Audit rows are additive; the seed
  publishes the first-party shell rows and the extension /
  embedded-surface gap rows, and adopting more surfaces is an
  additive-minor change.

## Reuse guarantee

This audit format is reusable by shell, component, trust,
onboarding, durable-attention, notification, embedded-surface,
docs / help, and screenshot-diff lanes without redefining core
audit semantics. A new lane MUST:

1. Publish or resolve a `design_token_export_manifest_record` and
   read per-entity records by id; never re-derive token-family,
   state, theme, posture, layer, or scrim values locally.
2. Emit one audit row per consuming surface, naming the closed
   `audited_surface_class`, `surface_owner_role_class`, and the
   ids the surface consumed.
3. Declare every inheritance gap explicitly with a closed
   `allowed_inheritance_gap_class`. An undeclared gap is drift.
4. Resolve `gate_state_class` mechanically from the highest-
   severity finding after gap resolution; reviewer prose does not
   override the verdict.
