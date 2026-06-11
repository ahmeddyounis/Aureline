# Usage-Export and Offboarding Packages, Grace-Window State, Org-Switch Semantics, and Deletion/Export Honesty

This document is the human-readable contract for the offboarding-depth lane: the
**usage-export packages** that record which usage-data export packages are offered,
whether each is available locally now or requires provider assembly, and whether the
package is complete; the **offboarding packages** that record the full
leave-the-product export bundles per data class; the **grace-window state** that
records, per scheduled deletion, its deletion scope and whether the grace window is
still open and reversible or already committed and irreversible; and the **org-switch
semantics** that record, per data class, what migrates with the user, what stays local
to the user, what is left with the prior org, and what requires admin approval. The
machine-readable truth source is the checked-in support export; later desktop companion
panel, CLI/headless, diagnostics, support export, and Help/About surfaces ingest it
instead of cloning status text.

- Record kind: `implement_usage_export_and_offboarding_packages_grace_window_state_org_switch_semantics_and_deletion_export_ho`
- Schema: `schemas/companion/implement-usage-export-and-offboarding-packages-grace-window-state-org-switch-semantics-and-deletion-export-ho.schema.json`
- Support export: `artifacts/companion/m5/implement_usage_export_and_offboarding_packages_grace_window_state_org_switch_semantics_and_deletion_export_ho/support_export.json`
- Markdown summary: `artifacts/companion/m5/implement_usage_export_and_offboarding_packages_grace_window_state_org_switch_semantics_and_deletion_export_ho.md`
- Fixtures: `fixtures/companion/m5/implement_usage_export_and_offboarding_packages_grace_window_state_org_switch_semantics_and_deletion_export_ho/`
- Producer crate: `aureline-companion`

## Sections and matrix inheritance

The packet has four sections. Every section inherits its qualification and staged
rollout stage from the frozen M5 companion-matrix `offboarding_continuity` lane (see
`docs/companion/m5/freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes.md`),
so a section never claims more than the matrix qualifies. The usage-export-package and
offboarding-package sections earn the lane's Beta/staged-rollout qualification because a
local-first path is always available and local work is never stranded; the
grace-window-state and org-switch-semantics sections inherit the Preview/early-access
qualification because their managed and admin-dependent paths are less mature.

| Section | Matrix lane | Scope | Qualification | Rollout stage |
| --- | --- | --- | --- | --- |
| `usage_export_package` | `offboarding_continuity` | `read_only` | beta | staged_rollout |
| `offboarding_package` | `offboarding_continuity` | `read_only` | beta | staged_rollout |
| `grace_window_state` | `offboarding_continuity` | `read_only` | preview | early_access |
| `org_switch_semantics` | `offboarding_continuity` | `read_only` | preview | early_access |

## Read-only projection, with the local core authoritative

Every section is read-only. The surface **projects** the offboarding state but never
applies it: an export, a deletion, or an org switch is applied by the local core, never
authored from this surface (`action_applied_by_local_core_not_surface`). A local-first
usage-export path and a local-first offboarding-package path are always offered as a
fallback, so a degraded provider never strands the user.

- **Usage-export packages** record each `data_class` (`local_workspace`,
  `usage_history`, `managed_profile`, `managed_snapshots`, `audit_trail`), its
  `availability` (`local_ready`, `local_staging`, `requires_provider_assembly`,
  `unavailable`), and its `completeness`. A local-path option (`local_ready` or
  `local_staging`) is always present (`usage_export_local_path_always_available`).
- **Offboarding packages** record the full leave-the-product bundle per data class,
  with the same `availability` and `completeness`, and assert `local_work_preserved`. A
  local-path option is always present
  (`offboarding_package_local_path_always_available`).
- **Grace-window state** records, per scheduled deletion, its `deletion_scope`
  (`local_and_managed`, `managed_only_local_retained`, `local_only`), its
  `grace_posture` (`open_reversible`, `closing_reversible`, `committed_irreversible`,
  `not_scheduled`), the `reversible` flag, the `irreversible_labeled` flag, and
  `local_work_preserved`.
- **Org-switch semantics** record, per data class, its `disposition`
  (`migrates_with_user`, `stays_local_to_user`, `requires_admin_approval`,
  `left_with_prior_org`), whether the class is `user_owned`, whether
  `user_owned_local_retained`, and whether the migration `requires_admin`.

## Deletion/export honesty

An export or offboarding package claims completeness (`complete_verified`) only when it
is backed by evidence (`claim_verified = true`). An unverifiable claim narrows to
`complete_unverified` and sets `proof_label_shown = true` so it is labeled, never shown
as proven; a known-partial package states `partial` honestly.

A scheduled deletion is reversible (`reversible = true`) only while its grace window is
open or closing; the `reversible` flag always mirrors the `grace_posture`. Once a
deletion has committed (`committed_irreversible`), it is irreversible and
`irreversible_labeled` is set so it is never shown as still reversible.

Every offered deletion scope leaves the user's authoritative local work intact, and
every grace-window row asserts `local_work_preserved = true`.

## Never strand local work on an org switch

User-owned local work is never left with the prior org: an org-switch row whose
`user_owned` is true always sets `user_owned_local_retained = true` and its
`disposition` is never `left_with_prior_org`. The `left_with_prior_org` disposition is
reserved for org-owned data (for example, a prior-org audit trail), which is not
user-owned.

## Stale-state honesty

Every item carries a `freshness` state (`live`, `cached`, `stale`, `unknown`). When
freshness `requires_label` (stale or unknown), `stale_label_shown` is set, and a
degraded item is never shown as live.

## Degraded behavior

`apply_offboarding_degradation` narrows sections, narrows package availability to its
local path, downgrades completeness claims, holds grace windows open when deletion
cannot commit, and downgrades freshness from a per-observation signal, recording the
reasons in `degraded_labels`:

| Observation | Effect |
| --- | --- |
| `managed_service_available = false` | every section narrows one step, every live/cached item goes stale; labels `managed_service_degraded`, `freshness_downgraded_to_stale` |
| `export_assembler_available = false` | every provider-assembled package narrows to `unavailable` while the local path remains; the usage-export and offboarding sections narrow; labels `export_assembler_unavailable`, `package_narrowed_to_local_path` |
| `completeness_verified = false` | every verified completeness claim downgrades to `complete_unverified` and is labeled; the usage-export and offboarding sections narrow; labels `completeness_unverified`, `completeness_claim_downgraded` |
| `deletion_service_available = false` | every closing grace window is held open again, widening the reversible window; the grace-window section narrows; labels `deletion_service_unavailable`, `grace_window_held_open` |
| `admin_continuity_available = false` | the offboarding-package, grace-window, and org-switch sections narrow; labels `admin_continuity_unavailable` |
| `proof_fresh = false` / `upstream_matrix_narrowed = true` | every section narrows; labels `proof_stale` / `upstream_matrix_narrowed` |
| `host_session_active = false` | every host-dependent exact handoff narrows to `unresolved`; labels `host_session_inactive`, `handoff_target_unresolved` |

The local path always remains, local work is always preserved, and an irreversible
deletion stays committed and labeled. Degraded state is labeled, never hidden.

## Boundary and redaction

Credential bodies, raw account payloads, raw provider payloads, and raw export record
contents never cross this boundary; the packet carries only redacted summaries and
opaque refs (`record_ref`, `deep_link_ref`).

## Regenerating the artifacts

The checked-in support export, the Markdown summary, and the degraded fixtures are
generated deterministically from the first-consumer surface builder:

```text
cargo run -p aureline-companion --example dump_usage_export_offboarding_surface -- canonical
cargo run -p aureline-companion --example dump_usage_export_offboarding_surface -- markdown
cargo run -p aureline-companion --example dump_usage_export_offboarding_surface -- managed_service_degraded
cargo run -p aureline-companion --example dump_usage_export_offboarding_surface -- export_assembler_down
cargo run -p aureline-companion --example dump_usage_export_offboarding_surface -- completeness_unverified
cargo run -p aureline-companion --example dump_usage_export_offboarding_surface -- deletion_service_down
cargo run -p aureline-companion --example dump_usage_export_offboarding_surface -- admin_continuity_lost
```
