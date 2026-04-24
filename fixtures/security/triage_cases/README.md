# Private-triage workspace packet worked cases

These fixtures are short, reviewable intake-and-triage scenarios that
anchor the operational vocabulary frozen in
[`/docs/security/intake_and_triage.md`](../../../docs/security/intake_and_triage.md)
and validated by the schema at
[`/schemas/security/private_triage_workspace_packet.schema.json`](../../../schemas/security/private_triage_workspace_packet.schema.json).

Each fixture names the intake route it arrived on, the private-triage
workspace scope it was opened under, the severity class it was triaged
at, the affected exact-build identity / install-profile card /
deployment-profile scope / channel class / publication posture the
triage carries, and the mitigation, break-glass, compensating-control,
and disclosure postures the packet currently holds.

**Distribution coverage.** The four cases together exercise the four
shipped distribution lanes in scope at this milestone:

| Case                                                                                                       | Distribution lane        | Severity                                  | Workspace scope                       | Mitigation state                                | Break-glass state                                      | Disclosure posture                                |
|------------------------------------------------------------------------------------------------------------|---------------------------|--------------------------------------------|----------------------------------------|--------------------------------------------------|---------------------------------------------------------|----------------------------------------------------|
| [`hosted_managed_cloud_session_hijack.yaml`](./hosted_managed_cloud_session_hijack.yaml)                   | Hosted / managed cloud    | `security_severity.critical`              | `private_security_channel`             | `mitigation_shipped_full`                         | `break_glass_invoked_reconciled`                        | `disclosure_posture_public_on_advisory`            |
| [`self_hosted_capability_gate_bypass.yaml`](./self_hosted_capability_gate_bypass.yaml)                     | Self-hosted (on-prem)     | `security_severity.high`                  | `private_security_channel`             | `mitigation_compensating_control_only`            | `break_glass_not_invoked`                               | `disclosure_posture_public_on_advisory`            |
| [`mirror_only_docs_pack_tampering.yaml`](./mirror_only_docs_pack_tampering.yaml)                           | Customer-managed mirror   | `security_severity.medium`                | `coordinated_disclosure_group`         | `mitigation_drafted`                              | `break_glass_not_invoked`                               | `disclosure_posture_public_on_fix`                 |
| [`offline_airgapped_expired_bundle_rekey.yaml`](./offline_airgapped_expired_bundle_rekey.yaml)             | Offline / air-gapped      | `security_severity.operational_emergency` | `vendor_only`                          | `mitigation_shipped_partial`                      | `break_glass_invoked_pending_reconciliation`            | `disclosure_posture_public_on_advisory`            |

**Scope rules**

- Fixtures validate against
  `schemas/security/private_triage_workspace_packet.schema.json` as
  `private_triage_workspace_packet_record`; they do not encode wire
  bytes, ADR-0005 subscription envelopes, or ADR-0004 RPC envelopes.
- A fixture MUST resolve the monitored contact ref, reserve an
  `aureline_advisory_id`, name the intake route, and carry the
  affected-install linkage in the same vocabulary the advisory record
  uses.
- Raw secret material, raw exploit payloads, raw reporter identities,
  raw signing material, and raw binary bytes MUST NOT appear;
  placeholders of the shape `<redacted: <secret_class>>` or
  `<exploit_payload_reference>` stand in.
- Advisory ids, CVE ids, GHSA ids, install-profile card refs, and
  exact-build identity refs are opaque. Where the repository already
  carries seeded ids (install-profile cards in
  `artifacts/release/install_topology_matrix.yaml`), these fixtures
  reuse that id verbatim; other refs remain illustrative placeholders.
- At this milestone there is still no live on-call rotation or
  paging pipeline. These fixtures remain pre-implementation
  governance artifacts.
