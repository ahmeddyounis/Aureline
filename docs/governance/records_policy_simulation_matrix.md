# Records, Deletion, Chronology, and Policy Simulation Matrix

This page is the human-readable companion to the canonical governance packet at
`artifacts/governance/records_policy_simulation_matrix.yaml`.

## Why this packet exists

Managed/provider/support surfaces now create or retain durable artifacts that
must not hide:

- what record class exists,
- whether the platform has local-only or managed authority,
- whether delete or export is complete, queued, partial, policy-retained, or
  outside platform scope,
- what chronology the evidence preserves,
- and whether policy simulation, exception preview, and remembered-decision
  revalidation are actually in scope.

This packet is the one control source for those answers across product,
CLI/headless, help/docs, support export, and release evidence.

Each governed row now also carries a family-specific record-class descriptor, a
retention-policy assignment, a chronology row id with imported-versus-live
disclosure, and the concrete producer `record_kind`s that must validate
against the row's class.

## Required contract

- Local-only rows must never imply remote delete, remote export, or remote
  legal hold.
- Claimed M5 durable artifact families must bind to their concrete packet
  `record_kind`s rather than generic managed-state placeholders.
- Chronology must preserve absolute time, local context, source, actor lineage,
  and imported-versus-live distinction.
- Policy-bearing rows must point to one policy owner and the four stable
  remembered-decision reapproval triggers:
  `target_drift`, `policy_drift`, `version_drift`, and `authority_drift`.
- Stale or missing proof narrows the affected row immediately; release-blocking
  rows would hold publication.

## First consumers

- Product surfaces render row qualification, record class, retention
  assignment, and delete/export honesty directly.
- CLI/headless explain output uses the typed row projection instead of cloning
  prose.
- Help/docs cite the packet by reference.
- Support export embeds the packet and cross-checks it against the stable policy
  snapshot while projecting the same record/retention/chronology fields.
- Release evidence cites the packet directly for governed managed-depth rows.

## Current narrowing

The current packet intentionally keeps `browser_handoff_manifest` at
`needs_review`: the row is local-only and honest about that boundary, but its
proof packet is stale, so browser-handoff claims must remain narrowed until the
proof is refreshed.
