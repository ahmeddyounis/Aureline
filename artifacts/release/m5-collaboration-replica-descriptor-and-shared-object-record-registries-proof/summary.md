# M5 Collaboration-Replica-Descriptor and Shared-Object-Record Registries

- Packet: `m5-collaboration-replica-descriptor-and-shared-object-record-registries:stable:0001`
- Label: `M5 collaboration-replica-descriptor and shared-object-record registries emitting one durable machine-readable collaboration replica descriptor per shared object entering share mode — one typed field per record section: the workspace-root identity, the buffer / object identity, the revision and session epoch, the trust and policy epoch, and the export posture — so entering share mode creates an explicit replica record rather than replacing local buffer, VFS, or Git canonical truth outright, with canonical / accessible / audit resolution-form coverage, and a machine-readable shared-object record per object class (CRDT-convergent shared editable text, sampled presence / cursor / selection samples, server-ordered comments / annotations / review pins, presenter / follow state, linked higher-risk control objects, and immutable sealed-archive evidence) that declares which authority model each object holds — CRDT-convergent, server-ordered, host-authoritative, or immutable evidence — its convergence and merge-drift posture, its append-only anchor-drift history, and its policy-labeled export lineage — so a permission or relay downgrade preserves local unsent work first, anchor drift stays append-only and reviewable, convergence- or awareness-degraded state is never collapsed into a generic stale badge, and op-logs, snapshots, or archives never export without policy-labeled redaction and actor lineage — rather than a green summary across the shared editor replica view, presence / cursor layer, comment / annotation / review-pin layer, collaboration degradation banner, help / docs, and support / export surfaces`
- Consumer surfaces: 6
- Scope selectors: changed_files_scope, pull_request_scope, base_head_range_scope, worktree_uncommitted_scope, full_tree_scope, saved_pack_snapshot_scope, pack_scope_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **shared_editor_replica_view**: `stable`
  - Owner: Shared-editor-replica-view owner
  - Scope: The shared editor replica view resolves each shared object it renders to one typed collaboration replica descriptor — its workspace-root and buffer / object identity, revision and session epoch, trust and policy epoch, and export posture — and to the shared-object record naming the CRDT-convergent shared editable text as a convergent replica of the local buffer rather than the canonical buffer itself; entering share mode creates the replica record without replacing local buffer authority, and a descriptor that cannot bind its buffer identity or that would discard unsent local edits on a permission or relay downgrade degrades honestly instead of letting the replica overwrite local canonical truth
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **presence_cursor_layer**: `stable`
  - Owner: Presence-cursor-layer owner
  - Scope: The presence / cursor layer resolves the sampled presence, cursor, and selection shared objects to one shared-object record that declares them sampled and non-authoritative — presence-only, never convergence truth — and carries the session provenance and freshness a consumer must see; a presence sample presented as authoritative shared state and a stale sample shown as current degrade honestly instead of letting sampled presence read as the converged buffer
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support resolves the collaboration replica descriptor and its shared-object records while keeping each object's declared authority model, convergence state, and export posture bound to the export; a shared object whose authority model is unstated and an op-log, snapshot, or archive that would export without policy-labeled redaction and actor lineage degrade honestly so the replica identity, authority model, and export posture are never dropped on export or companion handoff
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **comment_annotation_review_pin_layer**: `stable`
  - Owner: Comment-annotation-review-pin-layer owner
  - Scope: The comment / annotation / review-pin layer resolves the server-ordered comments, annotations, and review pins to one shared-object record that declares server-ordered authority and keeps an append-only, reviewable anchor-drift history; a pin whose authority model is unstated and an anchor rebound without drift history degrade honestly instead of silently rebinding a comment or pin
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **collaboration_degradation_banner**: `stable`
  - Owner: Collaboration-degradation-banner owner
  - Scope: The collaboration degradation banner resolves the higher-risk control-plane and the convergence- or awareness-degraded shared object to one shared-object record that names the distinct degraded state — convergence-degraded, awareness-degraded, anchor-unresolved, or relay-partitioned — rather than a generic stale or broken badge; a shared object whose convergence state is unstated and a degraded state collapsed into a generic stale badge degrade honestly so a host-authoritative control object never reads as convergent
  - Review-pack-record entries: 1 / review-pack-result entries: 1
- **help_docs**: `stable`
  - Owner: Help-docs owner
  - Scope: The help / docs feed carries the same collaboration replica descriptor and shared-object-record truth, so a shared object whose authority model is unstated, unsent local edits discarded on a downgrade, or an immutable sealed-archive object mislabeled as convergent is visible in evidence — each object class named as CRDT-convergent, server-ordered, host-authoritative, or immutable evidence — rather than hidden behind a green summary
  - Review-pack-record entries: 1 / review-pack-result entries: 1
