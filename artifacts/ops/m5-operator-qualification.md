# Operator-surface qualification packet

## Release evidence

This artifact documents the operator-surface qualification packet produced by
`crates/aureline-support/src/m5_operator_qualification/`. The packet binds the
M5 operator-surface proof sources into one per-family claim verdict and
auto-narrows a claimed operator family when its ownership / freshness /
continuity proof is stale or failing, so no operator lane stays fully supported
while its overview / triage / handoff / maintenance / boundary proof has silently
aged out.

## Record family

| Record | Kind | Schema | Version |
|---|---|---|---|
| `OperatorQualificationPacket` | `m5_operator_qualification_packet` | `schemas/ops/m5-operator-qualification.schema.json` | 1 |

- Packet id: `m5-operator-qualification:packet:0001`
- As of: `2026-06-22T00:00:00Z`
- Coverage: 10 claimed operator families × 9 proof dimensions
- Overall: all 7 invariants hold; 10/10 families fully supported in the canonical binding

## Proof dimensions (release-evidence rows)

Each dimension carries its own freshness state and failure mode. Service
ownership, runbook-step authority, handoff-bundle fidelity, maintenance/failover
communication, and embedded-boundary honesty are present as explicit rows; the
canonical-matrix binding, runbook-step authority, and embedded-boundary honesty
are the three critical safety rows.

| Dimension | Critical | Primary proof source |
|---|---|---|
| `canonical_matrix_binding` | yes | `schemas/ops/m5-operator-surfaces.schema.json` |
| `overview_truth` | no | `schemas/ops/m5-operator-boards.schema.json` |
| `triage_truth` | no | `schemas/ops/m5-triage-inbox.schema.json` |
| `action_plan_continuity` | no | `schemas/ops/m5-action-plans.schema.json` |
| `handoff_bundle_fidelity` | no | `schemas/ops/m5-handoff-digests.schema.json` |
| `service_ownership` | no | `schemas/ops/m5-response-panes.schema.json` |
| `runbook_step_authority` | yes | `schemas/ops/m5-response-panes.schema.json` |
| `maintenance_failover_communication` | no | `schemas/ops/m5-maintenance-windows.schema.json` |
| `embedded_boundary_honesty` | yes | `schemas/ops/m5-embedded-dashboards.schema.json` |

## Auto-narrow contract (all must pass)

1. `dimension_set_complete` — every proof dimension resolves to exactly one global proof.
2. `every_claimed_family_present` — every claimed operator family has a qualification row.
3. `every_family_anchored_to_canonical_matrix` — every family claims the canonical-matrix binding, so dashboards and queues resolve through the same frozen matrix.
4. `no_fully_supported_family_with_nonfresh_proof` — a family stays fully supported only when every dimension it claims is fresh.
5. `every_downgrade_is_named` — every narrowed or blocked family names the responsible dimension(s).
6. `critical_failure_blocks_claim` — a failing or missing critical dimension blocks every family that claims it.
7. `release_evidence_dimensions_present` — service-ownership, runbook-step authority, handoff-bundle fidelity, maintenance/failover communication, and embedded-boundary honesty rows are all present.

## Family coverage

Generated and pinned in `fixtures/ops/m5-operator-qualification/canonical_packet.json`.
In the canonical binding every upstream lane's invariants hold and every proof is
captured on the evaluation date, so every family is fully supported. The packet
narrows or blocks automatically when a proof goes stale, failing, or missing.

| Family | Primary dimension | Canonical support |
|---|---|---|
| operational_overview_board | overview_truth | fully_supported |
| triage_inbox | triage_truth | fully_supported |
| action_plan | action_plan_continuity | fully_supported |
| handoff_bundle | handoff_bundle_fidelity | fully_supported |
| shift_digest | handoff_bundle_fidelity | fully_supported |
| service_ownership_strip | service_ownership | fully_supported |
| runbook_step_card | runbook_step_authority | fully_supported |
| maintenance_notice | maintenance_failover_communication | fully_supported |
| failover_notice | maintenance_failover_communication | fully_supported |
| embedded_boundary_state | embedded_boundary_honesty | fully_supported |

## Verification

Emit the canonical packet:

```sh
cargo run -p aureline-support --example dump_m5_operator_qualification
cargo run -p aureline-support --example dump_m5_operator_qualification -- --lines
```

Run the freeze gate (rebuilds the packet from in-code proof sources and asserts
it equals the fixture, plus proves auto-narrowing on stale proof):

```sh
cargo test -p aureline-support --test m5_operator_qualification
```

Run the unit contract suite:

```sh
cargo test -p aureline-support m5_operator_qualification
```

Validate the fixture against the schema:

```sh
python3 -c "import json,jsonschema; jsonschema.validate(json.load(open('fixtures/ops/m5-operator-qualification/canonical_packet.json')), json.load(open('schemas/ops/m5-operator-qualification.schema.json')))"
```

## Risks and follow-ups

- **The canonical binding measures in-code lane invariants, not live CI evidence
  ages.** The release-automation entry point (`project_operator_qualification`)
  accepts real capture stamps and budgets; wiring the live evidence store to feed
  those stamps is incremental follow-up.
- **The freshness budget is uniform (30 days).** Per-dimension budgets are a
  field on the proof input; tightening them per surface is a tuning follow-up.
- **About/help/service-health/compatibility consume the packet, but the live
  wiring of each surface to read `families[].support` is incremental** as those
  surfaces mature; the packet and its export-safe line projection are the stable
  contract they bind to.
