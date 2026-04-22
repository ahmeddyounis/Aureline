# Emergency-action and revocation example fixtures

These fixtures anchor the emergency-action and revocation contract frozen
in
[`/docs/security/emergency_action_model.md`](../../../docs/security/emergency_action_model.md)
and validated by
[`/schemas/security/emergency_action_record.schema.json`](../../../schemas/security/emergency_action_record.schema.json).

They intentionally reuse the same exact-build, install-profile, and
advisory/incident refs that the security advisory fixtures already use
where applicable. That is the point of the model: emergency actions and
revocations do not mint a second build/install identity system.

**Scope rules**

- Fixtures validate as either `emergency_action_record` or
  `revocation_record`.
- Every fixture exercises signer continuity, distribution freshness on
  at least one non-live path, local continuity, required actions, and
  at least one supersedence or revocation relation.
- `receipt_ref` values are opaque ids standing in for mirror-import or
  manual-import receipt objects. The receipt surface is expected to
  quote the same object fields rather than inventing a second contract.
- Raw bundle bytes, raw signatures, raw trust-root payloads, raw
  incident notes, and raw support attachments never appear.

**Index**

| Fixture | Record kind | Main path exercised | What it proves |
|---|---|---|---|
| [`signed_update_channel_freeze.yaml`](./signed_update_channel_freeze.yaml) | `emergency_action_record` | authoritative + mirror + manual-import + runtime-preload | `Emergency action required`, channel freeze, exact-build/install linkage, mirror/manual-import freshness, local continuity, required user/admin action, and post-incident reconciliation live on one object |
| [`trust_root_rotation_revocation.yaml`](./trust_root_rotation_revocation.yaml) | `revocation_record` | trust-root continuity on air-gapped/manual-import path | revocation remains durable, signer continuity and successor root live on the same record, and air-gapped profiles can represent freshness without pretending to be live-connected |
