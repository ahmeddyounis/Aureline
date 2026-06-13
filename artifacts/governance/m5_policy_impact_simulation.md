# M5 Policy-Impact Simulation Packet — Artifact Summary

Canonical fixture: `fixtures/governance/m5_policy_impact_simulation/canonical_packet.yaml`

Schema: `schemas/governance/m5_policy_impact_simulation.schema.json`

Human-readable companion: `docs/governance/m5_policy_impact_simulation.md`

Producer: `aureline-records::m5_policy_simulation`
(`seeded_m5_policy_simulation_packet`).

Runtime surface whose identities are reused: `aureline-records::m5_records_policy`
(`seeded_m5_records_policy_packet`).

Policy companion: `aureline-policy::m5_exception_expiry`
(`seeded_m5_exception_expiry_packet`).

First support consumer: `aureline-support::m5_records_policy_governance`
(`M5RecordsPolicyGovernanceSupportExport::current`).

## Purpose

This artifact freezes the canonical pre-apply policy simulation for the durable
M5 policy-bearing artifact families. It compares the current policy against a
draft so admins can see what a proposed change makes different, what it leaves
unchanged, which saved objects it affects, and how expiry changes runtime
behavior before publishing the draft. It is metadata-only and carries no
credential bodies or raw provider payloads.

## Invariants enforced by `validate()`

- Schema version and record kind match the frozen constants.
- Every governed family covers both the `delete` and `export` actions, and each
  action diff's `changed` flag agrees with whether the current and draft
  outcomes differ.
- Each row's record class is the canonical class for its family, so the
  simulation and the runtime surface it previews refer to the same objects.
- A local-only family's draft never claims a managed hold, export, or delete.
- A changing expiry effect states its runtime consequence; a downgrade path
  states its before/after behavior and is visible before publish.
- A draft always advances the policy epoch.
- Every governed family is covered, and the machine-readable `impact_summary`
  roll-up agrees with the rows.

## Roll-up (canonical packet)

- Governed families: 8
- Families with changes: 6
- Families unchanged: 2
- Changed actions: 6
- Unchanged actions: 10
- Impacted objects: 8
- Families with an expiry change: 5
- Families with a downgrade: 3

## Regeneration

Regenerate the fixture whenever the seeded packet changes:

```
cargo run -p aureline-records --example dump_m5_policy_impact_simulation \
  > fixtures/governance/m5_policy_impact_simulation/canonical_packet.yaml
```

The crate test `checked_in_canonical_fixture_matches_seeded_packet` asserts the
checked-in fixture equals the seeded packet.
