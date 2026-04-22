# Security advisory and incident-packet example fixtures

These fixtures are short, reviewable scenarios that anchor the
vocabulary frozen in
[`/docs/security/severity_matrix.md`](../../../docs/security/severity_matrix.md)
and validated by the schemas at
[`/schemas/security/advisory_record.schema.json`](../../../schemas/security/advisory_record.schema.json)
and
[`/schemas/security/incident_workspace_packet.schema.json`](../../../schemas/security/incident_workspace_packet.schema.json).

Each fixture names the severity class it exercises, the advisory
identity aliases it reserves (Aureline advisory id plus CVE and GHSA
slots), the subject kinds it names, the install-profile card and
exact-build identity refs it resolves into, and the embedding states
it carries on its evidence items. Together they anchor the severity
vocabulary, the alias-ready advisory identity, the affected-install
assessment linkage, and the four-way embedding state
(omitted / embedded / redacted / by_reference) to concrete inputs and
observable outcomes.

**Scope rules**

- Fixtures validate against
  `schemas/security/advisory_record.schema.json` as
  `advisory_record` or against
  `schemas/security/incident_workspace_packet.schema.json` as
  `incident_workspace_packet_record`; they do not encode wire bytes,
  ADR-0005 subscription envelopes, or ADR-0004 RPC envelopes.
- A fixture MUST exercise at least one frozen
  `security_severity_class`, `advisory_subject_kind`,
  `embedding_state`, `redaction_class`, `handoff_routing_class`, or
  `export_routing_class`, and MUST name the severity-matrix section
  that motivates it.
- Raw secret bytes, raw exploit payloads, raw reporter identities,
  raw signing material, and raw binary bytes MUST NOT appear;
  placeholders of the shape `<redacted: <secret_class>>` or
  `<exploit_payload_reference>` stand in for every input that would
  otherwise carry raw material.
- Advisory ids, CVE ids, GHSA ids, install-profile card refs,
  exact-build identity refs, support-bundle refs, release-evidence
  packet refs, and monotonic timestamps are opaque. Where the repo
  already carries a seeded install-profile card or exact-build
  identity, these fixtures reuse that id verbatim; later-lane packet
  refs remain illustrative placeholders.
- At this milestone there is still no production incident-response
  tooling or on-call lane. The emergency-action / revocation object
  model now lives separately under
  `docs/security/emergency_action_model.md`; these fixtures remain
  pre-implementation governance artefacts.

**Index**

| Fixture                                                                                                | Record kind                          | Severity                        | Aliases reserved                                                  | Embedding states exercised                               | Severity-matrix section                                                        |
|--------------------------------------------------------------------------------------------------------|--------------------------------------|---------------------------------|-------------------------------------------------------------------|----------------------------------------------------------|--------------------------------------------------------------------------------|
| [`signed_binary_chain_bypass_advisory.yaml`](./signed_binary_chain_bypass_advisory.yaml)               | `advisory_record`                    | `security_severity.critical`    | `AURELINE-ADV-2026-0001` + `CVE-2026-10001` + `GHSA-aaaa-bbbb-cccc` | `by_reference`                                           | Severity vocabulary (critical) + Advisory identity and subject kind + Affected-install assessment linkage |
| [`signed_binary_chain_bypass_incident_packet.yaml`](./signed_binary_chain_bypass_incident_packet.yaml) | `incident_workspace_packet_record`   | `security_severity.critical`    | (packet — aliases carried on the linked advisory)                 | `omitted` + `embedded` + `redacted` + `by_reference`     | Evidence embedding vocabulary + Handoff and export routing                     |
