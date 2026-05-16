# SDK v1 manifest authoring guide: update and rollback

This guide is the canonical walkthrough for shipping a new version of
an extension, being revoked, and recovering under the update / rollback
contract. It is referenced as
`manifest_guide:update_and_rollback_walkthrough:1.0.0` in the
[SDK v1 starter pack](./README.md).

The narrative below sits on top of the typed truth in:

- [`docs/extensions/m3/permission_manifest_beta.md`](../permission_manifest_beta.md)
- [`docs/extensions/extension_lifecycle_and_quarantine_sequence.md`](../../extension_lifecycle_and_quarantine_sequence.md)
- [`docs/release/update_and_rollback_contract.md`](../../../release/update_and_rollback_contract.md)

## Step 1: ship the new manifest

When you ship a new extension version:

1. The new manifest baseline's `extension_version` MUST differ from
   the prior version; the permission-manifest delta evaluator refuses
   `refused_prior_and_next_same_version` otherwise.
2. The new `permission_manifest_id` and `manifest_baseline_id` MUST
   be prefixed `permission_manifest:` and `manifest_baseline:`; a
   missing prefix is refused with the matching reason.
3. The new manifest's permission set is projected onto the closed
   capability-class vocabulary and diffed against the prior version.

## Step 2: route through the closed re-consent vocabulary

The delta evaluator returns one of:

- `not_required_no_change` — declared permissions are identical.
- `not_required_narrowing_only` — only narrowing changes.
- `inform_only_rationale_changed` — only rationale text changed.
- `re_consent_required_widening` — at least one widening was
  detected.
- `re_consent_required_new_capability_class` — the next manifest
  introduces a new capability class.
- `refused_inconsistent_input` — a structural / lifecycle / publisher
  guardrail refused the delta.

The install / review chrome MUST render the typed decision verbatim
and MUST NOT invent a local "permissions changed" string.

## Step 3: handle revocation

If a row is revoked, the registry emits a
`RevocationAlphaRecord` with a typed
`RevocationStateClass` and `RevocationSubjectClass`. The install /
review surface holds without enabling the row; the support export
quotes the same closed tokens.

## Step 4: rehearse rollback

The extension-author cohort scorecard checks the rollback drill. The
canonical drill stays in
[`docs/release/update_and_rollback_contract.md`](../../../release/update_and_rollback_contract.md);
the starter pack does not invent its own drill.

## Repair affordance

If the delta evaluator refuses, fix the manifest declaration locally,
recompute the delta, and rerun the rollback drill against the SDK
release bundle:

```text
cargo test -p aureline-extensions permission_manifest
cargo test -p aureline-extensions review_alpha
```

The starter-pack lane refuses a sample row whose
`runtime_contract_ref` is missing the `runtime_v1_beta:`
prefix; without that ref no rollback drill is well defined.
