# Fixtures: M5 source-acquisition review sheets

This directory contains fixture metadata for the `m5_source_acquisition_review_packet`.

The canonical full corpus is checked in at:

`artifacts/workspace/m5/m5-source-acquisition-review.json`

and is validated by `schemas/workspace/m5-source-acquisition-review.schema.json` and the typed
model in the `aureline-workspace` crate (`m5_source_acquisition_review`).

## Coverage

- One review sheet per M5 starter family: `template_starter`, `framework_pack_starter`,
  `remote_clone_starter`, `sync_handoff`, `companion_handoff`, `migration_import`,
  `session_resume`, and `local_folder_open`.
- Every entry verb is exercised and stays distinct: `clone` (3 sheets), `open` (2), `import`
  (2), and `resume` (1). Each recorded verb is canonical for its source kind
  (`EntryVerb::is_canonical_for`) and locked.
- The clone/open and import/resume guardrails are both exercised: `remote-shallow-clone` keeps a
  `clone` even though a local copy already exists, and `companion-handoff-import` keeps an
  `import` even though a handoff bundle looks resumable.
- All seven topology cue kinds are exercised — `nested_repo`, `submodule`, `shallow_history`,
  `sparse_checkout`, `lfs_pointer`, `interrupted_bootstrap`, and `omitted_data` — across the
  cue states `active`, `pending`, `partial`, and `not_present`. Every applicable cue offers a
  one-step recovery, every cue that blocks first-useful-work is recoverable, and `not_present`
  cues offer no recovery.
- All eight recovery actions are exercised: `init_submodules`, `hydrate_lfs`,
  `widen_sparse_scope`, `unshallow_history`, `resume_bootstrap`, `fetch_omitted_data`,
  `review_nested_repo`, and `none`.
- All six follow-up kinds are exercised — `submodule_init`, `lfs_hydrate`, `docs_import`,
  `package_restore_suggestion`, `index_warm_up`, and `bundle_recommendation` — across the
  postures `deferred`, `awaiting_user_action`, `awaiting_trust_admission`, and `suggested`.
  Every follow-up item carries `runs_implicitly: false`.
- The review gate is exercised in both directions: seven sheets require review before
  acquisition (a clone/import/resume verb, a network-fetch cost band, an applicable cue, or a
  deferred follow-up), while the clean `local_folder_open` baseline is review-free, proving the
  sheet is not a blanket gate.
- Every sheet carries a `source_locator_ref` and a `checkout_plan_ref` so diagnostics and
  support export can reconstruct which locator and checkout plan Aureline used, plus
  diagnostics, support-export, help-surface, docs-badge, and release-evidence refs.

## How it is validated

The typed model parses the embedded packet and runs `validate()`, which checks the closed
vocabularies, per-sheet verb canonicality and lock, the no-implicit-follow-up guard, complete
provenance, the recomputed review requirement, topology-cue and follow-up consistency, required
caveats, and the recomputed summary. The unit tests in
`crates/aureline-workspace/src/m5_source_acquisition_review/tests.rs` assert the embedded packet
validates clean and that every verb, cue kind, and follow-up kind is exercised.
