# Ship Mutation-Journal and Generated-Artifact Lineage Across Editor, Formatter, AI, Build, Lockfile, and Preview Flows — proof packet

Reviewer-facing proof packet for the mutation-journal and
generated-artifact lineage lane: every required mutation path
(editor, formatter, AI apply, build runner, lockfile resolver, and
preview runtime) is bound to a journal entry, a closed no-rerun
posture, and (where the path touches privileged surfaces) a commit
action id and commit disclosure id. Every required generated or
mirrored artifact class (build output, generated source sibling,
structured lockfile, notebook output, preview snapshot — plus the
optional design-snapshot and mirrored doc / schema / model / registry
rows) is bound to one closed artifact kind, references a canonical
source ref, a generator or signer identity, and an output digest,
declares one closed drift state, one closed default edit posture,
and the surfaces it labels itself on across tree, breadcrumb, search,
review, AI-context, save, and support. Diverged-from-generator
artifacts cannot ship Stable without both an override disclosure id
and a recovery / regenerate guidance disclosure id. A destructive
override or regenerate never fires without the controlled inspection
/ repair hook table (`inspect_lineage`, `compare_canonical`,
`regenerate`, `export_before_override`, `rollback_override`,
`export`, `repair`) being reachable; a missing hook narrows the
record below Stable with a named reason. This packet is the
stable-line anchor for this lane; dashboards, docs, Help/About
surfaces, and support exports should ingest the typed sources below
rather than cloning this packet's text.

## Canonical machine sources

- Lineage projection and contract types:
  [`/crates/aureline-workspace/src/mutation_and_generated_artifact_lineage/`](../../../crates/aureline-workspace/src/mutation_and_generated_artifact_lineage/)
- Schema:
  [`/schemas/workspace/mutation_and_generated_artifact_lineage.schema.json`](../../../schemas/workspace/mutation_and_generated_artifact_lineage.schema.json)
- Headless emitter / CLI:
  [`/crates/aureline-workspace/src/bin/aureline_mutation_and_generated_artifact_lineage.rs`](../../../crates/aureline-workspace/src/bin/aureline_mutation_and_generated_artifact_lineage.rs)
- Fixtures:
  [`/fixtures/workspace/m4/mutation_and_generated_artifact_lineage/`](../../../fixtures/workspace/m4/mutation_and_generated_artifact_lineage/)
- Replay gate:
  [`/crates/aureline-workspace/tests/mutation_and_generated_artifact_lineage_replay.rs`](../../../crates/aureline-workspace/tests/mutation_and_generated_artifact_lineage_replay.rs)
- Companion contract doc:
  [`/docs/workspace/m4/ship-mutation-journal-and-generated-artifact-lineage-across.md`](../../../docs/workspace/m4/ship-mutation-journal-and-generated-artifact-lineage-across.md)
- Typed consumer:
  `aureline_workspace::project_mutation_and_generated_artifact_lineage`

## What this packet proves

1. **Mutation-path coverage truth.** Each record carries one row per
   governed mutation path declaring one closed `mutation_path_kind`.
   A corpus missing any of the six required paths (`editor`,
   `formatter`, `ai_apply`, `build_runner`, `lockfile_resolver`,
   `preview_runtime`) narrows the record with
   `required_mutation_path_missing`. Worked example:
   [`baseline_mutation_artifact_stable.json`](../../../fixtures/workspace/m4/mutation_and_generated_artifact_lineage/baseline_mutation_artifact_stable.json).

2. **Generated-artifact coverage truth.** Each record carries one
   row per governed artifact class declaring one closed
   `generated_artifact_kind`. The required set is `build_output`,
   `generated_source_sibling`, `structured_lockfile`,
   `notebook_output`, and `preview_snapshot`. Optional artifact
   classes (`design_snapshot`, `mirrored_doc_artifact`,
   `mirrored_schema_artifact`, `mirrored_model_artifact`,
   `mirrored_registry_artifact`) ride on top without changing the
   required set. Worked example:
   [`extended_with_mirrored_and_design_snapshot_stable.json`](../../../fixtures/workspace/m4/mutation_and_generated_artifact_lineage/extended_with_mirrored_and_design_snapshot_stable.json).

3. **Canonical-lineage truth.** Every artifact row references a
   non-empty canonical source ref, a generator or signer identity,
   and an output digest. Missing any of those narrows with
   `canonical_source_ref_missing`, `generator_identity_missing`, or
   `output_digest_missing`.

4. **Drift truth.** Every artifact row declares one closed
   `drift_state_class` (`in_sync`, `drifted_from_generator`,
   `regeneration_pending`, `unknown_drift`). An artifact in
   `drifted_from_generator` must reference a non-empty
   `recovery_guidance_disclosure_id`; missing it narrows with
   `drift_disclosure_missing`.

5. **Edit-posture honesty.** Every artifact row declares one closed
   `default_edit_posture_class` (`block_writes_default`,
   `round_trip_safe`, `diverged_from_generator`). Only artifact
   classes that explicitly support round-trip-safe editing
   (currently `structured_lockfile`) may declare `round_trip_safe`;
   any other class declaring it narrows with
   `edit_posture_unsafe_default`. Whenever the user overrides the
   default edit posture on a non-authoritative generated artifact,
   the row enters `diverged_from_generator`, which requires both an
   `override_disclosure_id` and a `recovery_guidance_disclosure_id`;
   missing either narrows with `diverged_disclosure_missing`. Worked
   example:
   [`diverged_from_generator_with_disclosures_stable.json`](../../../fixtures/workspace/m4/mutation_and_generated_artifact_lineage/diverged_from_generator_with_disclosures_stable.json).

6. **Labeling-surface coverage truth.** Each artifact row carries
   the labeling surfaces it discloses itself on. The required set is
   `tree`, `breadcrumb`, `search`, `review`, `ai_context`, `save`,
   and `support`. A governed artifact that omits any required
   surface narrows with `labeling_surface_missing` so the user, the
   review surfaces, and the AI never treat a generated artifact as
   the canonical edit target.

7. **Mutation no-rerun honesty.** Every mutation-path row declares
   one closed `mutation_no_rerun_posture` (`explicit_user_action_required`,
   `deterministic_replay_after_checkpoint`,
   `terminal_no_further_run`). Privileged mutation paths (`ai_apply`,
   `build_runner`, `lockfile_resolver`, `preview_runtime`) must
   declare `explicit_user_action_required` or
   `terminal_no_further_run` — never
   `deterministic_replay_after_checkpoint`. Worked example:
   [`ai_apply_deterministic_replay_narrowed.json`](../../../fixtures/workspace/m4/mutation_and_generated_artifact_lineage/ai_apply_deterministic_replay_narrowed.json)
   downgrades `ai_apply` to deterministic replay, surfacing
   `mutation_no_rerun_posture_unsafe`. Every
   `explicit_user_action_required` mutation path also references a
   commit action id and a commit disclosure id; missing metadata
   narrows with `explicit_action_metadata_missing`.

8. **Inspection precedes destructive override.** The controlled
   inspection / repair hook table must be available before any
   destructive override or regenerate commits. A missing hook
   narrows with `inspection_hook_unavailable`. Worked example:
   [`missing_compare_canonical_hook_narrowed.json`](../../../fixtures/workspace/m4/mutation_and_generated_artifact_lineage/missing_compare_canonical_hook_narrowed.json)
   demonstrates the narrow path when `compare_canonical` is
   unavailable.

9. **Support-export honesty.** Each row's support-export projection
   must preserve `path_or_class`, `canonical_source_ref`,
   `generator_identity`, `output_digest`, `drift_state`,
   `default_edit_posture`, `labeling_surfaces`, and `disclosure_ids`,
   and redact raw secrets, approval tickets, delegated credentials,
   and live authority handles. Dropping a field narrows with
   `support_export_fields_dropped`; raising raw material narrows with
   `support_export_redaction_unsafe`.

10. **Producer attribution is pinnable for replay.** Each record
    carries the producer ref, the schema version, the capture
    timestamp, and an integrity hash derived from the input
    identities so replay and support pipelines can pin the source
    before applying. Incomplete attribution narrows with
    `producer_attribution_incomplete`.

11. **Lineage and export stay honest.** Every record sets
    `raw_payload_excluded = true` and carries only opaque refs to the
    source workspace, corpus, and producer. An empty workspace or
    corpus ref narrows with `lineage_export_unsafe`.

12. **The record is replay-gated.** The replay gate re-projects each
    fixture and asserts it equals the checked-in `expected`, so the
    projection cannot drift without failing CI.

## Fixture corpus

| Fixture                                                | Workspace state covered                                                                  | Qualification           | Proves                                                                                                                       |
| ------------------------------------------------------ | ---------------------------------------------------------------------------------------- | ----------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| `baseline_mutation_artifact_stable`                    | Six required mutation paths + five required artifact classes, all in sync                | `stable`                | A baseline release-branch corpus can prove the full contract                                                                 |
| `extended_with_mirrored_and_design_snapshot_stable`    | Adds design-snapshot + mirrored doc / schema / model / registry artifact rows            | `stable`                | The optional generated / mirrored artifact classes ride safely on the same projection                                        |
| `diverged_from_generator_with_disclosures_stable`      | A generated source sibling overrides the default edit posture with both disclosures      | `stable`                | The `diverged_from_generator` state remains Stable when override + recovery disclosures are present                          |
| `ai_apply_deterministic_replay_narrowed`               | AI-apply mutation path downgraded to deterministic replay                                | `narrowed_below_stable` | The contract refuses to let a privileged mutation path skip explicit user action, surfacing `mutation_no_rerun_posture_unsafe` |
| `missing_compare_canonical_hook_narrowed`              | `compare_canonical` inspection hook unavailable                                          | `narrowed_below_stable` | The contract refuses to ship Stable when a required pre-action hook is missing                                               |

## How to verify

```sh
# Unit + replay gate for the mutation / generated-artifact lineage projection.
cargo test -p aureline-workspace --lib mutation_and_generated_artifact_lineage
cargo test -p aureline-workspace --test mutation_and_generated_artifact_lineage_replay

# Headless emitter (JSON or --lines projection).
cargo run -p aureline-workspace --bin aureline_mutation_and_generated_artifact_lineage -- --lines \
  fixtures/workspace/m4/mutation_and_generated_artifact_lineage/baseline_mutation_artifact_stable.json
```

## Stable-line registration

This lane's truth is the checked-in record, schema, fixtures, and
replay gate above. The lineage record self-describes its stable
qualification: postures that cannot prove the contract carry
`stable_qualification.level = narrowed_below_stable` with a named
reason, so they never inherit an adjacent green row. No public scope
is widened from this row.
