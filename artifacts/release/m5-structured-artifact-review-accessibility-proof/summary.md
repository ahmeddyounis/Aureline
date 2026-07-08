# Structured-Artifact Review Accessibility, Headless, and Export Parity

- Packet: `structured-artifact-review-accessibility:stable:0001`
- Surface: `Structured-artifact review accessibility, headless, and export parity`
- Accessibility rows: 9 (7 claim-narrowed)
- Proof freshness SLO: 168 hours (last refresh: 2026-06-07T00:00:00Z)

## Accessibility rows

- **artifact_identity_bar** [`row:identity-bar-trusted`]: condition `structured_truth_trusted`, claim `full_structured_fidelity`
- **diff_mode_switcher** [`row:diff-mode-render-untrusted`]: condition `render_trust_unavailable`, claim `raw_fallback_disclosed`
- **structure_row** [`row:structure-row-parser-uncertain`]: condition `parser_schema_uncertain`, claim `partial_structure`
- **merge_decision_row** [`row:merge-decision-write-back`]: condition `write_back_safety_unavailable`, claim `structured_compare_only`
- **generated_artifact_notice** [`row:generated-notice-trusted`]: condition `structured_truth_trusted`, claim `full_structured_fidelity`
- **rendered_compare_viewer** [`row:rendered-compare-render-untrusted`]: condition `render_trust_unavailable`, claim `raw_fallback_disclosed`
- **media_metadata_rail** [`row:media-rail-metadata`]: condition `metadata_unavailable`, claim `metadata_withheld`
- **redaction_or_trust_badge_set** [`row:redaction-badge-metadata`]: condition `metadata_unavailable`, claim `metadata_withheld`
- **compare_summary_card** [`row:compare-summary-write-back`]: condition `write_back_safety_unavailable`, claim `structured_compare_only`
