# Stabilized provider-linked object models, snapshot freshness, and publish-now/publish-later/handoff continuity

**Scope:** M04-107 — Stabilize provider-linked object models, actor/target modes, snapshot freshness, and publish-now/publish-later/handoff continuity for review lanes.

**Status:** Stable review lane — implemented in `crates/aureline-review`.

## Goal

Bind provider object rows to review surfaces with explicit mutation mode disclosure, preserved actor/target identity, snapshot freshness observation, and deferred intent records that survive restart. Every claimed provider-backed mutation is modeled in one of four explicit modes: draft-only, publish-later, publish-now, or handoff-only. The chosen mode, acting identity, target object, and required auth source remain inspectable before and after dispatch.

## Design principles

1. **Explicit four-mode disclosure** — Every provider-linked object row carries a `mutation_mode_class` (`draft_only`, `publish_later`, `publish_now`, `handoff_only`) and a human-readable `mode_disclosure_label` so the user always knows whether the action stays local, queues for later, mutates the provider now, or opens a browser handoff.
2. **Preserved actor/target identity** — Every provider-backed write path carries an `actor_identity_ref`, `target_identity_ref`, and `auth_source_ref`. These identities are preserved across review surfaces, support export, and audit-safe packets.
3. **Snapshot freshness and degradation** — `FreshnessSnapshotRecord` distinguishes inspect-only cached state from live provider-backed state. When freshness degrades, the row names a `downgraded_action_class` rather than appearing enabled or collapsing into a generic error.
4. **Deferred intent with replay safety** — `DeferredIntentRecord` persists publish-later and handoff-only intents with `command_id`, `idempotency_key`, `queued_at`, `expires_at`, `replay_ledger_ref`, and `reconnect_review_required`. Provider outages or stale credentials do not erase prepared work.
5. **Restart-resilient session truth** — The support-export packet embeds a `reopen_context_ref` and `reopen_command_id_ref` so the same structured session truth can be reopened after an IDE or CLI restart.
6. **Redaction-safe support export** — Raw URLs and raw provider payloads are explicitly forbidden from crossing the support boundary.

## Record kinds

| Record kind | Purpose |
|---|---|
| `provider_linked_review_stabilization_packet` | Top-level packet consumed by review surfaces and support exports. |
| `provider_linked_review_stabilization_record` | Core stabilization binding workspace, provider object rows, and explicit modes. |
| `provider_linked_object_row_record` | Binding of one provider object row to the review surface with mode disclosure, auth source, and target identity. |
| `actor_target_identity_record` | Preserved acting identity and target identity for every claimed provider-backed write path. |
| `deferred_intent_record` | Persisted deferred intent (publish-later or handoff-only) with idempotency, expiry, replay lineage, and reconnect review. |
| `freshness_snapshot_record` | Snapshot freshness observation with degradation class, provider source, and downgraded action. |
| `provider_linked_command_record` | Command-graph operations (preview, approve, queue, handoff, cancel, export, reconcile, replay). |
| `provider_linked_support_export_packet` | Redaction-safe export with reopen context. |
| `provider_linked_inspection_record` | Compact boolean projection for CLI and inspector surfaces. |

## Closed vocabularies

### Mutation mode classes
- `draft_only`, `publish_later`, `publish_now`, `handoff_only`

### Freshness degradation classes
- `none`, `stale_within_window`, `expired_beyond_window`, `provider_offline`, `revoked_or_disconnected`, `disagrees_with_local`

### Replay safety classes
- `idempotent`, `retry_safe`, `destructive_requires_review`

### Review states
- `active`, `degraded_freshness`, `blocked_drift`, `blocked_auth`, `blocked_scope`, `queued_publish_later`, `pending_handoff`, `completed`, `cancelled`

### Command classes
- `preview_mutation`, `approve_publish_now`, `queue_publish_later`, `open_handoff`, `cancel_intent`, `export_evidence`, `reconcile_drift`, `replay_after_reconnect`

## Key invariants

- Every provider-backed object row (`publish_now`, `publish_later`, `handoff_only`) must have a matching `actor_target_identity_record`.
- `draft_only` rows do not require an actor/target identity binding.
- `inspect_only_cached` and `live_provider_backed` are mutually exclusive on any freshness snapshot.
- Deferred intent `dependency_order_index` values must be zero-based and contiguous.
- All `raw_*_export_allowed` flags in support export must be `false`.
- Provider object rows must cite a non-empty `mode_disclosure_label` describing what the mode means to the user.

## File locations

| Artifact | Path |
|---|---|
| Implementation | `crates/aureline-review/src/stabilize_provider_linked_object_models_snapshot_freshness_and/mod.rs` |
| Schema | `schemas/review/provider_linked_review_stabilization.schema.json` |
| Fixtures | `fixtures/review/m4/stabilize-provider-linked-object-models-snapshot-freshness-and/` |

## Integration with existing lanes

- Consumes [`ReviewWorkspaceBetaPacket`] from the `workspace` module.
- References provider object model rows via opaque `provider_object_row_ref`.
- Projects into the same inspector/CLI/support-export surfaces as the `landing`, `stabilize_review_workspace_anchors_stale_base_labels_approval`, and `harden_browser_handoff_and_in_product_review_boundaries` modules.

## Verification

```bash
cargo test -p aureline-review stabilize_provider_linked_object_models_snapshot_freshness_and
```
