# AI Memory Delete and Export Contract

Runtime owner: `aureline_ai::memory`

The stable AI memory lane makes memory classes explicit and local-first. A
claimed stable AI thread, inspector, Help/About row, or support export must use
the same six class names and retention labels from
`artifacts/ai/m4/ai_memory_state/support_export.json`.

## Stable Classes

- `turn_state` is active-turn state only.
- `conversation_thread` is the user-visible local thread.
- `prompt_result_cache` is a TTL-bounded derived cache.
- `reusable_repo_facts_summaries` is provenance-labeled workspace-derived
  summary state.
- `retained_evidence_copy` is action-scoped evidence governed by retention
  rules.
- `explicit_saved_memory` is owner-labeled saved memory.

No class reuses or recalls context across workspaces, tenants, profiles, or
repos by default. Broader reuse requires an explicit policy label and scope
label on the row.

## Visible Surfaces

Thread headers, memory inspectors, delete/export review sheets, Help/About, and
support exports must show the same facts:

- scope chip;
- provider/model ref;
- retention mode;
- saved-memory owner and policy;
- retained-copy disclosure;
- delete/export path.

These surfaces read the checked packet rather than maintaining independent
copy.

## Durable Cache Keys

Prompt/result caches must key on workspace identity, repo identity, feature
class, provider/model version, prompt-pack version, tool-schema version, policy
epoch, graph/docs epoch, and retention posture. They invalidate on workspace,
repo, org/tenant/profile, trust, provider/model, prompt-pack, tool-schema,
policy, graph/docs, retention, or delete-request drift and surface the matching
reason code.

## Delete and Export

Deleting a thread deletes the visible thread state and invalidates matching
prompt/result caches. Deleting workspace AI state additionally clears reusable
repo facts and explicit saved memory selected by the owner/policy flow.
Retained evidence copies are never hidden; when policy keeps them, the review
sheet labels the retained class and emits an omission reason.

Exports separate conversation history, reusable facts, retained evidence, and
cache inventory hashes. Raw prompt bodies, response bodies, terminal
transcripts, credentials, and raw cache bodies are excluded by default.

## Verification

The Rust validator rejects packets that miss a memory class, omit required
surface disclosures, admit secret-adjacent reusable memory, fail to invalidate
durable caches on delete, or collapse support exports into raw bodies.

```sh
cargo test -p aureline-ai memory
```
