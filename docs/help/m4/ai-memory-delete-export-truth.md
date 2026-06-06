# AI Memory Delete/Export Truth

Help/About memory rows use the canonical packet at
`artifacts/ai/m4/ai_memory_state/support_export.json`.

The stable vocabulary is:

| Label | Meaning |
|---|---|
| `turn_state` | active turn only; not reusable |
| `conversation_thread` | user-visible thread for this workspace/repo |
| `prompt_result_cache` | TTL-bounded cache keyed by workspace, model, prompt pack, tool schema, policy, and graph/docs epochs |
| `reusable_repo_facts_summaries` | provenance-labeled derived repo facts |
| `retained_evidence_copy` | action-scoped retained evidence, shown when policy keeps it |
| `explicit_saved_memory` | owner-labeled saved memory |

Help/About must not describe these as one generic AI history toggle. It should
show scope, provider/model, retention mode, saved-memory owner/policy, retained
copy disclosure, and delete/export availability from the same packet consumed
by support export.
