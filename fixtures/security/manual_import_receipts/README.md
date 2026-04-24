# Manual emergency-import receipt fixtures

These fixtures anchor the manual-import receipt contract frozen in
[`/docs/security/emergency_distribution_policy.md`](../../../docs/security/emergency_distribution_policy.md)
and validated by
[`/schemas/security/manual_import_receipt.schema.json`](../../../schemas/security/manual_import_receipt.schema.json).

They reuse the opaque `receipt_ref` ids that the emergency-action and
revocation fixtures under
[`/fixtures/security/emergency_action_examples/`](../emergency_action_examples/)
already cite. That is the point of the model: the receipt surface is
the same object the emergency-action distribution row refers to, not a
parallel artifact.

**Scope rules**

- Fixtures validate as `manual_import_receipt_record`.
- Every fixture exercises a detached-signature verification envelope,
  an observed signer-continuity state, a target scope, an applied
  state, a supersedence state, an expiry envelope, at least one
  typed follow-up obligation, and a metadata-chain stub rooted at the
  authoritative emergency_action_record or revocation_record.
- `receipt_id` values match the `receipt_ref` values on the parent
  emergency-action / revocation fixture.
- Raw bundle bytes, raw signatures, raw trust-root payloads, raw
  operator identifiers, raw file paths, and raw media serial numbers
  never appear.

**Index**

| Fixture | Imports | Path exercised | What it proves |
|---|---|---|---|
| [`stable_channel_freeze_manual_import.yaml`](./stable_channel_freeze_manual_import.yaml) | `emergency_action.AURELINE-ACT-2026-0001` (stable channel freeze) | `manual_file_import_channel` (file_import) | Air-gapped admin imported the signed freeze; detached signature verified, receipt is pending_operator_confirmation until the reviewing operator commits, follow-up obligations include export_support_packet and propagate_to_offline_bundles |
| [`trust_root_rotation_offline_transfer.yaml`](./trust_root_rotation_offline_transfer.yaml) | `revocation.AURELINE-REV-2026-0001` (trust-root rotation revocation) | `offline_transfer_channel` (offline_transfer) | Offline-transfer receipt records an unexpired snapshot on an air-gapped target, the observed signer continuity is unknown_offline, and the receipt chains through an upstream manual import while naming the next-step import_successor_trust_root obligation |
