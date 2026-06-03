# Workspace Archetype Detection Readiness Preflight Fixtures

Pinned deterministic records for the claimed-stable preflight matrix.

Each JSON file is a [`WorkspaceArchetypeReadinessPreflightRecord`] produced by
`aureline_stabilize_workspace_archetype_detection_readiness_preflight --corpus`.

## Fixture index

| File | Detection outcome | Key invariant exercised |
|------|-------------------|------------------------|
| `certified_ts_web_app.json` | `certified_archetype_match` | Evidence freshness present; source-labeled signals; same-weight bypasses; no auto-install/trust |
| `probable_python_service.json` | `probable_archetype` | Cached evidence acceptable for probable; retest-needed detector state; source-labeled signals |
| `mixed_ts_python_repo.json` | `mixed_or_ambiguous_workspace` | All four boundary choices present; user-boundary-choice blocking task; no forced single view |
| `unknown_plain_folder.json` | `unknown_or_generic_workspace` | Generic safe default; no unsupported/broken copy; optional tasks only; diagnostic-only signal |
| `restricted_policy_block.json` | `restricted_or_policy_blocked` | Policy block distinct from optional guidance; OpenMinimal offered; plain editing available |
| `missing_devcontainer_engine.json` | `missing_prerequisite` | Missing prerequisite named; OpenMinimal offered; plain editing preserved where safe |

## Regenerating

```bash
cargo run --bin aureline_stabilize_workspace_archetype_detection_readiness_preflight -- --corpus
```
