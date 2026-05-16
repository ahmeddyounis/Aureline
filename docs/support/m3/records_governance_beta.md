# Records-governance, hold-awareness, and chain-of-custody for exported support artifacts

This beta surface makes exported support artifacts contractually honest.
Every artifact a support bundle preview carries — local workspace
state, managed-copy index entries, support-bundle archives, usage
exports, offboarding packets, destruction receipts, AI evidence
packets — also carries a typed records-governance packet so reviewers
see, in one record, the same governance truth the alpha
record-class registry already pins.

A packet projects six facts onto closed-vocabulary tokens:

| Fact | Closed vocabulary |
| ---- | ----------------- |
| Artifact class | `local_only`, `managed_copy`, `held`, `queued_for_delete`, `export_only` |
| Hold state | `none`, `on_hold`, `release_pending` |
| Hold classes (when on hold) | `administrative_legal`, `support_investigation`, `retention_minimum`, `export_pending` |
| Retention owner | `local_user`, `operator_admin`, `support_export`, `governance_packets`, `mixed` |
| Destruction caveat | `none`, `receipt_retained`, `retained_subset_remains`, `hold_blocks_completion`, `legal_hold_prevents`, `retention_minimum_applies`, `exported_local_copy_remains`, `outside_platform_scope`, `provider_backlog` |
| Custody timeline | Ordered chain of typed events (`created`, `packaged_for_export`, `exported_locally`, `mirrored_to_managed`, `placed_on_hold`, `hold_released`, `delete_requested`, `delete_completed`, `receipt_issued`, `access_handed_off`, `imported_from_handoff`) |

The packet's redaction class is pinned to `metadata_safe_default` and
`raw_content_exported` is pinned to `false`. The packet never embeds
raw payload bytes, raw credentials, raw policy bundle bytes, raw
prompts, or raw hold-justification bytes.

## What's in scope for this beta row

- A typed evaluator (`evaluate_records_governance_packet`) that
  validates caller-supplied inputs against the active record-class
  registry. It cross-checks the asserted `artifact_class` against the
  hold set, the chain of custody, and the registry row's `class_scope`
  so the chip cannot lie.
- A boundary JSON schema at
  [`/schemas/support/record_class.schema.json`](../../../schemas/support/record_class.schema.json)
  with closed-vocabulary enums and conditional schema rules: an
  `on_hold` packet must list at least one `hold_class`; a non-`none`
  destruction caveat must carry a reviewer-safe note; `raw_content_exported`
  is pinned to `false`.
- A support-bundle preview seed that queues the packet as one
  metadata-only governance row (`support.item.records_governance_packet`)
  on a `SupportBundlePreviewBuilder`. The row participates in the
  preview's redaction report, classification summary, and preview/export
  parity rules verbatim.
- Five canonical fixtures under
  [`/fixtures/support/records_governance/`](../../../fixtures/support/records_governance/)
  covering every acceptance class: `local_only_workspace_state`,
  `managed_copy_index`, `held_support_bundle`,
  `queued_for_delete_offboarding`, `export_only_usage_packet`.
- A producer record-kind binding registered with `aureline-records`
  so `records_governance_packet_record` is a governed record kind
  bound to the `support_bundle_archive` lifecycle.

## Acceptance and how the packet meets it

- **Support bundles can distinguish local-only, managed-copy, held,
  queued-for-delete, and export-only artifact classes.** The
  `ArtifactClass` enum is the sole source of truth. The evaluator
  derives the expected class from the hold state, custody chain, and
  registry row, then refuses to mint a packet whose asserted class
  disagrees.
- **Chain-of-custody metadata survives export and later escalation.**
  The `chain_of_custody` array is required (minimum one event),
  monotonic by `sequence`, and unique by `event_id`. Each event is
  metadata-only (no raw payload, no credential). The packet is
  serialized into the support-bundle preview snapshot, which a
  reviewer can reopen offline without contacting any service.
- **Deletion or hold caveats are visible instead of being implied or
  omitted.** A non-`none` `destruction_caveat_class` mandates a
  non-empty `destruction_caveat_note`. The packet's reviewer-visible
  summary quotes the hold state, hold class count, and destruction
  caveat verbatim. The packet's `exported_copy_remains_local`
  flag explicitly discloses any user-controlled copy outside the
  managed delete scope.

## Failure-drill posture

The evaluator fails closed before widening authority:

- Packet ids missing the `records_governance_packet:` prefix are
  refused.
- A `hold_state=on_hold` packet without any hold classes is refused.
- A hold class outside the registry row's allowed set is refused.
- A record class whose registry row is hold-ineligible is refused for
  `on_hold` and `release_pending` packets.
- A destruction caveat that is not `none` without a reviewer-visible
  note is refused.
- An empty or non-monotonic chain of custody is refused.
- A duplicate `event_id` is refused.
- An asserted artifact class that disagrees with the derived class is
  refused — the chip can never say "local only" if the chain records a
  managed mirror or a pending delete.

## First consumers

- The support-bundle preview builder (`aureline_support::bundle`)
  consumes the packet via `add_records_governance_preview_item` and
  surfaces it as one metadata-only row in the preview manifest. The
  row joins the same redaction-report and preview/export parity rules
  as every other queued row.
- The boundary schema is the contract the headless export writer and
  the support-export chrome both honor — they reconstruct the same
  packet shape from the on-disk manifest verbatim.

## Out of scope for this beta row

- Live byte-level redaction implementation, upload transport, hosted
  intake, or ticket routing.
- M5/M6 commercial control-plane breadth beyond the policy,
  entitlement, and admin truth required for the claimed beta rows.
- Automatic derivation of `artifact_class` from raw audit logs — the
  caller passes the asserted class and the evaluator cross-checks; a
  later milestone may add a typed projector from raw audit evidence.

## Related contracts

- [Alpha record-class registry](../../../artifacts/governance/record_class_registry_alpha.yaml) —
  the canonical hold-eligibility and retention-owner truth this packet
  cites verbatim.
- [Records-chronology and delete-honesty packet](../../../artifacts/governance/records_chronology_delete_packet.md)
  — the composition layer that ties retention/delete states to the
  chronology and remaining-location vocabularies this packet narrows.
- [Support-bundle contract](../support_bundle_contract.md) — the
  parent contract for every preview row.
