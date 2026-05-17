# Deletion, hold, and destruction-receipt truth for support exports

This beta support surface makes deletion language shared across the
product, CLI/headless output, support-bundle previews, repair exports,
and later admin handoff packets. It narrows the existing
[records-governance beta](records_governance_beta.md) packet with
three inspectable additions:

- held-record selectors for every hold-eligible contractual class in
  the alpha record-class registry;
- a support destruction-receipt projection at
  [`/schemas/support/destruction_receipt.schema.json`](../../../schemas/support/destruction_receipt.schema.json);
- stable deletion-honesty labels that prevent support exports from
  saying "deleted" while data is held, queued, policy-retained, or
  outside the platform delete scope.
- a chronology-focused companion packet at
  [`/schemas/support/evidence_timeline.schema.json`](../../../schemas/support/evidence_timeline.schema.json)
  that splits request receipt from queue acceptance while preserving
  source timezone and actor ordering.

## Shared vocabulary

| State | Stable support label | When to use it |
| --- | --- | --- |
| `held` | `Legal hold` | A legal, support-investigation, retention-minimum, or export-pending hold blocks destructive lifecycle. |
| `queued_for_deletion` | `Delete requested` | A delete was requested but no terminal completion event is present. |
| `deleted` | `Delete completed` | Delete completion is recorded and no in-scope managed, held, receipt-only, or exported copy remains. |
| `retained_for_evidence` | `Policy retention` | Receipt metadata, policy-retained subsets, support evidence, or redaction/omission evidence remains. |
| `export_only` | `Exported copy remains local` | The artifact is generated output under user/device control rather than a durable in-product row. |

The companion evidence-timeline packet uses the same operator labels
and adds per-event state tokens for chronology exports:
`requested_deletion`, `queued_deletion`, `held_data`,
`retained_evidence`, and `completed_deletion`. Use the timeline packet
when the operator needs to inspect whether a request was merely received,
accepted into the queue, blocked by hold, narrowed to retained evidence,
or completed.

The lower-level records-governance packet still exposes
`local_only` and `managed_copy` when no delete lifecycle is in
progress. Export and repair surfaces use the labels above whenever a
delete, hold, or destruction receipt is in play.

## Implementation hooks

- `held_record_selectors_for_beta_contractual_classes()` emits one
  selector per hold-eligible record class in
  `artifacts/governance/record_class_registry_alpha.yaml`.
- `select_held_records()` applies those selectors to
  `RecordsGovernancePacket` rows, so support export and repair flows
  can locate held artifacts without string-matching reviewer prose.
- `evaluate_support_destruction_receipt()` validates a metadata-only
  destruction receipt, computes bucket counts, and adds a
  `deletion_honesty_disclosure` block.
- `add_destruction_receipt_preview_item()` queues the receipt into
  `SupportBundlePreviewBuilder` as `support.item.destruction_receipt`.

## Receipt guarantees

Support destruction receipts are metadata-only. They may cite refs,
record classes, policy refs, verifier refs, custody refs, counts, and
reviewer-safe summaries. They must not include raw payload bodies,
raw credentials, raw policy payloads, raw prompts, raw paths, or raw
hold-justification bytes.

The evaluator refuses a receipt when:

- `available` does not include both `emitted_receipt_ref` and
  `executed_at`;
- pending or unavailable receipt states claim an emitted receipt or
  execution timestamp;
- a blocked-by-hold receipt has no held refs;
- a policy-retained receipt has no retained refs;
- a completed receipt lists retained, held, outside-scope, manual, or
  redacted refs.

## Fixtures

- [`fixtures/support/records_governance/deleted_support_bundle_archive.json`](../../../fixtures/support/records_governance/deleted_support_bundle_archive.json)
- [`fixtures/support/records_governance/retained_destruction_receipt.json`](../../../fixtures/support/records_governance/retained_destruction_receipt.json)
- [`fixtures/support/deletion_and_hold/destruction_receipt_available.json`](../../../fixtures/support/deletion_and_hold/destruction_receipt_available.json)
- [`fixtures/support/deletion_and_hold/destruction_receipt_blocked_by_hold.json`](../../../fixtures/support/deletion_and_hold/destruction_receipt_blocked_by_hold.json)

## Verification

```bash
cargo test -p aureline-support records_governance_beta deletion_and_hold_beta
cargo test -p aureline-support evidence_timeline_beta
```
