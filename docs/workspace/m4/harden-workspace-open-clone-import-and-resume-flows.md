# Entry hardening lineage — contract

This document describes the entry hardening lineage record: the workspace's
governed, export-safe projection that finalizes verb truth, target-kind /
topology truth, durable post-entry checkpoints, side-effect posture,
failure-repair truth, and cross-surface parity for the Open / Clone /
Import / Add root / Restore / Resume flows into one record per posture.

The record is the artifact every consuming surface (workspace status,
post-entry handoff card, admission checkpoint review, support export,
Help/About, headless CLI) ingests instead of cloning entry copy. It promotes
the existing live entry review record into a stable-line lineage proof so
later dashboards and audit packets cite the same projection.

## Inputs

The projection ingests one live record verbatim:

- **`ProjectEntryReviewRecord`** — built by
  `aureline_workspace::build_project_entry_review` from a typed
  `ProjectEntryReviewRequest`. The entry review already carries the
  admission packet, verb-specific review sheet, destination collision
  review, post-entry handoff card, failure repair state, surface parity
  rows, and admission checkpoint route. The lineage projection consumes
  the record without re-running validation; it adds the export-safe
  projection layer the stable line needs.

For deterministic replay, fixtures and the headless emitter serialize the
`ProjectEntryReviewRequest` (rather than the projected record) so the
projection is exercised end-to-end on every replay.

## What the record proves

The four pillars the entry-hardening lane is anchored on:

- **Verb truth.** The entry verb stayed distinct (no collapse to a generic
  "Get started" action), the resulting mode is one of the verb's allowed
  outcomes, and the verb-specific review sheet (Open local target, Open
  workspace manifest, Clone repository, Add root, Import artifact, Restore
  state) is the one the verb / target combination requires.
- **Target-kind / topology truth.** The post-entry topology class is named
  explicitly so later search, Git, trust, restore, and support surfaces
  inherit truthful topology metadata instead of reverse-engineering it
  from path strings. The class vocabulary is `durable_open`,
  `acquired_not_fetched`, `opened_sparse`, `pointer_only`, `nested_child`,
  `parent_root`, `imported_packet`, `inspect_only_staging`, and
  `restore_target`. Destination collisions force an explicit user choice;
  non-durable staging stays labelled.
- **Durable post-entry checkpoint.** The handoff card carries deferred
  work classes, blocking-now / recommended-soon / optional-later readiness
  tasks, the primary next action, and same-weight `Set up later`,
  `Open minimal`, and `Cancel` continuity actions. The admission
  checkpoint route is present and citable from the support packet.
- **No hidden side effects.** Clone never silently grants trust, defers
  dependency restore and task execution. Import refuses durable writes
  and state rehydration before review. Open-workspace forbids silent
  schema upgrades. Failed attempts preserve typed inputs, chosen
  destination, and redacted diagnostics; repair actions stay reachable.

## Narrow reasons

When a claim cannot be proven on the captured posture the record auto-
narrows below Stable with a named reason. Protective postures (a clean
clone-only, an inspect-only import staging, a restore from a recovery
checkpoint, a duplicate-clone-target collision the review correctly
guards) stay Stable — the contract working as designed is a pass, not a
gap.

| Narrow reason                              | Fires when                                                                            |
| ------------------------------------------ | ------------------------------------------------------------------------------------- |
| `review_sheet_mismatch`                    | The verb-specific review sheet does not match the verb / target pairing               |
| `clone_grants_trust_silently`              | Clone review is missing the no-trust guard                                            |
| `clone_runs_setup_silently`                | Clone review admits silent dependency restore or task / hook execution                |
| `import_writes_before_review`              | Import review admits durable writes before review                                     |
| `import_rehydrates_before_review`          | Import review admits state rehydration before review                                  |
| `workspace_manifest_upgrades_silently`     | Open-workspace admits silent schema upgrades                                          |
| `destination_collision_no_explicit_choice` | A collision is present but no explicit user choice is required                        |
| `handoff_missing_continuity_paths`         | Neither `Set up later` nor `Open minimal` is offered on the handoff card              |
| `failure_repair_loses_state`               | Typed source input, destination, diagnostics, or repair actions are not preserved    |
| `failure_repair_leaks_secret`              | The redacted source-input label still appears to leak credentials                     |
| `surface_parity_drift`                     | A surface drifts verb, target, mode, or review model                                  |
| `deep_link_intent_review_missing`          | DeepLink surface is covered without the deep-link intent review requirement           |
| `inspection_hook_unavailable`              | A required pre-commit inspection hook is unavailable                                  |
| `lineage_export_unsafe`                    | The lineage refs are empty (would break support export)                               |

## Inspection hooks

The pre-commit hook table lets the user inspect the entry, the destination
collision, the post-entry handoff card, the failure repair state, and
export the lineage record before any durable write or trust change.
Fixtures may model a degraded subset to prove the corresponding narrow
reason; the lineage record narrows below Stable when any non-collision
hook is unavailable.

| Hook class                | Action id                                  | Purpose                                                       |
| ------------------------- | ------------------------------------------ | ------------------------------------------------------------- |
| `review_entry`            | `entry_hardening.review_entry`             | Open the verb-specific entry review sheet                     |
| `inspect_collision`       | `entry_hardening.inspect_collision`        | Inspect the destination collision review                      |
| `inspect_handoff`         | `entry_hardening.inspect_handoff`          | Inspect the post-entry handoff card                           |
| `inspect_failure_repair`  | `entry_hardening.inspect_failure_repair`   | Inspect the failure repair state                              |
| `export`                  | `entry_hardening.export`                   | Export the entry hardening lineage record without raw inputs  |

## Consumer surfaces

The same projection is consumed by:

- The workspace post-entry handoff card and admission checkpoint review.
- The headless CLI emitter
  (`crates/aureline-workspace/src/bin/aureline_entry_hardening_lineage.rs`).
- Help/About and support export (via `entry_hardening_lineage_lines`).
- The replay gate
  (`crates/aureline-workspace/tests/entry_hardening_lineage_replay.rs`),
  which re-builds the entry review, re-projects the lineage from every
  fixture, and asserts equality with the checked-in `expected` record.
