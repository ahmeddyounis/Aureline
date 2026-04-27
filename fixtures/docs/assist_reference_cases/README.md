# Assist-reference and assist-help handoff cases

Worked fixtures for the assist-to-help bridge provenance contract in
[`/docs/docs_integrity/assist_to_help_bridge_contract.md`](../../../docs/docs_integrity/assist_to_help_bridge_contract.md).

`assist_reference_record` fixtures validate against
[`/schemas/docs/assist_reference.schema.json`](../../../schemas/docs/assist_reference.schema.json).
`assist_help_handoff_record` fixtures validate against
[`/schemas/docs/assist_help_handoff.schema.json`](../../../schemas/docs/assist_help_handoff.schema.json).

The corpus exists so hover, peek, completion-detail, diagnostic-help,
inline-assist, docs-browser, glossary, onboarding, AI-explanation, and
support/export implementations can preserve the same provenance model:
what caused the lookup, which source backed it, whether fallback/stale
or browser/provider handoff labels were required, and how the user or
reviewer can return to the origin.

## Fixtures

- [`hover_symbol_project_docs.json`](./hover_symbol_project_docs.json)
  — exact symbol hover resolved to project docs with primary citation
  retention and no browser handoff.
- [`peek_stale_mirrored_docs_fallback.json`](./peek_stale_mirrored_docs_fallback.json)
  — docs peek falls back to a stale mirrored official pack and must
  disclose both fallback mapping and stale mirror posture.
- [`ai_explanation_to_docs_handoff.json`](./ai_explanation_to_docs_handoff.json)
  — AI explanation opens docs while retaining upstream citation anchors
  and the AI thread return path.
- [`provider_native_browser_handoff.json`](./provider_native_browser_handoff.json)
  — provider-native docs require an ADR-0010 browser handoff packet and
  cannot present as primary in-product authority.
- [`file_support_export_reconstruction.json`](./file_support_export_reconstruction.json)
  — support export captures a file-level docs lookup with enough source
  and return metadata for a reviewer to reconstruct the row.

## Required Reconstruction Fields

Every case preserves:

- subject identity (`subject` or `subject_snapshot`);
- mapping quality;
- source selection and source class;
- `pack_id` + `pack_revision_ref`;
- citation and symbol-linked-reference refs where available;
- `locale`, `target_build_identity_ref`,
  `freshness_class_at_mint`, and `version_match_state_at_mint`;
- return path or an explicit export/browser no-return posture.
