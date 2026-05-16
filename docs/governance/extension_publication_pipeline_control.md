# Extension publication pipeline control

This control note pins the governance rule for extension publication
packets. The operational contract is the extension publication pipeline
schema and the headless tool; this page states how release, ecosystem,
security, support, and mirror operators should interpret the packet.

## Control rule

An extension publication is admissible only when the packet decision is
`ready_for_promotion` and the reason is
`ready_signed_provenance_rollback_safe`.

The publication must be held or refused when any of these conditions
are true:

| Condition | Required result |
|---|---|
| Artifact is unsigned or uses an unsupported signature posture | `refused_unsigned_artifact` |
| Provenance is missing or does not bind to the artifact digest | `refused_provenance_missing` |
| Compatibility metadata or conformance report refs are missing | `refused_compatibility_missing` |
| Promotion digest, signature digest, or provenance digest differs from the artifact digest | `refused_promotion_identity_mutation` |
| Promotion has no evidence or approver refs | `refused_promotion_evidence_missing` |
| Prior installable artifact is not preserved | `refused_rollback_target_missing` |
| Catalog writes can leave partial rows | `refused_transactional_write_guard_missing` |
| Revocation state can be written without a committed catalog row | `refused_orphaned_revocation_state` |

## Evidence

The checked packet and fixture report are:

- [`artifacts/extensions/m3/publication_pipeline/publication_pipeline_record.json`](../../artifacts/extensions/m3/publication_pipeline/publication_pipeline_record.json)
- [`artifacts/compat/m3/extension_publication_pipeline_report.json`](../../artifacts/compat/m3/extension_publication_pipeline_report.json)

Operators and support exports should read those machine records instead
of inferring publication state from prose or a registry listing.
