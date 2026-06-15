# Offboarding continuity plan contract (M5 heavy artifact families)

The offboarding continuity plan is the operator-facing object the shell shows
*before* an account offboarding, device reset, workspace wipe, or sign-out
cleanup removes anything. Where the class-selective clear-data review sheet is the
per-class cleanup review, this plan is the honest, whole-offboarding summary. For
every heavy artifact family it touches it states:

- **what the bytes are** — exportable durable state the user should take with
  them (user-owned recovery state, captured evidence), or non-portable derived
  data that simply rebuilds (caches, packs);
- **what its removal would break** — the offline, mirror, certified-workspace,
  policy-bundle, evidence, or recovery-state continuity at stake, kept visible
  before any deletion;
- **whether it is protected** — captured evidence and user-owned recovery state,
  plus families pinned by an offline bundle, a certified template/archetype, a
  release/mirror artifact graph, or a last-known-good policy bundle, stay
  retained unless the operator explicitly reviews them away;
- **the export-before-delete posture** — required on protected classes, offered
  on continuity-pinned packs, not applicable on pure caches;
- **the portability headline** — a plan-level honesty statement that never
  implies the user exported everything when only caches were cleared.

The canonical product object is `m5_offboarding_continuity_plan`, owned by
`crates/aureline-support/src/m5_offboarding_continuity` and bound to the boundary
schema at `schemas/storage/m5_offboarding_continuity.schema.json`. It mints no new
storage primitive: the storage-class, artifact-family, authority, and pin-source
vocabularies re-export verbatim from
`artifacts/storage/m5_artifact_family_storage_matrix.yaml` and
`artifacts/runtime/storage_classes.yaml`. The composer `compose_offboarding_plan`
folds the frozen artifact-family matrix, so the plan, the clear-data review, and
the storage-governance matrix can never disagree about which classes are protected
or require export-before-delete.

## Invariants

A plan is admissible only when it holds every invariant below; the validator in
`m5_offboarding_continuity` and the schema both enforce them, and the scenario
corpus under `fixtures/storage/m5_offboarding_continuity_cases/` exercises them.

1. **Protected and continuity-pinned families are never silently disposed.**
   Captured evidence (`evidence_support_cache`), user-owned recovery state
   (`user_owned_recovery_state`), and any family pinned by an offline bundle,
   certified template/archetype, release/mirror artifact graph, or last-known-good
   policy bundle is retained by default. It can only enter the disposed bucket
   (`export_then_dispose`) when the operator has explicitly reviewed it away.
2. **Protected classes require export-before-delete.** Evidence and user-owned
   recovery rows always carry `export_before_delete_class =
   export_required_before_delete` and an `export_action_ref`, in either bucket.
3. **Portability is honest.** `portability_class` distinguishes exportable durable
   state and captured evidence from rebuildable / non-portable derived data, and
   `portability_honesty_class` is computed: `nothing_disposed_all_retained` when
   no row is disposed, `durable_state_exported_before_removal` when any disposed
   row is durable (each then `export_then_dispose` with export required), and
   `caches_only_removed_durable_retained` otherwise. A caches-only offboarding can
   never claim it exported everything.
4. **Continuity is kept visible before deletion.** Every row carries the
   continuity warnings its removal would raise, derived from its storage class and
   the pins actually present. The plan's `continuity_warnings` are exactly the
   active losses across the disposed rows; reviewing an offline / certified /
   policy / mirror pin away surfaces a guardrail notice naming the broken promise.
5. **Byte arithmetic is exact and follows the disposition.** `total_bytes =
   disposed_bytes + retained_bytes`; a disposed row removes all of its bytes, a
   retained row keeps all of them, and the plan totals equal the row sums.
6. **The plan offers the inspector and the class-selective review.** Every plan
   carries `open_inspector_action_ref = action.storage.open_inspector` and
   `open_clear_data_review_action_ref = action.storage.open_clear_data_review`, so
   offboarding is never a generic, irreversible delete-all button.
7. **Pins outside the matrix carry no continuity.** The composer drops any pin a
   family's matrix row does not admit, so a caller can never invent offline or
   certified continuity an artifact family does not actually carry.

## Support export

`OffboardingContinuityCorpus::support_export` projects the corpus into a
metadata-safe envelope (`m5_offboarding_continuity_support_export`) the
Help / About / diagnostics / support-bundle surfaces quote without leaking raw
payloads, paths, or credentials. It counts plans, retained protected/continuity
families, plans that exported durable state away, and plans with active continuity
warnings, and carries the storage-class and pin-state summary per plan. The
checked-in golden lives at
`fixtures/storage/m5_offboarding_continuity/support_export.golden.json` and is
regenerated with
`cargo run -p aureline-support --example dump_m5_offboarding_continuity_support_export`.
