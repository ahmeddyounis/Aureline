# Harden Workspace Open, Clone, Import, And Resume Flows — proof packet

Reviewer-facing proof packet for the workspace entry hardening lane: verb
truth, target-kind / topology truth, durable post-entry checkpoints,
side-effect posture, failure-repair truth, and cross-surface parity
composed into one governed, export-safe record per posture. This packet is
the stable-line anchor for this lane: dashboards, docs, Help/About
surfaces, and support exports should ingest the typed sources below rather
than cloning this packet's text.

## Canonical machine sources

- Lineage projection and contract types:
  [`/crates/aureline-workspace/src/entry_hardening_lineage/`](../../../crates/aureline-workspace/src/entry_hardening_lineage/)
- Live entry review record (input):
  [`/crates/aureline-workspace/src/entry/`](../../../crates/aureline-workspace/src/entry/)
- Admission packet and checkpoint route:
  [`/crates/aureline-workspace/src/admission/`](../../../crates/aureline-workspace/src/admission/)
- Entry flow vocabulary:
  [`/crates/aureline-workspace/src/entry_flows/`](../../../crates/aureline-workspace/src/entry_flows/)
- Schema:
  [`/schemas/workspace/entry_hardening_lineage.schema.json`](../../../schemas/workspace/entry_hardening_lineage.schema.json)
- Headless emitter / CLI:
  [`/crates/aureline-workspace/src/bin/aureline_entry_hardening_lineage.rs`](../../../crates/aureline-workspace/src/bin/aureline_entry_hardening_lineage.rs)
- Fixtures:
  [`/fixtures/workspace/m4/entry_hardening_lineage/`](../../../fixtures/workspace/m4/entry_hardening_lineage/)
- Replay gate:
  [`/crates/aureline-workspace/tests/entry_hardening_lineage_replay.rs`](../../../crates/aureline-workspace/tests/entry_hardening_lineage_replay.rs)
- Companion contract doc:
  [`/docs/workspace/m4/harden-workspace-open-clone-import-and-resume-flows.md`](../../../docs/workspace/m4/harden-workspace-open-clone-import-and-resume-flows.md)
- Typed consumer: `aureline_workspace::project_entry_hardening_lineage`

## What this packet proves

1. **Verb truth.** The lineage record carries the entry verb (`open`,
   `clone`, `import`, `add_root`, `restore`, `resume`,
   `start_from_snapshot`), the resolved target kind, the resulting mode,
   and the verb-specific review sheet kind. `verb_stays_distinct` is true
   for every supported verb; `sheet_matches_verb` is true only when the
   sheet kind matches the verb / target pairing. Worked examples:
   [`clone_full_history_stable.json`](../../../fixtures/workspace/m4/entry_hardening_lineage/clone_full_history_stable.json),
   [`import_inspect_only_staging_stable.json`](../../../fixtures/workspace/m4/entry_hardening_lineage/import_inspect_only_staging_stable.json).

2. **Target-kind / topology truth.** Each posture names its topology class
   explicitly (`durable_open`, `acquired_not_fetched`, `opened_sparse`,
   `pointer_only`, `nested_child`, `parent_root`, `imported_packet`,
   `inspect_only_staging`, `restore_target`) so later search, Git,
   trust, restore, and support surfaces inherit truthful topology
   metadata. Worked examples:
   [`clone_partial_filter_opened_sparse_stable.json`](../../../fixtures/workspace/m4/entry_hardening_lineage/clone_partial_filter_opened_sparse_stable.json),
   [`clone_pointer_only_stable.json`](../../../fixtures/workspace/m4/entry_hardening_lineage/clone_pointer_only_stable.json),
   [`clone_acquired_not_fetched_stable.json`](../../../fixtures/workspace/m4/entry_hardening_lineage/clone_acquired_not_fetched_stable.json),
   [`restore_last_session_stable.json`](../../../fixtures/workspace/m4/entry_hardening_lineage/restore_last_session_stable.json).

3. **Durable post-entry checkpoint.** Every record carries the admission
   checkpoint id, the handoff card id, the deferred work classes, the
   blocking-now / recommended-soon / optional-later readiness tasks, the
   primary next action, and the same-weight `Set up later`,
   `Open minimal`, and `Cancel` continuity actions. The narrow reason
   `handoff_missing_continuity_paths` fires only when neither
   continuity action is offered.

4. **No hidden side effects.** Clone never silently grants trust, defers
   dependency restore and task / hook execution. Import refuses durable
   writes and state rehydration before review. Open-workspace forbids
   silent schema upgrades. Failed attempts preserve typed inputs, chosen
   destination, and redacted diagnostics; repair actions stay reachable.
   The narrow reasons `clone_grants_trust_silently`,
   `clone_runs_setup_silently`, `import_writes_before_review`,
   `import_rehydrates_before_review`,
   `workspace_manifest_upgrades_silently`,
   `failure_repair_loses_state`, and `failure_repair_leaks_secret`
   fire if any of these invariants is broken.

5. **Cross-surface parity.** The record snapshots the surface parity rows
   so support packets can prove the same verb, target, mode, and review
   model survive Start Center, Command palette, Drag-and-drop, System
   file association, Deep link, CLI/headless, and Workspace switcher
   entry. DeepLink surfaces must additionally carry the deep-link intent
   review requirement, or the record narrows below Stable with
   `deep_link_intent_review_missing`.

6. **Export honesty.** `raw_payload_excluded = true` on every record; the
   record only carries redaction-aware labels and opaque refs to the
   live entry review, admission packet, and admission checkpoint. The
   replay gate proves every fixture is support-export safe.

## Fixture corpus

| Fixture                                                   | Verb     | Topology               | Stable?  |
| --------------------------------------------------------- | -------- | ---------------------- | -------- |
| `clone_full_history_stable.json`                          | clone    | durable_open           | Stable   |
| `clone_partial_filter_opened_sparse_stable.json`          | clone    | opened_sparse          | Stable   |
| `clone_pointer_only_stable.json`                          | clone    | pointer_only           | Stable   |
| `clone_acquired_not_fetched_stable.json`                  | clone    | acquired_not_fetched   | Stable   |
| `import_inspect_only_staging_stable.json`                 | import   | inspect_only_staging   | Stable   |
| `restore_last_session_stable.json`                        | restore  | restore_target         | Stable   |
| `clone_missing_hook_narrowed.json`                        | clone    | durable_open           | Narrowed |

The replay gate (`projection_replays_each_fixture_exactly`) re-builds the
entry review and re-projects the lineage from each fixture's request and
inspection-hook set, then asserts the result equals the checked-in
`expected` record exactly. Drift fails CI.

## Stable-line claim

- The checked-in implementation, fixtures, schema, headless emitter,
  contract doc, and replay gate for the entry hardening lane are current
  on the release branch.
- The replay gate proves every fixture is support-export safe, that the
  corpus covers both Stable and narrowed-below-Stable postures, and that
  Stable postures preserve verb truth, target-kind truth, durable
  checkpoint truth, side-effect posture, failure-repair truth, and
  cross-surface parity. The corpus exercises six distinct topology
  classes so later surfaces have a deterministic class vocabulary to
  inherit.
- Any surface still lacking stable qualification narrows below Stable
  with a named reason instead of inheriting an adjacent green row.
