# Artifact: Stabilized provider-linked object models, snapshot freshness, and publish-now/publish-later/handoff continuity

**Task:** Stabilize provider-linked object models, actor/target modes, snapshot freshness, and publish-now/publish-later/handoff continuity for review lanes.
**Status:** Implemented
**Verification class:** Automated functional + Conformance

## Summary

The provider-linked review stabilization lane binds provider object rows to review surfaces with explicit mutation mode disclosure, preserved actor/target identity, snapshot freshness observation, and deferred intent records that survive restart. Every claimed provider-backed mutation is modeled in one of four explicit modes: draft-only, publish-later, publish-now, or handoff-only. The chosen mode, acting identity, target object, and required auth source remain inspectable before and after dispatch.

Snapshot freshness and degradation behavior distinguish inspect-only cached state from live provider-backed state. Unsupported mutations downgrade to export/copy/handoff rather than appearing enabled. Local draft and publish-later packets persist across restart with stable action identity, dependency order, and replay safety.

## What changed

- New Rust module: `crates/aureline-review/src/stabilize_provider_linked_object_models_snapshot_freshness_and/mod.rs`
- New fixtures: `fixtures/review/m4/stabilize-provider-linked-object-models-snapshot-freshness-and/`
  - `page.json`
- New schema: `schemas/review/provider_linked_review_stabilization.schema.json`
- New docs: `docs/review/m4/stabilize-provider-linked-object-models-snapshot-freshness-and.md`

## Acceptance criteria

- [x] The checked-in implementation, fixtures, and proof packet for provider-linked review stabilization are current and referenced by the stable proof index.
- [x] Any surface still lacking stable qualification is automatically narrowed below Stable in product copy, docs/help, and release packets.
- [x] Daily Git/review workflows stay previewable, attributable, and reversible.
- [x] Provider-linked or browser-handoff behavior is explicit about freshness and ownership.
- [x] Provider-backed objects clearly disclose whether the current action will stay local, queue for later publish, mutate the provider now, or open a browser handoff packet.
- [x] Provider outages or blocked auth degrade to local draft or handoff without data loss and without claiming provider success.
- [x] Acting-as labels and target identities are preserved in review surfaces, support export, and audit-safe packets for every claimed provider-backed write path.
- [x] Provider-backed actions survive restart, outage, and reconnect as reviewable outbox items with command ID, target identity, actor, queued time, expiry, and replay result instead of becoming orphaned drafts or implied provider success.

## How to verify

```bash
cargo test -p aureline-review stabilize_provider_linked_object_models_snapshot_freshness_and
```

## Risks / follow-ups

- The module currently consumes `ReviewWorkspaceBetaPacket` as the workspace input. When a unified review-state packet is introduced, the constructor should be updated to consume it directly.
- Provider object row refs are opaque strings referencing the provider object model. When the provider crate exposes strongly-typed row references, these should be narrowed.
- Freshness snapshot records model `freshness_class` as a string; when the provider crate stabilizes its freshness label enum, this should be narrowed to a typed enum.
- Deferred intent records currently link to replay ledgers via opaque refs. When the reconciliation crate provides strongly-typed replay ledger items, the intent records should consume them directly.
