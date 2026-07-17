# M5 Blocked-or-Escalate Card and Escalation-Outcome Registries

- Packet: `m5-blocked-escalate-card-registries:stable:0001`
- Label: `M5 blocked-escalate-card and escalation-outcome registries emitting one reusable machine-readable blocked-escalate card per blocked or escalated tracked work item — one typed field per card section: the blocker class, the missing dependency or approval, the suggested escalation path, the attach-evidence action, and the local note or handoff-packet fallback — each bound to one commit state with its lineage, so a blocked-or-escalate card never drops its blockers / linked evidence / attach-evidence-export-retry continuity and no local handoff packet reads as a provider-committed escalation, with canonical / accessible / audit resolution-form coverage, and a machine-readable escalation-outcome object (escalated to provider, queued as local handoff packet, exported locally, blocked by missing permission, or blocked by unresolved engineering state) that keeps each blocker cause a visible, typed action distinguishing dependency, approval, provider, policy, and unresolved-engineering causes instead of one generic warning — so a blocked-or-escalate card never implies the provider accepted the escalation when the target is offline, policy-blocked, or only partially writable — across work-item detail, review detail, Git / worktree, provider-handoff, help / docs, and support / export surfaces`
- Consumer surfaces: 6
- Scope selectors: changed_files_scope, pull_request_scope, base_head_range_scope, worktree_uncommitted_scope, full_tree_scope, saved_pack_snapshot_scope, pack_scope_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **work_item_detail**: `stable`
  - Owner: Work-item-detail owner
  - Scope: Work-item detail resolves a blocked tracked work item to one typed blocked-escalate card — its blocker class, missing dependency or approval, suggested escalation path, attach-evidence action, and local note or handoff-packet fallback — from the shared registry and proves the escalation authority for that item; users can attach evidence and export or retry from the same blocked state without losing the tracked-item context, and a card dropping its blockers or linked evidence and an escalation-outcome that would let a local handoff packet read as a provider-committed escalation degrade honestly instead of implying the provider accepted an escalation it has not
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **review_detail**: `stable`
  - Owner: Review-detail owner
  - Scope: Review detail resolves the same blocked-escalate card from the tracked item and shows the blocker class, the missing dependency or approval, and the suggested escalation path bound to their commit state; a card letting a queued or local handoff packet read as provider-committed and a dropped attach-evidence / export / retry path are caught before a green summary can hide them, so review detail renders the same blocker truth as work-item detail without contradiction
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support resolves the card's commit state while keeping the blocker class / linked evidence / suggested escalation path and the escalated-to-provider / queued-as-local-handoff-packet / exported-locally attribution bound to the export, and reports the escalation authority; a card that is a hand-copied per-item assumption and an escalation-outcome on an unclassified binding degrade honestly so the blockers, linked evidence, and attach-evidence / export / retry continuity are never dropped on export or retry
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **linked_change_panel**: `stable`
  - Owner: Linked-change-panel owner
  - Scope: The linked-change panel surface renders the same card's linked evidence and suggested escalation path bound to their commit state — escalated to provider, queued as local handoff packet, exported locally, blocked by missing permission, or blocked by unresolved engineering state — from the registry so the dependency, approval, provider, policy, and unresolved-engineering blocker causes can no longer be flattened into one generic warning, and a target that is offline, policy-blocked, or only partially writable stays visible and actionable instead of implying provider acceptance; an unstated commit state on a card is caught before it can drift
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **blocked_escalate_card**: `stable`
  - Owner: Blocked-escalate-card owner
  - Scope: The blocked-escalate card renders the same resolved card and escalation-outcome truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied card, letting users compare escalated-to-provider, queued-as-local-handoff-packet, and exported-locally outcomes from one card while preserving attach-evidence / export / retry continuity; the escalation-outcome state and the blocker-cause state stay inspectable off-renderer so a local handoff packet never reads as a provider-committed escalation
  - Review-pack-record entries: 1 / review-pack-result entries: 1
- **help_docs**: `stable`
  - Owner: Help-docs owner
  - Scope: The help / docs feed carries the same resolved card and escalation-outcome truth, so a dropped evidence field, an unstated commit state, a local handoff packet masquerading as a provider-committed escalation, or an offline / policy-blocked / partially-writable target shown as accepted is visible in evidence — a blocker-cause change or a commit-state change — rather than hidden behind a green summary
  - Review-pack-record entries: 1 / review-pack-result entries: 1
