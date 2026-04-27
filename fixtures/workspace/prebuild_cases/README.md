# Prebuild fingerprint fixture cases

Fixtures in this directory anchor the contract in
[`/docs/workspace/prebuild_fingerprint_contract.md`](../../../docs/workspace/prebuild_fingerprint_contract.md).
Each YAML file validates against either:

- [`/schemas/workspace/prebuild_fingerprint.schema.json`](../../../schemas/workspace/prebuild_fingerprint.schema.json)
- [`/schemas/workspace/prebuild_invalidation_reason.schema.json`](../../../schemas/workspace/prebuild_invalidation_reason.schema.json)

## Cases

| Fixture | Record kind | Demonstrates |
|---|---|---|
| [`valid_cached_prebuild_fingerprint.yaml`](./valid_cached_prebuild_fingerprint.yaml) | `prebuild_fingerprint_record` | Full fingerprint coverage, cache classes, redaction, and residue exclusions. |
| [`valid_cached_prebuild_reuse_decision.yaml`](./valid_cached_prebuild_reuse_decision.yaml) | `prebuild_reuse_decision_record` | Reuse allowed only when no invalidation or revalidation remains. |
| [`dependency_drift_invalidation.yaml`](./dependency_drift_invalidation.yaml) | `prebuild_invalidation_bundle_record` | Dependency lock drift forces rebuild and user-visible disclosure. |
| [`policy_trust_secret_revalidation.yaml`](./policy_trust_secret_revalidation.yaml) | `prebuild_invalidation_bundle_record` | Policy, trust, and secret-handle changes require revalidation before reuse. |
| [`claimed_warm_start_without_evidence.yaml`](./claimed_warm_start_without_evidence.yaml) | `prebuild_invalidation_bundle_record` | A warm-start claim without cache evidence is rejected instead of rendered as warm. |
| [`stale_index_missing_artifact_partial_warm.yaml`](./stale_index_missing_artifact_partial_warm.yaml) | `prebuild_invalidation_bundle_record` | Stale indexes and missing cache bodies allow partial warm reuse only. |
| [`stale_snapshot_resume_denied.yaml`](./stale_snapshot_resume_denied.yaml) | `prebuild_reuse_decision_record` | A stale snapshot cannot masquerade as live resume. |
| [`local_override_rebuild_disclosure.yaml`](./local_override_rebuild_disclosure.yaml) | `prebuild_disclosure_record` | Local overrides are disclosed and require rebuild before trust. |
| [`fresh_clone_prebuild_cache_excluded.yaml`](./fresh_clone_prebuild_cache_excluded.yaml) | `prebuild_disclosure_record` | Fresh clone path stays distinct from cached prebuild authority. |

Raw secrets, raw environment values, raw terminal history, raw
machine-specific residue, and raw uncommitted user content do not
appear in these fixtures.
