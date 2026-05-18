# Incident Workspace Runbook Beta

This contract defines the support-owned beta incident workspace packet
that joins evidence, a versioned runbook, step authority, action-ledger
outcomes, explicit browser or console handoffs, and a redacted export
bundle around one incident object.

Implementation:

- Rust types and validator:
  `crates/aureline-support/src/incident_workspace/mod.rs`
- Boundary schema:
  `schemas/support/incident_action_ledger.schema.json`
- Runbook packet companion schema:
  `schemas/support/runbook_packet.schema.json`
- Fixture corpus:
  `fixtures/support/m3/runbook_packet_and_handoff/`

## Packet Contract

Every packet carries:

- an incident header with incident ID, severity, owner, start time, and
  current state;
- an environment scope with target context, deployment profile, trust
  posture, and write posture;
- an evidence timeline with source class, freshness, sensitivity, target
  context, and explicit omission markers;
- a `RunbookPacket` with source class, version, freshness, target
  selector rules, approval requirements, expected evidence outputs, step
  classes, and deviation-note policy;
- one `IncidentActionLedgerEntry` per observed, blocked, completed,
  deviated, rolled back, handoff, or export action;
- `ConsoleHandoffMetadata` whenever the true control plane is outside
  Aureline;
- a redacted export bundle that includes action-ledger and handoff refs
  by metadata and declares omissions.

## Authority Rules

Runbook text is not authority. A protected mutating step may claim live
execution only when all of these are present:

- current approval state;
- approval ticket ref;
- preview hash ref;
- resolved target identity;
- expected evidence outputs that gate completion;
- action ledger entry with outcome, actor, target, approval, evidence,
  and redaction class.

If any of those are absent, the step must remain `blocked`,
`waiting_approval`, `planned`, or `handoff_required`, and the action
ledger must record a fail-closed outcome.

## Handoff Truth

Browser-only vendor docs are reference-only. When a provider console or
browser docs surface is the real control plane, Aureline records
`ConsoleHandoffMetadata` with:

- target context;
- actor;
- originating step;
- evidence refs;
- return anchor when available;
- `external_control_plane_authoritative = true`;
- `in_product_mutation_supported = false`.

This keeps the boundary explicit and exportable instead of implying
in-product mutation parity.

## Drill Corpus

The protected fixture corpus covers:

- missing evidence;
- stale runbook versions;
- blocked approvals;
- browser-only vendor docs;
- redacted export bundles.

The validator refuses packets that omit required evidence declarations,
claim mutating success without approval or target identity, drop
deviation-note support, overclaim console parity, or export action
outcomes without reconstructable refs.
