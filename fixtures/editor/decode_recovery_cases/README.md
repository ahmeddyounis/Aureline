# Decode-recovery review cases

Worked YAML records for
[`docs/editor/decode_recovery_and_save_consequence_contract.md`](../../../docs/editor/decode_recovery_and_save_consequence_contract.md).
Each fixture validates the `decode_recovery_review_record` shape and
embeds `text_representation_choice_record` rows for raw-byte,
rendered-text, and escaped-text availability.

| Fixture | Primary coverage |
|---|---|
| [`mixed_encoding_import_preview.yaml`](./mixed_encoding_import_preview.yaml) | Mixed encodings, import preview, blocked save/import commit. |
| [`invalid_utf8_editor_blocked_save.yaml`](./invalid_utf8_editor_blocked_save.yaml) | Undecodable bytes, replacement projection, raw-byte preservation, blocked save. |
| [`bom_eol_normalization_warning.yaml`](./bom_eol_normalization_warning.yaml) | BOM and EOL normalization with explicit rewrite warning. |
| [`diff_view_raw_rendered_divergence.yaml`](./diff_view_raw_rendered_divergence.yaml) | Diff view raw/rendered divergence with suspicious-text and mixed-direction linkage. |
| [`lossy_reencode_acknowledgment.yaml`](./lossy_reencode_acknowledgment.yaml) | Lossy re-encode warning requiring explicit acknowledgment. |
| [`replace_undecodable_spans_warning.yaml`](./replace_undecodable_spans_warning.yaml) | Explicit replacement of undecodable bytes with warning rewrite. |
| [`support_export_security_review_raw_bytes.yaml`](./support_export_security_review_raw_bytes.yaml) | Support export and security review defaulting to raw bytes plus escaped evidence. |
