# M5 session-provenance-freshness and consumer-truth-label registries

Implement lane over the frozen [M5 collaboration-state authority matrix][matrix]
(`m5_collaboration_replica_shared_object_authority_anchor_drift_convergence_and_session_archive_matrix`).
It keeps *collaboration-derived context* from masquerading as canonical repository truth when search, AI
context, review panes, docs links, and support packets reuse it — building on the matrix's CRDT-backed
shared-text domain (the live shared session state other surfaces read from) and its sealed-session-archive
domain — by carrying honest projections of two registries so the claimed M5 shared editor replica view,
presence / cursor layer, comment / annotation / review-pin layer, collaboration degradation banner, support /
export packets, and help / docs surfaces inherit one canonical provenance-freshness-label record and one
consumer-truth-disposition descriptor per consumed context rather than a hand-authored parallel prose that
has to be kept consistent. It closes the gap the source set now expects between collaboration state truth
(shared-object authority, convergence, anchor drift, compaction, degraded-session recovery) and the consumer
rules that keep session provenance and freshness visible: whenever search, AI context, a review pane, a docs
link, or a support packet reads collaboration state — live session state, a captured snapshot, an archived
session artifact, or canonical Git / VFS truth — the provenance class, its freshness or archive class, its
source session and source link, and its actor provenance are recorded first, and session truth may enrich
repo truth but is never presented as canonical Git / VFS state without an explicit label and source link.

## Registry-A — provenance-freshness-label record

One durable, append-only provenance-freshness-label record per piece of collaboration-derived context a
consumer reads, carrying:

- the provenance class of the source (live session state, captured snapshot, archived session artifact, or
  canonical Git / VFS truth), kept mechanically distinct so a session-derived class never collapses into a
  generic canonical badge;
- the freshness or archive class the context carries;
- the source session and source link the context resolves back to;
- the actor provenance of the capture;
- whether the context may enrich but never masquerade as canonical repo truth;
- the disposition offered on it (label-as-session-derived, link-to-source, enrich-repo-truth,
  block-as-canonical, defer);
- the resolution-form coverage (canonical object, accessible summary, audit record).

The provenance label and its source link are recorded before any consumer presents the context. A record
that would let session-derived state read as canonical Git / VFS state without an explicit label and source
link, that is a hand-copied per-entry assumption instead of tracing to the shared registry, that would
promote session state to canonical truth on the user's behalf, or that collapses a distinct live-session /
captured-snapshot / archived-session-artifact class into a generic canonical badge degrades honestly instead
of hiding that the context is session-derived. The registry reuses the matrix
`m5-collaboration-replica-state.schema.json` domain schema.

## Registry-B — consumer-truth-disposition descriptor

The typed consumer-truth-disposition descriptor naming the disposition a consumer can take on
collaboration-derived context — label-as-session-derived, link-to-source, enrich-repo-truth,
block-as-canonical, or defer — plus its actor / time provenance, its provenance / freshness facts, and its
canonical-versus-session lineage. The descriptor keeps the provenance dimensions distinct rather than
collapsing a live-session, captured-snapshot, or archived-artifact source into one canonical lane, never
promotes session state to canonical truth without a label and source link, never acts without actor / time
provenance, and never drops the source link to take a disposition. The registry reuses the matrix
`m5-session-compaction-manifest.schema.json` sealed-session-archive domain schema.

## Acceptance criteria proven by the resolved examples

1. Consumers can show when they are using collaboration-derived context and what its freshness or archive
   class is: the provenance class (live session state, captured snapshot, archived session artifact,
   canonical Git / VFS truth), the freshness / archive class, the source link, the disposition
   (label-as-session-derived, link-to-source, enrich-repo-truth, block-as-canonical, defer), and its actor /
   time provenance stay visible in the UI projection, the CSV / export, and the support packet instead of
   collapsing into a generic status pill.
2. Session truth may enrich repo truth but cannot masquerade as canonical Git / VFS state on claimed M5
   consumers: a record that would present session-derived state as canonical without an explicit label and
   source link, or a disposition that would promote session state to canonical truth on the user's behalf,
   degrades instead of reading as a clean, safe object, so session state is never silently promoted to
   canonical repository truth.
3. The provenance-label-first order is preserved (provenance label and source link first, then enrichment), a
   disposition never acts without actor / time provenance, and no disposition drops the source link; the
   registries keep each provenance class and consumer disposition distinct.

Raw secrets, raw command text, variable bodies, clipboard contents, and private endpoints never cross
this boundary. The Rust validator in `crates/aureline-ui` is the authoritative gate; the checked-in
combined registries schema
(`schemas/collaboration/m5-session-provenance-freshness-and-consumer-truth-label-registries.schema.json`)
documents the shape.

[matrix]: ../../crates/aureline-ui/src/m5_collaboration_replica_shared_object_authority_anchor_drift_convergence_and_session_archive_matrix/mod.rs
