# Appearance evidence packet fixtures

Worked fixtures for the appearance row coverage matrix and
evidence packet template frozen in
[`/artifacts/design/appearance_row_coverage_matrix.yaml`](../../../artifacts/design/appearance_row_coverage_matrix.yaml)
and
[`/docs/design/appearance_evidence_packet_template.md`](../../../docs/design/appearance_evidence_packet_template.md).

Every YAML file in this directory pins an addressable row shape
that conforms to the closed vocabularies in the matrix and the
template. The fixtures exist so design, accessibility,
high-contrast, reduced-motion, extension-parity, claims, and
release reviewers can all cite one row shape instead of inventing
local checklists.

The fixtures resolve refs verbatim against:

- [`/artifacts/design/appearance_row_coverage_matrix.yaml`](../../../artifacts/design/appearance_row_coverage_matrix.yaml)
  — for `coverage_row_id`, `launch_critical_surface_class`,
  `component_family_class`, the closed appearance axes, the
  inheritance posture map, the appearance-audit finding map,
  and the power-saver / low-motion and extension / embedded
  review rules.
- [`/artifacts/design/component_state_screenshot_corpus.yaml`](../../../artifacts/design/component_state_screenshot_corpus.yaml)
  — for `canonical_corpus_row_id` and the launch-critical
  surface / component-family taxonomy.
- [`/artifacts/design/theme_support_rows.yaml`](../../../artifacts/design/theme_support_rows.yaml)
  — for the four first-party theme rows and five accessibility
  postures.
- [`/artifacts/design/token_drift_rules.yaml`](../../../artifacts/design/token_drift_rules.yaml)
  — for the rule_id values each non-pass packet row routes
  through.
- [`/schemas/design/theme_support_row.schema.json`](../../../schemas/design/theme_support_row.schema.json)
  and
  [`/schemas/design/token_export_manifest.schema.json`](../../../schemas/design/token_export_manifest.schema.json)
  — for the `canonical_theme_support_claim_id` and
  `canonical_token_export_manifest_ref` every coverage row cites.

## Fixture index

### Baselines

- [`baseline_shell_chrome_tabs_supported.yaml`](./baseline_shell_chrome_tabs_supported.yaml)
  — `appearance_evidence_baseline`. Shell-chrome tabs with full
  parity across every claimed appearance axis; the verdict is
  `pass`.
- [`baseline_trust_prompt_permission_sheet_supported.yaml`](./baseline_trust_prompt_permission_sheet_supported.yaml)
  — `appearance_evidence_baseline`. Trust-prompt permission sheets
  with full theme parity, contrast_high, motion_low_motion, and
  text-scale axes; the verdict is `pass`.

### Disclosed inheritance gaps

- [`partial_inherited_notification_banner_disclosed_gap.yaml`](./partial_inherited_notification_banner_disclosed_gap.yaml)
  — `appearance_evidence_baseline`. Notification banners declare
  `partial_inherited_notification_inherits_durable_attention`
  citing the permitted inheritance gap; the verdict is
  `pass_with_disclosed_gap`.
- [`partial_inherited_docs_help_banner_disclosed_gap.yaml`](./partial_inherited_docs_help_banner_disclosed_gap.yaml)
  — `appearance_evidence_baseline`. Docs / help banners declare
  `partial_inherited_docs_help_inherits_shell_palette`; the
  verdict is `pass_with_disclosed_gap`.

### Refused appearance audits

- [`blocked_settings_row_color_alone_state_cue.yaml`](./blocked_settings_row_color_alone_state_cue.yaml)
  — `appearance_evidence_build_to_build`. Settings row error
  posture used colour-alone state cue; routes to
  `color_alone_state_cue_detected` and through
  `token_drift_rules:block:color_alone_conveyed_required_meaning`.
  The verdict is `block`.
- [`blocked_status_strip_power_saver_silent_relax.yaml`](./blocked_status_strip_power_saver_silent_relax.yaml)
  — `appearance_evidence_build_to_build`. Status item engaged
  motion_power_saver and silently relaxed; routes to
  `power_saver_silent_relax` and through
  `token_drift_rules:block:accessibility_posture_silent_downgrade`.
  The verdict is `block`.
- [`extension_partial_inheritance_undeclared_refused.yaml`](./extension_partial_inheritance_undeclared_refused.yaml)
  — `appearance_evidence_build_to_build`. Extension-contributed
  inspector panel inherited a subset of the first-party palette
  without declaring the gap class; the verdict is `block`.

### Release evidence

- [`release_evidence_packet_narrowed_to_partial_support.yaml`](./release_evidence_packet_narrowed_to_partial_support.yaml)
  — `appearance_evidence_release_packet`. M0 release packet
  narrows the appearance claim to `narrowed_to_partial_support`
  because the inspector surface's high_contrast_light row is
  partial. Drives claim narrowing through one decision row.

## Intended usage

- **Schema and matrix conformance.** The YAML shape is the
  contract of record. Every fixture cites
  `appearance_row_coverage_schema_version: 1` and resolves refs
  against the coverage matrix.
- **Reviewers.** Design, accessibility, high-contrast,
  reduced-motion, support, claims, release, and extension-parity
  reviewers can walk a fixture from `coverage_row_id` to
  per-axis evidence, posture review, inheritance review, and gate
  verdict without negotiating field names.
- **Conformance gates.** A later runner diffs implementation
  against these records (and the canonical token export) without
  reinterpretation.
