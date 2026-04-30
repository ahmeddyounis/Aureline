# Theme support, import-mapping, and extension UI appearance fixtures

Worked fixtures for the theme/contrast support row, import-mapping
report, and extension/embedded UI appearance inheritance contract
frozen in
[`/docs/design/theme_support_and_inheritance_contract.md`](../../../docs/design/theme_support_and_inheritance_contract.md).

Each YAML file conforms to one of the boundary schemas:

- [`/schemas/design/theme_support_row.schema.json`](../../../schemas/design/theme_support_row.schema.json)
  — `theme_support_row_claim_record` and
  `theme_support_audit_event_record`.
- [`/schemas/design/theme_import_mapping_report.schema.json`](../../../schemas/design/theme_import_mapping_report.schema.json)
  — `theme_import_mapping_report_record` and
  `theme_import_mapping_audit_event_record`.
- [`/schemas/design/extension_ui_appearance_descriptor.schema.json`](../../../schemas/design/extension_ui_appearance_descriptor.schema.json)
  — `extension_ui_appearance_descriptor_record` and
  `extension_ui_appearance_audit_event_record`.

Every record cites the canonical
[`/schemas/design/token_export_manifest.schema.json`](../../../schemas/design/token_export_manifest.schema.json)
manifest by id so a reviewer, screenshot-diff packet, or conformance
gate can trace the row back to the canonical token export.

## Fixtures

### Theme/contrast support rows

- [`shell_chrome_dark_reference_supported_claim.yaml`](./shell_chrome_dark_reference_supported_claim.yaml)
  — `theme_support_row_claim_record`. First-party shell-chrome surface
  with full parity across all four theme classes; density-aware
  behavior preserves information architecture, focus visibility, and
  state conveyance; protected-cue preservation covers all four cues.
- [`inspector_surface_high_contrast_dark_supported_claim.yaml`](./inspector_surface_high_contrast_dark_supported_claim.yaml)
  — `theme_support_row_claim_record`. Inspector-surface claim with
  `high_contrast_light_support_state = partial` and a recorded gap;
  `forced_colors_behavior = applies_high_contrast_token_overrides`.
- [`notification_surface_reduced_motion_partial_claim.yaml`](./notification_surface_reduced_motion_partial_claim.yaml)
  — `theme_support_row_claim_record`. `motion_low_motion` posture with
  six suppressed motion families, `allowed_duration_tokens =
  motion.instant`, and two recorded gap rows.
- [`density_changed_information_architecture_denied_claim.yaml`](./density_changed_information_architecture_denied_claim.yaml)
  — `theme_support_audit_event_record`. Claim refused because density
  changed information architecture; closed denial reason
  `density_changed_information_architecture`.

### Theme import-mapping reports

- [`imported_translated_theme_mapping_report_with_warnings.yaml`](./imported_translated_theme_mapping_report_with_warnings.yaml)
  — `theme_import_mapping_report_record`. VS Code import with
  translated, substituted-fallback, unsupported, and unresolved rows;
  `parity_claim_state = partial_claim_with_gaps`; rollback path pins
  the appearance checkpoint.
- [`imported_jetbrains_theme_blocked_honesty_denied_report.yaml`](./imported_jetbrains_theme_blocked_honesty_denied_report.yaml)
  — `theme_import_mapping_report_record`. JetBrains import with one
  blocked-honesty row; `parity_claim_state =
  denied_unresolved_or_blocked`; the rollback path was executed and
  `import_outcome = rolled_back`.

### Extension/embedded UI appearance descriptors

- [`extension_webview_partial_inheritance_descriptor.yaml`](./extension_webview_partial_inheritance_descriptor.yaml)
  — `extension_ui_appearance_descriptor_record`. Embedded webview with
  one partial and one does_not_inherit row; cites a non-null embedded
  boundary card ref.
- [`marketplace_account_does_not_inherit_descriptor.yaml`](./marketplace_account_does_not_inherit_descriptor.yaml)
  — `extension_ui_appearance_descriptor_record`. Provider-owned
  marketplace/account surface; density does_not_inherit; focus and
  contrast partially inherit; embedded boundary card pinned.
- [`extension_descriptor_missing_boundary_card_denied_event.yaml`](./extension_descriptor_missing_boundary_card_denied_event.yaml)
  — `extension_ui_appearance_audit_event_record`. Refusal of a
  webview-backed descriptor that omitted the boundary-card ref;
  closed denial reason `embedded_boundary_card_missing`.

## Intended usage

- **Schema conformance:** the YAML shape is the contract of record.
- **Reviewers (design, QA, support):** walk a claim or descriptor to
  its theme-package, token-export, and (when applicable) embedded
  boundary-card refs without negotiating field names.
- **Imported themes:** read the audited mapping report alongside the
  user-facing import-review surface to inspect translated, fallback,
  unresolved, blocked, and rollback rows.
- **Conformance gates:** later runners diff implementation against
  these records (and the canonical token export) without
  reinterpretation.
