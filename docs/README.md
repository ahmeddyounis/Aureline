# Docs index

Aureline is an open-source next-generation IDE (working name). The
repository is in its pre-implementation stage; these documents describe
the governance, ownership, and build discipline that precede source
code.

## Governance

- [`governance/dri_map.md`](./governance/dri_map.md) — DRI, backup
  owners, blocker aging, and narrowing authority.
- [`governance/decision_backlog.md`](./governance/decision_backlog.md)
  — seeded architecture decisions with freeze dates and default
  narrowing postures.
- [`governance/decision_workflow.md`](./governance/decision_workflow.md)
  — how decisions open, close, supersede, and narrow.
- [`governance/templates/`](./governance/templates/) — waiver,
  verification packet, and freeze-exception templates.

## Decision records

- [`adr/README.md`](./adr/README.md) — Architecture Decision Records
  (how and when to write one).
- [`adr/0000-template.md`](./adr/0000-template.md) — ADR template.
- [`rfc/README.md`](./rfc/README.md) — Requests for Comment (how and
  when to open one).
- [`rfc/0000-template.md`](./rfc/0000-template.md) — RFC template.

## Repository topology and build

- [`repo/topology.md`](./repo/topology.md) — package topology.
- [`repo/dependency_rules.md`](./repo/dependency_rules.md) — allowed
  dependency directions between packages.
- [`build/reproducible_build_baseline.md`](./build/reproducible_build_baseline.md)
  — pinned toolchain, bootstrap command, and build-identity record.

## Machine-readable registers

These live outside `docs/` because tooling reads them; the narrative
above is paired with a YAML form that is authoritative for automation:

- [`/artifacts/governance/ownership_matrix.yaml`](../artifacts/governance/ownership_matrix.yaml)
  — DRI, backup owners, and waivers.
- [`/artifacts/governance/decision_index.yaml`](../artifacts/governance/decision_index.yaml)
  — decision rows with freeze dates and default-if-unresolved postures.
- [`/artifacts/governance/package_inventory.yaml`](../artifacts/governance/package_inventory.yaml)
  — package topology and protected-path posture.
- [`/artifacts/governance/milestone_scorecard_template.yaml`](../artifacts/governance/milestone_scorecard_template.yaml)
  — per-milestone lane status.
- [`/artifacts/governance/governance_packet_template.yaml`](../artifacts/governance/governance_packet_template.yaml)
  — verification, benchmark-report, compatibility-report, claim-manifest,
  shiproom, and waiver-register packet families.
- [`/schemas/governance/`](../schemas/governance/) — schemas the YAML
  registers conform to.
