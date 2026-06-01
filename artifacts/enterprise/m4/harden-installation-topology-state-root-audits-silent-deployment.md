# Hardened Installation Topology — Proof Packet

- Packet: `install:harden-installation-topology:seeded:0001`
- Schema version: `1`
- Contract ref: `install:harden_installation_topology:v1`
- Qualification: `stable` (derived, not asserted)
- Defects: 0
- Withdrawn rows: 0
- Stable rows: all 2 managed-fleet rows

## Lane coverage

| Row | Platform | Ring | Updater owner | Binary root | State roots audited | Policy source | Fleet evidence | Admin view |
|---|---|---|---|---|---|---|---|---|
| `harden-topology.managed.windows.stable` | `windows` | `pilot` | `managed_fleet` | `per_machine_program_area` | 5 | `fleet.policy_source.windows.gpo.managed.stable` | all 7 | complete |
| `harden-topology.managed.airgap.stable` | `air_gap_bundle_target` | `broad` | `admin` | `offline_bundle_extracted_program_area` | 2 | `fleet.policy_source.airgap.offline_bundle.stable` | all 7 | complete |

## Silent-deployment coverage

| Row | Support class | Limits declared | Return codes named |
|---|---|---|---|
| `harden-topology.silent.windows.stable` | `managed_only` | yes (3) | yes (5) |
| `harden-topology.silent.airgap.stable` | `full` | yes (3) | yes (5) |

## Fleet evidence classes verified (per managed row)

All seven required evidence classes are present on both managed rows:

1. `ring_assignment`
2. `exact_build_inventory`
3. `managed_package_report`
4. `policy_root`
5. `rollback_target`
6. `verification_status`
7. `support_export`

## Key invariants verified

1. Every managed-fleet row has a non-empty `tenant_ref` anchoring it to an organization or tenant.
2. Every managed-fleet row names a `rollout_ring_class` from the closed vocabulary.
3. Every managed-fleet row names an `updater_owner_class` and `binary_root_class`.
4. Every managed-fleet row has a non-empty `policy_source_ref`.
5. Every managed-fleet row has at least one `state_root_audit` entry with `isolation_class`, `review_class`, and `exposed_in_admin_view`.
6. Every managed-fleet row carries all seven required `fleet_evidence` classes.
7. Every managed-fleet row has `admin_view_complete: true`.
8. Every silent-deployment row has `limits_declared: true` with at least one non-empty limit label.
9. Every silent-deployment row has `return_code_families_named: true` with at least one ref.
10. `raw_private_material_excluded: true` on every support export; no tenant credentials, raw policy bodies, binary paths, or secret material cross this boundary.

## Hard guardrails — withdrawal condition

One condition forces `Withdrawn` immediately and cannot be overridden:

- Any managed-fleet row with `admin_view_complete: false`
  (narrow reason: `admin_view_incomplete`). The auditor returns immediately
  with this single defect and skips all other checks.

## Qualification narrowing table

| Condition | Narrowing |
|---|---|
| `admin_view_complete: false` on any managed row | `Withdrawn` (immediate) |
| No managed-fleet rows present | `Preview` |
| Silent deployment limits not declared | `Beta` |
| Return-code families not named | `Beta` |
| All conditions met | `Stable` |

## Canonical paths

- Doc: `docs/enterprise/m4/harden-installation-topology-state-root-audits-silent-deployment.md`
- Runtime owner: `aureline_install::harden_installation_topology_state_root_audits_silent_deployment`
- Fixtures: `fixtures/enterprise/m4/harden-installation-topology-state-root-audits-silent-deployment/`
- Schema: `schemas/enterprise/harden-installation-topology-state-root-audits-silent-deployment.schema.json`
