# M5 Portable-Shelf and Reopen-Parity Registries

- Packet: `m5-portable-shelf-and-reopen-parity-registries:stable:0001`
- Label: `M5 portable-shelf and reopen-parity registries emitting one durable machine-readable portable shelf / bundle per change object or landing candidate — one typed field per section: the bundle ID, the diff refs, the evidence refs, the review-pack version, the redaction profile, and the import / reopen status — with the reopen-parity posture of that shelf kept separate, so stacked work survives browser handoff, offline follow-up, support escalation, incident bridge, and review export without requiring one specific code host or cloud service and no member is silently reordered, collapsed, or retargeted, with canonical / accessible / audit resolution-form coverage, and a machine-readable reopen parity that never lets an imported shelf overclaim current hosted truth — it names the handoff channel, the reopen state, the reopened truth posture (local-only, provider-linked, stale, or redacted), and whether the diff / evidence identity is preserved, so an imported shelf stays blocked from reading as a live provider object for background agents and broad automation unless the user explicitly reopens and confirms it, and nothing lands from an overclaimed imported shelf — rather than a green summary across Git, patch-stack / queue, review, provider-landing, help / docs, and support / export surfaces`
- Consumer surfaces: 6
- Scope selectors: changed_files_scope, pull_request_scope, base_head_range_scope, worktree_uncommitted_scope, full_tree_scope, saved_pack_snapshot_scope, pack_scope_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **change_object_detail**: `stable`
  - Owner: Change-object-detail owner
  - Scope: The change-object detail resolves the portable shelf / bundle for a selected change from the shared registry — the bundle ID, the diff refs, the evidence refs, the review-pack version, the redaction profile, and the import / reopen status — and keeps the reopen-parity posture of that shelf bound separately; a portable shelf missing its bundle ID or diff / evidence refs and a reopen parity that would reopen an imported shelf without naming its handoff channel, reopen state, reopened truth posture, or preserved diff / evidence identity degrade honestly instead of letting a local imported shelf read as a live provider object
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **patch_stack_queue**: `stable`
  - Owner: Patch-stack-queue owner
  - Scope: The patch-stack / queue resolves the portable shelf / bundle for each stack member's change object — bundle ID, diff refs, evidence refs, review-pack version, and import / reopen status — so which member is exported, which is imported as local-only, and which reopened provider-linked are visible before any landing rather than after; a shelf that would land from a stale imported estimate and a member whose diff / evidence identity was dropped on reopen are caught before a green summary can hide them, so no member is silently reordered, collapsed, or retargeted and nothing lands from an overclaimed imported shelf
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support resolves each portable shelf's bundle ID, diff refs, evidence refs, and review-pack version while keeping the reopen-parity posture bound separately to the export; a portable shelf that is a hand-copied per-entry assumption and a reopen parity left unclassified degrade honestly so the diff / evidence identity, its recovery checkpoint, and its export-safe evidence are never dropped on export or reopen — a user can reopen the exported bundle offline or through a support escalation instead of losing the diff / evidence identity when a provider link goes stale or a redaction profile applies
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **stack_edit_review_sheet**: `stable`
  - Owner: Stack-edit-review-sheet owner
  - Scope: The stack-edit / review sheet renders the same resolved portable-shelf and reopen-parity truth bound to the registry — bundle ID, diff refs, evidence refs, review-pack version, and redaction profile — so a shelf's diff / evidence identity and reopen state can no longer be flattened into one generic badge; a portable shelf with an unstated bundle ID or diff / evidence refs is caught before Aureline reopens anything but an explicitly identified, identity-preserved shelf
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **provider_merge_queue**: `stable`
  - Owner: Provider-merge-queue owner
  - Scope: The provider merge queue renders the same resolved portable-shelf and reopen-parity truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied sheet; the reopen-parity posture and the reopened truth (local-only, provider-linked, stale, or redacted) stay inspectable off-renderer so an imported shelf stays blocked from reading as provider-authoritative for background agents and broad automation unless the user explicitly reopens and confirms it, and a stale provider link never reads as current hosted truth
  - Review-pack-record entries: 1 / review-pack-result entries: 1
- **help_docs**: `stable`
  - Owner: Help-docs owner
  - Scope: The help / docs feed carries the same resolved portable-shelf and reopen-parity truth, so a dropped bundle ID, an unstated diff / evidence ref, a diff / evidence identity shown as preserved when it was lost, or an imported shelf reopened by background automation as provider-authoritative is visible in evidence — a handoff channel, a reopen state, or a reopened truth posture — rather than hidden behind a green summary
  - Review-pack-record entries: 1 / review-pack-result entries: 1
