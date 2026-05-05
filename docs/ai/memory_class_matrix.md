# AI memory-class matrix and durable-state boundary contract (matrix)

This document is the **compact, review-friendly projection** of the AI
memory contract onto one table so delete/export promises, offline
reconciliation behavior, provenance requirements, and boundary rules are
explicit **per `ai_memory_class`**.

This matrix is normative for:

- what each `ai_memory_class` means;
- which boundaries the class MUST respect by default; and
- which delete/export expectations apply per class.

Where this matrix disagrees with:

- [`/docs/ai/memory_and_reconciliation_contract.md`](./memory_and_reconciliation_contract.md), or
- [`/schemas/ai/memory_object.schema.json`](../../schemas/ai/memory_object.schema.json),

those sources win and this matrix MUST be updated in the same change.

## Companion artifacts

- [`/artifacts/ai/memory_classes.yaml`](../../artifacts/ai/memory_classes.yaml) — machine-readable matrix.
- [`/fixtures/ai/memory_class_examples/`](../../fixtures/ai/memory_class_examples/) — worked examples in YAML.

## 1. Memory classes (what exists, what it means)

Every AI-derived row that survives past the active turn MUST resolve to
exactly one `ai_memory_class`. A surface MAY NOT collapse two classes
into one row; doing so denies with `ai_memory_class_collapse_forbidden`.

The per-class defaults below are the contract-level “answer key” for
delete/export requests, cache invalidation, and boundary enforcement.

| `ai_memory_class` | Holds (examples) | Default scope | Durable by default | Default delete posture | Default export posture | Provenance requirements (minimum) | Forbidden by default (examples) |
|---|---|---|---|---|---|---|---|
| `turn_state_ephemeral` | In-flight prompt assembly, selected context snippets, tool outputs, candidate edits | process / session / workspace | no | `delete_clears_local_only` | `not_exportable_local_authority_only` | `conversation_thread_ref` MUST be non-null; never emits a durable row | durable retention; cross-workspace reuse |
| `conversation_history_user_visible` | User-visible thread history, attachments list, user notes | user + workspace | yes (local-first) | `delete_request_supported_with_destruction_receipt` | `exportable_as_user_visible` | `conversation_thread_ref` MUST be non-null; `storage_authority_class` MUST be `user_authored_durable_truth` | “shadow history” outside the user-visible export; unlabeled cross-workspace recall |
| `derived_cache_regeneratable` | Prompt/result caches, retrieval segment caches, summarization caches | workspace + feature + provider/model (keyed) | yes (GC/TTL bounded) | `delete_clears_local_only` | `not_exportable_regeneratable_only` | `invalidation_fingerprints` MUST be non-empty and MUST include at least one of `model_identity_ref`, `prompt_pack_ref`, or `tool_pack_ref` | cross-workspace reuse without scope label; treating caches as user history |
| `embedding_row_regeneratable` | Embedding rows for retrieval/indexing inputs | workspace + embedder identity (keyed) | yes (GC/TTL bounded) | `delete_clears_local_only` | `not_exportable_regeneratable_only` | `invalidation_fingerprints` MUST be non-empty and MUST include `embedder_identity_ref` | cross-tenant or cross-workspace embedding reuse by default |
| `reusable_repo_fact_workspace_scope` | Workspace-scoped derived facts (graph snapshots, doc index summaries) | workspace (identity-keyed) | yes (GC/TTL bounded) | `delete_clears_local_only` | `exportable_metadata_only` | `workspace_identity_ref` MUST be non-null; `invalidation_fingerprints` MUST be non-empty and MUST include `workspace_identity_ref` | unlabeled cross-workspace or cross-repo “memory” |
| `retained_evidence_packet_copy` | Local copy of a retained evidence packet (route/spend receipts, citations anchors) | action-scoped | only when a feature/policy mints evidence | `delete_denied_evidence_packet_overrides` | `exportable_as_evidence_packet` | `originating_evidence_packet_ref` MUST be non-null; `storage_authority_class` MUST be `ai_retained_evidence_packet_class` | using evidence copy as a general cache; rewriting evidence in-place |
| `explicit_saved_memory_user_owned` | User-pinned “remember this” note / instruction | user (pin-scoped) | yes (user-controlled) | `delete_request_supported_with_destruction_receipt` | `exportable_as_user_visible` | `user_pin_ref` MUST be non-null; `storage_authority_class` MUST be `user_owned_recovery_state` | silently applying a pin outside its declared scope; implicit cross-tenant carryover |

Notes:

- “Durable by default” distinguishes “survives process exit” from
  ephemeral turn state. Durable classes still vary: some are **user
  content** (`conversation_history_user_visible`, `explicit_saved_memory_user_owned`),
  some are **regeneratable caches** (`derived_cache_regeneratable`, `embedding_row_regeneratable`, `reusable_repo_fact_workspace_scope`),
  and some are **evidence-governed** (`retained_evidence_packet_copy`).
- A class’s export posture governs whether the class’s **payload** is in
  an export. Regeneratable caches are typically excluded; exports MUST
  still disclose exclusions per class.

## 2. Durable-state boundary rules (what must never happen silently)

### 2.1 Boundary invariants

- AI state MUST NOT cross workspace, repository, tenant, account, or
  provider boundaries unless:
  1) the row’s `ai_memory_class` explicitly admits the scope, and
  2) the row’s provenance (refs + fingerprints) binds it to the new
     scope under an explicit policy decision that is inspectable.
- Every durable (non-`turn_state_ephemeral`) row MUST be deletable,
  exportable, or explicitly non-exportable **by class**. A surface MAY
  NOT offer “AI history” deletion/export as one opaque blob.
- Any durable row whose correctness depends on an identity (provider
  model, prompt pack, tool pack, embedder, policy epoch, workspace
  identity, or workspace trust) MUST carry `invalidation_fingerprints`
  so model/provider changes and trust/policy changes do not produce
  silent cross-context reuse.

### 2.2 Boundary-trigger mapping (what changes cause invalidation or reconciliation)

The memory contract defines the closed invalidation trigger set; the
matrix applies those triggers to classes:

- Provider/model change: invalidates `derived_cache_regeneratable` rows
  keyed on the old identity; conversation history and user pins remain
  authoritative.
- Embedder change: invalidates `embedding_row_regeneratable` rows keyed
  on the old embedder identity.
- Workspace identity/trust/policy changes: invalidates or quarantines
  derived caches and reusable repo facts bound to the prior trust/policy
  posture; any queued offline work MUST revalidate before send.
- Delete/export requests: MUST produce per-class outcomes so reviewers
  can distinguish “history deleted”, “cache cleared”, “evidence preserved
  under retention rules”, and “user pin retained/removed”.

## 3. Delete/export fan-out (worked intent)

The worked examples under
[`/fixtures/ai/memory_class_examples/`](../../fixtures/ai/memory_class_examples/)
demonstrate:

- delete-thread vs delete-workspace behavior per class; and
- export-by-class inclusion/exclusion with explicit per-class reasons.

