# node_and_express_advanced_workflow_pack_truth_packet fixture corpus

Fixture corpus for the M4 stable Node and Express Advanced workflow
pack truth packet
(`schemas/language/node_and_express_advanced_workflow_pack_truth.schema.json`).

Each fixture is a `NodeAndExpressAdvancedWorkflowPackTruthPacketInput`
with an `expect` block that pins the materialized packet's promotion
state, finding count, pack and row-class token sets, support-class,
workflow-loop, known-limit, downgrade-automation, and evidence-class
tokens, and the support-export safety verdict. Tests in
`crates/aureline-language/tests/node_and_express_advanced_workflow_pack_truth_packet.rs`
load each case and assert that
`NodeAndExpressAdvancedWorkflowPackTruthPacket::materialize` agrees.

Cases:

- `baseline_stable.json` — The Node and Express Advanced workflow
  pack carries a pack-qualification row at `expert_grade` plus every
  certified workflow-loop row (create, open, run, test, debug,
  rename, review), a framework-migration row certifying the Express
  4 CommonJS → Express 5 ESM migration, a server-project-model row
  certifying CommonJS vs ESM and the Express
  `app`/router/middleware/controller boundary, and a launch-profile
  parity row certifying the dev / start / debug / test launch
  profiles (npm/pnpm/yarn scripts, nodemon, ts-node-dev, `node
  --inspect`, Jest / Mocha / Vitest, and pm2-style supervisors).
  Every row binds support, known limit, downgrade automation, and
  evidence classes; narrowed rows carry their disclosure refs, and
  all eight required consumer projections preserve the packet
  verbatim.
- `expert_grade_with_unbound_evidence_blocks_stable.json` — The pack
  qualification row claims `expert_grade` while its evidence class is
  `evidence_unbound`; the packet blocks the stable claim because no
  fixture-repo, migration, archetype, server-project-model, or
  launch-profile parity evidence backs the row.
- `missing_workflow_loop_for_expert_grade_blocks_stable.json` — The
  pack claims `expert_grade` but the `rename` workflow-loop row is
  missing; the packet blocks the stable claim.
- `narrowed_row_missing_disclosure_ref_blocks_stable.json` — A
  known-limit row narrows below expert grade with
  `launch_profile_parity_subset_only` but drops its disclosure ref;
  the packet blocks the stable claim.
- `projection_collapses_workflow_loop_vocabulary_blocks_stable.json`
  — The `conformance_dashboard` consumer projection drops the
  workflow-loop vocabulary; the packet blocks the stable claim.
- `raw_source_material_blocks_stable.json` — A workflow-pack row
  admits raw source bodies past the boundary; the packet blocks the
  stable claim because raw material must never leak through the
  workflow-pack boundary.
