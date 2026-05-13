# Portable Profile Alpha Contract

This page is the reviewer-facing contract for portable profile exports that
cross a machine, account, or support boundary. The canonical profile body
remains [`schemas/profile/portable_profile.schema.json`](../../schemas/profile/portable_profile.schema.json);
the alpha export-review projection is
[`schemas/profiles/portable_profile.schema.json`](../../schemas/profiles/portable_profile.schema.json);
the workspace runtime projection is
[`crates/aureline-workspace/src/profiles/mod.rs`](../../crates/aureline-workspace/src/profiles/mod.rs).

## Required Export Truth

Every alpha profile export names:

- `schema_version`, profile id, profile revision, source device, and explicit
  profile scope;
- the artifact classes in scope, including keymaps and saved views when the
  user expects their setup to follow the profile;
- capability dependencies that may downgrade an artifact on another device;
- non-portable exclusions for secret material, delegated credentials,
  machine-local bindings, trust grants, admin policy, transient selection,
  stale provider cursors, and secret-bearing parameters.

Keymap artifacts carry stable command identity and resolver-layer metadata.
They do not carry policy-only command authority.

Saved-view artifacts carry owner scope, privacy class, portability label, filter
and column refs, and source revision attribution. They must not carry transient
selection, stale provider cursors, or secret-bearing filter parameters. If a
saved view cannot round-trip unchanged, the export labels it `local_only`,
`downgraded`, or `excluded` and names why.

## Apply Rules

Profile import and sync apply are scope-explicit. A request without a target
scope is blocked before apply. The profile lane may narrow behavior or preserve
an inert reference, but it may not silently widen workspace trust, extension
permissions, managed entitlements, AI egress, network egress, credential
exposure, policy ownership, or provider ownership.

Policy-pinned and provider-owned artifacts remain attributable to their source.
They may appear as downgraded or excluded rows in the profile review, but they
are not rewritten into user-authored portable state.

## Protected Fixture

[`fixtures/profile/alpha/portable_profile_keymap_saved_view.json`](../../fixtures/profile/alpha/portable_profile_keymap_saved_view.json)
exercises the alpha export path:

- portable keymap artifact;
- portable saved view;
- provider-owned saved view downgraded by a missing capability;
- secret-bearing saved view excluded by class;
- explicit non-portable exclusions.

Run:

```sh
cargo test -p aureline-workspace profile_alpha
```
