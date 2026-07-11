# M5 Workspace-Trust / Guided-Repair Component Surface Certification

- Packet: `m5-workspace-trust-repair-component-certification:stable:0001`
- As of: `2026-07-11T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-workspace-trust-repair-proof/support_export.json`
- Profiles: 8 / 8 certified (4 green, 4 yellow, 0 red)
- Families covered: true
- Guardrails held: true
- Auto-narrowed profiles: 4
- Report clean: true

## Profiles

- **cert:local-trusted-workspace** — profile=local_trusted_workspace claimed=full_trust_reviewed_result certified=full_trust_reviewed_result status=green narrowed_axes=0
- **cert:remote-reviewed-workspace** — profile=remote_reviewed_workspace claimed=reviewable_result certified=reviewable_result status=green narrowed_axes=0
- **cert:managed-policy-workspace** — profile=managed_policy_workspace claimed=reviewable_result certified=reviewable_result status=green narrowed_axes=0
- **cert:exact-reversal-repair** — profile=exact_reversal_repair claimed=reviewable_result certified=reviewable_result status=green narrowed_axes=0
- **cert:restricted-workspace** — profile=restricted_workspace claimed=reviewable_result certified=narrowed_capability_projection status=yellow narrowed_axes=1
- **cert:mixed-root-workspace** — profile=mixed_root_workspace claimed=reviewable_result certified=mixed_root_projection status=yellow narrowed_axes=1
- **cert:checkpoint-missing-repair** — profile=checkpoint_missing_repair claimed=reviewable_result certified=missing_checkpoint_projection status=yellow narrowed_axes=1
- **cert:manual-follow-up-repair** — profile=manual_follow_up_repair claimed=reviewable_result certified=unproven_reversal_projection status=yellow narrowed_axes=1
