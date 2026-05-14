# Endpoint Policy Inspector Alpha

The schema and endpoint-policy inspector is the first runtime inspection path
for telemetry, support-export, and operational-signal policy rows. It lives in
`crates/aureline-shell/src/inspectors/schema_registry/` and reads the checked-in
artifact registers directly:

- `artifacts/governance/consent_ledger_seed.yaml` for telemetry and
  support payload-family consent, endpoint, retention, and local-only posture.
- `artifacts/governance/schema_registry_alpha.yaml` for alpha support-export
  schema rows and record-class bindings.

The inspector does not copy registry prose into a private table. If a claimed
row is not present in one of those artifacts, inspection fails closed.

## Surfaced State

Each endpoint-policy row names:

- the destination class, such as `optional_telemetry_upload` or
  `manual_support_export`;
- the consent or policy state, such as `explicit_opt_in_required` or
  `export_only_user_request`;
- the egress class and outcome class;
- the local-only alternative quoted from the owning registry row; and
- the redaction plus retention/export posture used by support export.

Operational signal slices use one vocabulary across desktop inspector, support
export, and runbook/help handoff:

- freshness: `live`, `buffering`, `cached`, `stale`, `partial`, `offline`;
- redaction: `metadata_only`, `redacted_payload`, `by_reference_only`,
  `withheld_by_policy`, `retained_local_only`;
- signal kind: logs, metrics, traces, and incident timeline;
- source backend, time window, timezone, truncation state, destination class,
  and retention/export posture.

Raw URLs, hostnames, payload bodies, credentials, prompts, logs, traces, and
secret material are not embedded in the inspector support export. Rows carry
opaque refs, metadata, redaction labels, omission notes, and local-only
alternatives.

## Proof Path

Protected fixtures live in
`fixtures/inspectors/endpoint_policy_alpha/`. They exercise:

- telemetry and support schema claims resolved from the current artifact
  registers;
- endpoint-policy inspection for claimed rows;
- logs, metrics, traces, and incident timeline signal slices; and
- parity of freshness/redaction vocabulary across desktop, support export, and
  runbook/help handoff.

Run the focused proof with:

```sh
cargo test -p aureline-shell schema_registry_endpoint_policy
```
