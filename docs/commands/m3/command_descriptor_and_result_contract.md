# Command Descriptor And Result Contract

This contract pins the command object model used by palette, menus, keybindings,
CLI, AI tools, automation, support export, and telemetry. Every command surface
resolves a canonical descriptor first, then emits the same invocation-session and
result-packet shapes after dispatch.

## Descriptor Contract

A command descriptor is a public product object, not an implementation detail.
Stable and beta-bearing commands must declare:

- `command_id`, `command_revision_ref`, and `canonical_verb` for durable identity.
- `category_refs`, `origin`, and `discoverability_record_refs` for shared surface projection.
- `invocation_schema_ref` and `result_schema_ref` for machine-verifiable packets.
- `capability_scope_class`, `preview_class`, and `approval_posture_class` for trust and policy gates.
- `enablement_rule_refs` so every surface reaches the same enabled, disabled, or hidden decision for the same context snapshot.
- `automation_labels` so recipes, headless callers, and AI tools do not infer safety from UI placement.
- `aliases` with canonical resolution and lifecycle metadata for migration and support.

Extensions may not contribute stable commands unless their descriptors include
schema refs, capability metadata, lifecycle state, discoverability refs,
automation labels, and alias lifecycle metadata. Missing fields keep the command
out of stable surfaces.

## Alias Contract

Aliases never replace canonical identity. Invocation and result packets record
the alias used, but `resolves_to_canonical_command_id` must point back to the
descriptor `command_id`. Deprecated aliases may still succeed while emitting
`deprecated_alias_used`; retired aliases resolve to a typed denial instead of
falling through to a best-effort command lookup.

## Invocation Session

The canonical invocation-session schema is
`schemas/commands/command_invocation_session.schema.json`. It requires the
shared command-descriptor packet fields plus:

- `canonical_verb`
- `alias_used`

Each invocation session captures issuing surface, authority class, argument
provenance, context snapshot, enablement decision, preview posture, approval
posture, execution intent, outcome, artifacts, evidence, policy context,
redaction class, and mint time. Deep links, companion handoffs, AI tool calls,
and automation recipes must enter through this packet path; they must not bypass
trust, policy, preview, approval, freshness, capability, or result-schema guards.

## Result Packet

The canonical result-packet schema is
`schemas/commands/command_result_packet.schema.json`. Result packets carry the
minimum invocation identity plus:

- terminal `outcome_code`
- structured `warning_codes` and `error_codes`
- created artifacts with result-contract classes and roles
- notification, activity, rollback, checkpoint, and evidence refs
- export posture
- strict no-bypass guard booleans

Support export, telemetry, automation, and parity tooling consume result packets
instead of parsing UI copy or command-specific payloads.

## Fixtures

Worked examples live in
`fixtures/commands/m3/descriptor_and_invocation/`. They cover a canonical
descriptor, palette invocation, structured result packet, and deprecated CLI
alias path.
