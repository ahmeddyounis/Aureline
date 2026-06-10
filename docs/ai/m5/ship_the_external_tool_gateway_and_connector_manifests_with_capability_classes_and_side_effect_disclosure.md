# External-tool gateway connector manifests with capability classes and side-effect disclosure

This contract ships the external-tool gateway into one export-safe truth packet
whose unit of truth is a connector manifest row. Shell, docs, support export, and
release tooling consume the packet directly instead of re-describing connector
capability, side-effect, or trust state by hand.

- Packet type: `aureline_ai::ship_the_external_tool_gateway_and_connector_manifests_with_capability_classes_and_side_effect_disclosure::ConnectorManifestPacket`
- Schema: [`schemas/ai/ship-the-external-tool-gateway-and-connector-manifests-with-capability-classes-and-side-effect-disclosure.schema.json`](../../../schemas/ai/ship-the-external-tool-gateway-and-connector-manifests-with-capability-classes-and-side-effect-disclosure.schema.json)
- Support export: [`artifacts/ai/m5/ship_the_external_tool_gateway_and_connector_manifests_with_capability_classes_and_side_effect_disclosure/support_export.json`](../../../artifacts/ai/m5/ship_the_external_tool_gateway_and_connector_manifests_with_capability_classes_and_side_effect_disclosure/support_export.json)
- Fixtures: [`fixtures/ai/m5/ship_the_external_tool_gateway_and_connector_manifests_with_capability_classes_and_side_effect_disclosure/`](../../../fixtures/ai/m5/ship_the_external_tool_gateway_and_connector_manifests_with_capability_classes_and_side_effect_disclosure/)

This lane ships the canonical M5 connector manifests on top of the tool-gateway
baseline. It reuses the gateway's capability, side-effect, runtime-boundary,
network-behavior, credential-posture, and output-trust vocabularies, the
routing-policy provider/locality mode vocabulary, and the frozen M5
qualification, downgrade, and rollback-posture vocabularies — it does not fork a
parallel set of terms.

## The connector manifest row

Each `ConnectorManifestRow` binds, for one governed external-tool connector:

| Field | Meaning |
| --- | --- |
| `manifest_id`, `connector_label`, `connector_family_label`, `connector_capability_version` | Identity, label, family, and capability version. |
| `descriptor_ref` | Opaque link to the matching tool-gateway descriptor, when one exists. |
| `publisher_source_class`, `publisher_identity_ref` | Who published the connector and the signed identity record. |
| `resolved_mode` | Local, BYOK, managed, or enterprise-gateway mode the connector resolves to. |
| `runtime_boundary_class`, `network_behavior_class`, `credential_posture_class`, `output_trust_posture_class` | Where the connector runs, how it reaches the network, how it is credentialed, and whether its output is tainted. |
| `state` | Admitted, pending first-use review, policy-blocked, trust-blocked, quarantined, or withdrawn. |
| `claimed_qualification` | Stable, Beta, Preview, Experimental, Held, or Unavailable. |
| `capability_classes` | The capability classes the connector advertises. |
| `side_effect_disclosures` | One disclosure per effect the connector can produce: side-effect class, preview, approval gate, audit, reversibility, and a review-safe disclosure label. |
| `downgrade_rules` | Closed set of triggers that narrow the claim. |
| `rollback_posture`, `rollback_verified` | Reversal posture for a connector-policy change and whether it was drilled. |
| `evidence_packet_refs` | Evidence backing a claimed connector. |

## Side-effect disclosure

A side-effect disclosure is the connector's promise about one effect it can
produce. Each disclosure carries:

- `preview` — whether the effect previews before it applies (a full preview, a
  diff, a dry run), is inspect-only and needs none, or has no preview and must
  block.
- `approval_posture` — the gate required before the effect applies, reusing the
  gateway approval vocabulary.
- `audit` — whether the effect is audited to the evidence timeline, the support
  export, local history only, or not at all.
- `reversibility` — whether the effect produces no change, is reversible in the
  workspace, is checkpoint-reversible, or is an irreversible external publish.

## Invariants enforced by validation

`ConnectorManifestPacket::validate` returns a closed set of typed violations.
Signed/shared connectors follow the same preview, policy, and audit rules as
first-party commands:

- Every connector advertises at least one capability class and discloses at least
  one side effect; each disclosure carries a label; no side-effect class is
  disclosed twice.
- A mutating side effect must preview before it applies, carry a real approval
  gate, and be audited.
- An irreversible external publish must be externally auditable (evidence
  timeline or support export), and a declared reversibility must agree with the
  effect class.
- A connector whose output crosses the network is tainted by default; a
  trusted-output posture is allowed only on a local boundary published under a
  signed identity; a local boundary may never advertise a remote network
  behavior.
- A blocked connector — policy-blocked, trust-blocked, quarantined, or withdrawn
  — may not keep a Stable, Beta, or Preview claim; a connector pending first-use
  review may not claim Stable.
- A claimed connector carries evidence refs, has a verified rollback path when its
  posture can be reversed, and carries a closed downgrade rule set that includes
  the proof-stale and provider-unavailable triggers and only narrows below the
  claimed qualification.
- The packet carries a proof-freshness block so stale proof automatically narrows
  claimed connectors.

## Boundary

The packet carries modes, classes, and review-safe labels only. Raw endpoint
URLs, raw spawn commands, credential bodies, raw API keys, OAuth tokens, and raw
request/response bodies never cross this boundary; `validate` rejects export-safe
JSON that embeds raw transport or credential material.

## Regenerating the artifacts

The checked-in support export and fixtures are produced by the in-crate builder
and can be regenerated deterministically:

```bash
cargo run -p aureline-ai --example dump_connector_manifest_packet -- support
cargo run -p aureline-ai --example dump_connector_manifest_packet -- fixture
```
