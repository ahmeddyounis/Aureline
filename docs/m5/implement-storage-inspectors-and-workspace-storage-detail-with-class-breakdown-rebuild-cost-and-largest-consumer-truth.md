# M5 storage inspectors and workspace-storage detail

This document is the **reviewer contract** for the storage inspectors and the
workspace-storage detail surface that explain heavy M5 artifact storage to
users, support, and admins without manual filesystem inspection. It binds four
checked-in boundary schemas to one canonical product object so the storage
inspector, workspace-storage detail, low-disk banner, clear-data review, pin
manager, cleanup-history lane, admin storage console, About excerpt, and
support-bundle storage section all read the **same** truth.

The contract is normative. Where it disagrees with the PRD, TAD, TDD, UI/UX
Spec, or design-system style guide, those sources win and this document plus its
schemas, fixtures, and consumer update in the same change.

## What the surfaces explain

- **Total and per-class breakdown.** Every inspector card carries a
  `total_used_bytes` *and* a per-class breakdown row (class used / reclaimable /
  protected / pinned bytes, posture, rebuild cost, authority) so the surface
  never collapses storage into one opaque global number.
- **Workspace / profile / tenant scope.** Each card, breakdown row, and detail
  row carries an `inspector_scope` (device-total, workspace, workset, profile,
  tenant, or slice) so storage truth stays scoped, not device-blurred.
- **Largest-consumer truth.** Cards and breakdown rows carry the top consumers
  by reference, label, and bytes, with an authority-aware persisted order; the
  consumer's authority, rebuild cost, mirror/import origin, and pin summary
  travel with each row.
- **Rebuild cost.** Every workspace-detail row embeds a `rebuild_cost_hint`: the
  offline-rebuild-risk class, startup-impact class, provenance-continuity class,
  the closed inputs a rebuild must consume, and the four-class rebuild-safety
  summary. A surface cannot label an entry "cheap to rebuild" while declaring it
  not rebuildable after removal, nor describe authoritative state with disposable
  copy.
- **Sensitivity class.** Detail rows carry a `sensitivity_class` (`t0`–`t3`); the
  secret-adjacent `t3` tier is admissible only on evidence.
- **Pin and protection state.** Pinned bytes are broken down by pin source; the
  policy-protection state (admin / tenant pin, retention window, open case,
  user-owned authority) and corruption / freshness state are rendered verbatim
  instead of hidden under a single "last used" label.

## The disposable / rebuildable / evidence / recovery distinction

The detail-authority posture distinguishes, per row,
`disposable_derived_state`, `correctness_relevant_derived_state`,
`imported_durable_artifact`, `policy_held_evidence_state`, and
`authoritative_user_owned_state`. The clear and export actions are pinned to that
posture:

- user-owned recovery state refuses every generic clear and routes through the
  class-specific review with an export offered first;
- evidence requires a class-specific review;
- mirrored / offline / signed packs forbid an always-allowed generic clear and
  name their mirror / offline-bundle dependency;
- only unpinned disposable derived caches admit a generic clear.

This satisfies the guardrails: no generic clear-cache button can erase
authoritative recovery or referenced evidence state; deletion impact on offline
continuity and certified-workspace readiness stays explicit; and managed quota
or storage pressure can never silently delete user-owned state.

## Canonical artifacts

- Boundary schemas:
  [`/schemas/storage/storage_inspector_card.schema.json`](../../schemas/storage/storage_inspector_card.schema.json),
  [`/schemas/storage/storage_class_breakdown.schema.json`](../../schemas/storage/storage_class_breakdown.schema.json),
  [`/schemas/storage/workspace_storage_detail.schema.json`](../../schemas/storage/workspace_storage_detail.schema.json),
  [`/schemas/storage/rebuild_cost_hint.schema.json`](../../schemas/storage/rebuild_cost_hint.schema.json).
- Source contracts:
  [`/docs/storage/storage_inspector_contract.md`](../storage/storage_inspector_contract.md)
  and [`/docs/storage/workspace_storage_detail_contract.md`](../storage/workspace_storage_detail_contract.md).
- Canonical product object plus its validator and support projection:
  `crates/aureline-support/src/m5_storage_inspector/`.
- Fixture corpora:
  [`/fixtures/storage/storage_inspector_cases/`](../../fixtures/storage/storage_inspector_cases)
  and [`/fixtures/storage/workspace_storage_detail_cases/`](../../fixtures/storage/workspace_storage_detail_cases).
- Golden support-export projection and replay gate:
  [`/fixtures/storage/m5_storage_inspector/support_export.golden.json`](../../fixtures/storage/m5_storage_inspector/support_export.golden.json).
- Regenerator:
  `cargo run -p aureline-support --example dump_m5_storage_inspector_support_export`.

## Vocabulary reuse

The module mints no new storage primitive. `storage_class_id` re-exports from
`crate::storage_inspector`; `authority_class`, `rebuild_cost_class`,
`gc_policy_class`, `clear_cache_protection_class`, and `pin_source_class`
re-export from `crate::m5_storage_governance`. The inspector / detail surface
vocabularies (posture, quota basis, scan posture, sensitivity, freshness,
corruption, policy-protection, pin, clear, export, and the rebuild-cost axes)
are frozen by the source contracts above.
