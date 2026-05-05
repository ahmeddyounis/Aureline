# Manual emergency-import receipt and metadata-chain entry fixtures

These fixtures anchor the offline/manual emergency-import audit contract
frozen in:

- [`/docs/security/manual_emergency_import_contract.md`](../../../docs/security/manual_emergency_import_contract.md)
- [`/docs/security/emergency_distribution_policy.md`](../../../docs/security/emergency_distribution_policy.md)

They validate against:

- [`/schemas/security/manual_import_receipt.schema.json`](../../../schemas/security/manual_import_receipt.schema.json)
  (`manual_import_receipt_record`)
- [`/schemas/security/metadata_chain_entry.schema.json`](../../../schemas/security/metadata_chain_entry.schema.json)
  (`metadata_chain_entry_record`)

They reuse the opaque `receipt_ref` ids that the emergency-action and
revocation fixtures under
[`/fixtures/security/emergency_action_examples/`](../emergency_action_examples/)
already cite. That is the point of the model: the receipt surface is
the same object the emergency-action distribution row refers to, not a
parallel artifact.

**Scope rules**

- Receipt fixtures validate as `manual_import_receipt_record`.
- Chain-entry fixtures validate as `metadata_chain_entry_record`.
- Every receipt includes a detached-signature verification envelope, an
  observed signer-continuity state, explicit freshness/validation
  labeling, an explicit target scope, an applied/supersedence/expiry
  posture, at least one follow-up obligation, and a metadata-chain stub
  rooted at an authoritative origin.
- Chain entries mirror the export-safe scope facts and posture fields so
  offline evidence packets can join authoritative origins, receipts, and
  target scope without relying on volatile logs.
- Raw bundle bytes, raw signatures, raw trust-root payloads, raw
  operator identifiers, raw file paths, and raw media serial numbers
  never appear.

**Index**

- Manifest: [`manifest.yaml`](./manifest.yaml)
- Receipts:
  - [`stable_channel_freeze_manual_import.yaml`](./stable_channel_freeze_manual_import.yaml)
  - [`trust_root_rotation_offline_transfer.yaml`](./trust_root_rotation_offline_transfer.yaml)
  - [`preview_helper_extension_mirror_sync.yaml`](./preview_helper_extension_mirror_sync.yaml)
  - [`preview_helper_extension_manual_import_advisory.yaml`](./preview_helper_extension_manual_import_advisory.yaml)
  - [`offline_advisory_reimport_superseded.yaml`](./offline_advisory_reimport_superseded.yaml)
  - [`offline_advisory_reimport_active.yaml`](./offline_advisory_reimport_active.yaml)
- Chain entries:
  - [`stable_channel_freeze_chain_entry.yaml`](./stable_channel_freeze_chain_entry.yaml)
  - [`trust_root_rotation_chain_entry.yaml`](./trust_root_rotation_chain_entry.yaml)
  - [`preview_helper_extension_mirror_sync_chain_entry.yaml`](./preview_helper_extension_mirror_sync_chain_entry.yaml)
  - [`preview_helper_extension_manual_import_advisory_chain_entry.yaml`](./preview_helper_extension_manual_import_advisory_chain_entry.yaml)
  - [`offline_advisory_reimport_chain_entry.yaml`](./offline_advisory_reimport_chain_entry.yaml)
