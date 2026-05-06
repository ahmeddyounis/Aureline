# Recovery-rung matrix examples

These fixtures are worked examples for the recovery rungs frozen in:

- [`/docs/recovery/recovery_rung_matrix.md`](../../../docs/recovery/recovery_rung_matrix.md)
- [`/artifacts/recovery/recovery_rungs.yaml`](../../../artifacts/recovery/recovery_rungs.yaml)

They exist to make rung selection, required review gates, preserved
state, narrowed capabilities, exit artifacts, and escalation paths
obvious to reviewers without relying on improvised prose.

**Vocabulary**

Rung tokens (`rung_class`) and entry/exit reasons reuse the frozen
support vocabularies (`schemas/support/support_bundle.schema.json`).
Preserved/disabled capability tokens reuse the closed recovery-action
vocabularies (`schemas/support/recovery_action.schema.json`).

## Index

| Fixture | Scenario | Covered rungs |
|---|---|---|
| [`startup_crash_loop_safe_mode.yaml`](./startup_crash_loop_safe_mode.yaml) | repeated startup crash loop | `safe_mode` |
| [`extension_regression_bisect_then_quarantine.yaml`](./extension_regression_bisect_then_quarantine.yaml) | extension regression suspicion | `extension_bisect` → `extension_quarantine` |
| [`cache_corruption_cache_reset_candidate.yaml`](./cache_corruption_cache_reset_candidate.yaml) | suspected disposable cache corruption | `cache_reset_candidate` |
| [`uncertain_trust_state_restricted_reopen.yaml`](./uncertain_trust_state_restricted_reopen.yaml) | uncertain trust posture / policy block | `restricted_reopen` |
| [`bad_update_or_agent_skew_rollback_candidate.yaml`](./bad_update_or_agent_skew_rollback_candidate.yaml) | bad update or helper/agent skew | `rollback_reinstall_candidate` |

