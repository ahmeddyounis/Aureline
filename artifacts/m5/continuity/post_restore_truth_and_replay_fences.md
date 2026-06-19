# Artifact: post-restore truth, replay fences, and compare/export parity

**Contract ref:** `continuity:m5_restore_from_backup_reviews:v1`  
**Schema:** `schemas/continuity/restore_identity_summary.schema.json`  
**Doc:** `docs/m5/continuity/post-restore-truth-and-replay-fences.md`  
**Runtime owner:** `aureline_continuity::m5_restore_from_backup_reviews`

## Qualification

| Condition | Status |
|---|---|
| Exact vs narrower-than-normal restore identity labeled on every managed review | ✓ Stable |
| Narrower-than-normal restores name the affected capability class or data slice | ✓ Stable |
| Restore identity declared on every managed-continuity review | ✓ Stable |
| Privileged and externally mutating lanes do not auto-replay (fenced) | ✓ Stable |
| Restored-vs-current compare and export available | ✓ Stable |
| At least one managed + one support/export family carries compare/export parity | ✓ Stable |
| Every claimed restored row points to a current clean review | ✓ Stable |
| Surface fact reuse complete + vocabulary identical | ✓ Stable |
| No green-status overclaim, no restore-lane conflation | ✓ Stable |
| **Overall** | **Stable** |

## Reviews

| Restored artifact | Profile | Family | Lane | Fidelity | Affected slice | Compare/export |
|---|---|---|---|---|---|---|
| Managed records restore | `managed` | `managed_record` | `continuity_restore` | `exact_restore` | `none_narrowed` | available |
| Policy bundle restore | `managed` | `policy_bundle` | `continuity_restore` | `narrower_than_normal_restore` | `policy_bundle_revision_gap` | available |
| Sync metadata restore | `self_hosted` | `sync_metadata` | `continuity_restore` | `exact_restore` | `none_narrowed` | available |
| Support and export records restore | `managed` | `support_record` | `continuity_restore` | `narrower_than_normal_restore` | `support_record_gap` | available |
| Local workspace restore | `local_only` | `local_workspace_state` | `ordinary_workspace_restore` | `exact_restore` | `none_narrowed` | n/a (exempt) |

## Replay fences

| Restored artifact | Action lane | Posture | Fence state |
|---|---|---|---|
| Managed records restore | Administrative policy apply | `privileged` | `held_for_review` |
| Managed records restore | Outbound webhook redelivery | `externally_mutating` | `held_for_review` |
| Policy bundle restore | Policy bundle activation | `privileged` | `cleared_after_review` |
| Sync metadata restore | Sync push to peers | `externally_mutating` | `held_for_review` |
| Support and export records restore | Support record re-export | `privileged` | `cleared_after_review` |

## Claim-narrowing cases

Each case mutates one seeded review and shows the claim narrowing automatically:

- `case_full_normal_status_overclaim_withdrawn` — a narrower-than-normal restore
  claims full normal status → **withdrawn** (`full_normal_status_overclaimed`)
- `case_restore_lane_conflated_withdrawn` — a managed row presents an ordinary
  workspace restore as continuity restore → **withdrawn**
  (`restore_lane_conflated`)
- `case_privileged_lane_auto_replayed_withdrawn` — a privileged lane is left
  unfenced and would auto-replay → **withdrawn** (`privileged_lane_auto_replayed`)
- `case_affected_slice_unnamed_beta` — a narrower-than-normal restore does not
  name the affected slice → **beta** (`affected_slice_unnamed`)
- `case_replay_fence_review_missing_beta` — a cleared fence names no explicit
  reviewed step → **beta** (`replay_fence_review_missing`)
- `case_compare_parity_missing_preview` — a managed-continuity review cannot
  compare restored-vs-current state → **preview** (`compare_parity_missing`)
- `case_review_evidence_missing_preview` — a claimed restored row carries no
  review → **preview** (`review_evidence_missing`)

## Canonical evidence packets

- `artifacts/m5/continuity/restore_reviews/restore_review_page.json`
- `artifacts/m5/continuity/restore_reviews/restore_review_registry.json`
- `artifacts/m5/continuity/restore_reviews/restore_review_support_export.json`

## Fixture references

- `fixtures/continuity/post_restore_narrowing/page.json`
- `fixtures/continuity/post_restore_narrowing/summary.json`
- `fixtures/continuity/post_restore_narrowing/registry.json`
- `fixtures/continuity/post_restore_narrowing/support_export.json`
- `fixtures/continuity/post_restore_narrowing/case_full_normal_status_overclaim_withdrawn.json`
- `fixtures/continuity/post_restore_narrowing/case_restore_lane_conflated_withdrawn.json`
- `fixtures/continuity/post_restore_narrowing/case_privileged_lane_auto_replayed_withdrawn.json`
- `fixtures/continuity/post_restore_narrowing/case_affected_slice_unnamed_beta.json`
- `fixtures/continuity/post_restore_narrowing/case_replay_fence_review_missing_beta.json`
- `fixtures/continuity/post_restore_narrowing/case_compare_parity_missing_preview.json`
- `fixtures/continuity/post_restore_narrowing/case_review_evidence_missing_preview.json`
