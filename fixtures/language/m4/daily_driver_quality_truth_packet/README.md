# daily_driver_quality_truth_packet fixture corpus

Fixture corpus for the M4 stable TypeScript, JavaScript, HTML, and CSS
daily-driver quality truth packet
(`schemas/language/daily_driver_quality_truth.schema.json`).

Each fixture is a `DailyDriverQualityTruthPacketInput` with an `expect`
block that pins the materialized packet's promotion state, finding
count, lane and row-class token sets, support-class, daily-loop-step,
known-limit, downgrade-automation, and evidence-class tokens, and the
support-export safety verdict. Tests in
`crates/aureline-language/tests/daily_driver_quality_truth_packet.rs`
load each case and assert that
`DailyDriverQualityTruthPacket::materialize` agrees.

Cases:

- `baseline_stable.json` — Every governed language lane carries a
  daily-driver quality row plus every certified daily-loop step row,
  rows bind their support, known limit, downgrade automation, and
  evidence classes; narrowed rows carry their disclosure refs, and all
  eight required consumer projections preserve the packet verbatim.
- `replacement_grade_with_unbound_evidence_blocks_stable.json` — A
  TypeScript daily-driver quality row claims `replacement_grade` while
  its evidence class is `evidence_unbound`; the packet blocks the
  stable claim because no fixture-repo, migration, or archetype
  evidence backs the row.
- `missing_daily_loop_step_for_replacement_grade_blocks_stable.json` —
  A JavaScript lane claims `replacement_grade` daily-driver quality
  but the `recover` daily-loop step row is missing; the packet blocks
  the stable claim.
- `narrowed_row_missing_disclosure_ref_blocks_stable.json` — A CSS
  known-limit row narrows below replacement grade but drops its
  disclosure ref; the packet blocks the stable claim.
- `projection_collapses_evidence_class_blocks_stable.json` — The
  `conformance_dashboard` consumer projection drops the
  evidence-class vocabulary; the packet blocks the stable claim.
- `raw_source_material_blocks_stable.json` — A daily-driver quality
  row admits raw source bodies past the boundary; the packet blocks
  the stable claim because raw material must never leak through the
  daily-driver quality boundary.
