# Artifact: stabilize-deployment-and-residency-truth

**Lane:** Deployment profile truth, region/residency clarity, and residual-dependency honesty\
**Contract ref:** `policy:deployment_residency_stabilize:v1`\
**Schema:** `schemas/policy/deployment-profile.schema.json`\
**Doc:** `docs/policy/m4/stabilize-deployment-and-residency-truth.md`\
**Runtime owner:** `aureline_policy::stabilize_deployment_and_residency_truth`

## Qualification

| Condition | Status |
|-----------|--------|
| Vocabulary consistent across surfaces | ✓ Stable |
| Residual-dependency ledger complete | ✓ Stable |
| Plane separation enforced | ✓ Stable |
| Mirror/offline artifact rows present | ✓ Stable |
| Sign-out/deprovision scope declared | ✓ Stable |
| **Overall** | **Stable** |

## Profile coverage

| Profile | Rows | Residual deps | Mirror artifacts | Sign-out scope | Sovereignty |
|---------|------|---------------|-----------------|---------------|-------------|
| `individual_local` | 1 | 0 (not required) | 0 (not required) | Not required | Not claimed |
| `managed_cloud` | 1 | 3 | 0 | Declared | Not claimed |
| `enterprise_online` | 1 | 2 | 0 | Declared | Not claimed |
| `self_hosted` | 1 | 2 | 1 | Declared | Evidenced |
| `air_gapped` | 1 | 2 | 2 | Declared | Evidenced |

## Plane-status strips audited

| Strip | Profile | CP separated | Continue-local preserved |
|-------|---------|-------------|--------------------------|
| `strip.plane.managed_cloud.baseline` | `managed_cloud` | Yes | Yes |
| `strip.plane.enterprise_online.baseline` | `enterprise_online` | Yes | Yes |
| `strip.plane.self_hosted.baseline` | `self_hosted` | Yes | Yes |
| `strip.plane.air_gapped.baseline` | `air_gapped` | Yes | Yes |

## Residual-dependency coverage summary

- **`individual_local`**: No hosted control-plane; zero residual-dependency rows required and present.
- **`managed_cloud`**: Vendor-operated control plane; three residual-dependency rows (AI provider, browser handoff, hosted control-plane reachability) cover all vendor-bound services.
- **`enterprise_online`**: Hybrid managed services; two residual-dependency rows (AI provider, browser handoff) cover vendor-bound services.
- **`self_hosted`**: Customer-operated control plane; two residual-dependency rows (sign-in against customer IdP, optional AI provider) plus one mirror artifact row for the signed policy bundle.
- **`air_gapped`**: Air-gapped mirror; two residual-dependency rows (AI provider forbidden, docs pack mirrored) plus two mirror artifact rows (policy bundle, docs pack).

## Sign-out/deprovision scope

All `customer_tenant`-scoped profiles declare sign-out/deprovision scope metadata
covering:

- **Remains local on device**: Open files, local Git history, local task state, local diagnostics.
- **Tenant-scoped (removed on sign-out)**: Remote workspace state, tenant identity cache, policy bundle cache, AI conversation history.
- **Retained for audit/policy**: Support packet evidence, last-known-good policy snapshot, audit event log (retention per `retention_class` for the running profile).

## Outage drill coverage

The following outage drills are evidenced by impairment-case fixtures:

| Drill | Profile | Control-plane state | Data-plane state | Safest next action |
|-------|---------|--------------------|-----------------|--------------------|
| Managed-cloud relay disconnect | `managed_cloud` | relay unavailable, others healthy | `available_local_safe` | `continue_local` |
| Enterprise failover boundary recheck | `enterprise_online` | `boundary_recheck_required` | `available_local_safe` | `recheck_boundary` |
| Self-hosted stale policy session | `self_hosted` | `stale_cache` | `available_local_safe` | `retry_policy_sync` |
| Air-gapped mirror only | `air_gapped` | `mirror_only` | `available_mirror_backed` | `continue_local` |

## Guardrails active

- `implied_sovereignty_unproven`: Any profile claiming `self_hosted` or `air_gapped` without supporting evidence rows is withdrawn immediately.
- `implied_no_residual_dependency_when_required_present`: Non-local profiles may not claim zero residual dependencies when vendor-bound services are present.
- `implied_self_hosted_when_managed_cloud`: Managed-cloud profiles may not widen into a self-hosted claim.
- `implied_managed_independence_when_local_dependent`: Profiles with local-dependent data paths may not claim managed independence.
- `implied_offline_parity_when_mirror_only`: Mirror-only profiles may not claim full offline parity.

## Fixture references

- `fixtures/policy/m4/stabilize-deployment-and-residency-truth/all_profiles_stable_input.yaml`
- `fixtures/policy/m4/stabilize-deployment-and-residency-truth/managed_cloud_relay_disconnect_input.yaml`
- `fixtures/policy/m4/stabilize-deployment-and-residency-truth/self_hosted_stale_policy_session_input.yaml`
- `fixtures/policy/m4/stabilize-deployment-and-residency-truth/air_gapped_mirror_only_input.yaml`
- `fixtures/policy/m4/stabilize-deployment-and-residency-truth/sovereignty_unproven_withdrawal_input.yaml`
