# Component-state screenshot corpus, hover / focus honesty review, and state-semantic diff packet template

This document is the **review-packet template** for component-state
semantics on launch-critical surfaces. It exists so component-state
evidence travels as reviewable rows instead of prose about
`Loading`, `Blocked`, or `Degraded`. The template is shared by
design, engineering, QA, docs, accessibility, and extension-parity
reviewers so two builds, two design revisions, or one build at two
points in time can be compared row-for-row.

The vocabulary and corpus this packet consumes are frozen in:

- [`/artifacts/design/component_state_screenshot_corpus.yaml`](../../artifacts/design/component_state_screenshot_corpus.yaml)
  — the closed `corpus_state_class`, `taxonomy_state_class`,
  `launch_critical_surface_class`, `component_surface_class`,
  `theme_class`, `accessibility_posture_class`, `density_class`,
  `forced_colors_behavior_class`, `non_color_cue_class`,
  `required_screenshot_capture_class`,
  `honesty_review_axis_class`, `honesty_violation_class`,
  `state_evidence_status_class`, and `diff_severity_class`
  vocabularies plus the per-state and per-surface coverage rows.
- [`/docs/design/component_state_taxonomy.md`](./component_state_taxonomy.md)
  — the 13-class shared component review taxonomy reusable
  component packets cite.
- [`/docs/design/design_token_component_state_vocabulary.md`](./design_token_component_state_vocabulary.md)
  — the closed 22-token component-state vocabulary the corpus
  resolves spec-named `corpus_state_class` rows against.
- [`/schemas/design/component_state_machine.schema.json`](../../schemas/design/component_state_machine.schema.json)
  — the state-machine boundary schema whose evidence-hook ref
  family the diff packet reuses verbatim.
- [`/artifacts/design/token_drift_rules.yaml`](../../artifacts/design/token_drift_rules.yaml)
  — closed `audit_finding_class`, `severity_class`, and
  `gate_state_class` vocabularies the diff verdict resolves
  through.
- [`/artifacts/design/theme_support_rows.yaml`](../../artifacts/design/theme_support_rows.yaml)
  and
  [`/artifacts/design/layer_and_scrim_tokens.yaml`](../../artifacts/design/layer_and_scrim_tokens.yaml)
  — the four first-party theme rows, five accessibility postures,
  seven layer tokens, and four scrim tokens the captures must
  honour.
- [`/artifacts/design/component_review_checklist.md`](../../artifacts/design/component_review_checklist.md)
  — the reusable component review gate. The diff packet sits
  alongside that checklist; it does not replace it.

If this document and the corpus YAML disagree, the YAML wins for
tooling and this document MUST be updated in the same change.

## Why this packet exists

Component-state semantics are slippery in prose: "the row goes
quiet during quiet hours" and "the badge looks degraded" hide
whether `quiet_hours_held` is addressable, whether a degraded row
names preserved capability, and whether the user can still reach
the recovery route from a keyboard. The diff packet replaces those
sentences with row-shaped evidence:

1. Each launch-critical surface publishes a screenshot baseline
   per `corpus_state_class` row, with the four first-party theme
   rows, the density matrix, the reduced-motion / low-motion
   captures where motion semantics matter, and the forced-colors
   capture where the row participates in trust, lock, or severity
   posture.
2. Each row is reviewed against the closed
   `honesty_review_axis_class` set (hover, focus_visible,
   selected, disabled, tooltip, keyboard_only_visibility) so
   hover-only or tooltip-only dependencies on critical actions,
   source explanations, lock reasons, freshness, recovery, and
   state labels are detectable mechanically.
3. Two builds, two revisions, or one build at two points in time
   diff their corpus rows. The diff names what changed
   (`diff_kind_class`), how the support row narrowed
   (`support_row_state_change_class`), and whether the change
   passes, narrows pass, warns, blocks, or blocks release
   (`diff_severity_class`).
4. The verdict ties back to existing token-conformance rules. A
   color-alone violation routes to
   `color_alone_conveyed_required_meaning`; a brand-gold-on-
   restricted violation routes to
   `brand_gold_on_restricted_state`; a hard-coded z-index
   substitution at the shell boundary routes to
   `layer_order_hard_coded`.

## Who reads this template

- **Design reviewers** building, comparing, and signing off
  baselines for the corpus.
- **Engineers** publishing the per-surface screenshot, keyboard
  journey, AT, token-drift, and state-machine validation refs.
- **Docs and copy reviewers** confirming each row's user-visible
  summary, state label, source explanation, recovery route, and
  freshness language match the canonical contracts.
- **Accessibility, high-contrast, and reduced-motion reviewers**
  cross-checking later. The corpus reserves the captures and the
  honesty-axis set so those reviews can read one row shape.
- **Extension and embedded-surface reviewers** comparing host
  parity against extension-contributed approximations using the
  same `corpus_state_class` and `taxonomy_state_class` ids.

## Out of scope at this milestone

- Screenshot automation infrastructure or visual-baseline tooling.
  The packet reserves the row shape, the closed vocabulary, and
  the honesty-review rules; the runner is a later task.
- Final per-surface state copy. State copy lives in the UX style
  guide and the surface-specific contracts; this packet pins
  axes, refs, and review verdicts only.
- Marketplace theming or extension theme packs. Extension and
  embedded-surface rows resolve through the existing
  `extension_inherits_first_party_palette`,
  `extension_partial_high_contrast_inheritance`, and
  `embedded_surface_inherits_outer_chrome` allowed-inheritance
  gap classes from `artifacts/design/token_drift_rules.yaml`.

## Packet sections

A complete diff packet contains the seven sections below. Sections
are evaluated in order; an earlier section's failure short-
circuits the packet's verdict but the packet MUST still emit every
section's row shape so downstream tooling can read partial
results.

1. **Packet header.** Build identity, manifest ref, previous
   packet ref, scope, surfaces in scope, claim narrowing
   posture.
2. **Per-row state-evidence table.** One row per
   `corpus_state_class` × `launch_critical_surface_class`
   instance in scope. Each row pins screenshot capture refs,
   keyboard journey ref, AT ref, token-drift refs, docs-link
   verification refs, extension-parity refs, and state-machine
   validation ref.
3. **Per-row honesty-review table.** One row per
   `honesty_review_axis_class` × in-scope corpus row. Each row
   pins the resolved `honesty_violation_class` and the
   `diff_severity_class` from
   `artifacts/design/component_state_screenshot_corpus.yaml#honesty_review_rules`.
4. **State-semantic diff table.** One row per detected change
   between the previous packet snapshot and the current packet
   snapshot. Each row pins a `diff_kind_class`, the
   `support_row_state_change_class`, the
   `corpus_state_class` and `taxonomy_state_class` ids it
   touches, and the resolved `diff_severity_class`.
5. **Token-conformance routing.** Each non-pass row resolves to
   one rule_id in `artifacts/design/token_drift_rules.yaml`.
   Reviewers MUST NOT mint parallel denial reasons.
6. **Gate verdict.** The highest-severity row in the packet sets
   the gate verdict. Severity ordering: `pass` <
   `pass_with_disclosed_gap` < `warn` < `block` <
   `block_release`.
7. **Audit trail.** Reviewer roles, evidence-pack id, waiver
   refs and waiver state, and minted-at timestamp.

## Section 1 — Packet header

Required fields:

- `packet_id` — opaque, stable id for this packet.
- `packet_kind` — closed: `state_diff_baseline`,
  `state_diff_build_to_build`, `state_diff_revision_to_revision`,
  `state_diff_release_evidence`.
- `running_build_identity_ref` — opaque ref to the build
  identity the packet was minted against (re-exported from
  `schemas/design/token_export_manifest.schema.json`).
- `design_token_export_manifest_ref` — id of the manifest the
  captures resolved.
- `previous_packet_ref` — id of the prior packet for build-to-
  build, revision-to-revision, and release-evidence packets;
  null only for `state_diff_baseline`.
- `scope_summary` — plain-language scope (e.g. "shell-chrome and
  durable-attention canvases for the M0 spike review").
- `surfaces_in_scope` — array of `corpus_row_id` values from
  `artifacts/design/component_state_screenshot_corpus.yaml#launch_critical_surface_rows`.
- `claim_narrowing_posture_class` — closed:
  `no_narrowing_required`, `narrowed_to_partial_support`,
  `narrowed_to_known_limit`, `narrowed_to_extension_only`,
  `narrowed_to_embedded_only`. Mirrors the narrowing vocabulary
  from
  `docs/design/theme_support_and_inheritance_contract.md`.
- `policy_context` — re-exported policy-epoch / trust / execution
  context.
- `redaction_class` — re-exported from ADR-0011; the packet
  carries refs and ids only, never raw screenshots or asset
  bytes.

The packet header MUST cite at least one
`corpus:surface:*` row from the corpus YAML. A header that names
a surface not published in the corpus is non-conforming and the
packet refuses to render.

## Section 2 — Per-row state-evidence table

One row per `(corpus_state_class, launch_critical_surface_class,
component_surface_class)` instance in scope. Required columns:

| Column | Vocabulary | Purpose |
| --- | --- | --- |
| `corpus_row_id` | from corpus YAML | The surface row binding. |
| `corpus_state_class` | closed (12 classes) | The user-visible state name from the spec. |
| `taxonomy_state_class` | closed (22 tokens) | The canonical component-state ref this row resolves to via `corpus_to_taxonomy_state_map`. |
| `component_surface_class` | closed | Mirrors `schemas/design/component_state_machine.schema.json#component_surface_class`. |
| `screenshot_baseline_refs` | nonempty refs | Captures for every `required_screenshot_capture_class` value the per-state row pins. |
| `keyboard_journey_refs` | nonempty refs | Keyboard-only path, activation, dismissal, and recovery routes. |
| `assistive_technology_refs` | nonempty refs | Screen-reader announcement and AT-tree captures. |
| `token_drift_check_refs` | nonempty refs | Refs to `artifacts/design/token_drift_rules.yaml` rule ids exercised by the row. |
| `docs_link_verification_refs` | nonempty refs | Verified docs, help, source, policy, or lock explanation links. |
| `extension_parity_fixture_refs` | nonempty refs | Extension, embedded, companion, or handoff parity fixtures when the surface admits extension contribution. |
| `state_machine_validation_refs` | nonempty refs | Refs to fixtures conforming to `schemas/design/component_state_machine.schema.json`. |
| `non_color_cues` | closed | Cues the captures must show; mirrors the corpus per-state `required_non_color_cues`. |
| `state_evidence_status_class` | closed | `required_captured`, `required_awaiting_capture`, `required_waived`, `not_applicable`. |
| `waiver_ref` | optional | Required when status is `required_waived`. |

Rules:

1. Every row's `corpus_state_class` × `launch_critical_surface_class`
   pair MUST appear in the corpus YAML's
   `launch_critical_surface_rows` row's `in_scope_corpus_state_classes`.
2. Every row MUST publish at least the four first-party theme
   captures unless the surface row sets
   `requires_four_theme_parity = false`.
3. A row whose status is `required_awaiting_capture` blocks the
   packet's verdict at `block` until the capture lands.
4. A row whose status is `required_waived` MUST cite a
   `waiver_ref` and is treated as `pass_with_disclosed_gap` for
   the verdict ladder.

## Section 3 — Per-row honesty-review table

One row per `(honesty_review_axis_class, corpus_row_id,
corpus_state_class)` triple in scope. Required columns:

| Column | Vocabulary | Purpose |
| --- | --- | --- |
| `corpus_row_id` | from corpus YAML | The surface row binding. |
| `corpus_state_class` | closed | The user-visible state name. |
| `honesty_review_axis_class` | closed (6 axes) | hover, focus_visible, selected, disabled, tooltip, keyboard_only_visibility. |
| `honesty_violation_class` | closed (21 classes) | Resolved violation; `none` for pass. |
| `evidence_ref` | nonempty ref | The capture, AT trace, or keyboard journey that supports the verdict. |
| `diff_severity_class` | closed | Resolved through `honesty_review_rules` in the corpus YAML. |
| `rule_ref` | nonempty ref | Rule id from `artifacts/design/component_state_screenshot_corpus.yaml#honesty_review_rules`. |
| `notes` | optional | Reviewer prose. |

Rules:

1. A row that fails to enumerate every `required_honesty_review_axes`
   value from the corpus per-state row blocks the packet's
   verdict at `block`.
2. The `tooltip` axis is always reviewed even when the per-state
   row does not require it; tooltip-only critical-action,
   source-explanation, lock-reason, freshness, recovery-route,
   and state-label dependencies are non-conforming everywhere.
3. The `keyboard_only_visibility` axis is reviewed by mirroring
   the `screenshot_baseline_refs` capture under keyboard-only
   focus and confirming state, source, and recovery visibility
   match the pointer-focus capture.

## Section 4 — State-semantic diff table

One row per detected change between `previous_packet_ref` and
the current packet snapshot. The `state_diff_baseline` packet
emits an empty diff table (every row appears for the first time);
all other packet kinds MUST emit a diff table.

Closed `diff_kind_class` vocabulary:

| Class | Meaning | Default severity |
| --- | --- | --- |
| `state_added` | A new `corpus_state_class` row appeared on a surface that previously did not publish it. | `pass` |
| `state_removed_with_decision_row` | A `corpus_state_class` row was removed and a decision-row ref justifies the removal. | `pass_with_disclosed_gap` |
| `state_removed_without_decision_row` | A `corpus_state_class` row was removed without a decision row. | `block_release` |
| `state_label_renamed_same_meaning` | The `display_label` changed but `taxonomy_state_class` is unchanged. | `warn` |
| `state_token_changed_with_decision_row` | A semantic token ref changed under a state and a decision-row ref justifies the change. | `pass_with_disclosed_gap` |
| `state_token_changed_without_decision_row` | A semantic token ref changed under a state without a decision row. | `block` |
| `state_taxonomy_remap` | A `corpus_state_class` row remapped to a different `taxonomy_state_class`. | `block` |
| `state_repurposed_breaking` | A `taxonomy_state_class` was repurposed to mean something different on this surface. | `block` |
| `theme_support_narrowed` | A theme support row narrowed (e.g. high_contrast_dark dropped to partial). | `pass_with_disclosed_gap` |
| `theme_support_widened_blocked` | A theme support claim widened beyond the published `theme_support_row_record`. | `block_release` |
| `posture_support_narrowed` | An accessibility posture support row narrowed. | `pass_with_disclosed_gap` |
| `posture_support_widened_blocked` | A posture support claim widened beyond the published row. | `block_release` |
| `density_support_narrowed` | A density variant narrowed. | `pass_with_disclosed_gap` |
| `forced_colors_support_narrowed` | A forced-colors capture or behavior narrowed. | `pass_with_disclosed_gap` |
| `color_alone_violation_introduced` | A previously compliant row now relies on color alone. | `block_release` |
| `reduced_opacity_alone_violation_introduced` | A previously compliant row now relies on reduced opacity alone. | `block_release` |
| `hover_only_dependency_introduced` | A previously persistent affordance now depends on hover. | `block` |
| `tooltip_only_dependency_introduced` | A previously persistent affordance now depends on tooltip. | `block` |
| `hover_only_dependency_removed` | A hover-only dependency was promoted to persistent disclosure. | `pass` |
| `tooltip_only_dependency_removed` | A tooltip-only dependency was promoted to persistent disclosure. | `pass` |
| `focus_visibility_regression` | The focus ring is hidden, weakened, or collapsed with hover or selected. | `block_release` |
| `lock_explanation_removed` | A previously inline lock or policy reason was removed. | `block` |
| `current_context_collapse_into_selected` | The `current` axis is no longer visually distinct from `selected`. | `block` |
| `pending_collapsed_into_loading` | The `pending` axis is no longer visually distinct from `loading`. | `block` |
| `disabled_collapsed_into_locked` | The `disabled` axis is no longer visually distinct from `locked`. | `block` |
| `disabled_collapsed_into_read_only` | The `disabled` axis is no longer visually distinct from `read_only`. | `block` |
| `restricted_collapsed_into_policy_blocked` | The `restricted` axis is no longer visually distinct from `policy_blocked`. | `block` |
| `degraded_collapsed_into_warning` | The `degraded` axis is no longer visually distinct from `warning_error`. | `block` |
| `quiet_hours_held_unaddressable` | A `quiet_hours_held` row no longer remains addressable for parity audit and support export. | `block` |
| `no_change` | The row is identical to the previous packet's row. | `pass` |

Closed `support_row_state_change_class` vocabulary:

| Class | Meaning |
| --- | --- |
| `narrowed` | The support row covers strictly fewer themes, postures, densities, or captures than the previous packet's row. |
| `widened_with_decision_row` | The support row widened, with a decision-row ref. |
| `widened_blocked` | The support row widened without a decision row. |
| `unchanged` | The support row matches the previous packet's row. |
| `removed_with_decision_row` | The support row was removed, with a decision-row ref. |
| `not_applicable` | The diff row does not touch the support claim. |

Rules:

1. Every `state_diff_build_to_build` and
   `state_diff_revision_to_revision` packet MUST emit at least
   one diff row; the absence of any change is recorded as a
   single `no_change` row, not as silence.
2. A `theme_support_widened_blocked`,
   `posture_support_widened_blocked`,
   `state_repurposed_breaking`,
   `state_removed_without_decision_row`,
   `color_alone_violation_introduced`,
   `reduced_opacity_alone_violation_introduced`,
   `focus_visibility_regression`, or
   `quiet_hours_held_unaddressable` row sets the verdict to
   `block_release` regardless of other rows.
3. A `state_taxonomy_remap` row MUST cite the previous and the
   new `taxonomy_state_class` id explicitly; an absent ref
   blocks the packet at `block_release`.

## Section 5 — Token-conformance routing

Every non-pass row in sections 2–4 routes to one rule id in
`artifacts/design/token_drift_rules.yaml`. The mapping below is
the reviewer's index; tooling resolves the rule id from the
detected violation rather than re-deriving it.

| Detected violation | Routes to rule id | Routes to denial reason |
| --- | --- | --- |
| `color_alone_violation_introduced` | `token_drift_rules:block:color_alone_conveyed_required_meaning` | `color_alone_conveyed_required_meaning` |
| `reduced_opacity_alone_violation_introduced` | `token_drift_rules:block:scrim_used_as_sole_state_indicator` | `scrim_used_as_sole_state_indicator` |
| `focus_visibility_regression` | `token_drift_rules:block:density_changed_focus_visibility` | `density_changed_focus_visibility` |
| `state_taxonomy_remap` (without decision row) | `token_drift_rules:block:component_state_repurposed` | `component_state_repurposed_without_decision_row` |
| `state_repurposed_breaking` | `token_drift_rules:block:component_state_repurposed` | `component_state_repurposed_without_decision_row` |
| `state_added` (when state class is unresolved against the manifest) | `token_drift_rules:block:component_state_class_unresolved` | `component_state_class_unresolved` |
| `theme_support_widened_blocked` | `token_drift_rules:block:theme_class_unresolved` | `theme_class_unresolved` |
| `posture_support_widened_blocked` | `token_drift_rules:block:accessibility_posture_silent_downgrade` | `accessibility_posture_silent_downgrade` |
| `quiet_hours_held_unaddressable` | `token_drift_rules:block:component_state_class_unresolved` | `component_state_class_unresolved` |

Rules:

1. Reviewers MUST NOT mint parallel denial reasons. A violation
   that does not route through one of the rule ids above is a
   corpus or rule-set bug, not a packet finding.
2. Brand-gold-on-restricted and brand-gold-on-policy-locked
   violations route through the existing
   `token_drift_rules:block:brand_gold_on_restricted_state` and
   `token_drift_rules:block:brand_gold_on_policy_locked_state`
   rules; the diff packet does not introduce parallel routes.

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
5. The verdict block MUST cite the contributing row ids so
   reviewers can navigate to the failing evidence rows directly.

## Section 7 — Audit trail

Required fields:

- `reviewer_role_refs` — closed: `design`, `engineering`, `qa`,
  `docs`, `accessibility`, `extension_parity`, `support`,
  `release`.
- `evidence_pack_id` — stable design release evidence pack id.
- `waiver_refs` — empty by default; populated when any row
  carried a waiver.
- `waiver_state` — closed: `none`, `active`, `expired`.
- `minted_at` — RFC 3339 timestamp.
- `decision_row_refs` — refs that justify any
  `state_removed_with_decision_row`,
  `state_token_changed_with_decision_row`, or
  `widened_with_decision_row` row. Empty when no decision row
  was needed.

## Worked walk-through

A walk-through reviewer reads a packet in this order:

1. **Header.** Confirm `surfaces_in_scope` matches the corpus
   YAML's `launch_critical_surface_rows`. If a surface is named
   that the corpus does not publish, the packet refuses to
   render.
2. **Per-row state evidence.** Open the per-state requirements
   block in the corpus YAML and confirm that every required
   capture class appears in `screenshot_baseline_refs` for every
   row in scope. Confirm `state_evidence_status_class` is
   `required_captured` or `required_waived` everywhere; any
   `required_awaiting_capture` row blocks merge.
3. **Per-row honesty review.** Walk every required
   `honesty_review_axis_class` for every row. For each axis,
   confirm the resolved `honesty_violation_class` is `none`. A
   non-`none` violation routes through the corpus's
   `honesty_review_rules` to a `diff_severity_class`.
4. **Diff table.** For each diff row, resolve the
   `diff_kind_class` to a `diff_severity_class` per Section 4.
   Confirm `support_row_state_change_class` is consistent with
   the change. Confirm decision-row refs exist where the row
   class requires one.
5. **Token-conformance routing.** Resolve each non-pass row to a
   rule id in `artifacts/design/token_drift_rules.yaml` per
   Section 5.
6. **Gate verdict.** Resolve the highest severity across
   sections 2–4 and emit the verdict.
7. **Audit trail.** Confirm the reviewer roles, evidence-pack
   id, waiver state, minted-at timestamp, and any decision-row
   refs.

## Reuse guarantee

The packet template is reusable by shell, components, docs / help,
trust, onboarding, durable-attention, notification,
embedded-surface, and extension-parity lanes without redefining
state semantics. A new lane consuming the template MUST:

1. Quote the closed vocabularies from
   `artifacts/design/component_state_screenshot_corpus.yaml`
   verbatim.
2. Cite a `corpus:surface:*` row id rather than minting a new
   surface row inline.
3. Resolve every spec-named state through the
   `corpus_to_taxonomy_state_map`; surface-local state names are
   non-conforming.
4. Route every non-pass row to a rule id in
   `artifacts/design/token_drift_rules.yaml`.
5. Preserve the honesty-review posture: hover, tooltip, and
   keyboard-only-invisible dependencies on critical actions,
   source explanations, lock reasons, freshness, recovery, and
   state labels are fail-closed.
