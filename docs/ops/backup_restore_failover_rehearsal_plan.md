# Backup, Restore, and Failover Rehearsal Plan

This plan publishes the alpha rehearsal contract for continuity claims:
which outage classes Aureline recognizes, who owns the drill, how often
it runs, what proof artifacts must exist, and which recovery actions are
allowed before any destructive repair or restore.

The canonical machine-readable taxonomy is
[`/artifacts/ops/outage_taxonomy_alpha.yaml`](../../artifacts/ops/outage_taxonomy_alpha.yaml).
The protected proof path is
[`/fixtures/ops/backup_restore_failover_rehearsal_cases/`](../../fixtures/ops/backup_restore_failover_rehearsal_cases/)
validated by
[`/ci/check_backup_restore_failover_alpha.py`](../../ci/check_backup_restore_failover_alpha.py).

## Scope

This plan covers desktop-local, helper-backed, provider-linked, and
managed-adjacent alpha lanes. It does not implement backup storage,
managed failover automation, status-page routing, or destructive reset
shortcuts. It freezes the rehearsal and evidence contract that product,
support, QA, release, and docs use when they claim continuity.

The plan composes with the existing ops contracts:

- failover banners and local-safe baseline:
  [`/docs/ops/failover_continuity_banner_contract.md`](./failover_continuity_banner_contract.md);
- planned maintenance and migration events:
  [`/docs/ops/maintenance_migration_failover_contract.md`](./maintenance_migration_failover_contract.md);
- incident workspace and runbook evidence:
  [`/docs/ops/incident_workspace_contract.md`](./incident_workspace_contract.md);
- control-plane and data-plane status:
  [`/docs/ux/control_data_plane_status_contract.md`](../ux/control_data_plane_status_contract.md);
- managed boundary truth:
  [`/docs/managed/region_residency_alpha.md`](../managed/region_residency_alpha.md).

## Outage Classes

The taxonomy distinguishes four required outage classes. A drill may
exercise more than one class, but every surfaced event must choose the
primary class that decides the first recovery posture.

| Class | Expected posture | First recovery action |
| --- | --- | --- |
| `local_core_continuity` | Local edit, save, search, Git, export, diagnostics, and cached docs remain available while optional helper, provider, or managed lanes degrade. | Continue local work and export a local continuity packet if review needs evidence. |
| `control_plane_impairment` | Identity, policy, entitlement, catalog, quota, tenant, region, residency, key, or endpoint authority is stale, unavailable, or failed over. Local work continues; managed authority-changing actions wait for reconnect, reauth, or boundary review. | Label cached authority as stale and review tenant, region, residency, key ownership, and endpoint identity before replay or reconnect. |
| `data_plane_impairment` | Live session traffic, remote attach streams, artifact bytes, prompt/response streams, upload/download replication, or presence traffic is impaired. Control-plane metadata may still explain the narrowed state. | Preserve local state, run metadata-only Doctor transport probes, and compare before replacing durable state. |
| `full_target_loss` | The target, workspace root, device, managed workspace, remote agent, or mounted filesystem cannot be reached or cannot be trusted as the same target. | Stop live target actions, locate or replace the target identity, then restore only from a reviewed source. |

## Owners

Roles are used so the plan stays durable across staffing changes.

| Role | Responsibility |
| --- | --- |
| Support-room owner | Accountable owner for the rehearsal, support projection, and field-readiness gap log. |
| Release captain | Decides whether rehearsal evidence is current enough for channel or cohort widening. |
| Supportability engineer | Runs the protected fixture lane, captures validation output, and updates repair or Doctor follow-ups. |
| Security/privacy reviewer | Confirms redaction, exact-build, raw-secret exclusion, and no overclaim of residency, key, or target authority. |
| Docs/comms owner | Keeps docs, examples, known limits, and support language aligned in the same change as taxonomy updates. |

## Cadence

| Event | Required cadence | Output |
| --- | --- | --- |
| Alpha continuity rehearsal | Monthly while alpha claims are active | Validator report plus support/release projection. |
| Release-candidate rehearsal | Before each release candidate or channel widening | Current proof packet, known-limit updates, and owner signoff. |
| Material boundary change | Same change set as any new managed, provider, backup, restore, or failover claim | Taxonomy, fixture, examples, and docs updates together. |
| Escaped incident review | Next readiness review after an incident escapes the seeded cases | New fixture case or explicit waiver with narrowed claim language. |

## Required Proof Artifacts

Each rehearsal must produce or refresh these artifacts:

- canonical taxonomy:
  [`/artifacts/ops/outage_taxonomy_alpha.yaml`](../../artifacts/ops/outage_taxonomy_alpha.yaml);
- proof packet:
  [`/artifacts/ops/backup_restore_failover_rehearsal_proof.md`](../../artifacts/ops/backup_restore_failover_rehearsal_proof.md);
- control-plane versus data-plane examples:
  [`/artifacts/ops/control_plane_vs_data_plane_examples.md`](../../artifacts/ops/control_plane_vs_data_plane_examples.md);
- protected fixture manifest and cases:
  [`/fixtures/ops/backup_restore_failover_rehearsal_cases/manifest.yaml`](../../fixtures/ops/backup_restore_failover_rehearsal_cases/manifest.yaml);
- validation hook:
  [`/ci/check_backup_restore_failover_alpha.py`](../../ci/check_backup_restore_failover_alpha.py);
- local validation report generated by:
  `python3 ci/check_backup_restore_failover_alpha.py --repo-root . --report target/backup_restore_failover_alpha_report.json`;
- metadata-only support/release projection generated by:
  `python3 ci/check_backup_restore_failover_alpha.py --repo-root . --render-support-projection`.

Proof packets must include exact-build identity, deployment profile,
affected outage class, control-plane state, data-plane state, local-safe
baseline posture, recovery actions attempted, excluded evidence classes,
and any manual follow-up. They must not include raw tenant names, raw
hostnames, raw URLs, raw endpoint credentials, raw logs, source bodies,
or secret material by default.

## Rehearsal Flow

1. Capture exact-build identity, release channel, deployment profile,
   and the fixture or seeded scenario under test.
2. Classify the outage using
   [`/artifacts/ops/outage_taxonomy_alpha.yaml`](../../artifacts/ops/outage_taxonomy_alpha.yaml).
3. Confirm the local-safe baseline is visible before recommending
   reconnect, restore, repair, or escalation.
4. Separate control-plane state from data-plane state. A healthy
   control plane does not prove live data traffic is healthy, and a
   failed control plane does not imply local work is lost.
5. Run read-only Project Doctor probes first. Mutating repairs must use
   preview, checkpoint, and reversal semantics from the repair contract.
6. Export a metadata-only packet when support or release review needs
   evidence.
7. If restore is required, perform restore destination review before
   any overwrite.
8. Record the outcome as recovered, recovered with limits, still
   blocked, or needs manual escalation.

## Restore Destination Review

Before a restore overwrites durable user-owned state, the review must
show:

- restore source class: authoritative backup, local checkpoint, sync
  replica, mirror cache, convenience export, or imported handoff packet;
- producer build, schema version, channel, and compatibility note;
- target identity and whether it matches the prior target;
- retained classes and replaced classes;
- checkpoint creation or a clear statement that no checkpoint exists;
- reversal class: exact, compensate, regenerate, manual, or audit-only;
- evidence classes included and excluded from support export.

An exact restore claim is allowed only when target identity, source
compatibility, and checkpoint evidence all match. Otherwise the surface
must say compatible restore, partial restore, evidence-only recovery, or
manual escalation.

## Review Gates

A rehearsal is green only when all of these are true:

- all four required outage classes are covered by protected fixtures;
- every class has an expected product posture and recovery action list;
- control-plane and data-plane examples distinguish authority metadata
  from live bytes or runtime traffic;
- local-safe continuation is visible for every class where local core is
  still available;
- full target loss stops live target actions and never implies automatic
  reattach, rerun, or exact restore without matching evidence;
- support projection is metadata-only and redaction-safe;
- docs, examples, fixture manifest, and validator move together when the
  taxonomy changes.

When any gate is red, release or cohort widening must either hold or
narrow the affected continuity claim.
