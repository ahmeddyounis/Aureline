# M5 AI-review publish-later-draft and compare-reconcile-review registries

Provider-write-missing, local-draft / publish-later, and compare-reconcile fallback lane over the
frozen [M5 AI-review-assist matrix][matrix] (`m5_ai_review_assist_matrix`). It makes the matrix's
`publish_to_review_sheet` and `review_scope_selector` object classes operable by carrying resolved,
honest projections of two registries so review, AI, provider, pending-review, and support / export
surfaces inherit one canonical model of *what happens to a local AI review draft when provider write
scope is missing, deferred, or racing a remote edit* — rather than a false provider-committed state.
Publication stays honest under missing write scope, offline / provider failure, or remote diff drift:
a finding kept local, exported, or copied forward never wears a provider-committed badge, and material
provider drift forces an explicit compare / reconcile instead of a silent last-writer-wins publish.

## Registry-A — publish-later draft (local-draft continuity)

One durable local draft per AI review finding that targets a provider object, carrying:

- the publish-continuity state (`provider_write_missing`, `kept_local_draft`, `exported_fallback`,
  `copied_forward`, `publish_later_queued`, `reconnect_repair_pending`) so a finding that stays local,
  is exported, or is copied forward never implies the provider accepted the mutation;
- the remote object identity the draft targets;
- the expected freshness floor the draft was authored against;
- the target scope and the intended actor;
- the conflict policy the draft carries into a later publish flow;
- the resolution-form coverage (canonical object, accessible summary, audit record).

A draft that cannot bind its remote target identity, that is a hand-copied per-entry assumption instead
of tracing to the shared registry, or that drops its target identity or conflict policy degrades honestly
instead of reading as provider-committed. The registry reuses the matrix
`m5-ai-review-publish-sheet.schema.json` domain schema.

## Registry-B — compare-reconcile review

The typed reconcile decision an outbound path resolves to when the target diff drifted or a provider edit
raced the local draft, keyed to the analyzed diff scope:

- `reconciled_publish_ready` — the local draft reconciles against current remote state and is ready to
  publish;
- `target_diff_drift_reconcile` — the target diff drifted materially before publish, so compare / reconcile
  is required instead of a silent last-writer-wins overwrite;
- `provider_edit_race_reconcile` — provider-side edits raced the local AI review draft, so compare /
  reconcile is required before commit.

Each decision keeps the same local-draft packet — remote object identity, freshness floor, target scope,
intended actor, and conflict policy — preserved in local history and support / export so a deferred draft
can be reopened safely after reconnect or auth repair. A decision that commits a material drift as a silent
last-writer-wins publish, hides the drift, or lets a missing scope masquerade as writable degrades. The
registry reuses the matrix `m5-ai-review-scope-selector.schema.json` domain schema for the analyzed-diff-scope
binding.

## Acceptance criteria proven by the resolved examples

1. When provider write access is missing, users can keep the finding local, export it, or copy it forward
   (`provider_write_missing` / `kept_local_draft` / `exported_fallback` / `copied_forward`) without any
   provider-committed badge appearing.
2. Local drafts carry enough target identity and conflict metadata — remote object identity, freshness floor,
   target scope, intended actor, conflict policy — to reopen safely after reconnect or auth repair.
3. Material provider drift forces compare / reconcile (`target_diff_drift_reconcile` /
   `provider_edit_race_reconcile`) instead of silent last-writer-wins publish behavior.

Raw secret values and private endpoints never cross this boundary. The Rust validator in
`crates/aureline-ui` is the authoritative gate; the checked-in combined registries schema
(`schemas/review/m5-ai-review-publish-continuity-and-reconcile-registries.schema.json`) documents the shape.

[matrix]: ../../crates/aureline-ui/src/m5_ai_review_assist_matrix/mod.rs
