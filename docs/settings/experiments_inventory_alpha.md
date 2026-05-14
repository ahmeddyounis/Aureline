# Experiments Inventory Alpha

This page is the reviewer entry point for the settings-owned experiments and
capability-dependency inventory. The canonical machine-readable source is
[`/artifacts/governance/experiments_inventory_alpha.yaml`](../../artifacts/governance/experiments_inventory_alpha.yaml);
runtime consumers load it through
[`/crates/aureline-settings/src/experiments/`](../../crates/aureline-settings/src/experiments/).

## Contract

Every visible row exposes:

- capability id, title, owner, and target workflow;
- declared and effective lifecycle state using the controlled vocabulary
  `Labs`, `Preview`, `Beta`, `Stable`, `Deprecated`, `DisabledByPolicy`,
  and `Retired`;
- review or expiry date, default posture, support promise, enrollment source,
  and cohort or ring;
- active policy-disable or kill-switch source, with preserved-data scope and a
  bounded fallback path;
- dependency markers for settings profiles, workspace manifests, saved views,
  migration packets, and support exports.

Kill-switch precedence is fixed in the artifact and enforced by the settings
crate:

1. `emergency_security_response`
2. `admin_policy_ceiling`
3. `release_channel_or_rollout_override`
4. `cohort_or_ring_assignment`
5. `user_opt_in_or_local_preview_toggle`

The effective state is derived from the declared state, active disable sources,
and dependency markers. A policy-disabled or retired row remains visible in CLI,
diagnostics, and support exports; it is not hidden from inventory views.

## Consumers

The settings CLI is the first surfaced consumer:

```sh
cargo run -q -p aureline-settings --bin aureline_settings_inspect -- experiments-inventory
cargo run -q -p aureline-settings --bin aureline_settings_inspect -- experiments-support-export
```

The shell diagnostics consumer lives in
[`/crates/aureline-shell/src/diagnostics/experiments_inventory.rs`](../../crates/aureline-shell/src/diagnostics/experiments_inventory.rs).
It renders blocked or retired rows, dependency-bearing rows, and copy-safe
fallback paths from the same inspection record as the CLI.

## Fixtures

Protected fixtures live in
[`/fixtures/settings/experiments_inventory_alpha/`](../../fixtures/settings/experiments_inventory_alpha/):

- `cli_projection_acceptance.json`
- `saved_artifact_dependency_warning.json`
- `support_export_shared_contract.json`

The fixtures prove that saved artifacts and support exports can carry
capability-dependency markers, including policy-disabled and retired rows,
without exposing raw workspace content or provider secrets.

## Verification

```sh
cargo test -p aureline-settings --test experiments_inventory_alpha
cargo test -p aureline-shell diagnostics::experiments_inventory
cargo run -q -p aureline-settings --bin aureline_settings_inspect -- experiments-inventory
cargo run -q -p aureline-settings --bin aureline_settings_inspect -- experiments-support-export
```
