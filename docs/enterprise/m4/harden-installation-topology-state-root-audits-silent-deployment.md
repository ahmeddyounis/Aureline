# Harden installation topology, state-root audits, silent deployment, and fleet rollout evidence for managed lanes

This lane closes the gap between the install-topology alpha rows and the
admin-facing accountability surface for managed, self-hosted, and air-gapped
deployments. It produces a single inspectable
[`HardenInstallTopologyPage`][runtime] that every diagnostics, Help/About,
support-export, and CLI surface can consume without reading localized prose or
external deployment notes.

The runtime owner is
`aureline_install::harden_installation_topology_state_root_audits_silent_deployment`.

## What the page asserts

For each managed-fleet or air-gapped install row the page exposes:

| Field | Type | Description |
|---|---|---|
| `tenant_ref` | opaque ref | Anchors the row to an organization or tenant without raw credentials. |
| `rollout_ring_class` | closed token | `canary`, `pilot`, `broad`, or `lts`. |
| `updater_owner_class` | closed token | `managed_fleet`, `admin`, `external_package_manager`, etc. |
| `binary_root_class` | closed token | `per_machine_program_area`, `offline_bundle_extracted_program_area`, etc. |
| `policy_source_ref` | opaque ref | GPO path token, MDM profile id, or config-profile source ref. |
| `state_root_audit` | array | One entry per durable state root with `isolation_class`, `review_class`, and `exposed_in_admin_view`. |
| `fleet_evidence` | array | All seven required evidence class tokens. |
| `silent_deployment_class` | closed token | `full`, `partial`, `managed_only`, or `unsupported`. |
| `admin_view_complete` | bool | True when the admin or support view can fully identify this install without prose. |

## Contract

For the `Stable` claim to hold, **all ten** of the following conditions must
be verified simultaneously:

1. **Tenant identity present** — every managed-fleet row has a non-empty `tenant_ref`.
2. **Ring named** — every managed-fleet row has a `rollout_ring_class` from the closed vocabulary.
3. **Updater owner named** — every managed-fleet row names an `updater_owner_class`.
4. **Binary root named** — every managed-fleet row names a `binary_root_class`.
5. **Policy source named** — every managed-fleet row has a non-empty `policy_source_ref`.
6. **State roots audited** — every managed-fleet row has at least one `state_root_audit` entry.
7. **Fleet evidence complete** — every managed-fleet row carries all seven required `fleet_evidence` classes: `ring_assignment`, `exact_build_inventory`, `managed_package_report`, `policy_root`, `rollback_target`, `verification_status`, and `support_export`.
8. **Admin view complete** — every managed-fleet row has `admin_view_complete: true`.
9. **Silent deployment limits declared** — every silent-deployment row has `limits_declared: true` with at least one non-empty limit label.
10. **Return codes named** — every silent-deployment row has `return_code_families_named: true` with at least one family ref.

## Required behavior

`validate_harden_install_topology_page` rejects a page when its `findings`
list is non-empty.

`audit_harden_install_topology_page` runs the combined check and returns a
typed `Vec<HardenInstallTopologyDefect>`. Each defect carries a closed
`narrow_reason_token` and an export-safe `note`. The absence of defects is the
`Stable` claim.

One condition forces `Withdrawn` immediately and cannot be overridden:

- Any managed-fleet row with `admin_view_complete: false` (narrow reason:
  `admin_view_incomplete`). The auditor returns immediately with this single
  defect and skips all remaining checks.

A missing managed-fleet row coverage narrows to `Preview` rather than `Beta`
because the structural gap prevents any verifiable claim for the managed lane.

Silent deployment limits not declared narrows to `Beta`.

## Fleet-evidence classes

Seven classes are required on every managed-fleet row:

| Class token | Description |
|---|---|
| `ring_assignment` | Rollout ring is present in the row. |
| `exact_build_inventory` | Exact-build inventory identity is present. |
| `managed_package_report` | Managed-package report identity is present. |
| `policy_root` | Policy-root identity is present. |
| `rollback_target` | Rollback target identity is present. |
| `verification_status` | Last verification status is present. |
| `support_export` | Support-export back-reference is present. |

## State-root audit entries

Each `state_root_audit` entry on a managed-fleet row carries:

| Field | Type | Description |
|---|---|---|
| `state_root_ref` | opaque ref | State-root ref from the install topology or state-root map. |
| `isolation_class` | closed token | `channel_owned`, `admin_policy_owned`, `mirror_metadata_owned`, etc. |
| `review_class` | closed token | `admin_policy_review_required`, `mirror_verification_review_required`, etc. |
| `exposed_in_admin_view` | bool | True when this root is visible in the admin or support view. |
| `contains_secret_material` | bool | True when the root may contain secret material (metadata only — never the content). |

## Qualification narrowing table

| Condition | Narrowing |
|---|---|
| `admin_view_complete: false` on any managed row | `Withdrawn` (immediate) |
| No managed-fleet rows present | `Preview` |
| Silent deployment limits not declared | `Beta` |
| Return-code families not named | `Beta` |
| All conditions met | `Stable` |

## Local-core continuity

This lane does not block local-core work. The audit covers only managed-fleet
and air-gapped rows; per-user, side-by-side, and portable rows are outside the
scope of this module. Failure or narrowing in the managed lane does not change
the qualification of adjacent local-core rows.

## Boundary

The following material stays outside this packet's support boundary:

- Raw tenant credentials, domain names, or email addresses.
- Raw policy body content, ADMX files, or MDM rule expressions.
- Raw binary paths, install-tree paths, or state-root filesystem paths.
- Raw secret material (keychain tokens, certificates, private keys).

Every exported field carries a closed-vocabulary token, an opaque ref, a
plain-language label, a count, a boolean, or a schema-version integer.

## Truth source

The seeded proof packet is `seeded_harden_install_topology_page()` in
[`/crates/aureline-install/src/harden_installation_topology_state_root_audits_silent_deployment/mod.rs`](../../../crates/aureline-install/src/harden_installation_topology_state_root_audits_silent_deployment/mod.rs).

That function is the single inspectable record for this lane. Dashboards,
Help/About surfaces, and support exports should ingest it rather than cloning
status text or maintaining parallel fleet deployment notes.

## Canonical paths

- Runtime owner: `aureline_install::harden_installation_topology_state_root_audits_silent_deployment`
- Artifact: `artifacts/enterprise/m4/harden-installation-topology-state-root-audits-silent-deployment.md`
- Fixtures: `fixtures/enterprise/m4/harden-installation-topology-state-root-audits-silent-deployment/`
- Schema: `schemas/enterprise/harden-installation-topology-state-root-audits-silent-deployment.schema.json`

[runtime]: ../../../crates/aureline-install/src/harden_installation_topology_state_root_audits_silent_deployment/mod.rs
