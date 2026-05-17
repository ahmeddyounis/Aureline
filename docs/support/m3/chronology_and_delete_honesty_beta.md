# Chronology and delete-honesty evidence timeline

This beta support surface gives operator-facing packets one chronology
for delete, hold, retention, and completion events. It composes the
existing records-governance packet and destruction-receipt projection
instead of replacing them.

The boundary schema is
[`/schemas/support/evidence_timeline.schema.json`](../../../schemas/support/evidence_timeline.schema.json).
The first Rust consumer is
[`crates/aureline-support/src/bundle/evidence_timeline/mod.rs`](../../../crates/aureline-support/src/bundle/evidence_timeline/mod.rs),
which queues the packet into the support-bundle preview as
`support.item.evidence_timeline_packet`.

## Controlled Vocabulary

| State token | Operator label | Meaning |
| --- | --- | --- |
| `requested_deletion` | `Delete requested` | The request was received and is attributable, but queue acceptance is not yet the next recorded event. |
| `queued_deletion` | `Delete queued` | The request is accepted and waiting on execution, purge propagation, or a policy gate. |
| `held_data` | `Legal hold` | A legal, support-investigation, retention-minimum, or export-pending hold blocks destructive completion. |
| `retained_evidence` | `Policy retention` | Payload deletion, narrowing, or blocking left metadata, receipts, or policy evidence behind. |
| `completed_deletion` | `Delete completed` | Delete completed and the event lists `no_remaining_location` as the only remaining location. |

These labels are the only labels support export, UI copy, and
headless output use for the packet states. Packet-level records that
only have one summary chip can still summarize a queued lifecycle as
`Delete requested`; the evidence timeline carries the finer
`requested_deletion` versus `queued_deletion` distinction when per-event
chronology is available.

## Chronology Rules

Each event keeps both:

- `source_display_order`, the order the source surface rendered; and
- `chronology_order`, derived by the support evaluator from
  `occurred_at` then `actor_order` then `source_display_order`.

Exports preserve the source `occurred_at` string, UTC offset, IANA
timezone, and local label. A row captured as `2026-05-16T16:00:05+03:00`
stays in that source context in the export even when another row at the
same instant was rendered as `2026-05-16T13:00:05Z`.

Rows also carry `evidence_source_class` and `current_state_class` from
the chronology-context vocabulary. Imported, mirrored, offline, replay,
and synthetic rows cannot render as live system truth.

## Support Preview Contract

The evidence timeline is metadata-only. It may include opaque refs,
counts, state tokens, actor classes, timezone labels, hold classes,
remaining-location classes, and reviewable notes. It must not include
raw payloads, raw prompts, raw credentials, raw policy bodies, raw paths,
or raw hold-justification bytes.

The preview row cites:

- `schemas/support/evidence_timeline.schema.json`
- `docs/support/m3/chronology_and_delete_honesty_beta.md`
- `artifacts/support/m3/evidence_timeline_packet.md`

## Fixtures

- [`fixtures/support/evidence_timeline/delete_hold_chronology_packet.json`](../../../fixtures/support/evidence_timeline/delete_hold_chronology_packet.json)

## Verification

```bash
cargo test -p aureline-support evidence_timeline_beta
```
