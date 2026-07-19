# M5 share-eligibility and downgrade-state registries

Implement lane over the frozen [M5 collaboration-state authority matrix][matrix]
(`m5_collaboration_replica_shared_object_authority_anchor_drift_convergence_and_session_archive_matrix`).
It keeps *unsupported object classes* — unshared local buffers, unsupported large-file modes, binary assets,
declared generated outputs, and policy-limited artifacts — from implying CRDT or full shared-edit support when
search, AI context, review panes, docs links, and support packets, or a user inviting participants or exporting
session artifacts, reach for them — building on the matrix's sampled-presence shared-object descriptor domain
(which classes a session may share, observe, or must keep local) and its higher-risk control-plane
degradation-banner domain — by carrying honest projections of two registries so the claimed M5 shared editor
replica view, presence / cursor layer, comment / annotation / review-pin layer, collaboration degradation banner,
support / export packets, and help / docs surfaces inherit one canonical share-eligibility descriptor and one
share-continuity disposition per candidate shared object rather than a hand-authored parallel prose that
has to be kept consistent. It closes the gap the source set now expects between collaboration state truth
(shared-object authority, convergence, anchor drift, compaction, degraded-session recovery) and the share rules
that keep object eligibility visible: whenever a surface considers a candidate shared object — an unshared local
buffer, an unsupported large-file mode, a binary asset, a declared generated output, or a policy-limited artifact
— the object class, the eligibility class it earns, the authority model that eligibility implies, and whether it
may be shared, observed, or must stay local are recorded first, and a limited-eligibility object may be observed
or commented on but is never presented as a full shared-edit CRDT replica without an eligibility class and source.

## Registry-A — share-eligibility descriptor

One durable, append-only share-eligibility descriptor per candidate shared object a surface could invite into a
session, carrying:

- the object class (unshared local buffer, unsupported large-file mode, binary asset, declared generated output,
  or policy-limited artifact), kept mechanically distinct so a limited-eligibility class never collapses into a
  generic shareable badge;
- the eligibility class it earns (local-only, view-only, comment-only, unsupported-share, or full-shared-edit);
- the authority model that eligibility implies and whether the object may be shared, observed, or must stay local;
- the actor provenance of the eligibility decision;
- whether the object may be shared or observed but never upgraded to CRDT or full shared-edit by default;
- the disposition offered on it (continue-local-only, share-view-only, share-comment-only, block-unsupported-share,
  proceed-full-shared-edit);
- the resolution-form coverage (canonical object, accessible summary, audit record).

The eligibility class and its authority model are recorded before any invite or share action continues. A
descriptor that would let an unsupported object class read as CRDT or full-shared-edit capable without an
eligibility class and authority model, that is a hand-copied per-object assumption instead of tracing to the
shared registry, that would promote a limited-eligibility object to full shared editing on the user's behalf, or
that collapses a distinct local-only / view-only / comment-only / unsupported-share class into a generic shareable
badge degrades honestly instead of hiding that the object cannot converge. The registry reuses the matrix
`m5-shared-object-descriptor.schema.json` domain schema.

## Registry-B — share-continuity disposition

The typed share-continuity disposition naming the choice a user is offered before invite, share, or export
continues — continue-local-only, share-view-only, share-comment-only, block-unsupported-share, or
proceed-full-shared-edit — plus its actor / time provenance, its eligibility facts, and its shared-versus-local
lineage. The descriptor keeps the eligibility dimensions distinct rather than collapsing a local-only, view-only,
comment-only, or unsupported-share object into one shareable lane, never promotes an unsupported object class to
full shared editing without an eligibility class and authority model, never acts without actor / time provenance,
and never drops the eligibility lineage to take a disposition. The registry reuses the matrix
`m5-collaboration-degradation-banner.schema.json` higher-risk control-plane domain schema.

## Acceptance criteria proven by the resolved examples

1. Unsupported object classes do not imply CRDT or full shared-edit support on M5 collaboration surfaces: the
   object class (unshared local buffer, unsupported large-file mode, binary asset, declared generated output,
   policy-limited artifact), the eligibility class it earns, the authority model, the disposition
   (continue-local-only, share-view-only, share-comment-only, block-unsupported-share, proceed-full-shared-edit),
   and its actor / time provenance stay visible in the UI projection, the CSV / export, and the support packet
   instead of collapsing into a generic shareable pill.
2. Users see local-only / view-only / comment-only / unsupported-share truth before inviting participants or
   exporting session artifacts: a descriptor that would present an unsupported object class as shareable without an
   explicit eligibility class and authority model, or a disposition that would promote a limited-eligibility object
   to full shared editing on the user's behalf, degrades instead of reading as a clean, safe object, so an
   unsupported object class is never silently promoted to full shared editing.
3. The eligibility-first order is preserved (eligibility class and authority model first, then invite / share /
   export), a disposition never acts without actor / time provenance, and no disposition drops the eligibility
   lineage; the registries keep each object class and continuity disposition distinct.

Raw secrets, raw command text, variable bodies, clipboard contents, and private endpoints never cross
this boundary. The Rust validator in `crates/aureline-ui` is the authoritative gate; the checked-in
combined registries schema
(`schemas/collaboration/m5-share-eligibility-and-downgrade-state-registries.schema.json`)
documents the shape.

[matrix]: ../../crates/aureline-ui/src/m5_collaboration_replica_shared_object_authority_anchor_drift_convergence_and_session_archive_matrix/mod.rs
