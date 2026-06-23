# Admin-plane render — evidence companion

Human-readable companion to
[`/fixtures/admin/m5-admin-render/canonical_render.json`](../../fixtures/admin/m5-admin-render/canonical_render.json)
and its boundary schema
[`/schemas/admin/m5-admin-render.schema.json`](../../schemas/admin/m5-admin-render.schema.json).
It gives reviewers the rendered per-profile surfaces without reading the JSON.
The contract narrative lives in
[`/docs/admin/m5-admin-render.md`](../../docs/admin/m5-admin-render.md), and the
frozen object model it binds back to lives in
[`/artifacts/admin/m5-admin-plane.md`](./m5-admin-plane.md).

- Bundle id: `m5-admin-render:bundle:0001`
- Record kind: `m5_admin_render_bundle`
- Binds matrix: `m5-admin-plane:matrix:0001`
- Profiles: 4 · Invariants: 12

## Profiles and endpoint posture

| Profile | Deployment | Endpoint state | Install | Update ring | Identity | Bundle freshness |
| --- | --- | --- | --- | --- | --- | --- |
| `managed_cloud` | managed_cloud | active_enforced | per_machine | stable | managed_session | fresh |
| `self_hosted` | self_hosted | active_enforced | managed_image | pinned_managed | managed_session | fresh |
| `sovereign_air_gapped` | sovereign_air_gapped | unconfirmed_stale | sovereign_image | pinned_offline | signed_out_local_only | stale |
| `mirrored_offline` | managed_cloud | mirror_offline_last_known | per_machine | pinned_offline | signed_out_local_only | recent |

Every endpoint card is locally inspectable and exportable
(`exportable: true`, diagnostics include `export_posture_snapshot`). The sovereign
card's stale bundle and the mirror card's offline source downgrade the posture
rather than showing it active/enforced.

## Effective-policy controls (winning source · state)

| Profile | Control | Feature family | Winning source | State | Verification | Evidence age |
| --- | --- | --- | --- | --- | --- | --- |
| `managed_cloud` | `ai.provider.allowed` | AI / assistants | managed_policy_bundle | locked_by_policy | signed_verified | fresh |
| `managed_cloud` | `telemetry.diagnostics` | Diagnostics | managed_policy_bundle | inherited_default | signed_verified | fresh |
| `managed_cloud` | `editor.theme` | Appearance | local_default | overridden_local | unsigned_local | fresh |
| `self_hosted` | `ai.provider.allowed` | AI / assistants | managed_policy_bundle | locked_by_policy | signed_verified | fresh |
| `self_hosted` | `network.egress` | Networking | managed_policy_bundle | locked_by_policy | signed_verified | recent |
| `sovereign_air_gapped` | `ai.provider.allowed` | AI / assistants | signed_offline_bundle | locked_by_policy | signed_verified | recent |
| `sovereign_air_gapped` | `update.channel` | Updates | signed_offline_bundle | unconfirmed_stale | unverifiable_offline | stale |
| `mirrored_offline` | `ai.provider.allowed` | AI / assistants | mirrored_policy_bundle | locked_by_policy | signed_verified | recent |
| `mirrored_offline` | `telemetry.diagnostics` | Diagnostics | mirrored_policy_bundle | mirror_offline_last_known | signed_verified | stale |

`update.channel` (sovereign) and `telemetry.diagnostics` (mirrored) carry stale
evidence and are therefore shown under downgraded states, never as confirmed-green
values.

## Locked-state explanations (every locked control links here)

| Profile | Explanation | Locked target | Lock source | Verification | Change owner | Escalation |
| --- | --- | --- | --- | --- | --- | --- |
| `managed_cloud` | `admin_render.lock.managed_cloud.ai_provider` | `ai.provider.allowed` | managed_policy_bundle | signed_verified | org_admin | security_owner |
| `self_hosted` | `admin_render.lock.self_hosted.ai_provider` | `ai.provider.allowed` | managed_policy_bundle | signed_verified | security_owner | compliance_owner |
| `self_hosted` | `admin_render.lock.self_hosted.network_egress` | `network.egress` | managed_policy_bundle | signed_verified | security_owner | — |
| `sovereign_air_gapped` | `admin_render.lock.sovereign.ai_provider` | `ai.provider.allowed` | signed_offline_bundle | signed_verified | security_owner | compliance_owner |
| `mirrored_offline` | `admin_render.lock.mirrored.ai_provider` | `ai.provider.allowed` | mirrored_policy_bundle | signed_verified | org_admin | security_owner |

## Policy-diff sheets

| Profile | From → To | Provisional | Changes |
| --- | --- | --- | --- |
| `managed_cloud` | rev 41 → rev 42 | no | `ai.provider.allowed` newly_locked; `telemetry.diagnostics` source_changed |
| `self_hosted` | rev 6 → rev 7 | no | `network.egress` newly_locked |
| `sovereign_air_gapped` | prior seal → seal 0xA1 | yes | `update.channel` value_changed |
| `mirrored_offline` | mirror rev 41 → rev 42 | yes | `ai.provider.allowed` newly_locked |

The sovereign and mirrored diffs are provisional because the current effective
values are stale; they are labeled rather than presented as confirmed
before/after.

## Invariants (all hold)

| Invariant | Statement |
| --- | --- |
| `admin_render.surface_states_within_matrix` | Every rendered state is one the frozen matrix admits for that surface family. |
| `admin_render.source_chain_resolves` | Every control has a non-empty source chain with exactly one winning link. |
| `admin_render.locked_controls_explained` | Every locked or forced control links to a complete explanation naming source, verification, and the next-step owner. |
| `admin_render.locked_explanation_complete` | Every explanation states a reason and at least one local-safe action. |
| `admin_render.no_silent_green` | Stale evidence never sits under a confirmed-value control or an active/enforced endpoint. |
| `admin_render.policy_diff_safe` | Every diff entry names its consequence and control; a diff over stale values is labeled provisional. |
| `admin_render.endpoint_posture_exportable` | Every profile's endpoint posture is locally inspectable and exportable. |
| `admin_render.ownership_visible` | Every owned object names an owner. |
| `admin_render.consumer_parity` | One typed packet serves shell, CLI/headless, Help/About, support export, and release evidence identically. |
| `admin_render.profiles_covered` | The managed-cloud, self-hosted, sovereign/air-gapped, and mirrored/offline profiles are all rendered. |
| `admin_render.stable_ids_unique` | Profile, control, change, and explanation ids are unique within scope. |
| `admin_render.export_safe` | Every stable id is an opaque token with no URL scheme or absolute path. |

## How to regenerate / verify

```sh
# Regenerate the fixture from the in-code builder
cargo run -p aureline-policy --example dump_m5_admin_render > \
  fixtures/admin/m5-admin-render/canonical_render.json

# Freeze gate: in-code bundle must equal the checked-in fixture
cargo test -p aureline-policy --test m5_admin_render

# Human-readable projection
cargo run -p aureline-policy --example dump_m5_admin_render -- --lines
```
