# M5 Topology Inspector Evidence

This artifact freezes the support/export contract for M5 runtime-heavy host
topology.

## Covered host lanes

- notebook kernel
- preview dev server
- profiler/replay session
- data/API connector session
- provider-run session
- AI/tool broker lane
- pipeline viewer
- remote workspace agent

## Guaranteed export truth

- every visible result remains mapped back to one or more host lanes
- host descriptors stay plain-language: family, role, locality, boundary badge,
  health, and fault-domain id
- restart-budget cards preserve strike count, strike window, budget state,
  preserved checkpoints, stale visible artifacts, and next quarantine trigger
- partial truth remains explicit with stable labels: `stale`, `rebuilding`,
  `provider unavailable`, `reconnecting`, `local fallback`, and
  `captured snapshot`
- the packet remains metadata-only and does not embed raw payload bodies,
  prompts, secrets, paths, or dumps

## Checked-in references

- Schema: `schemas/support/topology_inspector.schema.json`
- Doc: `docs/support/topology_inspector.md`
- Fixture: `fixtures/support/m5/topology_inspector/packet.json`
- Runtime source: `crates/aureline-runtime/src/topology_inspector/mod.rs`
- Support projection: `crates/aureline-support/src/fault_domain_views/mod.rs`
