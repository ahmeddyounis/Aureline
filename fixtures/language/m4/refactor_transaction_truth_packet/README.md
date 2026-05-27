# refactor_transaction_truth_packet fixture corpus

Fixture corpus for the M4 stable refactor transaction truth packet
(`schemas/language/refactor_transaction_truth.schema.json`).

Each fixture is a `RefactorTransactionTruthPacketInput` with an `expect`
block that pins the materialized packet's promotion state, finding
count, lane and row-class token sets, support-class, transaction-phase,
preview-completeness, validation-outcome, rollback-path,
launch-language, known-limit, downgrade-automation, and evidence-class
tokens, and the support-export safety verdict. Tests in
`crates/aureline-language/tests/refactor_transaction_truth_packet.rs`
load each case and assert that
`RefactorTransactionTruthPacket::materialize` agrees.

Cases:

- `baseline_stable.json` — Every refactor-class lane (rename, extract
  function, inline symbol, move symbol, update imports, cross-file
  signature change) carries a `refactor_transaction_quality` row at
  `launch_stable` plus all four `transaction_phase_truth` rows for
  `preview`, `validate`, `apply`, and `rollback`. Each lane also
  surfaces a `preview_outcome_admission` row with bound
  `preview_completeness_class`, a `validation_hook_admission` row with
  bound `validation_outcome_class`, a `rollback_drill_admission` row
  with bound `rollback_path_class`, and a `launch_language_coverage`
  row binding the launch language under proof. Every row binds
  support, evidence, known-limit, downgrade-automation,
  preview-completeness, validation-outcome, and rollback-path classes;
  narrowed rows carry their disclosure refs; and all eight required
  consumer projections preserve the packet verbatim.
- `launch_stable_with_unbound_evidence_blocks_stable.json` — The
  rename-symbol-lane `refactor_transaction_quality` row claims
  `launch_stable` while its evidence class is `evidence_unbound`; the
  packet blocks the stable claim.
- `missing_transaction_phase_for_launch_stable_blocks_stable.json` —
  The rename-symbol lane claims `launch_stable` but the `rollback`
  `transaction_phase_truth` row is missing; the packet blocks the
  stable claim because every launch-stable lane MUST cover preview,
  validate, apply, and rollback.
- `narrowed_row_missing_disclosure_ref_blocks_stable.json` — The
  rename-symbol-lane `refactor_transaction_quality` row narrows to
  `launch_stable_below` but drops its disclosure ref; the packet
  blocks the stable claim.
- `projection_collapses_rollback_path_vocabulary_blocks_stable.json` —
  The `help_about` consumer projection drops the rollback-path
  vocabulary; the packet blocks the stable claim because surfaces
  MUST preserve the closed rollback-path vocabulary that distinguishes
  exact undo, compensating revert, grouped mutation journal revert,
  regenerate-first replay, manual-review-only, and no-safe-rollback.
- `raw_source_material_blocks_stable.json` — The rename-symbol-lane
  `refactor_transaction_quality` row admits raw source bodies past
  the boundary; the packet blocks the stable claim because raw
  refactor diffs, source bodies, generated artifact bodies, secrets,
  and ambient credentials must never leak through the refactor-
  transaction boundary.
