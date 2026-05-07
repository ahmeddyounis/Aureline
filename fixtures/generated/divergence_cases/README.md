# Divergence / override cases

Worked YAML fixtures for:

- [`docs/generated/diverged_from_generator_contract.md`](../../../docs/generated/diverged_from_generator_contract.md)
- [`schemas/generated/divergence_record.schema.json`](../../../schemas/generated/divergence_record.schema.json)

Each file is a single `divergence_record` and is intended to be export-safe:
opaque refs, typed vocabulary, and digests only. Raw artifact bodies, raw diffs,
raw absolute paths, and credential-bearing URLs do not appear.

## Cases

| File | Scenario |
|---|---|
| `codegen_safe_override_admitted.yaml` | Generated code sibling: override admitted with recorded review + divergence provenance. |
| `mirrored_artifact_direct_edit_blocked.yaml` | Mirror-controlled artifact: direct edit refused; mirror refresh/promotion remains the recovery path. |
| `codegen_unknown_canonical_source_refused.yaml` | Canonical source unknown: direct override refused; manual recovery posture required. |
| `generator_version_drift_compare_required.yaml` | Generator/toolchain changed: compare-to-basis required before admitting override. |
| `imported_partial_lineage_override_admitted.yaml` | Imported artifact with partial lineage: override admitted but remains non-canonical; recovery intent is manual. |
| `recovery_back_to_canonical_regenerate_intent.yaml` | Recovery: an override is superseded by regeneration intent back to canonical-controlled output. |

