# Deferred Publish Queue Recovery Packets

This document is the contract for the deferred-publish recovery packet that
keeps publish-later continuity reviewable after auth denial, provider outage,
validation conflict, stale-target drift, and redaction-policy blocks. The
packet is the canonical export-safe source for queue rows, durable local
packets, activity-center projections, and support exports that all describe the
same deferred mutation.

- Record kind: `ship_deferred_publish_queue_recovery_packets`
- Schema: [`schemas/review/ship-deferred-publish-queue-recovery-packets.schema.json`](../../../schemas/review/ship-deferred-publish-queue-recovery-packets.schema.json)
- Canonical support export: [`artifacts/review/m5/ship_deferred_publish_queue_recovery_packets/support_export.json`](../../../artifacts/review/m5/ship_deferred_publish_queue_recovery_packets/support_export.json)
- Summary artifact: [`artifacts/review/m5/ship_deferred_publish_queue_recovery_packets.md`](../../../artifacts/review/m5/ship_deferred_publish_queue_recovery_packets.md)
- Fixtures: [`fixtures/review/m5/ship_deferred_publish_queue_recovery_packets/`](../../../fixtures/review/m5/ship_deferred_publish_queue_recovery_packets/)
- Producer: `aureline_review::current_deferred_publish_queue_recovery_export`

## Pillars

### Queue rows

Each `queue_rows[]` item preserves the queue id, authoritative object
identity, dependency order, freshness requirement, retry posture, conflict
policy, and audit lineage for one deferred mutation. `draft_only`,
`queued_for_publish`, `blocked`, `stale_target`, `conflict_review_required`,
and `published` stay explicit instead of collapsing into a generic “pending”
state.

### Durable local packets

Each `local_packets[]` item names the same authoritative object as the queue
row and proves that the local packet survives restart, export, and support
handoff. Auth denial, outage, validation conflict, and redaction-policy block
conditions preserve the packet rather than discarding intent.

### Activity projection

Each `activity_rows[]` item reuses the canonical object identity and exact
reopen target while quoting the same deferred-publish lifecycle token in
`phase_label`. The shell can distinguish blocked, stale-target, and
conflict-review-required rows without inventing a new vocabulary.

### Support export

The `support_export` block remains metadata-safe and preserves the same
authoritative object, lifecycle token, retry posture, freshness requirement,
conflict policy, and stable action ids that the queue and local packet rows
carry.

## Replay invariants

The `trust_review` block is the hard gate for this lane:

- queue rows keep replay contract details explicit;
- durable local packets survive restart and export;
- blocked publish states preserve retry, discard, export, and open-external
  actions;
- replay requires fresh target identity and current effective scope for every
  replay-review state;
- high-impact mutations never auto-replay across changed boundaries.

## Boundary

Raw provider payloads, credentials, live browser session material, comment
bodies, and provider response bodies never cross this boundary. The packet
contains only metadata-safe identifiers, lifecycle labels, policy/replay
posture, audit refs, and export-safe summaries.
