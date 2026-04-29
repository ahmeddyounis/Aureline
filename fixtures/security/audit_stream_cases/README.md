# Security audit-stream case fixtures

These fixtures validate the threat-model, audit-stream, and
evidence-window contract in
[`/docs/security/threat_model_and_audit_stream_contract.md`](../../../docs/security/threat_model_and_audit_stream_contract.md).

Audit stream records validate against
[`/schemas/security/audit_stream_record.schema.json`](../../../schemas/security/audit_stream_record.schema.json).
The evidence-window case validates against
[`/schemas/security/evidence_window.schema.json`](../../../schemas/security/evidence_window.schema.json).

Every fixture uses opaque refs, redaction-safe notes, explicit evidence
windows, and field-disposition rows. Raw credentials, raw tenant names,
raw hostnames, raw provider tokens, raw code content, raw terminal
bytes, raw participant identifiers, and raw evidence bodies do not
appear.

| Fixture | Record kind | Stream or window | What it proves |
|---|---|---|---|
| [`extension_lifecycle_install_quarantined.yaml`](./extension_lifecycle_install_quarantined.yaml) | `audit_stream_record` | `extension_lifecycle` | supply-chain quarantine with package bytes withheld and policy evidence present |
| [`policy_bundle_change_permission_expand_denied.yaml`](./policy_bundle_change_permission_expand_denied.yaml) | `audit_stream_record` | `policy_bundle_change` | permission expansion denied under signed policy and tenant/key isolation coverage |
| [`workspace_trust_policy_narrowed.yaml`](./workspace_trust_policy_narrowed.yaml) | `audit_stream_record` | `workspace_trust_change` | trusted workspace narrowed by policy with omitted prior decision body declared |
| [`ai_tool_call_denied_data_exfiltration.yaml`](./ai_tool_call_denied_data_exfiltration.yaml) | `audit_stream_record` | `ai_tool_action` | AI tool call denied for export/data boundary reasons with redacted prompt fields |
| [`ai_apply_action_applied_with_ticket.yaml`](./ai_apply_action_applied_with_ticket.yaml) | `audit_stream_record` | `ai_apply_action` | AI apply action tied to a spent approval ticket and redacted evidence packet |
| [`collaboration_control_grant_admitted.yaml`](./collaboration_control_grant_admitted.yaml) | `audit_stream_record` | `collaboration_control_grant` | elevated shared-control grant admitted under explicit approval |
| [`collaboration_control_grant_revoked.yaml`](./collaboration_control_grant_revoked.yaml) | `audit_stream_record` | `collaboration_control_revocation` | grant revocation and replay denial remain separately auditable |
| [`remote_session_join_admitted.yaml`](./remote_session_join_admitted.yaml) | `audit_stream_record` | `remote_session_join` | remote join preserves route, target, tenant, and key-boundary refs |
| [`remote_session_leave_observed.yaml`](./remote_session_leave_observed.yaml) | `audit_stream_record` | `remote_session_leave` | remote leave keeps expired and redacted fields distinguishable |
| [`evidence_window_support_export_legal_hold.yaml`](./evidence_window_support_export_legal_hold.yaml) | `evidence_window_record` | evidence window | current/reviewable/exportable/expired/redacted/legal-hold semantics |
