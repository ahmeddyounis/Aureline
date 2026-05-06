# Shared state-machine rules and cross-object state catalog

This document freezes the shared vocabulary Aureline uses to describe
high-level lifecycle posture across long-lived objects: workspaces,
extensions, remote sessions, collaboration sessions, AI actions,
update/rollback flows, and long-running background work.

It is a contract layer. Object-specific state machines remain
authoritative for detailed semantics; this document defines the shared
**state classes** and **transition classes** that UI surfaces, support
exports, automation logs, and recovery tools reuse to prevent semantic
drift.

The document is normative. If it disagrees with the PRD, Technical
Architecture Document, Technical Design Document, UI / UX Spec, or Design
System Style Guide, those source documents win and this document plus
its companion artifacts update in the same change.

## Companion artifacts

- [`/artifacts/runtime/state_catalog.yaml`](../../artifacts/runtime/state_catalog.yaml)
  — machine-readable state-class definitions plus per-domain mapping
  rows.
- [`/schemas/runtime/state_transition_rule.schema.json`](../../schemas/runtime/state_transition_rule.schema.json)
  — boundary schema for transition rules and worked state-machine
  examples.
- [`/fixtures/runtime/state_machine_examples/`](../../fixtures/runtime/state_machine_examples/)
  — worked examples showing rollback-capable and quarantinable objects.

## Upstream contracts this contract composes with

This contract does not replace or rename upstream state owners. It
projects and composes:

- [`/docs/architecture/lifecycle_statecharts.md`](../architecture/lifecycle_statecharts.md)
  and [`/schemas/governance/lifecycle_state.schema.json`](../../schemas/governance/lifecycle_state.schema.json)
  — protected-object lifecycle and recovery-transition vocabulary.
- [`/docs/runtime/resource_governor_contract.md`](./resource_governor_contract.md)
  and [`/artifacts/runtime/resource_governor_thresholds.yaml`](../../artifacts/runtime/resource_governor_thresholds.yaml)
  — shared “health projection” vocabulary for `ready`/`degraded`-style
  posture.
- [`/docs/recovery/recovery_rung_matrix.md`](../recovery/recovery_rung_matrix.md)
  and [`/artifacts/recovery/recovery_rungs.yaml`](../../artifacts/recovery/recovery_rungs.yaml)
  — quarantine and rollback ladder semantics.

## Three-layer state model (frozen)

Every long-lived object that is rendered outside its owning subsystem
MUST expose state in three distinct layers:

1. **User-visible state class** — one token from the closed set below.
   This is the primary label used on UI chrome, exports, and logs.
2. **Diagnostic substate** — optional, object-specific detail that
   explains *why* the state class applies (policy reason, failure reason,
   dependency, budget gate). Diagnostic substate MUST NOT replace the
   user-visible state class.
3. **Evidence reconstruction class** — how the current state was
   obtained: authoritative live truth vs. imported history vs. evidence-
   only reconstruction. Consumers MUST render reconstruction posture when
   it is not authoritative live.

The shared artifact
[`/artifacts/runtime/state_catalog.yaml`](../../artifacts/runtime/state_catalog.yaml)
is the vocabulary of record for these axes.

## User-visible state classes (frozen)

Surfaces MUST use these tokens (or map to them) whenever they present a
high-level lifecycle posture outside the owning subsystem.

| State class | Meaning (summary) |
|---|---|
| `initializing` | Admission or setup is in progress and may still be denied or degraded. |
| `ready` | Object is usable for its declared scope under current authority/policy/trust. |
| `degraded` | Object remains usable but with narrowed capability, reduced truth, or impaired dependency. |
| `blocked` | Progress is paused and requires an external unblock (user action, dependency, or authority). |
| `pending_review` | A reviewer decision is required before the object may advance to a mutating or widening step. |
| `applying` | A mutating or recovery action is in flight; outcome is not yet known. |
| `rollback_available` | The object is in a reversible posture with a retained rollback handle and a declared expiry/limit. |
| `quarantined` | The object is isolated by supervisor/policy/recovery ladder and cannot silently resume. |
| `recovered` | A recovery/rollback/quarantine-clear action succeeded; follow-up revalidation may still be required. |
| `stale` | Last-known-good projection is being shown; freshness/identity is not current. |
| `out_of_policy` | Policy/trust/provenance rules deny the requested posture; progress is blocked until policy changes or scope narrows. |
| `removed` | The object is not live (closed/removed/retired). Evidence may remain available for audit/export. |

Rules (frozen):

1. A surface MUST NOT mint new user-visible state names. If an owner has
   a finer-grained machine, it maps to one of these tokens plus a
   diagnostic substate.
2. `blocked`, `pending_review`, and `out_of_policy` are distinct.
   Collapsing them into one “stopped” posture is non-conforming.
3. `removed` is a terminal posture for *liveness*, not for evidence.
   Removal MUST NOT imply evidence deletion unless a retention policy
   record says so.

## Transition legend (frozen)

Object-specific state machines use object-specific transition names and
may add additional steps, but whenever they represent one of the
transition classes below they MUST preserve the class label (in logs,
exports, and help text) and satisfy the shared invariants.

| Transition class | Meaning | Invariants |
|---|---|---|
| `start` / `stop` | Begin or end a live object posture. | `stop` revokes live authority and leaves reconstructible breadcrumbs. |
| `suspend` / `resume` | Pause progress without claiming the object is removed. | Suspension preserves durable work; resume revalidates authority/policy/trust. |
| `retry` | Try again without erasing prior evidence. | Retry is append-only; cites predecessor transition and idempotency/checkpoint ref. |
| `abandon` | Explicitly give up on a non-terminal object. | Abandon preserves evidence and discloses what was not applied/recovered. |
| `rollback` | Return through a declared rollback/compensating path. | Rollback cites a rollback handle or declared reversal class; no “exact reversal” claim without proof. |
| `quarantine` | Isolate the object under a recovery rung or supervisor/policy action. | Quarantine cannot silently lift; clear requires explicit actor authority. |
| `handoff` | Ownership/authority for the object’s continuation moves to a new owner. | Handoff preserves lineage: prior owner refs remain linkable and audit-visible. |
| `imported_history` | State is imported from a capture/export rather than observed live. | Imported history is segregated from live truth; consumers label it as imported. |
| `evidence_only_reconstruction` | State is reconstructed from evidence only (logs, audits, snapshots) without authoritative live confirmation. | Consumers cap claims and render an explicit reconstruction posture; no mutating continuation without revalidation. |

These classes align with (and do not replace) the recovery-transition
classes in the protected-object lifecycle vocabulary. When both apply,
the protected lifecycle’s recovery class remains authoritative for
preview/checkpoint/actor rules, while this contract governs shared
cross-object naming and projection.

## Cross-object mapping rows (seed set)

Mapping is a projection. Owners keep their fine-grained state machine;
shared surfaces render the mapped state class plus optional diagnostic
substate.

The mapping rows in
[`/artifacts/runtime/state_catalog.yaml`](../../artifacts/runtime/state_catalog.yaml)
are the source of truth. If a domain’s owning contract cannot map a
state without losing essential meaning, it MUST document the exception
in its owning contract and cite why it is not representable.

## Change rules (frozen)

1. Adding a new user-visible state class, transition class, or evidence
   reconstruction class is additive-minor only when the doc, YAML
   catalog, schema, and at least one fixture are updated in the same
   change.
2. Repurposing an existing token is breaking and requires a governance
   decision row.
