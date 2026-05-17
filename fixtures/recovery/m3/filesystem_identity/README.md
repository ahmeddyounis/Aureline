# Filesystem-identity beta protected fixture corpus

This directory is the source of truth for the difficult-case filesystem-
identity scenarios the M3 beta lane must exercise. Each fixture mirrors
the boundary schema at
[`schemas/state/filesystem_identity_beta.schema.json`](../../../../schemas/state/filesystem_identity_beta.schema.json)
and is loaded by
[`crates/aureline-vfs/src/identity_beta/mod.rs`](../../../../crates/aureline-vfs/src/identity_beta/mod.rs).

Every case row binds the opened presentation path, the canonical mutable
object, the alias inspection chain, the save-target review token, the
compare-before-write outcome, and the support-export reference set into
one record so the chrome, the headless export, and the support packet
share one filesystem-identity truth.

## Cases

| Case | Difficulty class | What it covers |
| --- | --- | --- |
| [`symlink_alias_case.yaml`](symlink_alias_case.yaml) | `symlink_alias` | A symlink presentation path resolves to its canonical target; save lands at the canonical object. |
| [`case_only_drift_case.yaml`](case_only_drift_case.yaml) | `case_only_drift` | A case-only variant on an insensitive-preserving root resolves to the canonical case spelling. |
| [`unicode_normalization_case.yaml`](unicode_normalization_case.yaml) | `unicode_normalization` | An NFD presentation path resolves to the NFC canonical name; a concurrent external change forces compare-before-write actions. |
| [`bind_mount_overlay_case.yaml`](bind_mount_overlay_case.yaml) | `bind_mount_overlay` | A container bind-mount overlay surfaces the alias step, a restricted workspace blocker, and a compare-required conflict path. |

## Contract reminders

- `writes_to_canonical_uri` MUST equal `alias_inspection.canonical_uri`. No
  case rewrites the canonical address.
- Editor, git, restore, mutation, and support-export refs MUST equal the
  shared `filesystem_identity_ref`. The support packet inherits this
  alignment.
- `safety.raw_private_material_excluded` and
  `safety.ambient_authority_excluded` MUST be `true`.
- `safety.destructive_resets_present` MUST be `false`.
- `safety.preserves_user_authored_files` MUST be `true`.

Any new fixture added here MUST be appended to `manifest.yaml`,
mirrored in the `CASE_FIXTURES` table in the crate consumer, and
referenced by the reviewer doc.
