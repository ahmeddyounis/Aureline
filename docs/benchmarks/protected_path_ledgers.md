# Protected-path, latency-budget, and evidence-linkage ledgers

This document is the **normative** companion to:

- [`/artifacts/perf/protected_path_ledger.yaml`](../../artifacts/perf/protected_path_ledger.yaml)
- [`/artifacts/perf/latency_budget_ledger.yaml`](../../artifacts/perf/latency_budget_ledger.yaml)
- [`/artifacts/perf/evidence_linkage_seed.yaml`](../../artifacts/perf/evidence_linkage_seed.yaml)

These three artifacts turn the protected-path narrative into one
reviewable machine register:

- the **protected-path ledger** owns stable path ids, ownership,
  boundary definitions, and reserved segment ids;
- the **latency-budget ledger** owns threshold provenance, budget
  values, measurement sources, degraded/fallback posture, and waiver
  authority; and
- the **evidence-linkage seed** owns the joins from each path id to
  journey traces, benchmark-corpus rows, qualification rows, and the
  packet families that review the path.

If this document disagrees with the YAML, this document wins and the
YAML updates in the same change.

Companion artifacts:

- [`/fixtures/benchmarks/corpus_manifest.yaml`](../../fixtures/benchmarks/corpus_manifest.yaml)
  — benchmark fixtures and reference-workspace ids.
- [`/artifacts/bench/fitness_function_catalog.yaml`](../../artifacts/bench/fitness_function_catalog.yaml)
  — protected fitness rows and waiver authorities.
- [`/artifacts/bench/protected_metrics.yaml`](../../artifacts/bench/protected_metrics.yaml)
  — governed threshold snapshots and calibration posture.
- [`/docs/benchmarks/journey_trace_taxonomy.md`](./journey_trace_taxonomy.md)
  — journey-trace vocabulary and seeded segment ids.
- [`/artifacts/product/task_success_corpus_seed.yaml`](../../artifacts/product/task_success_corpus_seed.yaml)
  — onboarding and first-useful-work scenario ids.
- [`/artifacts/compat/qualification_matrix_seed.yaml`](../../artifacts/compat/qualification_matrix_seed.yaml)
  — compatibility / qualification rows later packets cite by stable id.
- [`/docs/benchmarks/benchmark_publication_pack_template.md`](./benchmark_publication_pack_template.md)
  — public benchmark packet family that must carry protected path ids
  and ledger revisions whenever it makes path-level claims.
- [`/docs/release/release_evidence_packet_template.md`](../release/release_evidence_packet_template.md)
  — release packet family that consumes path ids and budget lineage.
- [`/docs/benchmarks/corpus_governance.md`](./corpus_governance.md)
  and
  [`/artifacts/bench/corpus_change_control.yaml`](../../artifacts/bench/corpus_change_control.yaml)
  — change-control policy the ledgers inherit.

## Why these ledgers exist

The benchmark corpus, fitness catalog, journey traces, and run-result
schema already freeze the pieces of performance truth. What they did not
freeze was the **one public list of protected paths** that says:

1. which path ids are stable;
2. which segments are already wired versus merely reserved;
3. where each path's budget came from;
4. which fitness row or contract gate a path resolves through; and
5. which evidence and review packets must refresh when the path changes.

Without these ledgers, a startup trace, a save contract, a task-success
scenario, and a shiproom packet can each talk about the "same" path
using different names.

## Frozen path set at this revision

The seeded path ids are:

- `path.shell.launch`
- `path.shell.first_useful_chrome`
- `path.command_palette.open`
- `path.editor.placeholder_open`
- `path.editor.first_useful_edit`
- `path.editor.save`
- `path.workspace.restore`
- `path.onboarding.start_center_first_useful_edit`

Adding a path id is additive-minor. Renaming or silently dropping a path
id is breaking and requires a named change record.

## Frozen vocabularies

### Path status

- `seeded` — the path id is stable and already has a budget row and an
  evidence-link row.
- `provisional` — the path id is reserved now so later wiring cannot
  invent a second identity.

### Segment status

- `stable` — the segment id already appears in a seeded journey trace.
- `provisional` — the segment id is intentionally reserved here because
  the trace family has not emitted it yet.

### Budget-source kinds

- `published_ux_budget` — threshold comes from the PRD, TAD, or UX spec.
- `protected_metrics_contract` — threshold or gate comes from
  `artifacts/bench/protected_metrics.yaml`.
- `provisional_engineering_target` — threshold is reserved so future
  work does not invent a new id, but it is not yet ratified.
- `degraded_state_fallback_rule` — the path is governed at least partly
  by a truthful fail-soft contract rather than a pure latency number.

### Review-packet families

- `benchmark_report`
- `verification`
- `compatibility_report`
- `claim_manifest`
- `shiproom`
- `support_bundle`
- `release_evidence_packet`

## Change-control rules

Path additions, removals, renames, and scope splits are governed like
protected benchmark changes, not like casual notes.

At minimum, a protected-path scope change MUST:

1. append a named `change_record` in
   `artifacts/perf/protected_path_ledger.yaml`;
2. update the matching rows in
   `artifacts/perf/latency_budget_ledger.yaml` and
   `artifacts/perf/evidence_linkage_seed.yaml` in the same change;
3. state whether comparability or publication posture changed; and
4. refresh any claim-bearing packet or publication pack that quoted the
   affected path id.

The benchmark-governance policy is the outer workflow; these ledgers are
the canonical payloads that workflow updates.

## Save and restore linkage rule

`path.editor.save` and `path.workspace.restore` are not latency-only
rows. They are also intent-preservation rows.

Those two paths MUST keep the following hooks addressable:

- mutation-journal linkage (`schemas/workspace/mutation_journal.schema.json`);
- save-manifest linkage
  (`schemas/runtime/vfs_save_envelope.schema.json`);
- restore-provenance linkage
  (`schemas/state/restore_provenance.schema.json`); and
- a reserved local-history snapshot hook until the checkpoint record
  family lands.

A save or restore packet that quotes latency without quoting the linked
truth contracts is non-conforming.

## Relationship to the existing benchmark assets

- The **fitness catalog** still owns metric identity and waiver
  authority.
- The **protected metrics file** still owns governed threshold state.
- The **journey traces** still own checkpoints and emitted segments.
- These ledgers own the **path layer above those assets**: stable path
  ids, stitched segment reservations, budget provenance, and evidence
  joins.
