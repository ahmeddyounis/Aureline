# Protected user-journey trace taxonomy

This note is the normative companion to the machine-readable
boundary at
[`/schemas/traces/journey_trace.schema.json`](../../schemas/traces/journey_trace.schema.json),
the committed seeds under
[`/fixtures/journeys/`](../../fixtures/journeys/), and the harness
at [`/tools/journey_harness.sh`](../../tools/journey_harness.sh).

The journey-trace record measures **real end-to-end workflows** —
startup, shell open, placeholder file open / edit / save, and
restore-adjacent flows — not individual hot-path microbenchmarks.
It is the layer that later budget, trace-taxonomy, benchmark-report,
and support-packet lanes compose over. A harness emits one
`journey_trace_record` per journey per fixture.

## Why a separate record family

The spike-timing trace at
[`/schemas/traces/spike_timing.schema.json`](../../schemas/traces/spike_timing.schema.json)
carries individual hook firings and aggregate counters for one scene
of the shell spike. That is the right record family for "did the
`caret_move` hook fire and what was its latency"; it is the wrong
record family for "did the startup-to-first-useful-chrome journey
complete under this fixture with these degraded and fallback
postures". The journey-trace record carries the second question.

Two records of the two families **share vocabulary**:
`protected_journey` tokens, hook names, and fixture ids all resolve
against the same closed sets. A journey trace that cites
`spike_hook_ref: warm_start_to_first_paint` on a checkpoint is
naming the same hook the spike trace emits; two surfaces cannot
rename each other's concepts.

## Record shape

Every journey-trace record pins:

- `schema = aureline.journey_trace.v1` and `record_kind =
  journey_trace_record` so parsers can route by either field;
- one `trace_id`, one `journey_id`, one `journey_class` from the
  closed set in the schema;
- a non-empty `protected_journeys[]` whose tokens are drawn from
  the same vocabulary as
  [`spike_metric_names.md`](./spike_metric_names.md) §Protected-path
  vocabulary and
  [`fixture_classes.md`](./fixture_classes.md) §Protected journeys;
- one `fixture_ref` that MUST resolve against the corpus manifest
  at the named `corpus_manifest.manifest_revision`;
- a minimum `build` record (crate name, crate version, rustc target
  triple) that upgrades to the full exact-build-identity record via
  the reserved `exact_build_identity_ref` slot;
- reserved nullable slots for `hardware_definition_ref`,
  `environment_ref`, and `exact_build_identity_ref` so reference
  captures wire in later without a schema-version bump;
- one `degraded_posture` and one `fallback_posture` drawn from the
  closed product-visible posture sets;
- an ordered `checkpoints[]` beginning with `journey_start` and
  ending with `journey_end`;
- an ordered `segments[]` whose `from_checkpoint_id` and
  `to_checkpoint_id` fields both resolve into `checkpoints[]`;
- a `counters` record with `checkpoint_count`, `segment_count`,
  `degraded_segment_count`, `fallback_segment_count`, and
  `linked_spike_trace_count`;
- reserved `linked_spike_trace_refs[]`, `evidence_refs[]`, and
  `requirement_refs[]` arrays so later lanes can attach without a
  schema-version bump.

## Closed vocabularies

| Vocabulary               | Values                                                                                                                                                                                                                             |
|--------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `journey_class`          | `startup_to_first_useful_chrome`, `startup_to_first_paint`, `shell_open`, `placeholder_open`, `placeholder_edit`, `placeholder_save`, `open_edit_save`, `restore_adjacent`, `recovery_journal_restore_flow`, `boundary_truth_contract_replay` |
| `checkpoint_class`       | `journey_start`, `journey_end`, `protected_path_event`, `degraded_transition`, `fallback_transition`, `provisional_segment_boundary`                                                                                                 |
| `segment_class`          | (all `protected_journey_class` values) plus `provisional_segment`                                                                                                                                                                   |
| `degraded_posture_class` | `healthy`, `reduced_chrome_only`, `degraded_renderer_banner_visible`, `responsive_fallback_active`, `missing_target_recovered_to_layout_only`, `missing_target_recovered_to_compatible`                                             |
| `fallback_posture_class` | `none`, `glyph_fallback_active`, `atlas_shard_rebind`, `atlas_eviction_observed`, `software_renderer_active`, `recovery_journal_replay_active`                                                                                      |
| `backend_class`          | `headless`, `native_window`, `synthesised`                                                                                                                                                                                          |

Adding a value to any of these sets is additive-minor and requires a
`schema_version` bump. Repurposing an existing value is breaking and
requires a new decision row.

`provisional_segment` and `provisional_segment_boundary` are
deliberately reserved so a harness MAY stamp a span or boundary the
later trace-taxonomy lane will rename **without** inventing a
private vocabulary in the meantime.

## Seeded journeys

The three seeded journeys in
[`/fixtures/journeys/`](../../fixtures/journeys/) cover the three
M0 harness obligations named in the plan:

1. **`journey.startup_to_first_useful_chrome.micro_local_folder`** —
   `journey_class: startup_to_first_useful_chrome` against
   `corpus.reference.micro_local_folder`, four checkpoints
   (`warm_start`, `first_paint`, `first_useful_chrome`,
   `render_submit`) and three segments covering startup,
   first-paint, and render-submission. Linked to the existing
   shell-spike aggregate trace `shell_spike.fixture_v1.full_scene`.
2. **`journey.open_edit_save.first_useful_edit_rust_self_host`** —
   `journey_class: open_edit_save` against
   `corpus.workflow.first_useful_edit_rust_self_host`, six
   checkpoints covering placeholder open, edit, save begin,
   recovery-journal write, and save complete, with four segments
   for the open / edit / save / save-pipeline spans. Backed by
   `backend: synthesised` because the M0 shell spike does not yet
   drive the save pipeline end to end.
3. **`journey.restore_adjacent.restore_last_session_compatible`** —
   `journey_class: restore_adjacent` against
   `corpus.recovery.restore_last_session_compatible`, five
   checkpoints that include one `degraded_transition`
   (missing-target observed) and one `fallback_transition`
   (recovery-journal replay), plus three segments covering the
   restore journey, the boundary-truth contract verification, and
   the compatible-restore finalisation.
   `degraded_posture: missing_target_recovered_to_compatible` and
   `fallback_posture: recovery_journal_replay_active` so a trace
   consumer can read the degradation without re-deriving it from
   segment notes.

Together these cover the plan's "startup-to-first-useful-chrome"
and "open-edit-save or restore-adjacent" acceptance cases — both
acceptance bars clear on the seeded set. The harness is the single
entry point a later lane extends with additional journey classes;
the schema's reserved slots let those additions land without a
schema-version bump.

## Reproducibility posture

The harness is stdlib-only Python driven through a Bash wrapper
that pins `SOURCE_DATE_EPOCH`, `TZ=UTC`, and `LC_ALL=C` the same
way [`/tools/benchmark_lab.sh`](../../tools/benchmark_lab.sh)
does. Ticks are synthetic monotonic counters (`0..N`) so the
committed seeds stay byte-stable on any host.
`./tools/journey_harness.sh --verify-seed` re-emits every seeded
journey under a tempdir and unified-diffs against the committed
files; seed drift fails the check before any later lane admits the
record.

## Later work reserved, not closed

- **Reference capture** — `hardware_definition_ref` and
  `environment_ref` are nullable today. The benchmark-council-
  approved hardware baseline, once promoted, rides in those fields
  without renaming anything.
- **Exact-build identity** — the minimum `build` record is always
  present; the full record (see
  [`/schemas/build/exact_build_identity.schema.json`](../../schemas/build/exact_build_identity.schema.json))
  rides in the reserved `exact_build_identity_ref` slot when a lane
  has one.
- **Evidence linkage** — `evidence_refs[]` is opaque today; later
  release-evidence, support-bundle, and boundary-truth lanes attach
  through stable channel ids already admissible on the corpus
  manifest's `evidence_consumer_channels` set.
- **Requirement linkage** — `requirement_refs[]` is opaque today
  and empty on every seeded journey; later PRD / ADR / TAD
  requirement-linkage work attaches without a schema bump.
- **Live capture** — the M0 harness emits synthesised journeys
  (backed by `shell_spike.fixture_v1.full_scene` for the startup
  journey). A later lane that drives the shell spike end-to-end and
  reads its spike-timing output fills in `linked_spike_trace_refs`
  and the `spike_hook_ref` on each checkpoint without changing the
  record shape.

## See also

- [`spike_metric_names.md`](./spike_metric_names.md) — hook → protected-path mapping.
- [`fixture_classes.md`](./fixture_classes.md) — corpus classes and protected-journey tags.
- [`fitness_function_catalog.md`](./fitness_function_catalog.md) — fitness rows journey traces later feed.
- [`benchmark_lab_run_results.md`](./benchmark_lab_run_results.md) — benchmark-lab run-result schema the journey harness composes against.
