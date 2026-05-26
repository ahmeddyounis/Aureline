# Node and Express Advanced workflow pack truth packet — reviewer artifact

This is the reviewer-facing artifact for the M4 stable Node and
Express Advanced workflow pack truth packet covering the create,
open, run, test, debug, rename, and review loops with expert-grade
support, workflow loop coverage, Node/Express server project-model
evidence (CommonJS / ESM module system; Express
router/middleware/controller boundary), launch-profile parity
evidence (dev / start / debug / test launch profiles plus
`node --inspect`, nodemon, and ts-node-dev hot-reload surfaces),
framework-migration evidence, known limits, downgrade automation, and
evidence binding. The contract lives at
`docs/languages/m4/stabilize-the-node-and-express-advanced-workflow-pack.md`
and is replayed by
`crates/aureline-language/tests/node_and_express_advanced_workflow_pack_truth_packet.rs`.

## Stable claim

For the governed workflow pack class
(`node_and_express_advanced_workflow_pack`) the packet binds:

- at least one `pack_qualification` row (the pack's headline
  workflow-pack qualification),
- a `workflow_loop` row per certified step (create, open, run, test,
  debug, rename, review) when the pack claims `expert_grade`,
- at least one `framework_migration_row` certifying the Express 4
  CommonJS → Express 5 ESM migration archetype,
- at least one `server_project_model_row` certifying the
  Node/Express server project structure: CommonJS vs ESM module
  system; Express `app`, router, middleware, and controller
  boundary; `package.json` and `package-lock.json` dependency closure,
- at least one `launch_profile_parity_row` certifying the dev /
  start / debug / test launch profiles (npm/pnpm/yarn script entry
  points plus nodemon, ts-node-dev, `node --inspect` /
  `--inspect-brk`, Jest / Mocha / Vitest, and pm2-style supervisor
  surfaces),
- a closed `support_class` (no surface pretends `expert_grade` while a
  binding is unbound),
- a closed `workflow_loop_class` (every expert-grade pack covers the
  full workflow loop; non-loop rows bind `not_applicable`),
- a closed `evidence_class` (archetype-repo, framework-migration,
  design-partner, fixture-repo, conformance-suite, benchmark,
  server-project-model, launch-profile parity, or docs-disclosure),
- a closed `known_limit_class` (framework subset, language subset,
  archetype subset, migration subset, server-project-model subset,
  launch-profile parity subset, unsupported runtime target, beta
  capability sample, or `none_declared`),
- a closed `downgrade_automation_class` (auto-narrow on missing
  fixture/archetype, auto-narrow on failed migration / framework gap /
  unproven server project model / unproven launch-profile parity,
  auto-demote on low confidence, auto-block on missing evidence,
  manual-only, or `none`),
- a closed `workflow_pack_confidence_class`, and
- at least one `evidence_refs` entry plus a `disclosure_ref` whenever
  the row is not `expert_grade`, declares a non-`none_declared` known
  limit, or binds a non-`none` downgrade automation.

## Companion artifacts

- Schema: `schemas/language/node_and_express_advanced_workflow_pack_truth.schema.json`
- Checked-in packet:
  `artifacts/language/m4/node_and_express_advanced_workflow_pack_truth_packet.json`
- Fixture corpus:
  `fixtures/language/m4/node_and_express_advanced_workflow_pack_truth_packet/`
- Rust contract:
  `crates/aureline-language/src/node_and_express_advanced_workflow_pack_truth_packet/mod.rs`
- Replay tests:
  `crates/aureline-language/tests/node_and_express_advanced_workflow_pack_truth_packet.rs`
- Reviewer doc:
  `docs/languages/m4/stabilize-the-node-and-express-advanced-workflow-pack.md`

## Required consumer projections

The packet is preserved verbatim across eight consumer projections:

| Projection                    | Surface                              |
| ----------------------------- | ------------------------------------ |
| `editor_framework_pack_panel` | Editor framework pack panel          |
| `workflow_companion`          | Workflow companion / runner panel    |
| `framework_settings`          | Framework settings / help surface    |
| `cli_headless`                | CLI/headless inspector               |
| `support_export`              | Support export bundle                |
| `release_proof_index`         | Release proof index entry            |
| `help_about`                  | Help/About proof card                |
| `conformance_dashboard`       | Conformance dashboard row            |

A projection that collapses any closed vocabulary, drops the packet
id, drops the pack class, row class, support class, workflow-loop,
known-limit, downgrade-automation, or evidence-class vocabulary, or
leaks raw private material immediately blocks the stable claim.

## What blocks the stable claim

The packet blocks publication when any of the following appears:

- a row claims `expert_grade` while its support, known-limit,
  downgrade-automation, or evidence class is unbound,
- a pack that claims `expert_grade` workflow-pack support is missing a
  certified `workflow_loop` row for any of the seven required steps
  (create, open, run, test, debug, rename, review),
- a `workflow_loop` row drops its workflow-loop step binding,
- a non-`workflow_loop` row binds a workflow-loop step it cannot
  certify,
- a row narrowed below `expert_grade` or with a non-default known
  limit / non-`none` downgrade automation drops its disclosure ref,
- any of the eight required consumer projections is missing or
  collapses one of the closed vocabularies,
- raw source bodies, secrets, or ambient credentials slip past the
  boundary,
- the stored promotion state disagrees with the derived findings.

## How to read the packet

Consumers materialize the packet through
`NodeAndExpressAdvancedWorkflowPackTruthPacket::materialize` and then
read the projection that matches their surface. The packet is
metadata-only and suitable for inclusion in any support export or
release proof bundle.

## Where the packet lives

- Schema: [`schemas/language/node_and_express_advanced_workflow_pack_truth.schema.json`](../../../schemas/language/node_and_express_advanced_workflow_pack_truth.schema.json)
- Reviewer doc: [`docs/languages/m4/stabilize-the-node-and-express-advanced-workflow-pack.md`](../../../docs/languages/m4/stabilize-the-node-and-express-advanced-workflow-pack.md)
- Fixture corpus: [`fixtures/language/m4/node_and_express_advanced_workflow_pack_truth_packet/`](../../../fixtures/language/m4/node_and_express_advanced_workflow_pack_truth_packet/)
- Rust module: [`crates/aureline-language/src/node_and_express_advanced_workflow_pack_truth_packet/mod.rs`](../../../crates/aureline-language/src/node_and_express_advanced_workflow_pack_truth_packet/mod.rs)
