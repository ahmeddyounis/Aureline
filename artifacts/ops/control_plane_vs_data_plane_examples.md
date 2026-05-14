# Control-Plane vs Data-Plane Outage Examples

This is the human-readable companion to
[`/artifacts/ops/outage_taxonomy_alpha.yaml`](./outage_taxonomy_alpha.yaml).
It gives reviewers concrete examples for classifying an outage without
overstating what is healthy, recoverable, or safe to replay.

## Plane Split

| Plane | Examples | What healthy means | Common failure | Safe default |
| --- | --- | --- | --- | --- |
| Local core | Editor buffers, local save, local search, local Git, local export, local diagnostics | The installed desktop can keep working against local state. | Local cache corruption, crash loop, low disk, missing workspace root. | Preserve local evidence, open safe mode or Doctor, and avoid destructive reset before export or checkpoint review. |
| Control plane | Identity, policy, entitlement, catalog, quota, tenant routing, region assignment, key-state, orchestration metadata | Authority and metadata are fresh enough to decide whether managed actions may run. | Stale signed policy, failed auth refresh, region failover, key route unknown, quota service down. | Continue local work, label cached authority stale, require boundary review or reauth before replay. |
| Data plane | Remote attach traffic, artifact bytes, upload/download replication, prompt/response streams, presence fan-out, live runtime IO | Live bytes and runtime streams are flowing under the reviewed authority. | Remote stream down, upload replication stuck, prompt dispatch unavailable, presence relay drain. | Freeze local state, label cached data, block live writes or require manual rerun after reconnect. |
| Target authority | Workspace root, remote agent, mounted filesystem, managed workspace identity, device identity | The product can prove it is still acting on the intended target. | Missing root, detached device, expired managed workspace, replaced remote agent, untrusted mount. | Stop live target actions, locate or replace target identity, then restore from a reviewed source. |

## Example Classifications

| Scenario | Primary class | Control-plane state | Data-plane state | Expected product posture |
| --- | --- | --- | --- | --- |
| Managed AI broker unavailable; local edit, Git, export, diagnostics, and cached docs keep working. | `local_core_continuity` | Optional managed broker unavailable. | Local data path healthy; broker dispatch unavailable. | Show retained local-safe baseline, queue only explicitly idempotent managed writes, and refuse authority-changing paid dispatch. |
| Regional failover changes region, key route, and endpoint identity. | `control_plane_impairment` | Failed over and boundary review required. | Local data path healthy; managed remote attach waits. | Require review of tenant, region, residency, key ownership, and endpoint identity before managed replay or reconnect. |
| Collaboration relay drains active sessions and blocks new joins. | `data_plane_impairment` | Maintenance notice is current and explains the drain. | Relay traffic is draining; local state remains available. | Let existing safe local work continue, block new live joins, and require reconnect or manual rerun after the drain. |
| Workspace root or managed target is missing after reopen. | `full_target_loss` | Control plane may still know the account or workspace record. | No trustworthy live target bytes. | Stop live target actions, show locate or restore review, and avoid exact restore claims unless target identity matches. |

## Recovery Action Mapping

| Class | Required action list |
| --- | --- |
| `local_core_continuity` | `continue_local_work`, `export_local_continuity_packet`, optional `preserve_pre_repair_checkpoint`. |
| `control_plane_impairment` | `continue_local_and_label_stale_authority`, `open_boundary_details`, `reconnect_or_reauth_after_recovery`, `export_control_plane_impairment_packet`. |
| `data_plane_impairment` | `freeze_live_runtime_and_preserve_local_state`, `run_project_doctor_transport_probe`, `compare_before_data_restore`, `export_data_plane_impairment_packet`. |
| `full_target_loss` | `stop_live_target_actions`, `locate_or_replace_target_identity`, `restore_from_reviewed_source`, `escalate_if_target_identity_cannot_be_proved`. |

## Worked Fixture Links

- `local_core_continuity`:
  [`/fixtures/ops/backup_restore_failover_rehearsal_cases/local_core_continuity.yaml`](../../fixtures/ops/backup_restore_failover_rehearsal_cases/local_core_continuity.yaml)
- `control_plane_impairment`:
  [`/fixtures/ops/backup_restore_failover_rehearsal_cases/control_plane_impairment.yaml`](../../fixtures/ops/backup_restore_failover_rehearsal_cases/control_plane_impairment.yaml)
- `data_plane_impairment`:
  [`/fixtures/ops/backup_restore_failover_rehearsal_cases/data_plane_impairment.yaml`](../../fixtures/ops/backup_restore_failover_rehearsal_cases/data_plane_impairment.yaml)
- `full_target_loss`:
  [`/fixtures/ops/backup_restore_failover_rehearsal_cases/full_target_loss.yaml`](../../fixtures/ops/backup_restore_failover_rehearsal_cases/full_target_loss.yaml)

The validator uses these examples as a first consumer: each class must
appear here, must appear in the taxonomy, and must appear in the
protected fixture manifest.
