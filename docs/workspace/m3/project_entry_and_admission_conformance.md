# Project-entry and workspace-admission conformance (M3 beta)

This contract governs the M3 project-entry and workspace-admission beta boundary:
how Aureline admits a user into work through Open, Clone, Import, Add root,
Restore, recent-work reopen, OS open-with, CLI/headless, and high-authority
protocol-handler deep links. It is proven by a regression-gated conformance corpus
so the entry promise cannot quietly regress before a beta entry row hardens.

- Runtime model: `aureline_workspace::entry::ProjectEntryReviewRecord`, built by
  `aureline_workspace::build_project_entry_review`.
- Corpus: [`fixtures/workspace/m3/project_entry_and_admission/`](../../../fixtures/workspace/m3/project_entry_and_admission/)
  (`manifest.json` is the single source of truth).
- Harness: [`crates/aureline-qe/src/project_entry_admission/`](../../../crates/aureline-qe/src/project_entry_admission/),
  replayed by `cargo test -p aureline-qe --test project_entry_admission_conformance`.
- Corpus schema: [`schemas/workspace/entry_admission_conformance.schema.json`](../../../schemas/workspace/entry_admission_conformance.schema.json).
- Published evidence:
  [`artifacts/ux/m3/first_landing_truth_matrix.json`](../../../artifacts/ux/m3/first_landing_truth_matrix.json)
  and [`artifacts/migration/m3/project_entry_admission_report.md`](../../../artifacts/migration/m3/project_entry_admission_report.md).

## Normative requirements

Each requirement below is enforced by the corpus; a regression fails the
conformance test before the entry row can be claimed.

1. **Verb separation.** Every entry verb resolves to its own review sheet
   (`open_local_target`, `open_workspace_manifest`, `clone_repository`, `add_root`,
   `import_artifact`, `restore_state`). A clone is never confused with an open, and
   the review sheet kind MUST match the entry verb.
2. **Source-labelled access.** The reviewed source access class
   (`local_filesystem`, `direct_online`, `mirror_first`, `offline_snapshot`,
   `air_gapped_media`) is explicit and derived from the target and route, not
   guessed silently.
3. **Destination-collision review.** When a destination matches a prior clone, is
   policy-blocked, or otherwise collides, the entry surfaces a collision review
   that requires an explicit, reviewed choice instead of a silent overwrite.
4. **Truthful first landing.** The first landing surface is chosen from the
   first-useful entry source and explains why it was chosen. A review/incident deep
   link lands on the linked object (`linked_review_incident_or_work_item`) rather
   than collapsing into a generic open.
5. **Readiness grouping.** The post-entry handoff card preserves the Blocking now /
   Recommended soon / Optional later grouping and never runs setup, dependency
   installs, repo tasks, or hooks to satisfy it.
6. **No silent side effects.** No entry path silently grants trust, executes setup,
   runs repo tasks or hooks, auto-trusts the workspace, or auto-installs setup. The
   handoff card declares the work it deliberately defers (trust grant, dependency
   restore, task execution, hook execution, and verb-specific deferrals).
7. **Cross-surface parity.** The same activation preserves verb, target, resulting
   mode, and review model across Start Center, command palette, drag-and-drop, OS
   open-with, protocol-handler deep links, CLI/headless, and the workspace
   switcher. The deep-link surface always requires deep-link intent review.
8. **Failure recovery.** A failed entry preserves typed source input, the chosen
   destination, and redacted diagnostics, and offers a safe repair or
   safer-verb path without restarting the whole flow.
9. **Redaction.** Credential-bearing source URLs never survive into the built
   record; only typed labels and opaque refs cross the support-safe boundary.

## Drill model

Positive drills carry one `ProjectEntryReviewRequest`. The harness builds the
record, requires `contract_findings()` to be empty, enforces the universal
guarantees in requirement 6 and 7, and matches every pinned `expect` field
(review sheet, source access, first-useful entry source, landing surface,
resulting mode, primary next action, collision posture, readiness counts,
deferred work, and import inspect/write posture).

Negative drills build a valid record, apply one typed tamper, and require the
entry contract to reject it:

| Tamper | Required finding |
| --- | --- |
| `clone_grants_trust` | clone must defer trust, dependencies, and tasks |
| `clone_exposes_credentials` | clone remote label must not expose credentials |
| `import_writes_before_review` | import must defer durable write and state rehydration |
| `import_inspect_advertises_write` | inspect-only import must advertise no write |
| `collision_skips_explicit_choice` | destination collision requires explicit choice |
| `surface_parity_drift` | surface parity drift on |
| `failure_repair_drops_inputs` | failed entry repair state must preserve inputs and redacted diagnostics |
| `route_auto_trust` | auto_trust_allowed must remain false |
| `route_auto_install` | auto_install_allowed must remain false |
| `review_sheet_mismatch` | review sheet kind does not match entry verb |

## Downgrade discipline

The first-landing truth matrix carries a `status` per row. A row is `verified`
only when its drill passes. It downgrades to `restricted` when admission is
policy-blocked or restricted-mode (the row still lands safely but commit stays
gated behind an explicit choice), to `partial` when entry is proven but a claimed
surface or follow-on is not, and to `retest_pending` when evidence is stale or the
builder changes. Switching proof, migration docs, and claim-manifest rows MUST
follow the lowest row status; a Partial, Restricted, or Retest-pending entry row
cannot be cited as a clean beta switching claim.

## Change control

`manifest.json` is the single source of truth. Removing a positive or negative
drill without a replacement, or weakening an expectation, is a breaking change to
the `workspace.project_entry_and_admission.beta` corpus and must be reviewed as a
contract change. The published matrix and report are regenerated from the manifest
and are asserted by the suite to cover every drill, so they cannot drift.
