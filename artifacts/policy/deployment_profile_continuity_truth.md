# Artifact: deployment-profile-continuity-truth

**Contract ref:** `policy:deployment_profile_continuity_truth:v1`  
**Schema:** `schemas/policy/deployment_profile_continuity_truth.schema.json`  
**Doc:** `docs/policy/deployment_profile_continuity_truth.md`  
**Runtime owner:** `aureline_policy::deployment_profile_continuity_truth`

## Qualification

| Condition | Status |
|---|---|
| Claimed profile coverage complete | ✓ Stable |
| Deployment summary cards complete | ✓ Stable |
| Residual dependencies disclosed | ✓ Stable |
| Mirror freshness explicit | ✓ Stable |
| Local-safe fallback explicit | ✓ Stable |
| Surface fact reuse complete | ✓ Stable |
| **Overall** | **Stable** |

## Current running profile

- `self_hosted`
- Tenant: `Acme Platform`
- Region: `customer-operated eu-central region`
- Mirror/offline posture: `online_mirror_only`
- Key ownership: `Customer-managed KMS`

## Profile coverage

| Profile | Residual dependency rows | Mirror freshness cards | Local-safe state |
|---|---:|---:|---|
| `individual_local` | 0 | 0 | `healthy` |
| `managed_cloud` | 3 | 0 | `healthy` |
| `enterprise_online` | 2 | 0 | `control_plane_degraded` |
| `self_hosted` | 2 | 2 | `mirror_stale` |
| `air_gapped` | 0 | 2 | `offline_local_safe` |

## Residual dependency disclosure

- `managed_cloud` discloses vendor-hosted identity/policy, telemetry upload,
  and hosted AI routing.
- `enterprise_online` discloses approved external docs/catalog metadata and
  external model routing.
- `self_hosted` discloses vendor crash-symbol/support publication and hosted AI
  fallback routing.
- `air_gapped` discloses zero vendor-hosted dependencies because the seeded
  profile claims only local or customer-bounded runtime paths plus signed
  offline snapshots.

## Local-safe continuity

- Managed and enterprise-connected profiles preserve local editing, save,
  search, Git, export, and cached inspection when control-plane freshness
  narrows.
- Self-hosted mirror delay keeps customer-hosted runtime work and local desktop
  workflows available while fresh mirror-fed widenings remain paused.
- Air-gapped operation keeps the desktop fully useful from signed local or
  imported state while installs, updates, and policy widenings wait for the
  next approved import.

## Fixture references

- `fixtures/policy/deployment_profile_continuity_truth/page.json`
- `fixtures/policy/deployment_profile_continuity_truth/summary.json`
- `fixtures/policy/deployment_profile_continuity_truth/support_export.json`
- `fixtures/policy/deployment_profile_continuity_truth/drill_hidden_self_hosted_dependency_withdrawn.json`
- `fixtures/policy/deployment_profile_continuity_truth/drill_mirror_freshness_gap_beta.json`
- `fixtures/policy/deployment_profile_continuity_truth/drill_surface_reuse_gap_beta.json`
- `fixtures/policy/deployment_profile_continuity_truth/drill_missing_local_safe_fallback_preview.json`
