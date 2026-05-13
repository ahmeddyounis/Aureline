# Sync Conflict Review Alpha Contract

This page is the alpha review contract for profile, keymap, and saved-view
conflicts. It composes with the settings sync seed
[`docs/settings/sync_and_device_registry_seed.md`](./sync_and_device_registry_seed.md),
the profile conflict journal
[`docs/profile/profile_sync_and_conflict_contract.md`](../profile/profile_sync_and_conflict_contract.md),
and the alpha schemas:

- [`schemas/sync/device_registry_alpha.schema.json`](../../schemas/sync/device_registry_alpha.schema.json)
- [`schemas/sync/conflict_packet_alpha.schema.json`](../../schemas/sync/conflict_packet_alpha.schema.json)

The first runtime consumer is
[`crates/aureline-workspace/src/profiles/mod.rs`](../../crates/aureline-workspace/src/profiles/mod.rs).

## Device Registry Surface

Any claimed sync or device registry surface must render device identity,
revision, transport state, identity mode, and local-only fallback posture.
Transport failures, stale payloads, refused policy, or missing capability state
degrade to local-authoritative or refused posture. They do not imply that local
durable state was overwritten.

## Conflict Packet

The alpha conflict packet distinguishes:

- `same_key_divergence`;
- `policy_locked`;
- `missing_capability`;
- `delete_vs_modify`;
- `stale_remote`.

Each packet carries field diffs, local and incoming revision attribution,
source posture (`local_only`, `synced`, `imported`, `policy_pinned`,
`provider_owned`), owner scope, privacy class, and portability label.

Every conflict packet names the three review actions explicitly:

- `Keep local`;
- `Keep synced`;
- `Compare`.

Blocked states still name the action but mark it unavailable with a reason. For
example, policy-locked and missing-capability packets keep `Keep synced`
visible as unavailable rather than hiding the decision.

## Non-Widening Apply

The profile/sync lane blocks apply if the incoming revision would widen
workspace trust, extension permissions, managed entitlements, AI egress,
network egress, credential exposure, policy ownership, or provider ownership.
The safe floor is `Keep local`, `Compare`, or a separate owning-subsystem flow.

## Protected Fixtures

Fixtures:

- [`fixtures/sync/device_registry_alpha/local_authoritative_fallback.json`](../../fixtures/sync/device_registry_alpha/local_authoritative_fallback.json)
- [`fixtures/sync/conflict_review_alpha/keymap_and_saved_view_conflicts.json`](../../fixtures/sync/conflict_review_alpha/keymap_and_saved_view_conflicts.json)

Run:

```sh
cargo test -p aureline-workspace profile_alpha
```
