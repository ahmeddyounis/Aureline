# Region, tenant boundary, and key-mode truth (beta) with first drills

Reviewer-facing landing page for the beta projection that turns claimed managed
and enterprise deployment claims into operational truth across connected,
mirror-only, offline, and enterprise-managed beta profiles. The page surfaces
processing region, tenant boundary, and key-mode posture per managed action
lane, and pairs the rows with region, tenant, and key-mode drill packets that
exercise failure and failover scenarios.

The canonical record kind is
`security_region_tenant_key_mode_beta_page_record`. The schema lives at
[`/schemas/security/region_tenant_key_mode_beta.schema.json`](../../../schemas/security/region_tenant_key_mode_beta.schema.json).
The beta module lives at
[`/crates/aureline-auth/src/region_and_tenant/mod.rs`](../../../crates/aureline-auth/src/region_and_tenant/mod.rs)
and the shell / headless consumer at
[`/crates/aureline-shell/src/region_tenant_key_mode_beta/mod.rs`](../../../crates/aureline-shell/src/region_tenant_key_mode_beta/mod.rs).

## Why this lane exists

Managed and enterprise deployment claims (a US-East-1 pinned region, a bound
tenant `enterprise-pilot-alpha`, customer-managed keys) are not real until the
product can show them on the lane that depends on them and exercise the failure
modes. This page is the auditable record kind that makes those claims real.

Each row names:

- a managed action lane (`ai_inference`, `provider_tool_call`, `remote_attach`,
  `mirror_sync`, `support_export_upload`, `admin_policy_push`),
- a beta profile (`connected`, `mirror_only`, `offline`, `enterprise_managed`),
- the claimed posture (region mode, tenant id, key mode) and the active
  observed posture,
- a typed state (pinning, boundary, key custody) — one of
  `pinned_matches_claim`, `bound_matches_claim`, `matches_claim`, etc.,
- the effective authority the lane carries given that state, and
- the drill packet id that exercised the failure or failover for the same lane.

## Protected states covered

- **Disclosure.** Claimed managed / enterprise rows disclose region id, tenant
  id (or `not_applicable_local`), and active key mode in product and in the
  support-export wrapper. The validator's
  `claimed_managed_region_undisclosed`, `claimed_managed_tenant_undisclosed`,
  and `claimed_managed_key_mode_undisclosed` defects fail closed when a row
  asserts a managed posture but does not disclose the corresponding truth.
- **No silent widening.** Mismatch or degraded states (`pinning_lost`,
  `drifted_from_claim`, `binding_lost`, `custody_degraded`, `unresolved`)
  narrow only the affected managed lane's authority. The validator's
  `mismatch_masks_issue` defect refuses any row that claims a narrowing state
  but keeps authority `Allowed` / `ReadOnly`.
- **No undeclared public fallback.** Every row and drill packet declares
  `no_public_endpoint_fallback` and `raw_private_material_excluded`. The
  validator's `hidden_public_endpoint_fallback` and
  `raw_private_material_exposed` defects refuse rows that allow undeclared
  fallback or expose raw private material.
- **Local editing preserved.** Every row and drill packet declares
  `local_editing_preserved`. The validator refuses rows or drills that would
  block local-only work.
- **Drill coverage.** The validator's `drill_axis_coverage_missing` defect
  refuses a page that omits any of the three drill axes (`region`, `tenant`,
  `key_mode`). The drill packets are checked in
  [`/artifacts/security/m3/region_tenant_drills/`](../../../artifacts/security/m3/region_tenant_drills/).

## Drill packets

The seeded page includes drill packets for every required axis:

| Drill kind | Axis | Profile | Outcome |
| --- | --- | --- | --- |
| `region_pinning_failure` | region | `offline` | `narrowed_awaiting_admin` |
| `region_failover` | region | `connected` | `failed_over_to_declared_fallback` |
| `tenant_boundary_drift` | tenant | `connected` | `narrowed_then_recovered` |
| `tenant_failover` | tenant | `offline` | `failed_over_to_declared_fallback` |
| `key_mode_drift` | key_mode | `enterprise_managed` | `narrowed_then_recovered` |
| `key_mode_failover` | key_mode | `offline` | `failed_over_to_declared_fallback` |

Each drill packet records before/after state labels, the before/after authority
tokens on the affected lane, an export-safe explanation, and a hard
`sibling_lanes_unwidened` guarantee that the validator enforces. The packets
themselves live as JSON files under
[`/artifacts/security/m3/region_tenant_drills/`](../../../artifacts/security/m3/region_tenant_drills/).

## How to inspect

Run the headless inspector:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_region_tenant_key_mode_beta -- page
cargo run -q -p aureline-shell --bin aureline_shell_region_tenant_key_mode_beta -- region-rows
cargo run -q -p aureline-shell --bin aureline_shell_region_tenant_key_mode_beta -- tenant-rows
cargo run -q -p aureline-shell --bin aureline_shell_region_tenant_key_mode_beta -- key-mode-rows
cargo run -q -p aureline-shell --bin aureline_shell_region_tenant_key_mode_beta -- drill-packets
cargo run -q -p aureline-shell --bin aureline_shell_region_tenant_key_mode_beta -- support-export
cargo run -q -p aureline-shell --bin aureline_shell_region_tenant_key_mode_beta -- validate
```

The same records back the admin / settings center, support-export wrapper, and
fixture replay in
[`/fixtures/security/m3/region_tenant_key_mode/`](../../../fixtures/security/m3/region_tenant_key_mode/).

## Guardrails

- Fail closed before widening authority. A pinned-but-recheck-required region,
  a recheck-required tenant binding, or a recheck-required key custody must
  narrow the affected lane before silently widening.
- Never widen authority on a sibling lane during a drill. The validator
  enforces this with `drill_sibling_lane_widened`.
- Never silently fall back to public endpoints, plaintext secrets, or implicit
  managed assumptions on claimed beta rows. The validator enforces this with
  `hidden_public_endpoint_fallback`, `raw_private_material_exposed`, and the
  per-row managed-disclosure defects.
