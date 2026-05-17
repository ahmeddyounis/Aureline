# Filesystem-identity and save coordination hardened beta

The filesystem-identity beta lane is the bounded promotion of the five-layer
identity model frozen in
[`docs/adr/0006-vfs-save-cache-identity.md`](../../adr/0006-vfs-save-cache-identity.md)
and the cross-surface vocabulary in
[`docs/filesystem/filesystem_identity_vocabulary.md`](../../filesystem/filesystem_identity_vocabulary.md)
into one reviewer-facing contract. The alpha layers already declared
presentation path, logical workspace identity, canonical filesystem object,
alias set, and save-target token; the beta lane binds them — together with
the alias inspector, the save-target review, the compare-before-write
projection, and the shared `filesystem_identity_ref` — into one record per
difficult case, and folds the corpus into a metadata-safe support packet the
chrome, headless export, and support-export pipeline share verbatim.

The implementation lives in
[`crates/aureline-vfs/src/identity_beta/mod.rs`](../../../crates/aureline-vfs/src/identity_beta/mod.rs)
and the boundary schema lives at
[`schemas/state/filesystem_identity_beta.schema.json`](../../../schemas/state/filesystem_identity_beta.schema.json).
The protected fixture corpus lives under
[`fixtures/recovery/m3/filesystem_identity/`](../../../fixtures/recovery/m3/filesystem_identity/).

## What this beta row owns

- A typed [`FilesystemIdentityBetaCase`] record. Every case binds the opened
  presentation path, the canonical mutable object, the alias-set inspection
  (with `is_canonical` / `is_presentation` flags plus the resolution chain),
  the save-target review (with `path_truth_class`, atomic-write mode, pinned
  generation token, and blockers), the compare-before-write outcome (with
  the resolution-action list a reviewer may pick), the shared
  `filesystem_identity_ref` that editor, git, restore, mutation, and
  support-export flows use, and the safety baseline (raw private material
  excluded, ambient authority excluded, destructive resets refused,
  user-authored files preserved).
- A typed [`FilesystemIdentityBetaCorpus`] that the
  [`FilesystemIdentityBetaEvaluator`] folds into the support-packet
  projection [`FilesystemIdentityBetaSupportPacket`]. The evaluator refuses
  a corpus that drops a required difficulty class, declares duplicate
  `case_id`/`fixture_ref`, mismatches `writes_to_canonical_uri` against the
  canonical URI, drifts the alias chain against the path-truth class,
  offers a `write` action on a non-`unchanged` compare outcome, declares a
  destructive reset, drops user-authored-files preservation, admits raw
  private material, or breaks the support-export ref alignment.
- A `compile_from_alpha` entry point that consumes any
  [`SaveTargetToken`], its [`SaveTargetReviewRecord`], and its
  [`ExternalChangeCompareRecord`] and produces the typed beta case without
  re-deriving truth from a side channel.

## What this beta row does NOT own

- Live byte-level redaction. The packet is metadata-only; raw bytes never
  leave the case-row scope.
- Hosted ticket intake or share/upload. The packet is consumed locally by
  the support-export pipeline and the chrome's alias inspector / save-
  review surface; sharing belongs to later milestones.
- Discovery of new difficult cases via automated fuzzing. The required
  difficulty-class list is closed and seeded; new lanes land as fixtures
  with reviewer review, not at runtime.

## Difficulty classes the beta corpus must exercise

| Class | Fixture | What it proves |
| --- | --- | --- |
| `symlink_alias` | [`symlink_alias_case.yaml`](../../../fixtures/recovery/m3/filesystem_identity/symlink_alias_case.yaml) | Save lands at the canonical object even when the user opened through a symlink. |
| `case_only_drift` | [`case_only_drift_case.yaml`](../../../fixtures/recovery/m3/filesystem_identity/case_only_drift_case.yaml) | Case-only variants on insensitive-preserving roots resolve to the canonical case spelling. |
| `unicode_normalization` | [`unicode_normalization_case.yaml`](../../../fixtures/recovery/m3/filesystem_identity/unicode_normalization_case.yaml) | NFD presentations resolve to NFC canonicals; an external change forces compare/merge/reload/save-as/cancel instead of silent overwrite. |
| `bind_mount_overlay` | [`bind_mount_overlay_case.yaml`](../../../fixtures/recovery/m3/filesystem_identity/bind_mount_overlay_case.yaml) | Container bind-mount overlays disclose the alias step, register a `untrusted_workspace` blocker, and route to compare/save-as/cancel. |

## Acceptance and how this row meets it

- **The UI can explain opened path, canonical mutable object, alias set,
  and save target token where it matters.** Every case row carries the
  presentation, canonical, and logical URIs in the alias-inspection
  projection plus the save-target review's token kind, value, and atomic
  write mode. The chrome's alias inspector and the save-target review
  surface both render off the same record verbatim.
- **Difficult cases — symlinks, case-only drift, Unicode normalization,
  and overlays — are exercised by fixtures rather than anecdotes.** The
  corpus pins four required difficulty classes (symlink alias, case-only
  drift, Unicode normalization, bind-mount overlay) and the evaluator
  refuses a corpus that omits any of them.
- **Support exports preserve the same target identity truth used by save
  and conflict-resolution flows.** Every case row's
  `support_export_alignment.filesystem_identity_ref` MUST equal the
  editor, git, restore, mutation, and support-export refs. The support
  packet copies the same ref verbatim onto every emitted row.

## Failure-drill posture

The evaluator fails closed before widening any safety guarantee:

- A case whose `writes_to_canonical_uri` differs from the canonical URI is
  refused.
- A case whose support-export refs disagree with the
  `filesystem_identity_ref` is refused.
- A case that drops `safety.preserves_user_authored_files`, admits raw
  private material, admits ambient authority, or declares
  `destructive_resets_present = true` is refused.
- A case whose `compare_outcome` is non-`unchanged` but offers a `write`
  action, or whose `silent_overwrite_forbidden` is true when the compare
  outcome is `unchanged`, is refused.
- A case whose alias entries are empty for a non-direct path-truth class,
  or whose `path_truth_class` does not agree with `opens_via_alias_kind`,
  is refused.
- A corpus missing any required difficulty class is refused.

## First consumers

- The `aureline-vfs` `identity_beta` module is the canonical projection
  for alias-inspection, save-target review, and conflict-resolution truth.
  The chrome, the support-export pipeline, and the release-evidence lane
  read off this record so they never re-derive identity from a path
  string.
- The boundary schema is the contract the headless export writer and the
  support-export chrome share — both reconstruct the same shape from the
  on-disk record verbatim.
- The protected corpus is the proving ground for the four difficulty
  classes the beta release-candidate owes; the integration drill at
  [`crates/aureline-vfs/tests/filesystem_identity_beta.rs`](../../../crates/aureline-vfs/tests/filesystem_identity_beta.rs)
  re-proves schema, doc, fixture, and crate-consumer presence on disk and
  round-trips every case through serde.

## Out of scope

- Hosted intake, ticket routing, or upload transport for the support
  packet.
- Auto-fixing alias drift. The beta surface explains; it does not rewrite
  user-authored paths or rename canonical objects on the user's behalf.
- Adding new closed vocabulary tokens without updating the schema, the
  Rust module, the reviewer doc, and the protected corpus together.
