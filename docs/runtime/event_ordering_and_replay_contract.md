# Event ordering, causality, idempotency, and replay contract

This document freezes the shared contract Aureline uses to represent
**when an event happened**, **how it orders against other events**, **how
duplicates are deduped**, and **how replay is performed without inventing
new ordering fields**. It exists so logs, automation, diagnostics,
support exports, and recovery flows can stitch the same journey
together while being honest about clock skew, out-of-order arrival, and
unknown ordering.

The document is normative. If it disagrees with the PRD, Technical
Architecture Document, Technical Design Document, UI / UX Spec, or Design
System Style Guide, those source documents win and this document plus
the schemas update in the same change.

## Companion artifacts

- [`/schemas/runtime/event_envelope.schema.json`](../../schemas/runtime/event_envelope.schema.json)
  — boundary schema for the canonical event envelope and the worked-case
  fixture record.
- [`/artifacts/runtime/replay_rules.yaml`](../../artifacts/runtime/replay_rules.yaml)
  — machine-readable vocabulary and rules for ordering, dedupe, and replay
  posture. Consumers cite this artifact rather than re-stating rule text.
- [`/fixtures/runtime/event_ordering_cases/`](../../fixtures/runtime/event_ordering_cases/)
  — worked scenarios covering duplicate delivery, skewed clock, replay
  after crash, out-of-order arrival, and cross-surface reconstruction.

## Upstream contracts this contract rides on

This contract does not mint competing time, authority, or provider-callback
vocabulary. It composes with the existing contracts by name:

- [`/docs/governance/time_semantics.md`](../governance/time_semantics.md)
  and the timestamp-envelope schema — the canonical time model, clock
  source vocabulary, skew labelling, partial-order labelling, and import
  window disclosure this contract reuses.
- [`/docs/runtime/authority_class_matrix.md`](./authority_class_matrix.md)
  — the authority-class vocabulary used to label which subsystem owns the
  truth for an event family.
- [`/docs/governance/runtime_authority_contract.md`](../governance/runtime_authority_contract.md)
  — actor/authority ticket semantics and “no ambient privilege” lineage
  expectations.
- [`/docs/providers/provider_mode_contract.md`](../providers/provider_mode_contract.md)
  and the provider callback envelope — provider dedupe keys and webhook
  replay posture that map into this contract’s idempotency and replay
  fields.
- [`/docs/runtime/connectivity_and_reconciliation_contract.md`](./connectivity_and_reconciliation_contract.md)
  — deferred intent/outbox idempotency, replay posture, and “no silent
  retry” expectations that this contract reuses for queued and drained
  intents.
- [`/docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`](../adr/0005-subscription-envelope-and-invalidation-semantics.md)
  — minimal replay/import assumptions (“replay boundaries are explicit”,
  “imported history is isolated”) that this contract extends to event
  envelopes outside reactive subscriptions.

## Why this contract exists

Without a shared ordering and replay model, each lane invents its own
sorting and dedupe strategy:

- logs sort by wall clock and silently mask skew;
- automation “replays” by retrying until it works, duplicating effects;
- diagnostics render a plausible order even when it is not provable; and
- support exports stitch multiple sources while dropping which parts were
  imported, replayed, or out-of-order.

The event envelope fixes this by requiring every event to answer a small,
closed set of questions in one place so downstream consumers do not invent
new ordering fields to compensate.

## Core distinctions (frozen)

### Causality vs. display sort

An event envelope records **causal truth** and **ordering truth** as
separate axes:

- **Causality** is expressed via `causal_parent_event_id` (and optional
  correlation refs). It is the “happened because of” relation.
- **Ordering** is expressed via `ordering.ordering_class` plus (when
  applicable) `ordering.monotonic_sequence` in a declared stream. It is
  the “can be strictly ordered against” relation.

UI sort order is a **projection** derived from these fields and the
timestamp envelope; it is not a truth source. When the contract says
ordering is unknown, consumers MUST render an explicit “unknown ordering”
state rather than silently sorting by wall clock.

### Event identity vs. dedupe identity

The envelope carries two distinct identifiers:

- `event_id` — unique id for this envelope instance (a single capture or
  delivery observation).
- `idempotency_key` — stable key that identifies the logical effect or
  logical observation for dedupe and replay. Duplicates share an
  `idempotency_key` but have distinct `event_id`s.

Provider callback dedupe keys and deferred-intent idempotency keys map
directly into `idempotency_key` for this contract.

### Ordering is scoped

This contract does not promise a single global total order across all
producers. Ordering truth is scoped by the envelope:

- Within a declared `ordering.stream_id_ref` and `ordering.stream_epoch_ref`,
  `ordering.monotonic_sequence` provides a total order for that stream.
- Across streams (or across epochs) ordering may only be partial (via
  causality), import-window-scoped, or unknown.

## Canonical event envelope field set

Every event envelope MUST carry:

- `event_id` — stable id for this envelope instance.
- `causal_parent_event_id` — nullable causal parent pointer.
- `actor.actor_subject_ref` + `actor.actor_class` — who this event is
  attributable to.
- `authority.authority_class` + `authority.authority_source_class` — which
  subsystem owns truth for the event family and whether this envelope is
  live, imported, replayed, derived, or synthetic.
- `observed_time` — time/clock disclosure block (clock source, sync state,
  skew posture, partial-order label, import origin) aligned to the
  timestamp-envelope semantics.
- `ordering.ordering_class` — whether the event is totally ordered in a
  stream, partially ordered, import-window ordered, or unknown.
- `ordering.monotonic_sequence` — required when the ordering class is total
  within a stream.
- `idempotency_key` — required (non-empty) when the event is replayable or
  when duplicates are admissible for the event kind.
- `replay.replay_safety_class` — whether replay is evidence-only, idempotent
  with a key, requires fresh authority, or forbidden.

Consumers MUST treat the field names above as canonical. Re-naming them in
another packet family is non-conforming.

## Rules

### Out-of-order arrival

1. An envelope MAY arrive out of order relative to other envelopes in the
   same stream.
2. When `ordering.ordering_class = total_order_stream`, consumers MUST sort
   by `(stream_id_ref, stream_epoch_ref, monotonic_sequence)` for that
   stream, not by arrival time.
3. When a consumer observes an out-of-order arrival, it MUST preserve the
   envelope as observed (distinct `event_id`) and mark its ingest decision
   using `ingest.ingest_disposition_class` rather than mutating or
   back-dating other envelopes.

### Duplicate delivery

1. Dedupe is performed on `idempotency_key` within the declared
   `dedupe_scope_ref` (see `/artifacts/runtime/replay_rules.yaml`).
2. A second envelope with the same dedupe scope and `idempotency_key` MUST
   NOT apply the same side effect twice. It MAY update freshness/receipt
   evidence only.
3. Duplicates MUST remain attributable and inspectable: the deduped envelope
   is not dropped silently.

### Clock skew and imported logs

1. Every envelope MUST declare its clock source and sync state in
   `observed_time`.
2. If `observed_time.clock_sync_state_class` is any unsynchronized variant,
   the envelope MUST carry a non-null
   `observed_time.partial_order_label_class` and consumers MUST NOT claim a
   strict wall-clock order across unrelated streams.
3. Imported logs MUST set `observed_time.import_origin_class` to a
   non-`not_imported` value and MUST declare an import window.

### Resumed jobs and replay after crash

1. A stream restart or resumption MUST mint a new `ordering.stream_epoch_ref`.
   A consumer MUST NOT treat `monotonic_sequence` values from different
   epochs as comparable without a causal link.
2. When a logical job resumes, the resumed events SHOULD reuse the same
   `idempotency_key` (logical job key) and link causality to the last durable
   checkpoint event.
3. Replay is explicit: envelopes replayed from a capture MUST set
   `authority.authority_source_class = replayed_from_capture` and MUST NOT
   be merged into live producer truth.

### Deferred intents

1. Events that represent “intent queued”, “intent drained”, “intent expired”,
   or “intent cancelled” MUST carry `links.deferred_intent_id_ref` (when
   available) and MUST use the same `idempotency_key` as the deferred intent.
2. Replaying deferred-intent envelopes is permitted only under
   `replay.replay_safety_class = replay_safe_idempotent` and only when the
   idempotency key is non-empty.

## Cross-surface timeline reconstruction

Timeline and support-export lanes reconstruct journeys by consuming the
event envelope fields above:

- `event_id` is the stable join key for “this happened” across logs, history
  rows, diagnostics, and exported bundles.
- `causal_parent_event_id` provides causal grouping even when ordering is
  partial.
- `ordering.*` provides stream-local strict order when available, and honest
  “unknown ordering” when it is not.
- `observed_time.*` provides clock and import-window disclosure so skew and
  imported boundaries are not hidden.
- `idempotency_key` provides dedupe and replay-consistency keys.
- `replay.*` provides replay safety for state reconstruction and export.

A consumer that cannot reconstruct an order without inventing new fields MUST
preserve “unknown ordering” and expose the uncertainty, not guess.

