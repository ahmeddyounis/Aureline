# M5 collaboration-state headless-inspect and support-bundle parity registries

Implement lane over the frozen [M5 collaboration-state authority matrix][matrix]
(`m5_collaboration_replica_shared_object_authority_anchor_drift_convergence_and_session_archive_matrix`).
It makes *collaboration-state truth inspectable outside the live desktop UI* — collaboration replica descriptors,
shared-object records, anchor history, degraded session states, compaction manifests, and archive-finalization
descriptors — so that CLI / headless inspect surfaces, exported packets, and support bundles carry the same state
vocabulary and actor / source labels the desktop UI shows. Building on the matrix's presenter-follow
convergence-state domain (the convergence, awareness, and drift facts a session carries) and its sealed-session
compaction-manifest domain (what was compacted, retained, tombstoned, or redacted), it carries honest projections
of two registries so the claimed M5 shared editor replica view, presence / cursor layer, comment / annotation /
review-pin layer, collaboration degradation banner, support / export packets, and help / docs surfaces inherit one
canonical headless-inspect descriptor and one support-bundle export disposition per collaboration-state object
rather than a hand-authored parallel prose that has to be kept consistent. It closes the gap the source set now
expects between collaboration state truth (shared-object authority, convergence, anchor drift, compaction,
degraded-session recovery) and the headless / support parity rules that keep that truth visible outside the UI:
whenever a headless or support surface inspects a collaboration-state object — a collaboration replica descriptor,
a shared-object record, an anchor-history entry, a degraded-session state, a compaction manifest, or an
archive-finalization descriptor — the object class, the state class it carries, the convergence / drift / archive
facts, the actor / source labels, and the authority model those facts imply are projected first, and a CLI /
headless view carries the same vocabulary the UI shows rather than a reduced UI-only summary.

## Registry-A — headless-inspect descriptor

One durable, append-only headless-inspect descriptor per collaboration-state object a CLI / headless surface can
inspect, carrying:

- the object class (collaboration replica descriptor, shared-object record, anchor-history entry, degraded-session
  state, compaction manifest, or archive-finalization descriptor), kept mechanically distinct so a
  convergence-degraded, awareness-degraded, anchor-unresolved, or compaction-lineage class never collapses into a
  generic stale or broken badge;
- the state class it carries and the convergence / drift / archive facts behind it;
- the authority model those facts imply and the actor / source labels the desktop UI shows;
- the actor provenance of the inspected state;
- that the CLI / headless projection carries the same state vocabulary the UI shows, never a reduced UI-only view;
- the support-bundle export disposition offered on it (include-in-support-bundle, headless-inspect-only,
  redact-and-export, block-export, defer);
- the resolution-form coverage (canonical object, accessible summary, audit record).

The object class, state class, and actor / source labels are projected before any export or support-bundle action
continues. A descriptor that would present a headless or support projection that drops the state vocabulary the UI
shows, that is a hand-copied per-object assumption instead of tracing to the shared registry, that would emit a
collaboration-state fact without its actor / source labels, or that collapses a distinct convergence-degraded /
awareness-degraded / anchor-unresolved / compaction-lineage class into a generic stale badge degrades honestly
instead of hiding that convergence, drift, or archive truth is UI-only. The registry reuses the matrix
`m5-collaboration-convergence-state.schema.json` domain schema.

## Registry-B — support-bundle export disposition

The typed support-bundle export disposition naming the choice offered before a collaboration-state fact is emitted
into a CLI export or support bundle — include-in-support-bundle, headless-inspect-only, redact-and-export,
block-export, or defer — plus its actor / time provenance, its compaction / redaction facts, and its policy-labeled
redaction lineage. The descriptor keeps the state dimensions distinct rather than collapsing a convergence,
awareness, anchor-drift, or compaction fact into one generic exported lane, never exports an op-log, snapshot, or
archive without a policy-labeled redaction and actor lineage, never acts without actor / time provenance, and never
drops the redaction lineage to take a disposition. The registry reuses the matrix
`m5-session-compaction-manifest.schema.json` sealed-session-archive domain schema.

## Acceptance criteria proven by the resolved examples

1. Headless and support paths can inspect the same collaboration-state vocabulary the desktop UI shows: the object
   class (collaboration replica descriptor, shared-object record, anchor-history entry, degraded-session state,
   compaction manifest, archive-finalization descriptor), the state class it carries, the convergence / drift /
   archive facts, the actor / source labels, and the export disposition (include-in-support-bundle,
   headless-inspect-only, redact-and-export, block-export, defer) stay visible in the CLI / headless projection, the
   CSV / export, and the support packet instead of collapsing into a UI-only summary.
2. Collaboration convergence, drift, and archive facts are no longer UI-only knowledge: a descriptor that would
   present a headless or support projection stripped of the state vocabulary the UI shows, or a disposition that
   would export an op-log, snapshot, or archive without a policy-labeled redaction and actor lineage, degrades
   instead of reading as a clean, safe object, so a collaboration-state fact is never silently reduced to UI-only
   knowledge.
3. The parity-first order is preserved (state class and actor / source labels first, then export / support bundle),
   a disposition never acts without actor / time provenance, and no disposition drops the redaction lineage; the
   registries keep each object class and export disposition distinct.

Raw secrets, raw command text, variable bodies, clipboard contents, and private endpoints never cross
this boundary. The Rust validator in `crates/aureline-ui` is the authoritative gate; the checked-in
combined registries schema
(`schemas/collaboration/m5-collaboration-state-headless-inspect-and-support-bundle-parity-registries.schema.json`)
documents the shape.

[matrix]: ../../crates/aureline-ui/src/m5_collaboration_replica_shared_object_authority_anchor_drift_convergence_and_session_archive_matrix/mod.rs
