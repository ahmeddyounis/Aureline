# Locale fallback and representation review fixtures

These fixtures anchor the row contract frozen in
[`/docs/accessibility/locale_fallback_and_copy_representation_contract.md`](../../../docs/accessibility/locale_fallback_and_copy_representation_contract.md).

They validate against:

- [`/schemas/accessibility/locale_fallback_row.schema.json`](../../../schemas/accessibility/locale_fallback_row.schema.json)
- [`/schemas/security/text_representation_action.schema.json`](../../../schemas/security/text_representation_action.schema.json)

Each YAML file is a single record with a `# yaml-language-server:
$schema=...` header. The cases cover locale-pack fallback, bidi
controls, invisible characters, malformed text, rendered-versus-raw
divergence, sanitized snapshot export, metadata-only export, and raw or
escaped copy parity.

| Fixture | Record kind | Why it is here |
|---|---|---|
| `locale_pack_partial_fallback.yaml` | `locale_fallback_row_record` | Partial requested-locale pack falls back to source language with pack freshness and source-language action visible. |
| `mixed_direction_diff_bidi_controls.yaml` | `mixed_direction_text_inspector_record` | Bidi controls in a diff hunk expose directionality, codepoint classes, raw view, and escaped view. |
| `invisible_install_review_inspector.yaml` | `mixed_direction_text_inspector_record` | Invisible formatting in a package identifier stays inspectable on a trust-decision surface. |
| `copy_raw_editor_bidi_controls.yaml` | `text_representation_action_record` | `Copy raw` is the primary editor action when exact source text is available. |
| `copy_escaped_security_review.yaml` | `text_representation_action_record` | `Copy escaped` is required for suspicious identifiers in security review. |
| `copy_rendered_docs_preview_divergence.yaml` | `text_representation_action_record` | `Copy rendered` remains distinct from raw source in docs preview. |
| `export_sanitized_snapshot_support.yaml` | `text_representation_action_record` | Support export uses a sanitized snapshot with active content removed. |
| `export_metadata_only_terminal_malformed.yaml` | `text_representation_action_record` | Malformed terminal transcript evidence narrows to metadata-only export when source bytes cannot safely leave. |
