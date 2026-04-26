# Parser Substrate Cases

These fixtures exercise the parser substrate and coordinate-normalization
contracts:

- `exact_tree_sitter_typescript.json` validates a current Tree-sitter
  parse with exact derived cues.
- `partial_parse_error_rust.json` validates a partial tree with parser
  error nodes and cue-by-cue degradation.
- `missing_grammar_plain_text.json` validates typed fallback when no
  grammar is available.
- `grammar_mismatch_stale_cache.json` validates grammar ABI mismatch and
  stale cache invalidation.
- `decode_recovery_parse_degraded.json` validates syntax degradation
  while raw bytes are unresolved.
- `utf16_protocol_coordinate_mapping.json` validates UTF-16 protocol
  range translation across a supplementary-plane character.
- `decode_recovery_blocks_projection.json` validates coordinate blocking
  over unresolved raw bytes.

The parse-session records validate against
`schemas/language/parse_session.schema.json`. The coordinate records
validate against `schemas/language/coordinate_mapping.schema.json`.
