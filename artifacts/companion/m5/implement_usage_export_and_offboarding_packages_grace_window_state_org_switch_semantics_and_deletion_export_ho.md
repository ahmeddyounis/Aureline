# Usage-Export and Offboarding Packages, Grace-Window State, Org-Switch Semantics, and Deletion/Export Honesty

- Packet: `usage-export-offboarding-surface:stable:0001`
- Label: `Usage-Export and Offboarding Packages, Grace-Window State, Org-Switch Semantics, and Deletion/Export Honesty`
- Sections: 4 | Usage-export packages: 3 | Offboarding packages: 3 | Grace-window rows: 3 | Org-switch rows: 4
- Exact desktop handoff for every item: yes
- Local usage-export path available: yes
- Local offboarding-package path available: yes
- Export completeness honestly qualified: yes
- Deletion honestly labeled: yes
- Local work never stranded: yes
- Stale state honestly labeled: yes
- Proof freshness SLO: 168 hours (last refresh: 2026-06-09T00:00:00Z)
- Degraded: none

## Sections

- **usage_export_package**: `beta` / `staged_rollout` [read_only] (matrix lane `offboarding_continuity`)
- **offboarding_package**: `beta` / `staged_rollout` [read_only] (matrix lane `offboarding_continuity`)
- **grace_window_state**: `preview` / `early_access` [read_only] (matrix lane `offboarding_continuity`)
- **org_switch_semantics**: `preview` / `early_access` [read_only] (matrix lane `offboarding_continuity`)

## Usage export packages

- `usage:0001` [usage_history/local_ready/complete_verified] (verified: yes) Full usage history exported locally now; complete, verified (live) → `review_panel` (exact)
- `usage:0002` [audit_trail/local_ready/complete_verified] (verified: yes) Local activity-log export ready locally; complete, verified (cached) → `review_panel` (exact)
- `usage:0003` [usage_history/requires_provider_assembly/complete_unverified] (verified: no) Provider-assembled billing usage; completeness not yet verified; labeled (unknown) → `review_panel` (exact)

## Offboarding packages

- `off:0001` [local_workspace/local_ready/complete_verified] (verified: yes) local_work_preserved `yes` Local workspace and edit history packaged locally now; complete, verified; local work retained (live) → `review_panel` (exact)
- `off:0002` [managed_snapshots/local_staging/complete_verified] (verified: yes) local_work_preserved `yes` Snapshot archive staging locally from the local core; complete, verified; local work retained (cached) → `review_panel` (exact)
- `off:0003` [managed_profile/requires_provider_assembly/complete_unverified] (verified: no) local_work_preserved `yes` Managed profile/settings bundle requires provider assembly; completeness not yet verified; labeled; local work retained (unknown) → `review_panel` (exact)

## Grace window

- `grace:0001` [managed_only_local_retained/open_reversible] reversible `yes` irreversible_labeled `no` local_work_preserved `yes` Managed profile deletion scheduled; grace window open and reversible; local work retained (live) → `review_panel` (exact)
- `grace:0002` [local_only/closing_reversible] reversible `yes` irreversible_labeled `no` local_work_preserved `yes` Local managed-mirror cache clear scheduled; window closing, still reversible; original local work retained (cached) → `review_panel` (exact)
- `grace:0003` [managed_only_local_retained/committed_irreversible] reversible `no` irreversible_labeled `yes` local_work_preserved `yes` Managed audit-trail deletion committed and irreversible; clearly labeled; local work retained (live) → `review_panel` (exact)

## Org-switch semantics

- `org:0001` [local_workspace/stays_local_to_user] user_owned `yes` retained `yes` requires_admin `no` Local workspace stays with the user across an org switch (live) → `review_panel` (exact)
- `org:0002` [managed_profile/migrates_with_user] user_owned `no` retained `yes` requires_admin `no` Managed profile migrates with the user to the new org (cached) → `review_panel` (exact)
- `org:0003` [managed_snapshots/requires_admin_approval] user_owned `no` retained `yes` requires_admin `yes` Managed snapshot migration requires prior-org admin approval; labeled (unknown) → `review_panel` (exact)
- `org:0004` [audit_trail/left_with_prior_org] user_owned `no` retained `yes` requires_admin `no` Prior-org audit trail stays with the prior org by policy (cached) → `review_panel` (exact)
