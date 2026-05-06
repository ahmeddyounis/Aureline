# Appearance evidence packet template

This document is the **review-packet template** for appearance
support on launch-critical surfaces. It exists so theme, contrast,
density, reduced-motion, power-saver, and text-scale support stop
travelling as a single blanket "theming supported" claim and become
**addressable rows** that reviewers can read per surface, per
component family, and per appearance axis.

The vocabulary and coverage matrix this packet consumes are frozen
in:

- [`/artifacts/design/appearance_row_coverage_matrix.yaml`](../../artifacts/design/appearance_row_coverage_matrix.yaml)
  — the closed `launch_critical_surface_class`,
  `component_family_class`, `theme_class`, `contrast_mode`,
  `density_class`, `accessibility_posture_class`,
  `text_scale_or_zoom_class`, `forced_colors_behavior_class`,
  `inheritance_posture_class`, `coverage_state_class`,
  `severity_class`, `appearance_audit_finding_class`,
  `claim_narrowing_posture_class`, `raw_color_exception_status_class`,
  and `required_capture_class` vocabularies plus the per-surface
  coverage rows.
- [`/docs/design/theme_support_and_inheritance_contract.md`](./theme_support_and_inheritance_contract.md)
  — the audited theme/contrast support row contract every coverage
  row resolves a `theme_support_row_claim_record` against.
- [`/docs/design/component_state_diff_packet_template.md`](./component_state_diff_packet_template.md)
  — the component-state screenshot corpus and review packet whose
  surface row ids and honesty-review axes the appearance packet
  reuses.
- [`/artifacts/design/theme_support_rows.yaml`](../../artifacts/design/theme_support_rows.yaml)
  and
  [`/artifacts/design/component_state_screenshot_corpus.yaml`](../../artifacts/design/component_state_screenshot_corpus.yaml)
  — the four first-party theme rows, five accessibility postures,
  launch-critical surface rows, and component-family taxonomy.
- [`/artifacts/design/token_drift_rules.yaml`](../../artifacts/design/token_drift_rules.yaml)
  — closed `audit_finding_class`, `severity_class`, `gate_state_class`,
  and `allowed_inheritance_gap_class` vocabularies the packet
  verdict resolves through.
- [`/docs/design/token_conformance_gate.md`](./token_conformance_gate.md)
  and
  [`/artifacts/design/raw_color_exception_registry.yaml`](../../artifacts/design/raw_color_exception_registry.yaml)
  — first-party raw-color ban enforcement and the time-bounded exception
  registry used when consuming surfaces cannot comply immediately.
- [`/schemas/design/theme_support_row.schema.json`](../../schemas/design/theme_support_row.schema.json)
  and
  [`/schemas/design/token_export_manifest.schema.json`](../../schemas/design/token_export_manifest.schema.json)
  — the boundary schemas every coverage row cites.

If this document and the coverage matrix YAML disagree, the YAML
wins for tooling and this document MUST be updated in the same
change.

## Why this packet exists

Appearance support is slippery in prose: "the shell is themed",
"high-contrast works", "the dialog respects reduced motion" hide
which component families ship parity, which theme rows are
inherited or out of scope, and whether power-saver or low-motion
postures preserve focus visibility and state conveyance. The
appearance evidence packet replaces those sentences with row-shaped
evidence that ties each appearance axis back to:

1. the canonical `theme_support_row_claim_record` for the surface
   and the `token_export_manifest_ref` it resolved against, so the
   visual coverage and the source-of-truth token inventory cannot
   drift apart;
2. the launch-critical surface row from
   `artifacts/design/component_state_screenshot_corpus.yaml#launch_critical_surface_rows`
   and the component family in scope (tabs, badges or pills,
   settings rows, permission sheets, status items, banners, dialog
   action rows) so reviewers see component-level appearance parity
   instead of shell-level screenshots;
3. the existing token-conformance rules in
   `artifacts/design/token_drift_rules.yaml` (and, for first-party raw-color
   usage outside the semantic-token contract, the exception registry at
   `artifacts/design/raw_color_exception_registry.yaml`), so an appearance gap
   routes to one of the closed `audit_finding_class` values rather
   than a parallel denial reason;
4. the closed `claim_narrowing_posture_class` vocabulary, so a
   missing appearance row drives explicit claim narrowing rather
   than a silent widening of "supported".

The packet is consumed by design, engineering, accessibility,
high-contrast, reduced-motion, support-export, and release
reviewers. Two builds, two design revisions, or one build at two
points in time can compare appearance coverage row-for-row.

## Who reads this template

- **Design reviewers** building, comparing, and signing off
  appearance baselines for the coverage matrix.
- **Engineers** publishing the per-surface appearance evidence
  refs (capture, AT, token-drift, theme-support claim, and
  raw-color exception refs).
- **Accessibility, high-contrast, reduced-motion reviewers**
  cross-checking the packet's posture and density floors,
  power-saver release behaviour, and text-scale reflow evidence.
- **Marketing / claims reviewers** narrowing or expanding
  appearance claims based on coverage rows. A coverage row that
  fails to publish a claimed appearance axis MUST drive a claim
  narrowing rather than survive as drift.
- **Extension and embedded-surface reviewers** comparing host
  parity against extension-contributed approximations using the
  same `inheritance_posture_class` vocabulary.

## Out of scope at this milestone

- Per-OS appearance implementation. The packet pins axes, refs,
  and review verdicts only; the runner is a later task.
- Final theme asset production. Concrete per-token values live in
  the UX style guide and the canonical token export.
- Screenshot automation infrastructure. The packet reserves the
  capture-class vocabulary; the runner is a later task.
- Marketplace theming or extension theme packs. Extension and
  embedded-surface coverage rows resolve through the existing
  `extension_inherits_first_party_palette`,
  `extension_partial_high_contrast_inheritance`,
  `embedded_surface_inherits_outer_chrome`, and
  `embedded_surface_inert_placeholder_for_unmapped_role` allowed-
  inheritance gap classes.

## Packet sections

A complete appearance evidence packet contains the seven sections
below. Sections are evaluated in order; an earlier section's
failure short-circuits the packet's verdict but the packet MUST
still emit every section's row shape so downstream tooling can read
partial results.

1. **Packet header.** Build identity, manifest ref, previous
   packet ref, scope, surfaces in scope, claim-narrowing posture.
2. **Per-row appearance-evidence table.** One row per
   `(launch_critical_surface_class, component_family_class)`
   instance in scope. Each row pins the theme, contrast, density,
   posture, text-scale axis, forced-colors behaviour, inheritance
   posture, coverage state, raw-color exception status, capture
   refs, keyboard journey ref, AT ref, token-drift refs, and
   theme-support claim ref.
3. **Power-saver / low-motion review table.** One row per
   posture-aware coverage row, evaluated against the
   `power_saver_low_motion_review_rules` block in the coverage
   matrix.
4. **Extension / embedded inheritance review table.** One row per
   coverage row whose `surface_owner_role_class` is
   `extension_contributed_surface` or
   `embedded_surface_contributed`, evaluated against the
   `extension_embedded_inheritance_review_rules` block in the
   coverage matrix.
5. **Token-conformance routing.** Each non-pass row resolves to
   one rule_id in `artifacts/design/token_drift_rules.yaml`.
   Reviewers MUST NOT mint parallel denial reasons.
6. **Gate verdict.** The highest-severity row in the packet sets
   the gate verdict. Severity ordering: `pass` <
   `pass_with_disclosed_gap` < `warn` < `block` <
   `block_release`.
7. **Audit trail.** Reviewer roles, evidence-pack id, waiver
   refs and waiver state, claim-narrowing decision-row refs, and
   minted-at timestamp.

## Section 1 — Packet header

Required fields:

- `packet_id` — opaque, stable id for this packet.
- `packet_kind` — closed: `appearance_evidence_baseline`,
  `appearance_evidence_build_to_build`,
  `appearance_evidence_revision_to_revision`,
  `appearance_evidence_release_packet`.
- `running_build_identity_ref` — opaque ref to the build
  identity the packet was minted against.
- `design_token_export_manifest_ref` — id of the manifest the
  captures resolved.
- `previous_packet_ref` — id of the prior packet for build-to-
  build, revision-to-revision, and release-evidence packets;
  null only for `appearance_evidence_baseline`.
- `scope_summary` — plain-language scope (e.g. "shell, dialogs,
  trust prompts, and status strip on the M0 launch path").
- `surfaces_in_scope` — array of `coverage_row_id` values from
  the coverage matrix.
- `claim_narrowing_posture_class` — closed (mirrors
  `appearance_row_coverage_matrix.yaml#claim_narrowing_posture_class_vocabulary`):
  `no_narrowing_required`, `narrowed_to_partial_support`,
  `narrowed_to_known_limit`, `narrowed_to_extension_only`,
  `narrowed_to_embedded_only`.
- `policy_context` — re-exported policy-epoch / trust /
  execution context.
- `redaction_class` — re-exported from ADR-0011; the packet
  carries refs and ids only, never raw screenshots, raw token
  bytes, raw URLs, raw absolute paths, or raw asset bytes.

The packet header MUST cite at least one `coverage_row_id` from
the matrix. A header that names a row not published in the matrix
is non-conforming and the packet refuses to render.

## Section 2 — Per-row appearance-evidence table

One row per `(launch_critical_surface_class, component_family_class)`
instance in scope. Required columns:

| Column | Vocabulary | Purpose |
| --- | --- | --- |
| `coverage_row_id` | from coverage matrix | The surface × component-family binding. |
| `launch_critical_surface_class` | closed | Mirrors the corpus YAML. |
| `component_family_class` | closed | Mirrors the coverage matrix. |
| `surface_owner_role_class` | closed | Mirrors `artifacts/design/token_drift_rules.yaml#surface_owner_role_class_vocabulary`. |
| `canonical_corpus_row_id` | from corpus YAML | The launch-critical surface row this appearance row attaches to. |
| `canonical_theme_support_claim_id` | from theme-support contract | The audited `theme_support_row_claim_record` the appearance row resolves through. |
| `canonical_token_export_manifest_ref` | from token-export manifest | The manifest id the captures resolved. |
| `claimed_theme_classes` | closed × four | The theme rows this coverage row claims; the four-theme floor applies to first-party shell, trust, onboarding, and durable-attention surfaces. |
| `claimed_contrast_modes` | closed | `contrast_standard` plus at least `contrast_high` for first-party launch surfaces. |
| `claimed_density_classes` | closed | At least `standard`; compact and comfortable where the surface admits density. |
| `claimed_accessibility_posture_classes` | closed | At least `motion_standard` and `motion_reduced`. |
| `claimed_text_scale_or_zoom_classes` | closed | At least `text_scale_100` and one further breakpoint. |
| `forced_colors_behavior_class` | closed | Mirrors `theme_support_row.schema.json#forced_colors_behavior_class`. |
| `inheritance_posture_class` | closed | `fully_inherited`, one of the partial-inherited classes (with its allowed-gap mapping), `explicitly_out_of_scope`, or `denied_drift`. |
| `coverage_state_class` | closed | `supported`, `partial`, `not_claimed`, `narrowed_for_release`, `denied_drift`. |
| `raw_color_exception_status_class` | closed | `no_raw_color_in_row`, `raw_color_under_documented_exception`, `raw_color_under_decision_row`, `raw_color_pending_review`, `raw_color_review_denied`. |
| `screenshot_baseline_refs` | nonempty refs | Captures for every `required_capture_class` value the coverage row pins. |
| `keyboard_journey_refs` | nonempty refs | Keyboard-only path, focus visibility under every density and posture, and recovery routes. |
| `assistive_technology_refs` | nonempty refs | Screen-reader announcement and AT-tree captures for the row's claimed states. |
| `token_drift_check_refs` | nonempty refs | Refs to `artifacts/design/token_drift_rules.yaml` rule ids exercised by the row. |
| `theme_support_claim_evidence_refs` | nonempty refs | Refs the theme-support claim cites under `evidence_refs`. |
| `decision_row_refs` | optional | Required when `coverage_state_class = narrowed_for_release` or when a partial-inherited posture cites a decision row. |
| `appearance_audit_finding_class` | closed | The matrix's resolved finding for the row; `appearance_row_resolved_clean` for pass. |
| `severity_class` | closed | Resolved through the matrix's `appearance_audit_finding_to_rule_id_map`. |
| `notes` | optional | Reviewer prose. |

Rules:

1. Every row's `(launch_critical_surface_class,
   component_family_class)` pair MUST appear in the coverage
   matrix's `appearance_coverage_rows`.
2. Every row MUST claim all four first-party theme classes when
   the surface owner is one of `first_party_shell`,
   `first_party_component`, `first_party_trust`,
   `first_party_onboarding`, or `first_party_durable_attention`.
3. A row whose `inheritance_posture_class` is
   `partial_inherited_*` MUST cite the matching
   `allowed_inheritance_gap_class` from the coverage matrix's
   `inheritance_posture_to_allowed_gap_class_map` and the surface
   owner MUST permit that gap class per
   `artifacts/design/token_drift_rules.yaml#surface_owner_gap_policy`.
4. A row whose `raw_color_exception_status_class` is
   `raw_color_under_decision_row` MUST cite a non-empty
   `decision_row_refs`.
5. A row whose `coverage_state_class` is `partial` MUST cite at
   least one gap_ref under
   `theme_support_claim_evidence_refs.gap_refs` (mirrored from
   the theme-support claim).

## Section 3 — Power-saver / low-motion review table

One row per coverage row whose
`claimed_accessibility_posture_classes` includes
`motion_low_motion` or `motion_power_saver`. Required columns:

| Column | Vocabulary | Purpose |
| --- | --- | --- |
| `coverage_row_id` | from coverage matrix | The surface × component-family binding. |
| `accessibility_posture_class` | closed | The posture under review. |
| `posture_review_rule_id` | from coverage matrix | The matrix's `power_saver_low_motion_review_rules.rule_id`. |
| `appearance_audit_finding_class` | closed | The finding resolved by the rule (e.g. `power_saver_silent_relax`, `low_motion_strips_state_conveyance`, `focus_visibility_lost_under_posture`). |
| `evidence_ref` | nonempty ref | Capture, AT trace, or keyboard journey supporting the verdict. |
| `severity_class` | closed | Resolved through the matrix. |
| `routes_to_rule_id` | from token-drift rules | The `artifacts/design/token_drift_rules.yaml` rule id the finding routes through. |

Rules:

1. A row whose `appearance_audit_finding_class` is
   `power_saver_silent_relax` MUST attach an evidence ref showing
   the relaxation event and the original engagement event.
2. A row whose `appearance_audit_finding_class` is
   `low_motion_strips_state_conveyance` MUST attach the original
   motion capture and the low-motion capture so reviewers can see
   the lost state cue.
3. A row whose `appearance_audit_finding_class` is
   `focus_visibility_lost_under_posture` MUST attach the
   pointer-focus and keyboard-only-focus captures so reviewers
   can see the focus-ring loss.

## Section 4 — Extension / embedded inheritance review table

One row per coverage row whose `surface_owner_role_class` is
`extension_contributed_surface` or `embedded_surface_contributed`.
Required columns:

| Column | Vocabulary | Purpose |
| --- | --- | --- |
| `coverage_row_id` | from coverage matrix | The surface × component-family binding. |
| `surface_owner_role_class` | closed | `extension_contributed_surface` or `embedded_surface_contributed`. |
| `inheritance_posture_class` | closed | The declared posture; MUST be one of the matrix's `permitted_inheritance_posture_classes` for the rule. |
| `extension_or_embedded_descriptor_ref` | nonempty ref | The extension UI appearance descriptor or embedded boundary card the row attaches to. |
| `appearance_audit_finding_class` | closed | The finding the rule resolves; `appearance_row_resolved_clean` when the gap is declared and permitted. |
| `evidence_ref` | nonempty ref | The capture or descriptor row backing the verdict. |
| `severity_class` | closed | Resolved through the matrix. |
| `routes_to_rule_id` | from token-drift rules | The rule id the finding routes through. |

Rules:

1. An undeclared partial inheritance MUST route to
   `extension_partial_inheritance_undeclared` or
   `embedded_partial_inheritance_undeclared` and is fail-closed at
   `block`.
2. A declared partial inheritance MUST cite the
   `allowed_inheritance_gap_class` mapped from
   `inheritance_posture_class` in the matrix and is treated as
   `pass_with_disclosed_gap`.

## Section 5 — Token-conformance routing

Every non-pass row in sections 2–4 routes to one rule id in
`artifacts/design/token_drift_rules.yaml`. The mapping is the
matrix's `appearance_audit_finding_to_rule_id_map`; tooling
resolves the rule id from the detected violation rather than
re-deriving it.

The most common routes:

| Detected violation | Routes to rule id | Routes to denial reason |
| --- | --- | --- |
| `theme_row_missing_for_claimed_surface` | `token_drift_rules:block:theme_class_unresolved` | `theme_class_unresolved` |
| `posture_row_missing_for_claimed_surface` | `token_drift_rules:block:accessibility_posture_silent_downgrade` | `accessibility_posture_silent_downgrade` |
| `density_row_missing_for_claimed_surface` | `token_drift_rules:block:density_changed_information_architecture` | `density_changed_information_architecture` |
| `forced_colors_axis_missing` | `token_drift_rules:block:color_alone_conveyed_required_meaning` | `color_alone_conveyed_required_meaning` |
| `color_alone_state_cue_detected` | `token_drift_rules:block:color_alone_conveyed_required_meaning` | `color_alone_conveyed_required_meaning` |
| `reduced_opacity_alone_state_cue_detected` | `token_drift_rules:block:scrim_used_as_sole_state_indicator` | `scrim_used_as_sole_state_indicator` |
| `focus_visibility_lost_under_density` | `token_drift_rules:block:density_changed_focus_visibility` | `density_changed_focus_visibility` |
| `focus_visibility_lost_under_posture` | `token_drift_rules:block:accessibility_posture_silent_downgrade` | `accessibility_posture_silent_downgrade` |
| `power_saver_silent_relax` | `token_drift_rules:block:accessibility_posture_silent_downgrade` | `accessibility_posture_silent_downgrade` |
| `low_motion_strips_state_conveyance` | `token_drift_rules:block:accessibility_posture_silent_downgrade` | `accessibility_posture_silent_downgrade` |
| `extension_partial_inheritance_undeclared` | `token_drift_rules:block:component_state_class_unresolved` | `component_state_class_unresolved` |
| `embedded_partial_inheritance_undeclared` | `token_drift_rules:block:component_state_class_unresolved` | `component_state_class_unresolved` |
| `claim_widened_beyond_evidence` | `token_drift_rules:block:theme_class_unresolved` | `theme_class_unresolved` |
| `raw_color_exception_unreviewed` | `token_drift_rules:block:source_of_truth_unresolved` | `source_of_truth_unresolved` |
| `text_scale_axis_missing` | `token_drift_rules:warn:seed_subset_published` | (warn — no denial reason) |

Rules:

1. Reviewers MUST NOT mint parallel denial reasons. A violation
   that does not route through one of the rule ids above is a
   coverage matrix or rule-set bug, not a packet finding.
2. Brand-gold-on-restricted and brand-gold-on-policy-locked
   violations route through the existing
   `token_drift_rules:block:brand_gold_on_restricted_state` and
   `token_drift_rules:block:brand_gold_on_policy_locked_state`
   rules; the appearance packet does not introduce parallel
   routes.

## Section 6 — Gate verdict

The packet's gate verdict is the highest-severity row across
sections 2–4. Severity ordering, mirroring
`artifacts/design/token_drift_rules.yaml#severity_ordering`:

```
pass < pass_with_disclosed_gap < warn < block < block_release
```

Verdict rules:

1. A `block_release` row produces a verdict of `block_release`
   and SHOULD be cited in the release-evidence packet.
2. A `block` verdict blocks merge for the change set in scope.
3. A `warn` verdict surfaces a reviewer note but does not block.
4. A `pass_with_disclosed_gap` verdict is allowed only when the
   gap class is in
   `artifacts/design/token_drift_rules.yaml#allowed_inheritance_gap_class_vocabulary`
   for the surface owner role of the row in question.
5. A `claim_widened_beyond_evidence` row drives the packet's
   `claim_narrowing_posture_class` to one of
   `narrowed_to_partial_support`, `narrowed_to_known_limit`,
   `narrowed_to_extension_only`, or `narrowed_to_embedded_only`,
   per
   `docs/design/theme_support_and_inheritance_contract.md`. The
   packet refuses to render with `no_narrowing_required` when a
   widened-claim row is present.
6. The verdict block MUST cite the contributing row ids so
   reviewers can navigate to the failing evidence rows directly.

## Section 7 — Audit trail

Required fields:

- `reviewer_role_refs` — closed: `design`, `engineering`, `qa`,
  `docs`, `accessibility`, `extension_parity`, `support`,
  `release`, `claims`.
- `evidence_pack_id` — stable design release evidence pack id.
- `waiver_refs` — empty by default; populated when any row
  carried a waiver.
- `waiver_state` — closed: `none`, `active`, `expired`.
- `decision_row_refs` — refs that justify any
  `narrowed_for_release` row, any documented raw-color exception,
  or any partial-inherited posture cited under a decision row.
- `minted_at` — RFC 3339 timestamp.

## Worked walk-through

A walk-through reviewer reads a packet in this order:

1. **Header.** Confirm `surfaces_in_scope` matches the coverage
   matrix's `appearance_coverage_rows`. If a `coverage_row_id` is
   named that the matrix does not publish, the packet refuses to
   render.
2. **Per-row appearance evidence.** Open the matrix row and
   confirm every claimed theme, contrast, density, posture, and
   text-scale axis has a capture under
   `screenshot_baseline_refs`. Confirm the
   `canonical_theme_support_claim_id` resolves to a published
   `theme_support_row_claim_record` and the
   `canonical_token_export_manifest_ref` resolves through
   `schemas/design/token_export_manifest.schema.json`.
3. **Power-saver / low-motion review.** For every coverage row
   whose posture set includes `motion_power_saver` or
   `motion_low_motion`, walk the
   `power_saver_low_motion_review_rules` block in the matrix and
   confirm the resolved finding is `appearance_row_resolved_clean`
   or routes through one of the allowed rule ids.
4. **Extension / embedded inheritance review.** For every
   extension- or embedded-owned coverage row, confirm the
   declared `inheritance_posture_class` is in the matrix's
   `permitted_inheritance_posture_classes` and the surface owner
   permits the gap.
5. **Token-conformance routing.** Resolve each non-pass row to a
   rule id in `artifacts/design/token_drift_rules.yaml` per
   Section 5.
6. **Gate verdict.** Resolve the highest severity across sections
   2–4 and emit the verdict. If a `claim_widened_beyond_evidence`
   row is present, set `claim_narrowing_posture_class`
   accordingly; do not emit `no_narrowing_required`.
7. **Audit trail.** Confirm the reviewer roles, evidence-pack id,
   waiver state, decision-row refs, and minted-at timestamp.

## Reuse guarantee

The packet template is reusable by shell, components, docs / help,
trust, onboarding, durable-attention, notification,
embedded-surface, and extension-parity lanes without redefining
appearance semantics. A new lane consuming the template MUST:

1. Quote the closed vocabularies from
   `artifacts/design/appearance_row_coverage_matrix.yaml`
   verbatim.
2. Cite a `coverage_row_id` rather than minting a new coverage
   row inline.
3. Resolve every claim through a published
   `theme_support_row_claim_record` and the canonical
   `token_export_manifest_ref`.
4. Route every non-pass row to a rule id in
   `artifacts/design/token_drift_rules.yaml`.
5. Preserve the appearance-axis floor: four-theme parity on
   first-party launch surfaces, contrast_high coverage where
   applicable, motion_reduced and motion_low_motion captures
   where motion is admitted, and at least one text-scale axis
   beyond `text_scale_100`.
