# Secret-broker redaction example fixtures

These fixtures are short, reviewable scenarios that anchor the
vocabulary frozen in
[`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../../../docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
and validated by the schema at
[`/schemas/security/secret_class_rows.schema.json`](../../../schemas/security/secret_class_rows.schema.json).

Each fixture names the surface it redacts (the frozen
`redaction_class`), the secret classes, trust-store classes, unlock
states, projection modes, denial reasons, and audit events it
exercises, the surface's raw-input shape (expressed entirely through
`<redacted: ...>` placeholders; raw secret bytes never appear in a
fixture), the redacted-output shape as it would reach the sink, and
the audit events the broker emits. Together they anchor the class
names, projection modes, unlock states, denial reasons, and audit
event ids to concrete inputs and observable outcomes.

**Scope rules**

- Fixtures validate against `schemas/security/secret_class_rows.schema.json`
  as one of `redaction_example_record` or `trust_store_posture_record`;
  they do not encode wire bytes, ADR-0005 subscription envelopes, or
  ADR-0004 RPC envelopes.
- A fixture MUST exercise at least one frozen `secret_class`,
  `trust_store_class`, `unlock_state`, `projection_mode`,
  `denial_reason`, or `audit_event_id`, and MUST name the ADR
  section that motivates it.
- Raw secret bytes MUST NOT appear; placeholders of the shape
  `<redacted: <secret_class>>` stand in for every input that would
  otherwise carry raw material.
- Handle ids, aliases, consumer identities, target refs, approval
  ticket refs, and monotonic timestamps are opaque; they are chosen
  to read well rather than to reflect any real deployment.

**Index**

| Fixture                                                                            | Record kind                  | Redaction class / posture      | Key classes exercised                                                                                   | ADR section                                                             |
|------------------------------------------------------------------------------------|------------------------------|--------------------------------|---------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------|
| [`logs_redaction.yaml`](./logs_redaction.yaml)                                     | `redaction_example_record`   | `logs_local`                   | `ai_provider_token` / `request_header_signer` / `secret_projection_failed`                              | Redaction defaults (frozen) - `logs_local`                              |
| [`support_bundle_redaction.yaml`](./support_bundle_redaction.yaml)                 | `redaction_example_record`   | `support_bundle`               | `ai_provider_token` + `database_credential` + `signing_key_material` / env-var redaction / class counts | Support-bundle inclusion and exclusion (explicit defaults)              |
| [`evidence_packet_signature.yaml`](./evidence_packet_signature.yaml)               | `redaction_example_record`   | `evidence_packet`              | `signing_key_material` / `sign_only` / signature-bytes-only                                             | Redaction defaults (frozen) - `evidence_packet`                         |
| [`clipboard_reveal_audit.yaml`](./clipboard_reveal_audit.yaml)                     | `redaction_example_record`   | `clipboard_projection`         | `ai_provider_token` / `reveal_on_demand` / bounded-clipboard + OSC-52-denied                            | Clipboard / reveal-on-demand behaviour                                  |
| [`denied_projection.yaml`](./denied_projection.yaml)                               | `redaction_example_record`   | `logs_local`                   | `ai_tool_reveal_denied` + `policy_denied_projection` / `secret_denial` / fail-closed                    | Denial posture                                                          |
| [`degraded_session_only_fallback.yaml`](./degraded_session_only_fallback.yaml)     | `trust_store_posture_record` | `session_memory_cache` fallback| `degraded_session_only` / `session_fallback_class_forbidden` for signing / no plaintext file            | Trust-store classes (frozen) - plaintext fallback is not acceptable     |
