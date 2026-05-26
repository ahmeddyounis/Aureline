# Mutation-journal and generated-artifact lineage — contract

This document describes the mutation-journal and generated-artifact
lineage record: the workspace's governed, export-safe projection
that proves how Aureline tracks mutations across editor, formatter,
AI, build, lockfile, and preview paths and how it labels the
generated and mirrored artifacts those paths produce — and proves
that the lineage never silently replays privileged work, never
treats a generated artifact as the canonical edit target, and never
narrows trust in the artifact source-of-truth into an unreviewable
state.

Where the recovery-ladder lineage proves *the ordered recovery
sequence* Aureline walks when something goes wrong, and the cache /
storage-class lineage proves *the storage layer underneath caches
and durable state*, this record proves the *mutation surface* on top
of both: which mutation paths exist, how each path's journal entry
is no-rerun safe, which generated or mirrored artifact classes are
governed, which canonical source ref each artifact carries, which
generator or signer identity signed it, which output digest pins it,
which drift state and default edit posture it claims, and which
surfaces (tree, breadcrumb, search, review, AI-context, save,
support) label it so the user, the review surfaces, and the AI
never see a generated artifact treated as the canonical edit target.

The record is the single artifact every consuming surface (workspace
mutation lineage status, command-palette mutation actions, Help/About,
support cleanup tool, headless CLI, support export) ingests instead
of cloning status text.

## Input

The projection ingests a live
[`MutationAndGeneratedArtifactInputs`](../../../crates/aureline-workspace/src/mutation_and_generated_artifact_lineage/mod.rs)
envelope verbatim. The envelope carries:

- one
  [`MutationPathObservation`](../../../crates/aureline-workspace/src/mutation_and_generated_artifact_lineage/mod.rs)
  per governed mutation path (editor, formatter, AI apply, build
  runner, lockfile resolver, preview runtime), recording the journal
  entry id, the no-rerun posture, the commit action / commit
  disclosure ids, the privileged-surface flag, and the support-export
  projection.
- one
  [`GeneratedArtifactObservation`](../../../crates/aureline-workspace/src/mutation_and_generated_artifact_lineage/mod.rs)
  per governed generated / mirrored artifact (build output, generated
  source sibling, structured lockfile, notebook output, preview
  snapshot, plus the optional design snapshot and mirrored doc /
  schema / model / registry artifact rows), recording the canonical
  source ref, generator / signer identity, output digest, drift state,
  default edit posture, override / recovery disclosure ids, the
  labeling surfaces the artifact appears on, and the support-export
  projection.

For determinism and replay, the projection accepts the same envelope
shape the fixtures and the headless emitter consume.

## What the record proves

- **Mutation-path coverage truth.** Every governed mutation path
  ships a row bound to one closed `mutation_path_kind` (`editor`,
  `formatter`, `ai_apply`, `build_runner`, `lockfile_resolver`,
  `preview_runtime`). The corpus seeds one row per required path so
  the user never lands on a release where a mutation path is missing
  from the lineage.
- **Generated-artifact coverage truth.** Every governed artifact
  ships a row bound to one closed `generated_artifact_kind`. The
  required set covers `build_output`, `generated_source_sibling`,
  `structured_lockfile`, `notebook_output`, and `preview_snapshot`;
  the optional set covers `design_snapshot` and mirrored doc /
  schema / model / registry artifacts.
- **Canonical-lineage truth.** Every artifact row references a
  non-empty canonical source ref, generator or signer identity, and
  output digest so consumers can pin the canonical sibling and the
  regeneration provenance before re-running.
- **Drift truth.** Every artifact row declares one closed
  `drift_state_class`. `drifted_from_generator` rows must reference a
  recovery / regenerate guidance disclosure id.
- **Edit-posture honesty.** Every artifact row declares one closed
  `default_edit_posture_class`. Only artifact classes that explicitly
  support round-trip-safe editing (currently `structured_lockfile`)
  may declare `round_trip_safe`; every other generated artifact
  defaults to `block_writes_default`. Whenever the user overrides the
  default edit posture on a non-authoritative generated artifact, the
  row enters `diverged_from_generator`, which requires both an
  override disclosure id and a recovery / regenerate guidance
  disclosure id and surfaces visible diverged-from-generator state
  plus recovery / regenerate guidance in-product and in exported
  evidence.
- **Labeling-surface coverage truth.** Each artifact row records the
  surfaces it labels itself on. The required set is `tree`,
  `breadcrumb`, `search`, `review`, `ai_context`, `save`, and
  `support`. An artifact missing any required surface narrows the
  record.
- **Mutation no-rerun honesty.** Every mutation-path row declares
  one closed `mutation_no_rerun_posture`. Privileged paths
  (`ai_apply`, `build_runner`, `lockfile_resolver`, `preview_runtime`)
  must declare `explicit_user_action_required` (or
  `terminal_no_further_run`) and reference a commit action id plus a
  commit disclosure id — never
  `deterministic_replay_after_checkpoint`.
- **Support-export honesty.** Each row's support-export projection
  preserves the mutation path or artifact class, the canonical
  source ref, the generator or signer identity, the output digest,
  the drift state, the default edit posture, the labeling surfaces,
  and the disclosure ids, and excludes raw secrets, approval
  tickets, delegated credentials, and live authority handles.
- **Pre-action inspection-hook honesty.** A controlled set of
  pre-action inspection / repair hooks (`inspect_lineage`,
  `compare_canonical`, `regenerate`, `export_before_override`,
  `rollback_override`, `export`, `repair`) is reachable so
  destructive overrides and regenerations stay reviewable.
- **Producer attribution.** Each record carries a producer ref,
  schema version, capture timestamp, and integrity hash so replay
  and support pipelines can pin the source before applying.
- **Lineage and export honesty.** The record sets
  `raw_payload_excluded = true` and carries only opaque refs to the
  source workspace, corpus, and producer.

## Output record shape

The projection produces a single
[`MutationAndGeneratedArtifactLineageRecord`](../../../crates/aureline-workspace/src/mutation_and_generated_artifact_lineage/mod.rs)
with the following pillars:

- `mutation_path_coverage` — per-mutation-path rows plus the
  `all_required_paths_present` flag.
- `generated_artifact_coverage` — per-artifact rows plus the
  `all_required_artifact_classes_present` flag.
- `canonical_lineage_truth` — whether every artifact references a
  canonical source ref, generator identity, and output digest.
- `drift_truth` — drifted-artifact count plus the
  `all_drifted_artifacts_have_disclosure` flag.
- `edit_posture_honesty` — whether every `round_trip_safe` claim is
  supported by the artifact class, the diverged-artifact count, and
  whether every diverged artifact carries both required disclosures.
- `labeling_surface_coverage` — whether every governed artifact is
  labeled on every required surface.
- `mutation_no_rerun_honesty` — whether every privileged mutation
  path is safe and every explicit path carries metadata.
- `support_export_honesty` — per-row field preservation and
  redaction flags.
- `inspection_hooks` — the captured pre-action inspection / repair
  hook table.
- `producer_attribution` — opaque producer ref, schema version,
  capture timestamp, and integrity hash.
- `stable_qualification` — whether the record proves the contract on
  the claimed posture, with named narrow reasons when not.
- `summary` — a single-line human-readable summary.

## Stable qualification

A record is `stable` only when every pillar passes. Otherwise it is
`narrowed_below_stable` with one or more named narrow reasons drawn
from the closed
[`MutationAndGeneratedArtifactLineageNarrowReason`](../../../crates/aureline-workspace/src/mutation_and_generated_artifact_lineage/mod.rs)
vocabulary.

## Consumers

The workspace mutation-lineage status surface, the command-palette
mutation / artifact actions, Help/About, the support cleanup tool,
the headless CLI, and the support export ingest the same
human-readable projection
(`mutation_and_generated_artifact_lineage_lines`) so no surface
clones status text.

## Verification

```sh
cargo test -p aureline-workspace --lib mutation_and_generated_artifact_lineage
cargo test -p aureline-workspace --test mutation_and_generated_artifact_lineage_replay
cargo run -p aureline-workspace --bin aureline_mutation_and_generated_artifact_lineage -- --lines \
  fixtures/workspace/m4/mutation_and_generated_artifact_lineage/baseline_mutation_artifact_stable.json
```
