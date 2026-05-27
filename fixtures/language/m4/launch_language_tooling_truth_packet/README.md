# launch_language_tooling_truth_packet fixture corpus

Fixture corpus for the M4 stable launch-language tooling truth packet
(`schemas/language/launch_language_tooling_truth.schema.json`).

Each fixture is a `LaunchLanguageToolingTruthPacketInput` with an
`expect` block that pins the materialized packet's promotion state,
finding count, lane and row-class token sets, support-class,
daily-loop step, known-limit, downgrade-automation, and
evidence-class tokens, and the support-export safety verdict. Tests
in
`crates/aureline-language/tests/launch_language_tooling_truth_packet.rs`
load each case and assert that
`LaunchLanguageToolingTruthPacket::materialize` agrees.

Cases:

- `baseline_stable.json` — The shell/bash launch-tooling lane carries
  a `launch_tooling_quality` row at `launch_support` plus every
  certified `daily_loop_step` row (open_or_import, navigate, edit,
  complete, refactor, run_test_debug, review, migrate, recover). The
  SQL, Markdown, JSON/YAML, and Git-oriented lanes publish
  `launch_tooling_quality` rows that disclose their
  narrowed-below-launch posture with `launch_tooling_scope_only` and
  `language_subset_only` known limits. Every row binds support, known
  limit, downgrade automation, and evidence classes; narrowed rows
  carry their disclosure refs, and all eight required consumer
  projections preserve the packet verbatim.
- `launch_support_with_unbound_evidence_blocks_stable.json` — The
  shell/bash launch_tooling_quality row claims `launch_support` while
  its evidence class is `evidence_unbound`; the packet blocks the
  stable claim because no archetype, fixture-repo, framework-migration,
  or design-partner evidence backs the row.
- `missing_daily_loop_step_for_launch_support_blocks_stable.json` —
  The shell/bash lane claims `launch_support` but the `recover`
  daily-loop step is missing; the packet blocks the stable claim.
- `narrowed_row_missing_disclosure_ref_blocks_stable.json` — The SQL
  launch_tooling_quality row narrows to `launch_support_below` with
  `language_subset_only` but drops its disclosure ref; the packet
  blocks the stable claim.
- `projection_collapses_evidence_class_vocabulary_blocks_stable.json`
  — The `help_about` consumer projection drops the evidence-class
  vocabulary; the packet blocks the stable claim.
- `raw_source_material_blocks_stable.json` — The shell/bash
  launch_tooling_quality row admits raw source bodies past the
  boundary; the packet blocks the stable claim because raw shell
  scripts, SQL queries, Markdown bodies, JSON/YAML payloads, Git
  commit messages, and ambient `SHELL` / `PATH` / `HISTFILE` values
  must never leak through the launch-tooling boundary.
