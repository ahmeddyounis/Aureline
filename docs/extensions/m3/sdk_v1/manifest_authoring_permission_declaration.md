# SDK v1 manifest authoring guide: declaring permissions

This guide is the canonical walkthrough for declaring permission scopes
and rationale labels in an extension manifest under the SDK v1 beta
admission lane. It is referenced as
`manifest_guide:permission_declaration_walkthrough:1.0.0` in the
[SDK v1 starter pack](./README.md) and read verbatim by the install /
review chrome, the permission inspector, and the partner packet
template.

The narrative below sits on top of the typed truth in:

- [`docs/extensions/m1_permission_and_publisher_baseline.md`](../../m1_permission_and_publisher_baseline.md)
- [`docs/extensions/m3/permission_manifest_beta.md`](../permission_manifest_beta.md)

When the typed truth and this guide disagree, the typed truth wins and
this doc MUST be updated in the same change set.

## Step 1: declare a manifest baseline

Every extension MUST bind to one inspectable
`ExtensionManifestBaselineRecord` whose `manifest_baseline_id` is
prefixed `manifest_baseline:`. The record pins publisher identity,
lifecycle state, scope, declared permission classes, and origin /
source metadata. See
[`crates/aureline-extensions/src/manifest_baseline/mod.rs`](../../../../crates/aureline-extensions/src/manifest_baseline/mod.rs)
for the record shape.

## Step 2: list every declared permission scope

Each `PermissionScopeEntry` MUST carry:

- a typed `scope_class` from the closed `PermissionScopeClass`
  vocabulary (filesystem read/write, shell execute, network egress,
  AI / connected provider, secret handle, workspace settings,
  execution context, subscription, UI command, capability inherit);
- a non-empty `scope_target` (e.g. `workspace:/docs/**`);
- an optional `scope_constraint`; and
- a non-empty `rationale_label` (e.g. "Read prose documents for
  grammar suggestions.").

A permission with an empty rationale label is denied at install with
`declared_permission_rationale_required`.

## Step 3: project onto the capability-class vocabulary

The permission-manifest beta evaluator projects every declared scope
onto the closed `CapabilityClassClass` vocabulary (network, filesystem,
process, data, ui, credential) through
`capability_class_for_scope`. The same mapping is the only authorized
view the install / review surface, the permission inspector, the
support export, and the partner packet template read.

## Step 4: stage the version-to-version delta

When you publish a new version, the permission-manifest delta evaluator
computes one typed `PermissionManifestDeltaRecord` that pairs the
deterministic diff with a closed `ReConsentDecisionClass` and
`ReConsentReasonClass`. Any widening (new scope, relaxed constraint,
new capability class) requires re-consent before the update is enabled.

## Repair affordance

If the manifest validator or the permission-manifest delta evaluator
emits a finding, fix the manifest declaration locally and rerun:

```text
cargo test -p aureline-extensions manifest_baseline
cargo test -p aureline-extensions permission_manifest
```

The starter-pack lane refuses a sample row whose
`permission_manifest_ref` is missing the `permission_manifest:`
prefix.
