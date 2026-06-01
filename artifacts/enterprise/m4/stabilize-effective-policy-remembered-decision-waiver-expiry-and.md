# Effective Policy, Remembered-Decision, Waiver-Expiry, and Exception-Preview UX — Stable Packet

- Packet: `policy:effective_policy_stabilize:default`
- Schema version: `1`
- Contract ref: `policy:effective_policy_stabilize:v1`
- Qualification: `stable` (derived, not asserted)
- Upstream policy simulation beta page defects: 0
- Stabilize defects: 0
- Withdrawn rows: 0
- Stable rows: all

## Lane coverage

| Simulation | Change class | Exception previews | Remembered decisions | Action-time snapshot |
|---|---|---|---|---|
| policy-bundle-change-sim-001 | `policy_bundle_change` | linked | linked (with drift) | present |
| settings-lock-change-sim-001 | `settings_lock_change` | — | linked | present |

## Evidence sources

- Policy simulation beta audit:
  `policy:simulation_exception_memory_beta:v1`
  — `docs/verification/policy_simulation_packet.md`

## Key invariants verified

1. The upstream `PolicySimulationBetaPage` audits with zero defects.
2. Both required change classes (`policy_bundle_change` and `settings_lock_change`) have at least one simulation with complete affected-surface truth.
3. Every exception and waiver has an explicit expiry horizon, named owner, renewal path, and revocation path; dashboard buckets reflect current lifecycle status.
4. Every remembered decision is narrowly bound (actor, object, action family, environment, time horizon) and any drift is explained with at least one typed invalidation reason.
5. Simulation records link to overlapping exceptions and remembered decisions via `exception_preview_refs` and `remembered_decision_preview_refs` so the exception-preview UX can be populated from typed records.
6. Action-time policy snapshots preserve historical truth (`preserves_historical_truth: true`) for support and admin exports.

## Hard guardrail — withdrawal condition

The following forces `Withdrawn` immediately and cannot be overridden:

- A `RawPrivateMaterialExposed` defect in the upstream beta page
  (narrow reason: `raw_private_material_exposed`).

## Canonical paths

- Doc: `docs/enterprise/m4/stabilize-effective-policy-remembered-decision-waiver-expiry-and.md`
- Runtime owner: `aureline_policy::stabilize_effective_policy_remembered_decision_waiver_expiry_and`
- Fixtures: `fixtures/enterprise/m4/stabilize-effective-policy-remembered-decision-waiver-expiry-and/`
- Schema: `schemas/enterprise/stabilize-effective-policy-remembered-decision-waiver-expiry-and.schema.json`
