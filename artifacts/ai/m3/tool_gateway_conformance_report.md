# AI Tool-Gateway Conformance Report

Conformance report for the typed tool-gateway baseline owned by
`aureline_ai::tool_gateway`. The gateway extends the provider/model registry
into one inspectable contract for every MCP-style or external connector tool
the product can invoke from an AI turn or a user-facing command. Each
descriptor binds source/publisher, runtime boundary, capability classes,
network behavior, credential posture, availability state, first-use review
state, side-effect allowlists, data-class allowlists, and output trust
posture. Each tool-call timeline entry preserves descriptor lineage,
runtime-boundary label, side-effect class, outcome class, taint posture,
classification truth, and inspect / remove-from-context action refs.

## Source contracts

- `schemas/ai/tool_gateway_descriptor.schema.json` — descriptor boundary
  schema.
- `schemas/ai/tool_call_timeline_entry.schema.json` — tool-call timeline-entry
  boundary schema.
- `schemas/ai/external_tool_registry.schema.json` — external-tool registry
  boundary schema the gateway composes with.
- `schemas/ai/provider_model_registry.schema.json` — provider/model registry
  boundary schema the gateway extends.
- `schemas/ai/tainted_context.schema.json` — tainted-context fence schema
  preserved on every connector return.

Reviewer fixtures live under
`fixtures/ai/m3/mcp_gateway_and_tool_history/`.

## Descriptor boundary coverage

The canonical packet covers the three boundary classes that distinguish
"runs locally on this device" from "runs behind a gateway" from "runs on a
remote vendor service":

- `local_in_process` — first-party native filesystem snapshot tool. No
  network, no credential, trusted output after signing.
- `local_subprocess_same_device` — user-registered MCP filesystem snapshot
  tool. Local subprocess, signed manifest, per-invocation approval, output
  remains tainted by default.
- `enterprise_gateway_brokered_service` — enterprise-registered issues
  connector. Remote HTTPS, enterprise-managed credential, admin approval,
  output always tainted.

## Timeline taint posture coverage

The canonical timeline covers the postures replay/rerun/history surfaces must
preserve without flattening connector output into trusted context:

- `trusted_first_party_local_signed` — only admitted when the descriptor is a
  first-party native local tool and provenance, confidence, and effect class
  are all classified evidence-backed.
- `tainted_external_tool_output_default` — MCP tool returns are fenced until
  classification clears all three states.
- `tainted_unknown_effect_class` — used when an invocation is denied or
  errored before the effect class could be confirmed; the entry stays fenced
  and removable from context.

## Acceptance invariants

The conformance packet validates the following invariants:

1. **Connector identity preserved across surfaces.** Tool-call history,
   evidence packets, support exports, and rerun-review surfaces project the
   same descriptor refs, boundary labels, capability classes, and approval
   lineage. The composer, context-inspector, review-workspace, docs/help, and
   support-export rows all carry identical descriptor and timeline-entry
   refs.
2. **Inspectable allow / block reasons.** Each descriptor advertises its
   approval posture, lifecycle state, availability state, and first-use
   review state. The canonical enterprise row exercises an admin-approval
   posture with a cold handshake; the canonical MCP row exercises a
   per-invocation prompt with an admitted first-use ticket.
3. **Connector outputs cannot silently become trusted.** The only descriptor
   that ever produces trusted output is the first-party native row, and only
   when its runtime boundary is local and its publisher identity is signed.
   MCP and enterprise rows return tainted output regardless of the user's
   trust posture; the timeline entries enforce a fence ref on every tainted
   posture.
4. **Policy and trust changes narrow stale labels.** The availability state
   class is distinct from the lifecycle state class: a generally-admitted
   descriptor can still flip to `policy_blocked`, `trust_blocked`,
   `unavailable_quarantined_signature`, or `withdrawn` without leaving stale
   "ready" or "warm" labels on downstream surfaces. The conformance
   validation rejects descriptors that combine
   `unknown_must_disclose` network behavior with a warm admission state.

## Out of scope

This baseline does not broaden into unmanaged marketplace sprawl, arbitrary
third-party tool execution without declared contracts, or autonomous-agent
breadth. The descriptor and timeline records are additive over the existing
external-tool registry and never accept raw URLs, raw spawn commands, raw
environment variables, raw API keys, raw OAuth tokens, raw mTLS material,
raw request/response bodies, or raw stdio frames.
