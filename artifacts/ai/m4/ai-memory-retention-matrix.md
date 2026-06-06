# AI Memory Retention Matrix

Canonical packet: `artifacts/ai/m4/ai_memory_state/support_export.json`

This matrix is the stable product vocabulary for AI thread, memory, cache,
delete, and export truth. Help/About, shell inspectors, support export, and
future runtime stores should read the checked packet instead of cloning these
rows.

| AI state class | Scope | Durable by default | Retention | Sensitivity | Delete/export posture |
|---|---|---|---|---|---|
| `turn_state` | session + workspace | no | completion or short TTL | `T2` maximum; `T3` remains session-only and redacted | no durable export; retained only through disclosed evidence if policy requires |
| `conversation_thread` | user + workspace/repo | local-first when saved or policy-retained | until thread delete or policy expiry | `T2` bounded | user-visible export; delete emits a receipt and invalidates matching caches |
| `prompt_result_cache` | workspace + repo + feature + provider/model | TTL-bounded cache only | until TTL or cache-key invalidation | `T2` local bounded; `T3` forbidden | support export shows inventory hashes only; delete invalidates by key class |
| `reusable_repo_facts_summaries` | workspace, or tenant+repo with explicit policy | regeneratable derived state | graph/docs/policy/workspace epoch | `T1` derived by default | export as provenance-labeled metadata; delete clears derived rows |
| `retained_evidence_copy` | action-scoped | only when feature or policy requires | evidence expiry, case close, or policy hold | redaction-reviewed bounded copy | export through evidence packet; delete follows evidence retention rules |
| `explicit_saved_memory` | user/repo/org with owner label | yes, explicit only | until user or admin removes | `T1` by default; `T3` forbidden | export as saved-memory object; delete emits owner-scoped receipt |

## Cache Key Requirements

Durable prompt/result caches must key on all of:

- `workspace_identity`
- `repo_identity`
- `feature_class`
- `provider_model_version`
- `prompt_pack_version`
- `tool_schema_version`
- `policy_epoch`
- `graph_docs_epoch`
- `retention_posture`

Matching cache entries must invalidate with machine-readable reason codes for:
workspace identity, repo identity, org/tenant/profile, workspace trust,
provider/model, prompt pack, tool schema, policy epoch, graph/docs epoch,
retention posture, and delete request drift.

## Forbidden Reuse

Reusable memory fences deny `T3` secret-adjacent material, raw terminal
transcripts, credentials, and disallowed path contents. Bounded retained copies
are allowed only through reviewed redaction-aware evidence packets.

## Export Truth

Support-safe exports distinguish:

- conversation history inventory;
- reusable facts inventory;
- retained evidence inventory;
- prompt/result cache inventory hashes.

Raw prompt bodies, response bodies, terminal transcripts, credentials, and raw
cache bodies are excluded by default.
