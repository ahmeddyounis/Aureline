# Extension Advisory Notice Template

| Field | Required content |
|---|---|
| Advisory ID | Copy-safe advisory ID and incident ID |
| Severity | `critical`, `high`, `moderate`, `low`, or `operational_emergency` |
| Affected extension | Extension identity, package ID, version, publisher, installed-state refs |
| Reason code | Controlled reason code from the incident packet |
| Source and actor | Registry, mirror, policy, runtime, or user source plus accountable actor class |
| Registry and mirror state | Primary registry trust state, mirror trust state, freshness, signer continuity, import requirement |
| Current lifecycle | `advisory_active`, `disabled`, `quarantined`, `revoked`, `mitigated_locally`, or `resolved_retained` |
| Blocked operations | Install, update, auto-update, activation, execution, or mirror import |
| Local continuity | What remains available locally while the action is active |
| Recovery guidance | Last-known-good target, rollback manifest, pin/remove/refresh/admin actions |
| Support export | Metadata-safe export ID and audit event refs |

## Notice Body

`{affected_extension}` is in `{lifecycle_state}` because `{reason_code}` was reported by `{source_class}` and applied by `{actor_class}` at `{effective_at}`.

Blocked operations: `{blocked_operations}`.

Primary registry state: `{primary_registry_trust_state}`.

Mirror state: `{mirror_trust_state}`. If mirror import is required, do not treat the mirror as current until the signed advisory or revocation metadata is imported and verified.

Recovery: `{recovery_guidance}`.

Support and audit reference: `{incident_id}` / `{advisory_id}`.
