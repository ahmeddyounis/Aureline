# Structured Config, Manifest, And Environment Editor Qualification Evidence

Canonical schema:
`schemas/config/structured-config-manifest-environment-editor-qualification.schema.json`

Canonical fixtures:
`fixtures/config/m4/structured-config-manifest-environment-editor-qualification/`

Emitter:
`cargo run -q -p aureline-config --bin aureline_config_structured_editor_qualification -- index`

Evidence rows:

| Scenario | Expected claim | Purpose |
|---|---:|---|
| `source_effective_live_preserving` | Stable | Proves source/effective/preview/live truth, preservation, validation, redaction, and shared-surface parity. |
| `manifest_compare_only_downgrade` | Beta | Shows truthful downgrade when comments cannot be preserved. |
| `wrong_target_blocks_apply` | Stable | Proves target-context change blocks apply before mutation with an exact reason. |
| `live_state_inspect_only` | Preview | Keeps observed live state inspect-only and avoids implying save/apply convergence. |
| `unsafe_round_trip_overclaim` | Beta | Catches a structured write overclaim when round-trip risk and drill evidence are unsafe. |

Support export posture:

- Includes artifact class, source/effective/live mode, target context,
  round-trip-risk classification, apply timing, and repair lineage.
- Excludes raw secret values by default; secret fields export as handles,
  redacted placeholders, or key paths.
