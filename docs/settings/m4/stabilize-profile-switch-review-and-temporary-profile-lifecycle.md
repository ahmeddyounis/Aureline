# Stabilize profile switch review and temporary profile lifecycle

This settings contract makes profile switching a governed workflow. Claimed
stable profile surfaces consume
`settings:profile_switch_review_lifecycle:v1` instead of reconstructing profile
semantics in UI copy, support exports, sync conflict review, or help text.

The canonical record is emitted by:

```sh
cargo run -q -p aureline-settings \
  --bin aureline_settings_stabilize_profile_switch_review_and_temporary_profile_lifecycle \
  -- emit-fixtures fixtures/settings/m4/stabilize-profile-switch-review-and-temporary-profile-lifecycle
```

## Stable Contract

- Profile cards identify the profile source, included scopes, export/sync
  posture, and whether state is durable, temporary, imported, synced, or
  policy-shaped.
- Switch reviews show immediate changes, restart-required changes,
  machine-local exclusions, trust/network/extension narrowing effects, a change
  summary, and rollback checkpoint posture before apply.
- Temporary and troubleshooting profiles carry a visible badge, session lifetime
  or expiry, restricted persistence rules, and `Discard`, `Promote`, and
  `Compare to durable profile` actions.
- Profile artifacts remain text-based, schema-versioned, diffable, and
  exportable without secrets, keychain material, machine-unique trust anchors,
  delegated credentials, workspace trust approvals, or admin-policy bundles.
- Import, merge, and sync review rows are field-aware and scope-aware. Incoming
  profiles may narrow behavior, but trust, extension permissions, AI/network
  egress, and managed authority cannot widen silently.
- Every switch, import, merge, or sync apply that materially changes durable
  state carries an inspectable change summary and rollback checkpoint.
- Stale, unavailable, undecryptable, or policy-denied sync/device-registry state
  degrades visibly to local-authoritative file portability.

## Canonical Artifacts

- Schema: `schemas/settings/profile-switch-review.schema.json`
- Fixtures:
  `fixtures/settings/m4/stabilize-profile-switch-review-and-temporary-profile-lifecycle/`
- Rust model:
  `crates/aureline-settings/src/stabilize_profile_switch_review_and_temporary_profile_lifecycle/`
- Review artifact:
  `artifacts/settings/m4/stabilize-profile-switch-review-and-temporary-profile-lifecycle.md`

## Verification

```sh
cargo test -q -p aureline-settings --test profile_switch_review_temporary_lifecycle_fixtures
```
