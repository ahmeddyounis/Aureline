# External-Tool Gateway Connector Manifests With Capability Classes And Side-Effect Disclosure

- Packet: `connector-manifest:stable:0001`
- Schema: `schemas/ai/ship-the-external-tool-gateway-and-connector-manifests-with-capability-classes-and-side-effect-disclosure.schema.json`
- Support export: `artifacts/ai/m5/ship_the_external_tool_gateway_and_connector_manifests_with_capability_classes_and_side_effect_disclosure/support_export.json`
- Fixture: `fixtures/ai/m5/ship_the_external_tool_gateway_and_connector_manifests_with_capability_classes_and_side_effect_disclosure/`

## Coverage

The packet ships the external-tool gateway into one row per governed connector.
Every connector carries the capability classes it advertises and a side-effect
disclosure for every effect it can produce, plus the provider/locality mode,
runtime boundary, network behavior, credential posture, and output-trust posture
it runs under.

- The managed review-comment connector resolves to the managed mode at Stable: a
  remote vendor-managed service over HTTPS whose output is always tainted, an
  inspect-only read and a reversible external comment that previews and is
  per-invocation approved and audited to the evidence timeline, a
  checkpoint-reversible verified rollback, and downgrade rules that narrow to Beta
  on stale proof and to Unavailable on provider outage.
- The BYOK issue-publish connector resolves to BYOK at Beta: a self-hosted MCP
  service whose output is always tainted, an irreversible external publish that
  shows a diff, requires admin approval, and is audited to the support export, an
  evidence-preserved rollback, and downgrade rules that narrow to Preview on stale
  proof and to Held on provider outage.
- The local symbol-inspector connector resolves to the local mode at Preview: an
  in-process first-party tool with no network and trusted local output, an
  inspect-only disclosure with no side effect, and downgrade rules that narrow to
  Experimental.
- The quarantined deploy connector resolves to the enterprise-gateway mode but
  claims Held: its signature failed, so it is quarantined, its irreversible
  publish is denied by policy, and every downgrade rule narrows to Unavailable.

## Invariants

The support export validates against the same closed rule set the shell, docs,
and release tooling enforce: every mutating side effect previews, gates, and
audits; every irreversible publish is externally auditable; network-crossing
output is tainted by default; trusted output stays on a signed local boundary; a
blocked connector drops its public claim; and every claimed connector carries
evidence, a verified reversible rollback, and the proof-stale and
provider-unavailable downgrade triggers.

## Boundary

The packet carries modes, classes, and review-safe labels only. Raw endpoint
URLs, raw spawn commands, credential bodies, raw API keys, OAuth tokens, and raw
request/response bodies never cross this boundary.
