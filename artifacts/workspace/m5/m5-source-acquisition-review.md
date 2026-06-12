# M5 source-acquisition review sheets — reviewer artifact

Human-readable companion to the governed packet at
`artifacts/workspace/m5/m5-source-acquisition-review.json`. The full contract lives in
`docs/workspace/m5/m5-source-acquisition-review.md`; the typed model lives in the
`aureline-workspace` crate (`m5_source_acquisition_review`).

This artifact freezes one source-acquisition **review sheet** per M5 starter family. Each sheet
discloses the source-locator, checkout-plan, topology, and cost truth of a clone, open, import,
or resume **before** any irreversible network or disk action, keeps the verb distinct and
locked, and previews the follow-up setup queue without running it.

## Review-sheet roll-up (as of 2026-06-11)

| Sheet | Family | Verb | Source | Checkout | Cost | Trust | Review? | Active cues | Follow-ups |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `template-gallery-open` | template_starter | open | template_or_prebuild | full | light_fetch | first_party | yes | — | index warm-up, bundle rec |
| `framework-pack-clone` | framework_pack_starter | clone | remote_repository | sparse | moderate_fetch | review_required | yes | sparse | submodule, lfs, packages, index |
| `remote-shallow-clone` | remote_clone_starter | clone | remote_repository | shallow | heavy_fetch | review_required | yes | shallow, nested | docs import |
| `sync-handoff-mirror-clone` | sync_handoff | clone | mirror_or_proxy_repository | partial | moderate_fetch | trusted_continuation | yes | omitted data | packages, index |
| `companion-handoff-import` | companion_handoff | import | handoff_bundle | archive | light_fetch | review_required | yes | interrupted | docs, packages, bundle rec |
| `migration-import` | migration_import | import | imported_archive | archive | moderate_fetch | review_required | yes | nested | submodule, packages, index |
| `session-resume` | session_resume | resume | live_resume_session | live | light_fetch | trusted_continuation | yes | — | index warm-up |
| `local-folder-open` | local_folder_open | open | local_folder | full | local_no_fetch | first_party | **no** | — | — |

Seven sheets require review before acquisition; only the clean `local_folder_open` baseline is
review-free, proving the sheet is not a blanket gate. Five sheets carry an active topology cue
and three carry a cue that blocks first-useful-work — each one-step recoverable.

## What the sheets prove

- **Distinct, locked verbs.** Every sheet's verb is canonical for its source kind and locked.
  `remote-shallow-clone` keeps `clone` even though a local copy already exists, and
  `companion-handoff-import` keeps `import` even though the bundle looks resumable — the
  guardrail forbids rewriting clone into open or import into resume for convenience.
- **Topology truth before commitment.** Sparse (`framework-pack-clone`), shallow
  (`remote-shallow-clone`), partial/omitted (`sync-handoff-mirror-clone`), interrupted bootstrap
  (`companion-handoff-import`), nested repo (`remote-shallow-clone`, `migration-import`),
  submodule, and LFS-pointer states are all surfaced as cues with widen, unshallow, fetch,
  resume, review, init, and hydrate recoveries.
- **Cost truth before transfer.** Each sheet discloses an `expected_cost_band` (from
  `local_no_fetch` to `heavy_fetch`) and a `trust_stage` before any network byte.
- **Follow-up queue is previewed, not run.** All 16 follow-up items across the sheets carry
  `runs_implicitly: false`. Submodule init, LFS hydrate, docs import, package-restore
  suggestion, index warm-up, and bundle recommendation are each shown without being performed.
- **Reconstructable provenance.** Every sheet carries a `source_locator_ref` and a
  `checkout_plan_ref` so diagnostics and support export can explain which locator and checkout
  plan Aureline used.

## Summary counts

- 8 sheets: 3 clone, 2 open, 2 import, 1 resume.
- 7 require review before acquisition; 1 (`local_folder_open`) is review-free.
- 5 carry an active topology cue; 3 carry a blocking cue; 7 carry a deferred follow-up.
- 14 topology cues total, 12 one-step recoverable; 16 follow-up items total.
- 8 verbs locked; 2 sheets have an existing local copy and neither rewrites its verb.
