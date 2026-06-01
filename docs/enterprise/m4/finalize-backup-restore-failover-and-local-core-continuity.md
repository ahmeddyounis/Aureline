# Finalize backup, restore, failover, and local-core continuity packets for claimed enterprise profiles

This lane makes backup state, restore procedure testing, failover behavior, and
local-core continuity visible and verifiable for every claimed enterprise
deployment profile. Product copy, security review, support exports, and release
packets can all answer: what is backed up for this profile, when was the restore
procedure last drilled, what happens to local editing during a managed-connectivity
outage, and whether local-core capabilities are explicitly preserved. The runtime
owner is
`aureline_policy::finalize_backup_restore_failover_and_local_core_continuity`.

The packet does **not** re-derive raw backup artefacts, raw hostnames, raw
tenant identifiers, raw key bytes, or raw credentials. All references are opaque
tokens or export-safe labels. This packet adds the finalize invariants needed for
a single evidence packet that can be ingested by dashboards, docs, Help/About
surfaces, and support exports without cloning status text.

## Contract

For the stable claim to hold, **all seven** of the following conditions must be
verified simultaneously:

1. **All five enterprise profiles covered** — at least one row exists for each
   of: `individual_local`, `self_hosted`, `enterprise_online`, `air_gapped`,
   and `managed_cloud`.
2. **No failover behavior blocks local-core work** — no row carries
   `failover_behavior: local_core_blocked`; enterprise features must not block
   local editing, save, search, or Git by default during any failover scenario.
3. **Local-core continuity explicitly stated** — every row carries a non-empty
   `local_core_posture_token`; no row carries `local_core_posture: blocked_by_default`.
4. **Backup state verified for enterprise profiles** — every non-`individual_local`
   row carries `backup_state: current`; `unverified` or `overdue` states narrow
   the row.
5. **Restore procedure drilled for enterprise profiles** — every
   non-`individual_local` row carries `restore_test_posture: tested_and_current`;
   `never_tested` or `tested_overdue` states narrow the row.
6. **Tenant/region ownership and policy source declared** — every
   non-`individual_local` row carries non-empty `tenant_region_owner_ref`,
   `policy_source_ref`, and `dependency_class_token`.
7. **Failover behavior declared for enterprise profiles** — every
   non-`individual_local` row carries a non-empty `failover_behavior_token`.

## Required behavior

`validate_backup_restore_failover_page` rejects a page when its `defects` list
is non-empty.

`audit_backup_restore_failover_page` runs the combined check and returns a typed
`Vec<BackupRestoreFailoverDefect>`. Each defect carries a closed
`narrow_reason_token` and an export-safe `note`. The absence of defects is the
stable claim.

One condition forces `Withdrawn` immediately and cannot be overridden:

- A row where `failover_behavior: local_core_blocked` or
  `local_core_posture: blocked_by_default` is declared (narrow reason:
  `local_core_blocked_by_failover`). Enterprise features must not block
  local-core work by default; any profile that does so cannot qualify
  claimable.

A missing required enterprise profile narrows to `Preview` rather than `Beta`
because the coverage gap prevents any verifiable claim for that profile.

## Enterprise profiles

| Profile token | Description |
| --- | --- |
| `individual_local` | Desktop-local, single-user, no managed control plane. Backup and restore are not applicable. |
| `self_hosted` | Customer-operated control plane with customer-managed keys and region. |
| `enterprise_online` | Hybrid remote-attach with vendor-provided managed services. |
| `air_gapped` | Offline-capable air-gapped mirror; no public egress. |
| `managed_cloud` | Vendor-operated SaaS with vendor-managed keys by default. |

All five profiles must be covered for a stable claim.

## Backup state tokens

| Token | Description |
| --- | --- |
| `current` | Backup is within its declared retention window and has been verified. |
| `pending` | Backup is scheduled but not yet complete. |
| `overdue` | Backup is past its scheduled window without a completed run. |
| `unverified` | A backup artefact exists but has not been verified against the restore procedure. |
| `not_applicable` | No enterprise data scope exists for this profile. |

## Restore test posture tokens

| Token | Description |
| --- | --- |
| `tested_and_current` | Restore procedure was drilled and is within its declared validity window. |
| `tested_overdue` | Restore procedure was previously drilled but is now past its validity window. |
| `never_tested` | Restore procedure has never been drilled. |
| `not_applicable` | No restore path for this profile. |

## Failover behavior tokens

| Token | Description |
| --- | --- |
| `local_core_preserved` | Local editing and core capabilities remain fully operational during failover. |
| `degraded_managed_only` | Managed capabilities are degraded; the local editing floor is explicitly preserved. |
| `local_core_may_be_impaired` | Failover may temporarily impair some local-core capabilities but does not block editing. |
| `local_core_blocked` | Failover blocks local-core capabilities by default. **Hard guardrail — withdraws the row.** |
| `not_applicable` | No managed failover path; all work is local. |

## Local-core continuity posture tokens

| Token | Description |
| --- | --- |
| `preserved` | The local editing floor is fully preserved for this profile. |
| `impaired_managed_dependency` | A managed dependency may degrade some capabilities, but the local editing floor is intact. |
| `blocked_by_default` | The profile blocks local-core capabilities by default. **Hard guardrail — withdraws the row.** |

## Seeded coverage

The seeded page covers all five profiles (5 rows total). Each row carries a
fully declared backup state, restore test posture, failover behavior, and
explicit local-core continuity posture. The seeded page qualifies stable with
zero defects.

## Canonical paths

- Runtime owner: `aureline_policy::finalize_backup_restore_failover_and_local_core_continuity`
- Artifact: `artifacts/enterprise/m4/finalize-backup-restore-failover-and-local-core-continuity.md`
- Contract ref: `policy:backup_restore_failover_continuity:v1`
- Fixtures: `fixtures/enterprise/m4/finalize-backup-restore-failover-and-local-core-continuity/`
- Schema: `schemas/enterprise/finalize-backup-restore-failover-and-local-core-continuity.schema.json`
