# Late-copy change-control examples

Worked fixtures for the reviewed-pack and late-copy change-control
contract frozen in
[`/docs/docs/reviewed_pack_and_late_copy_policy.md`](../../../docs/docs/reviewed_pack_and_late_copy_policy.md).
Every fixture here conforms to
[`/schemas/docs/late_copy_change_packet.schema.json`](../../../schemas/docs/late_copy_change_packet.schema.json).

The fixtures exist so docs/help, migration, support-export,
release-note, CLI/help, evaluation, and public-proof work can reuse one
reviewed-pack model and one controlled late-copy packet family without
inventing channel-local override prose.

## Intended usage

- **Schema conformance.** Every fixture MUST validate against
  [`/schemas/docs/late_copy_change_packet.schema.json`](../../../schemas/docs/late_copy_change_packet.schema.json).
- **Review corpus.** Shiproom, docs/public-truth review, support, and
  migration tasks can inspect the same packet family when release-bearing
  copy changes after string freeze.
- **Binding-state reference.** The fixtures exercise the binding-state
  labels `reviewed_current`, `stale_reviewed_source`,
  `late_copy_override_active`, and `late_copy_override_reversed`.

## Fixtures

- [`reviewed_pack_release_candidate.json`](./reviewed_pack_release_candidate.json)
  — baseline reviewed-pack version for a stable release candidate. All
  bindings are reviewed and current except one Help/About and
  service-health disclosure that carries an active late-copy override.
- [`legal_disclosure_late_copy.json`](./legal_disclosure_late_copy.json)
  — post-string-freeze legal/trust disclosure correction across
  Help/About, service-health, and support export. The packet links the
  affected claim row, requires docs/public-truth, legal/policy, release,
  and security/trust review, and plans to supersede the override with a
  new reviewed-pack version.
- [`recovery_compatibility_late_copy_reversed.json`](./recovery_compatibility_late_copy_reversed.json)
  — recovery/support/compatibility wording correction that was applied,
  then reversed once a successor reviewed-pack version landed. The
  packet keeps the prior/new text refs, verification notes, linked
  compatibility row, and reversal notes visible for audit and support.

## Related artifacts

- [`/docs/docs/docs_pack_manifest_contract.md`](../../../docs/docs/docs_pack_manifest_contract.md)
  — reviewed source for docs/help manifests and support-runbook packs.
- [`/docs/governance/claim_manifest_contract.md`](../../../docs/governance/claim_manifest_contract.md)
  — reviewed source for canonical public copy and downgrade posture.
- [`/schemas/release/compatibility_row.schema.json`](../../../schemas/release/compatibility_row.schema.json)
  — reviewed source for compatibility, support-window, and migration
  wording.
- [`/docs/release/release_artifact_graph.md`](../../../docs/release/release_artifact_graph.md)
  — release-artifact graph that treats this packet family as part of the
  docs/help truth discipline rather than as ad hoc prose.
