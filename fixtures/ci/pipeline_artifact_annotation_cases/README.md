# Pipeline artifact-card / annotation-row worked fixtures

These fixtures exercise the contract frozen in
[`/docs/ci/pipeline_artifact_annotation_contract.md`](../../../docs/ci/pipeline_artifact_annotation_contract.md)
and the boundary schemas:

- [`/schemas/ci/pipeline_artifact_card.schema.json`](../../../schemas/ci/pipeline_artifact_card.schema.json)
- [`/schemas/ci/pipeline_annotation_row.schema.json`](../../../schemas/ci/pipeline_annotation_row.schema.json)

Cases at this revision:

- `structured_report_sarif_card.yaml` -
  a SARIF report uploaded by a SAST scanner step; the card
  resolves `artifact_kind_class = structured_report_artifact`,
  `media_type_class = sarif_report`, `safe_open_path =
  open_in_structured_viewer`, `trust_class = SanitizedRich`,
  `viewer_mode_class = open_detail`, and a non-null
  `output_viewer_object_ref`.
- `html_artifact_safe_preview_download_only.yaml` -
  an HTML coverage report; the card resolves
  `artifact_kind_class = html_bundle_artifact`, `media_type_class =
  html_document`, `safe_open_path = open_in_safe_preview_sanitized`,
  `trust_class = SanitizedRich`, `active_content_policy_class =
  active_content_blocked_trust`, `viewer_mode_class =
  blocked_active_content`, and surfaces a typed
  `download_only_alternative_label` so the user can fall back to
  download-only without the surface auto-opening the report as a
  trusted webview.
- `stale_annotation_workspace_drifted.yaml` -
  a third-party SARIF SAST finding whose original
  `file_line_anchor` has drifted against the current workspace;
  the row resolves `anchor_freshness_class =
  approximate_anchor_drifted_workspace`, `local_action_class =
  jump_to_local_anchor_with_drift_disclosure_admissible`, and a
  typed `anchor_drift_explanation_label`.
- `binary_bundle_with_checksum_retention.yaml` -
  a Linux x86_64 release binary; the card resolves
  `artifact_kind_class = binary_bundle_artifact`, `media_type_class
  = executable_binary`, `safe_open_path =
  download_only_no_in_product_open`, `viewer_mode_class =
  not_applicable_no_in_product_open`, a non-null
  `content_digest_disclosure` (sha256_content) and a typed
  `retention_window_class = provider_default_retention` with an
  `expires_at` timestamp.
