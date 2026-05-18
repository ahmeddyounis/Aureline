# MCP gateway and tool history fixtures

Canonical fixtures for the typed tool-gateway baseline exercised by
`aureline_ai::tool_gateway`.

## Files

- `tool_gateway_conformance_packet.json` — one tool-gateway conformance packet
  covering three descriptor classes (first-party native, MCP server,
  enterprise-gateway remote) and three matching timeline entries
  (trusted-first-party inspect, tainted MCP reversible edit, denied-by-approval
  remote call). The packet projects identical descriptor and timeline refs to
  the composer, context-inspector, review-workspace, docs/help, and
  support-export surfaces.

## Invariants exercised

- Descriptor identity, runtime boundary, capability classes, network behavior,
  credential posture, availability state, lifecycle state, approval posture,
  first-use review state, allowed side effects, allowed data classes, denied
  data classes, and output trust posture are typed and inspectable.
- Trusted-first-party-local descriptors must carry a local runtime boundary and
  a signed publisher identity; the canonical native row demonstrates this.
- MCP-server, user-registered, and enterprise-registered publishers require a
  first-use review ticket before any material run; the canonical MCP and
  enterprise rows demonstrate this.
- Tool-call timeline entries preserve descriptor lineage, runtime boundary,
  side-effect class, outcome, taint posture, classification truth, and inspect
  / remove-from-context action refs across composer, context-inspector,
  review-workspace, docs/help, and support-export surface projections.
- Tainted timeline entries always carry a tainted-context fence ref;
  trusted entries are admitted only with classifications evidence-backed.
- Credential and secret data classes are mechanically denied on every
  descriptor; no descriptor admits them as input.
