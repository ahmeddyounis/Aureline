# ADR 0005 — Shared subscription envelope and invalidation semantics

- **Decision id:** D-0005 (see `artifacts/governance/decision_index.yaml#D-0005`)
- **Status:** Accepted
- **Decision date:** 2026-04-19
- **Freeze deadline:** 2026-07-31
- **Owner:** `@ahmedyounis`
- **Backup owner:** `null` (covered by waiver `single-maintainer-backup` in `artifacts/governance/ownership_matrix.yaml#waivers`)
- **Forum:** architecture_council
- **Related requirement ids:** none

## Context

Every reactive consumer inside Aureline — the shell (file tree,
breadcrumb, tabs, status, activity center), the editor, the search
and symbol panes, the graph / review / AI sidebars, the diagnostics
and problem surfaces, the CLI mirrors, the browser companion, the
support-bundle exporter, and the eventual timeline / replay lane —
observes the same underlying truth through the same subscription
fabric. If each surface invents a private cache, a private freshness
label, or a private "this data may be stale" convention, the product
cannot truthfully claim that the editor, the breadcrumb, the file
tree, and the CLI agree on what the workspace currently contains. The
ownership matrix's reactive-state lane exists to prevent exactly that
drift.

The freeze matters because later work cannot land honestly on top of
an unfrozen envelope: the VFS lane cannot claim that external-change
merges are observable on every surface without a shared envelope; the
editor cannot promise that a buffer snapshot plus its delta stream
is causally consistent across the review pane, the AI sidebar, and
the support export without a shared sequence; the search lane cannot
label "partial" or "warming" without a shared completeness
vocabulary; the review lane cannot distinguish a live provider view
from an imported review bundle without a shared authority-versus-
derived rule; and the eventual diagnostics / timeline / replay work
cannot reason about "what the user saw at 14:07 local" without the
same vocabulary for `stale`, `replayed`, and `imported`.

This ADR closes `D-0005` (shared subscription envelope for reactive
truth) ahead of its `2026-07-31` freeze so shell, VFS, editor,
search, graph, review, AI, CLI, and support-export surfaces can
start instrumenting against one subscription contract. It is scoped
to the **M0 seed contract** — the fields, states, and semantics
that every reactive surface MUST agree on today. A full event-replay
engine, a timeline scrub UI, and a collaboration-grade CRDT merge
are out of scope until separate decision rows open; this ADR freezes
only the vocabulary those later lanes will reuse.

The subscription envelope rides the event-stream envelope frozen in
ADR 0004 (`docs/adr/0004-rpc-transport-and-schema-toolchain.md`) and
Appendix DB of the TAD. This ADR does not redefine transport,
cancellation, trace, deadline, workspace scope, or idempotency; it
defines the reactive-truth fields that sit inside the event
payload and the lifecycle rules that govern the subscription as a
first-class object.

## Decision

Aureline freezes a single **subscription envelope** (a typed header
every reactive event carries), one **subscription-lifecycle
contract** (subscribe, snapshot, delta, resync, terminate), one set
of **freshness and completeness labels**, one **authoritative-
versus-derived classification**, one **materialized-view-class**
taxonomy, one set of **stale-reason codes**, and a set of
**protected-hot-path hooks** that the benchmark lab, the support-
bundle exporter, and the eventual timeline / replay lane instrument
against.

All are stated in terms of contract, vocabulary, and hook names
rather than specific crates so dependency refresh is a hygiene
change, not a re-litigation.

### Subscription envelope fields

Every reactive event the transport carries is an ADR-0004
`event_envelope` whose typed `payload` is the subscription frame
described here. Field names are the frozen vocabulary across every
lane. No lane MAY invent parallel names for the same field.

| Field                 | Presence | Purpose                                                                                                     |
|-----------------------|----------|-------------------------------------------------------------------------------------------------------------|
| `subscription_schema_version` | required | Integer. Current value `1`. Bumped only on breaking payload changes; additive-optional fields do not bump. |
| `subscription_id`     | required | Copied from the ADR-0004 event envelope; stable handle for audit, diagnostics, and cancellation.           |
| `query_family`        | required | Canonical id of the query contract or view family (e.g. `vfs.tree`, `editor.buffer_snapshot`, `search.results`, `review.overlay`). Resolved against the method manifest. |
| `scope_ref`           | required | Typed scope (`workspace`, `window`, `review_workspace`, `remote_session`, `tenant`, or `companion_surface`) plus a stable id within that class. Unscoped ambient subscriptions are forbidden. |
| `authority_class`     | required | One of `workspace_vfs`, `buffer_editor`, `derived_knowledge`, `execution`, `policy_entitlement`, `provider_overlay`. Distinguishes authoritative truth from derived / overlay truth. |
| `derivation_class`    | required | `authoritative` (the row is the canonical owner's own state) or `derived` (the row is a projection produced by a named producer from named inputs). |
| `snapshot_epoch`      | required | Monotonic u64 per `(query_family, scope_ref)` that identifies the snapshot lineage. Changes only when a snapshot is republished. |
| `delta_seq`           | required | Monotonic u64 within the current `snapshot_epoch`. Starts at `0` on the snapshot frame; consumers gap-detect on this field. |
| `frame_class`         | required | One of `snapshot`, `delta`, `resync_required`, `terminal`. Determines how the consumer applies the payload. |
| `freshness`           | required | One of `authoritative`, `warming`, `cached`, `stale`, `replayed`, `imported`. Describes the row's freshness posture at emit time. |
| `completeness`        | required | One of `full`, `partial`, `unloaded`, `unavailable`. Describes whether the producer believes the current scope's results are complete. |
| `backpressure_mode`   | required | One of `realtime`, `coalesced`, `snapshot_required`. Frozen at subscribe time; switching modes forces a `resync_required` frame. |
| `producer_refs`       | required | One or more authority or producer references with producer id + producer instance + input-digest set. Supports provenance and support-bundle attribution. |
| `invalidation`        | conditional | Present whenever `frame_class = resync_required` or whenever `freshness != authoritative`. Carries a typed `stale_reason` (see below) and an optional `caused_by` event or epoch reference. |
| `view_class`          | required | One of `ephemeral_projection`, `durable_local_materialization`, `exportable_snapshot`, `managed_replicated_view`. Describes how the consumer is allowed to persist and replay the frame. |
| `payload`             | required | Typed, query-family-specific body. Opaque to this envelope; described in the method manifest. |

The envelope intentionally overlaps ADR-0004's event-stream envelope
only on fields that are already transport-level (`subscription_id`,
`delivery_mode` via the enclosing event envelope, `producer`). The
fields above are reactive-truth concerns and live inside the event
payload; they do not duplicate transport state.

### Subscription lifecycle

A subscription is a first-class object with a frozen state machine.
No lane MAY invent an alternative lifecycle.

1. **Subscribe.** The consumer issues a unary RPC under ADR 0004
   carrying `query_family`, `scope_ref`, the declared
   `backpressure_mode`, and a declared maximum acceptable staleness
   (observability only; it does not widen producer guarantees). The
   producer allocates a `subscription_id`, records the requested
   `delivery_mode` (usually `at_least_once`), and responds with the
   current `snapshot_epoch`. The subscription is considered
   established when the first frame is emitted, not on ack alone.
2. **Snapshot frame.** The first frame on the wire is a `frame_class
   = snapshot` whose `delta_seq = 0`. It carries the full materialized
   state of the scope at `snapshot_epoch`. Consumers MUST replace
   their local projection with the snapshot; they MUST NOT apply a
   later delta against a stale snapshot.
3. **Delta frames.** Subsequent frames are `frame_class = delta`
   with monotonically increasing `delta_seq` inside the current
   `snapshot_epoch`. Consumers apply deltas in order. A gap in
   `delta_seq` is a protocol error and triggers `resync_required`
   (see below).
4. **Resync-required frame.** When the producer cannot honour
   causal continuity (queue overflow, producer restart,
   authority-epoch rollover, policy invalidation, watcher loss,
   imported-snapshot handoff), it emits a `frame_class =
   resync_required` frame with a typed `stale_reason`. Consumers
   MUST mark their projection `stale` with the reason code and MUST
   NOT accept further deltas on the old `snapshot_epoch`. The
   producer follows with a fresh `snapshot` frame carrying a new
   `snapshot_epoch`.
5. **Terminal frame.** When the subscription ends cleanly (producer
   shutting down, scope removed, consumer-initiated cancel), the
   producer emits a `frame_class = terminal` with the final
   `delta_seq` and a typed terminal reason. No further frames may
   be observed on the `subscription_id`.
6. **Cancellation.** The consumer may cancel at any time through the
   ADR-0004 cancel frame. A cancel is idempotent; the producer
   emits at most one `terminal` frame in response.

Missing or out-of-order frames never silently drop: the consumer
observes `resync_required` with a stale_reason that attributes the
loss.

### Authoritative versus derived state

Every subscription declares `authority_class` and `derivation_class`.
Consumers surface the distinction.

- **Authoritative (`derivation_class = authoritative`).** The
  producing service is the canonical owner named in the authority
  matrix (ADR-backed, Appendix DB of the TAD). Authoritative frames
  are the only rows that mutating commands compare against before
  offering a "safe" affordance. Workspace / VFS, buffer / editor,
  execution, and policy / entitlement surfaces emit authoritative
  frames for their own entities.
- **Derived (`derivation_class = derived`).** The producing service
  projects or recomputes state from one or more named inputs
  (search indexes, graph neighbourhoods, language diagnostics, AI
  summaries, provider overlays, imported indexes). Derived frames
  MUST carry `producer_refs` that name every input digest, the
  producer version, and the derivation epoch. A derived frame whose
  inputs are stale is itself labelled `stale` with a stale reason
  that names the upstream cause.

A derived frame MAY NOT claim `freshness = authoritative`. A
mutating action that requires exact truth either re-consults the
authoritative producer or refuses to offer itself with an explicit
"data is derived; refresh to mutate" affordance.

### Freshness labels

`freshness` is frozen at six values. Consumers surface the label in
view chrome; support bundles quote the label verbatim.

| Value          | Meaning                                                                                                     |
|----------------|-------------------------------------------------------------------------------------------------------------|
| `authoritative`| The frame reflects the producer's current, fully populated, causally contiguous view. Only permitted on `derivation_class = authoritative` producers, or on a derived producer whose inputs are all `authoritative` and whose derivation epoch is current. |
| `warming`      | The producer is starting up, warming a cache, or running a first-time compute. The payload may be empty or partial; `completeness` will report what is loaded. |
| `cached`       | The frame is served from a local or mirror cache; the producer has not revalidated against its authority since a named checkpoint. Cached content is usable but flagged until revalidation. |
| `stale`        | The producer knows a freshness contract was lost (watcher gap, index drift, policy epoch change, upstream producer restart). Consumers MUST NOT present stale derived actions as exact truth. |
| `replayed`     | The frame was emitted from a captured event log, support bundle, or timeline scrub rather than from a live producer. Replayed frames are immutable and do not advance the live `snapshot_epoch`. |
| `imported`     | The frame originates from an external, non-authoritative source (LSIF-style precomputed index, imported VS Code workspace metadata, external review bundle). Imported frames never claim authority; consumers surface the source. |

The envelope carries exactly one value; a producer that cannot
choose between two (for example, a cache that is partially warmed
against stale inputs) MUST emit multiple frames with different
`freshness` labels per scope slice rather than collapse them.

### Completeness labels

`completeness` is frozen at four values. It is orthogonal to
`freshness`: a frame can be `authoritative` and `partial` at the
same time (the producer is authoritative for what it reports but
has not yet loaded the rest of the scope).

| Value         | Meaning                                                                                                      |
|---------------|--------------------------------------------------------------------------------------------------------------|
| `full`        | The producer believes every in-scope entity is represented in the snapshot lineage.                         |
| `partial`     | A declared, bounded subset of the scope is represented; the producer will emit follow-up frames (delta or snapshot) to widen coverage. |
| `unloaded`    | The producer has declined or deferred loading the scope; consumers MAY request load on demand.              |
| `unavailable` | The producer cannot serve the scope (watcher denied, provider offline, policy blocked). Consumers treat the subscription as terminal-in-unavailable until resubscribed. |

### Stale-reason vocabulary

`invalidation.stale_reason` is frozen at the values below. Every
`frame_class = resync_required` frame, and every frame carrying
`freshness` in `{cached, stale, replayed, imported}`, MUST populate
a reason.

| Code                        | Meaning                                                                                                     |
|-----------------------------|-------------------------------------------------------------------------------------------------------------|
| `producer_restart`          | The producer process or actor restarted; sequence continuity was lost.                                      |
| `authority_epoch_rolled`    | The canonical owner advanced its authority epoch (workspace identity change, trust change, tenant change).  |
| `policy_epoch_changed`      | A policy / entitlement snapshot changed; affordances and scope may have narrowed or widened.                |
| `watcher_dropped`           | A filesystem, provider, or network watcher reported loss of fidelity; fresh snapshot required.              |
| `queue_saturation`          | The transport or consumer queue saturated and the producer could not guarantee causal delivery.             |
| `upstream_input_stale`      | A derived producer's input digest no longer matches the source; the derivation is no longer current.        |
| `explicit_refresh_requested`| A consumer or admin asked for a re-snapshot (manual refresh, post-migration reload, repair probe).          |
| `cache_served`              | The frame was served from cache by design, without revalidation; companion to `freshness = cached`.         |
| `replayed_from_bundle`      | The frame was replayed from a captured event log or support bundle; companion to `freshness = replayed`.    |
| `imported_from_external`    | The frame was imported from a non-authoritative source; companion to `freshness = imported`.                |
| `scope_removed`             | The subscription's scope was deleted (workspace closed, review workspace discarded). Terminal.              |
| `causality_lost`            | The producer cannot prove causal continuity and could not attribute the cause to a more specific code.     |

A new stale reason requires opening a new decision row and extending
this taxonomy. Adding a code is an additive-minor change;
repurposing a code is a breaking change.

### Resubscribe and full-refresh behaviour

When causality is lost, the consumer's obligations are frozen. No
lane MAY substitute a silent behaviour.

1. The consumer observes `frame_class = resync_required` (or, at
   worst, a `delta_seq` gap on a subscription whose
   `delivery_mode = exactly_once` or whose `delivery_mode =
   at_least_once` does not carry a deduping key).
2. The consumer marks its local projection as `stale` with the
   carried `stale_reason`. It MUST NOT offer mutating affordances
   that depend on exact truth while the projection is `stale`.
3. The consumer requests a fresh `snapshot_epoch` by continuing to
   read on the existing subscription (the producer will emit a new
   `snapshot` frame) or, when the subscription was terminated, by
   opening a fresh subscribe call against the same
   `(query_family, scope_ref)`.
4. The consumer discards any in-flight deltas on the old epoch; it
   MAY retain the old snapshot locally for diagnostics or timeline
   scrub, but it MAY NOT present it as live.

`delivery_mode = at_least_once` producers MUST publish with an
ADR-0004 `idempotency_key` per logical change so consumers can
dedupe without relying on `delta_seq` alone.

### Materialized-view classes

Every surface that caches or persists a subscription declares a
`view_class`. The class is the same vocabulary Appendix DB uses.
Consumers MAY NOT promote a frame to a stronger class than the
producer advertised.

| View class                     | Persistence                | Canonical example                                         | Invalidation expectations                                                                                   |
|--------------------------------|----------------------------|-----------------------------------------------------------|-------------------------------------------------------------------------------------------------------------|
| `ephemeral_projection`         | memory only                | breadcrumb path, hover badge, quick status                | focus / scope change, producer delta, view closure; no persistence allowed.                                 |
| `durable_local_materialization`| local cache or DB          | file-tree snapshot, problem-list cache, search shard      | producer epoch change, policy / trust change, explicit clear / rebuild. Must be rebuildable from authority. |
| `exportable_snapshot`          | saved artifact             | support bundle, review bundle, incident export            | never updated in place; replaced by a new export artifact. Carries its own `snapshot_epoch`.                |
| `managed_replicated_view`      | optional service / mirror  | companion review list, usage summary, notification inbox  | provider refresh, entitlement / policy epoch change, reconnect reconcile. Never silently overwrites local.  |

### Minimal ordering, replay, and imported-history assumptions

M0 only promises the ordering and replay floor that diagnostics,
timeline, and support-export lanes will extend later.

- **Per-subscription monotonic order.** `delta_seq` is strictly
  monotonic within a `snapshot_epoch`; across epochs, consumers
  observe the epoch boundary and restart from `delta_seq = 0`.
- **No cross-subscription global order.** Two subscriptions do not
  have a shared ordering; consumers that need cross-cutting order
  reconcile through the ADR-0004 trace context or through the
  mutation-journal row that caused the change.
- **Replay boundaries are explicit.** `freshness = replayed` frames
  never advance a live `snapshot_epoch` and never forward into
  producer state. Replay tooling is responsible for declaring the
  replay window; the envelope does not enable open-ended rewrite.
- **Imported history is isolated.** `freshness = imported` frames
  carry a source identifier in `producer_refs` and are never merged
  into authoritative state; they may be projected alongside for
  display and for AI / search recall only.
- **Support-bundle export is a snapshot operation.** The exporter
  captures one `exportable_snapshot` per subscription it records;
  it does not capture unbounded delta streams at M0.
- **Compaction is producer-local.** Producers MAY compact their
  internal delta history; consumers observe compaction only via
  `resync_required` with `stale_reason = causality_lost` or a
  producer-specific reason, never as silent delta renumbering.

### Protected-hot-path hooks

The reactive fabric exposes the following named hooks. They are
the canonical instrumentation surface for the shell, VFS, editor,
search, graph, review, AI, CLI, support-export, and benchmark
lanes. No lane MAY invent alternative names for the same
measurement. These hooks sit above the ADR-0004 `event_stream_*`
hooks and do not replace them.

| Hook id                                 | Fires when                                                                                               | Protected hot-path budget |
|-----------------------------------------|----------------------------------------------------------------------------------------------------------|---------------------------|
| `subscription_subscribe`                | A consumer's subscribe call is accepted by the producer and a `subscription_id` is allocated             | yes                       |
| `subscription_snapshot_emit`            | The producer writes a `frame_class = snapshot` frame                                                     | yes                       |
| `subscription_delta_emit`               | The producer writes a `frame_class = delta` frame                                                        | yes                       |
| `subscription_delta_apply`              | A consumer applies a decoded delta to its local projection                                               | yes                       |
| `subscription_resync_required_emit`     | The producer writes a `frame_class = resync_required` frame                                              | yes                       |
| `subscription_freshness_downgrade`      | A consumer observes a `freshness` transition toward less-authoritative (e.g. `authoritative -> stale`)   | no (observability only)   |
| `subscription_completeness_changed`     | A consumer observes a `completeness` transition                                                          | no (observability only)   |
| `subscription_backpressure_coalesce`    | A producer coalesces deltas to respect a consumer's declared `backpressure_mode`                         | yes                       |
| `subscription_snapshot_required_switch` | A consumer requests `backpressure_mode = snapshot_required` after falling behind                         | yes                       |
| `subscription_terminate`                | The producer writes a `frame_class = terminal` frame                                                     | yes                       |
| `subscription_imported_attach`          | An imported-history frame is attached to an active projection                                            | no (observability only)   |
| `subscription_replay_begin`             | A replay session starts emitting `freshness = replayed` frames against a consumer                        | no (observability only)   |
| `subscription_replay_end`               | A replay session ends and live frames resume                                                             | no (observability only)   |

The benchmark lab reports every hot-path hook against its
protected budget on the claimed corpora (in-process, same-host
IPC, remote proxy) alongside the ADR-0004 transport hooks. Non-
hot-path hooks are observability-only and do not gate release.

### Non-goals at this decision

Out of scope until a superseding decision row opens:

- A full event-replay engine, a timeline scrub UI, or a causal-
  undo graph across subscriptions. M0 only promises the
  vocabulary later lanes will reuse.
- A collaboration-grade CRDT merge across subscriptions. CRDT
  delivery, if needed, opens a new decision row; the `exactly_once`
  / `at_least_once` / `best_effort` posture inherited from ADR
  0004 is the floor.
- A cross-subscription global ordering; consumers reconcile via
  trace context or mutation-journal correlation.
- Public-SDK stability of the subscription envelope. The envelope
  is internal at M0; public-SDK surfaces land behind a separate
  decision row.
- An external IDL or code generator for the subscription payload.
  The Rust types in the eventual reactive-state crate are the
  schema of record; the JSON Schema at
  `schemas/runtime/subscription_envelope.schema.json` is the
  boundary export, consistent with ADR 0004's posture.
- Any implicit per-surface freshness heuristic ("it looked fine so
  we labelled it authoritative"). Surfaces quote the producer's
  label; they do not invent one.

These lines move only by opening a new decision row, not by
editing this ADR.

### Tradeoff table

The structured tradeoff rows live in
`artifacts/architecture/subscription_tradeoff_rows.yaml`. Headline
summary:

| Axis                                | Chosen stack                                                                                                      | Best rejected alternative                                               | Why chosen wins                                                                                       |
|-------------------------------------|-------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------|
| **One envelope across surfaces**    | Single typed envelope rides ADR-0004 event streams; same fields on shell, CLI, support bundle, replay            | Per-surface envelopes (file tree vs. search vs. review all different)   | Per-surface envelopes make cross-surface truth claims unprovable; one vocabulary is the floor        |
| **Causal continuity vs. throughput**| Snapshot + monotonic `delta_seq` + explicit `resync_required` with typed stale reason                             | Best-effort pub/sub with silent gaps                                    | Silent gaps hide stale derived actions; explicit resync is what lets the UI label truth honestly     |
| **Authoritative vs. derived**       | Explicit `authority_class` + `derivation_class`; derived frames carry `producer_refs` with input digests          | Single "freshness score" across authority and derivation                | A single score collapses provenance; mutating affordances need to know what is authoritative        |
| **Freshness / completeness labels** | Six freshness values, four completeness values, orthogonal; stale-reason taxonomy                                | Binary "fresh/stale"                                                    | Binary labels cannot express warming, cached, replayed, imported; support bundles cannot quote them |
| **Materialized-view classes**       | Four view classes with declared persistence and invalidation rules                                                | Ad-hoc persistence per surface                                          | Ad-hoc persistence drifts across surfaces; exports and replay need a shared class vocabulary        |
| **Replay and import posture**       | `freshness = replayed` and `imported` are isolated; never advance live `snapshot_epoch`                           | Merge replayed / imported frames into live producer state                | Merging collapses provenance; users cannot tell what was live vs. reconstructed                      |
| **Schema of record**                | Rust-native types + JSON Schema boundary export, mirroring ADR 0004                                               | External IDL + code generator at M0                                      | No second-language consumer yet; the boundary schema reserves a clean integration point             |

Each row carries reopen triggers in the YAML. A benchmark finding
that `subscription_snapshot_emit` exceeds its budget on a claimed
corpus, or a support-bundle export that cannot reconstruct a
surface's state from the labelled frames, reopens the relevant
row.

### Decision-example fixtures

A small corpus of decision-example fixtures lives under
`fixtures/runtime/subscription_examples/`. They are short,
reviewable scenarios (ready, partial, stale, replayed, failed,
imported) used by shell, VFS, editor, search, and review
prototypes and by the support-export lane to anchor the hook
names, the envelope fields, and the freshness / stale-reason
vocabulary to concrete inputs and observable outcomes. They are
not a test suite; they are the language the ADR's hook list and
taxonomy refer to.

## Consequences

- **Frozen:** the subscription envelope fields, the subscription
  lifecycle (subscribe / snapshot / delta / resync_required /
  terminal), the freshness vocabulary (`authoritative`, `warming`,
  `cached`, `stale`, `replayed`, `imported`), the completeness
  vocabulary (`full`, `partial`, `unloaded`, `unavailable`), the
  stale-reason taxonomy, the authority / derivation classification,
  the materialized-view class list, and the protected-hot-path hook
  names.
- **Frozen:** the schema of record is Rust types in the eventual
  reactive-state crate; the boundary schema for the envelope lives
  under `schemas/runtime/subscription_envelope.schema.json`; there
  is no external IDL or codegen toolchain at M0. This mirrors ADR
  0004's posture.
- **Frozen:** derived frames never claim `freshness =
  authoritative`; consumers never promote a derived projection to
  authoritative by local policy.
- **Permitted:** adding a new `query_family` is an additive-minor
  change if it does not change the envelope shape; it is recorded
  in the method manifest.
- **Permitted:** adding a new `stale_reason` code is an additive-
  minor change; repurposing a code is breaking and requires a new
  decision row.
- **Permitted:** producers MAY compact internal delta history and
  serve `cache_served` frames; consumers interpret the `freshness`
  and `stale_reason` rather than inferring compaction.
- **Follow-up:** the shell, VFS, editor, search, graph, review,
  AI, CLI, and support-export lanes instrument every hot-path hook
  before claiming freshness or completeness labels. The benchmark
  lab stabilises traces against the hooks on claimed corpora.
- **Follow-up:** the eventual diagnostics / timeline / replay lane
  (a later decision row) extends the `replayed` branch of this
  envelope with capture-window rules; it does not invent a second
  envelope.
- **Follow-up:** the eventual public-SDK surface (a separate
  decision row) will promote a subset of the envelope with its
  own stability posture; this ADR does not make the internal
  envelope a public-compatibility surface.
- **Ratifies:** the envelope vocabulary becomes the vocabulary
  used by the support-export lane, the CLI JSON surfaces that
  mirror reactive state, and the trace viewers that cite a
  subscription frame. The freshness and stale-reason codes are
  the same vocabulary the Appendix DB authority matrix refers to.

## Alternatives considered

- **Per-surface envelopes.** Let the file tree, editor, search,
  review, AI, and CLI each define their own subscription shape.
  Rejected: the product's truth claim ("every visible surface
  agrees on current workspace truth") cannot be instrumented or
  enforced when each surface speaks a private freshness
  vocabulary. Support bundles and timeline replay would need a
  translation layer per surface.
- **Binary `fresh` / `stale` labelling.** Collapse warming,
  cached, replayed, and imported into one "stale" bucket.
  Rejected: users and support cannot distinguish "the index is
  still warming" from "the producer crashed" from "this is an
  imported LSIF index"; mutating affordances cannot choose a
  safe posture with only two values.
- **Single "freshness score" combining authority and derivation.**
  Map everything to a number. Rejected: it collapses provenance;
  mutating actions cannot decide whether to re-consult the
  authority because the score does not separate "I am the
  authority" from "I am derived from a stale input".
- **Best-effort pub/sub with silent gaps.** No `delta_seq`, no
  `resync_required`, rely on consumers to poll. Rejected:
  consumers would silently present stale derived affordances as
  exact truth; the benchmark lab could not instrument causal
  continuity; the support bundle could not reconstruct what the
  user saw.
- **Merge replayed and imported frames into live producer state.**
  Present a replay or an imported index as if it were authority.
  Rejected: it destroys provenance for AI and review, collides
  with policy epochs, and forecloses a future replay engine.
- **Full CRDT-based delivery at M0.** Ship a CRDT-backed
  subscription fabric as the floor. Rejected: the operator and
  reviewer cost is incompatible with the solo-maintainer posture,
  and the at-least-once + idempotency-key floor already supports
  the M0 lanes; CRDT delivery opens a new decision row when
  collaboration forces it.
- **External IDL + generator for the subscription payload.**
  Adopt a separate IDL. Rejected: the same argument ADR 0004
  makes — an IDL without a second-language consumer costs more
  than it buys; the JSON Schema export reserves the integration
  point.
- **Defer to a later milestone.** Leave `D-0005` open and let the
  `freeze_lane` default apply on `2026-07-31`. Rejected: the
  default posture would block any shell / VFS / editor / search
  / review work that exports a materialized view across the RPC
  boundary during the months when those lanes most need a shared
  envelope; the hook vocabulary and the support-bundle capture
  rules would each land with incompatible assumptions the lane
  would then have to reconcile.

The `D-0005` `freeze_lane` default-if-unresolved posture would
have blocked every lane from exporting a materialized view across
the RPC boundary until an ADR landed. Accepting this ADR replaces
that freeze with the frozen envelope, lifecycle, freshness /
completeness / stale-reason vocabularies, view-class list, and
hook list above; the `freeze_lane` default does not apply.

## Source anchors

- `.t2/docs/Aureline_Technical_Architecture_Document.md:44` —
  TOC: "12.3.1 Reactive state, subscription, and materialized-view
  architecture".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:201` —
  TOC: "Appendix DB — Reactive State, Subscription, and
  Materialized-View Matrix".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1674` —
  "Reactive state, subscription, and materialized-view
  architecture".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1676` —
  "ARCH-STATE-012 closes the gap between having many well-specified
  subsystems and having one truthful running product".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1688` —
  "every cross-surface consumer uses one typed subscription
  contract: query family + scope + snapshot epoch + delta sequence
  + freshness/completeness metadata + backpressure mode".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1691` —
  "materialized views are classified as ephemeral projection,
  durable local materialization, exportable snapshot, or managed
  replicated view".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1694` —
  "every view that renders derived truth must be able to explain
  where the data came from, how fresh it is, whether scope is
  partial, and what event or invalidation caused it to change".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1695` —
  "support and test tooling must be able to capture active
  subscriptions, slow-view backpressure state, stale
  materializations, and invalidation histories".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1697` —
  "Appendix DB defines the authority classes, subscription
  envelope, invalidation states, and materialized-view policies".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:5695` —
  "ARCH-STATE-012 — one truthful client-side state model".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:5762` —
  "ARCH-DATA-004 — replayable and attributable distributed state".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:5878` —
  "Every visible surface agrees on current workspace truth".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:7809` —
  "Appendix AF — Event Ordering, Consistency, and Replay Model".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:7820` —
  "Replay rules — stable event handlers must tolerate duplicate
  delivery and unknown additive fields".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:9632` —
  "Appendix DB — Reactive State, Subscription, and
  Materialized-View Matrix".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:9645` —
  "DB.2 Subscription envelope minimum".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:9660` —
  "DB.3 Materialized-view and invalidation rules".
- `.t2/docs/Aureline_Technical_Design_Document.md:1614` —
  "7.2.12 Reactive state, subscriptions, and materialized views".
- `.t2/docs/Aureline_Technical_Design_Document.md:1629` —
  "one typed subscription contract: query family + scope +
  snapshot epoch + delta sequence + freshness/completeness
  metadata + backpressure mode".
- `.t2/docs/Aureline_Technical_Design_Document.md:1635` —
  "every view that renders derived truth must be able to explain
  where the data came from, how fresh it is, whether scope is
  partial, and what event or invalidation caused it to change".

## Linked artifacts

- Decision register row: `artifacts/governance/decision_index.yaml#D-0005`
- RFC: none.
- Tradeoff register (machine form):
  `artifacts/architecture/subscription_tradeoff_rows.yaml`.
- Envelope schema (machine form):
  `schemas/runtime/subscription_envelope.schema.json`.
- Decision-example fixtures:
  `fixtures/runtime/subscription_examples/`.
- Transport contract this envelope rides:
  `docs/adr/0004-rpc-transport-and-schema-toolchain.md`.
- Affected lanes: `crates/aureline-rpc`, `crates/aureline-buffer`,
  `crates/aureline-vfs`, `crates/aureline-telemetry`,
  `crates/aureline-shell-spike`,
  `artifacts/governance/ownership_matrix.yaml#scorecard_lane_index:shell_command_system`,
  `artifacts/governance/ownership_matrix.yaml#scorecard_lane_index:benchmark_lab`,
  `artifacts/governance/ownership_matrix.yaml#scorecard_lane_index:support_export`.

## Supersession history

First acceptance. No supersession.
