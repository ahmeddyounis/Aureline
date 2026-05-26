# Stabilize the Node and Express Advanced workflow pack with run/test/debug and launch-profile parity — stable contract

Status: Stable lane proof for the Node and Express Advanced workflow pack.

This document is the reviewer-facing contract for the stable Node and
Express Advanced workflow pack truth packet. The packet is the single
source of truth that the editor framework pack panel, workflow
companion, framework settings/help, CLI/headless inspector, support
export, release proof index, Help/About proof card, and the
conformance dashboard all read; surfaces MUST NOT mint local copies or
paraphrase workflow-pack posture.

The packet pins the Node and Express Advanced workflow pack across
three intertwined truths:

1. The **app workflow loops** — create, open, run, test, debug,
   rename, and review on Node + Express archetype repos (Express 4
   CommonJS app and Express 5 ESM app archetypes).
2. The **server project model** — every row that crosses the
   Node/Express server structure (CommonJS `require`/`module.exports`
   vs ESM `import`/`export` module system; Express `app`, router,
   middleware, and controller boundary; `package.json` and
   `package-lock.json` dependency closure) binds a dedicated
   `server_project_model_row` and a disclosure ref so the workflow
   pack never confuses one project model for another.
3. The **launch-profile parity** — dev / start / debug / test launch
   profiles (npm/pnpm/yarn `dev`/`start`/`test`/`debug` scripts,
   nodemon hot-reload, ts-node-dev, `node --inspect` /
   `--inspect-brk`, Jest / Mocha / Vitest runners, and pm2-style
   process supervisors) are bound by a dedicated
   `launch_profile_parity_row` so that run/test/debug steps that
   depend on a specific launch profile surface the gap rather than
   over-claim.

## What the packet asserts

For each governed *workflow pack × workflow-pack row* the packet
asserts:

1. The **workflow pack class** — currently
   `node_and_express_advanced_workflow_pack`. Every certified packet
   MUST carry at least one row for each required pack.
2. The **workflow-pack row class** — one of `pack_qualification`,
   `workflow_loop`, `framework_migration_row`, `archetype_repo_row`,
   `server_project_model_row`, `launch_profile_parity_row`,
   `design_partner_row`, `unsupported_gap`, `known_limit`, or
   `downgrade_automation`. A `workflow_loop` row MUST bind a real
   workflow-loop step; no other row class is permitted to bind one.
3. The **support class** — one of `expert_grade`,
   `stable_below_expert`, `beta_grade_only`, `preview_only`,
   `unsupported`, or `support_unbound`. The validator refuses to
   certify a row that claims `expert_grade` while any binding is
   unbound (support, known limit, downgrade automation, or evidence).
4. The **workflow-loop class** — one of `create`, `open`, `run`,
   `test`, `debug`, `rename`, `review`, or `not_applicable`. A pack
   that claims `expert_grade` workflow-pack support MUST cover every
   certified workflow-loop step.
5. The **evidence class** — one of `archetype_repo_evidence`,
   `framework_migration_evidence`, `design_partner_evidence`,
   `fixture_repo_evidence`, `conformance_suite_evidence`,
   `benchmark_evidence`, `server_project_model_evidence`,
   `launch_profile_parity_evidence`, `docs_disclosure_evidence`, or
   `evidence_unbound`. A row whose evidence class is `evidence_unbound`
   is refused.
6. The **known-limit class** — one of `none_declared`,
   `framework_subset_only`, `language_subset_only`,
   `archetype_subset_only`, `migration_subset_only`,
   `server_project_model_subset_only`,
   `launch_profile_parity_subset_only`,
   `unsupported_runtime_target`, `beta_capability_sample_only`, or
   `limit_unbound`. A row whose known limit is `limit_unbound` is
   refused.
7. The **downgrade-automation class** — one of `none`,
   `auto_narrow_on_missing_fixture`,
   `auto_narrow_on_missing_archetype`,
   `auto_narrow_on_failed_migration`, `auto_narrow_on_framework_gap`,
   `auto_narrow_on_unproven_server_project_model`,
   `auto_narrow_on_unproven_launch_profile_parity`,
   `auto_demote_on_low_confidence`, `auto_block_on_missing_evidence`,
   `manual_only_pending_review`, or `automation_unbound`. A row whose
   automation is `automation_unbound` is refused.
8. The **workflow-pack confidence class** — `high_confidence`,
   `medium_confidence`, or `low_confidence`. A row that claims
   `expert_grade` at `low_confidence` is narrowed below stable until
   evidence grows.
9. The **evidence refs** — every row preserves at least one
   repo-relative evidence ref proving the workflow-pack claim.
10. The **disclosure ref** — every row that is not `expert_grade`, that
    declares a non-`none_declared` known limit, or that binds a
    non-`none` downgrade automation MUST carry a repo-relative
    disclosure ref shown to the user.

## Boundary safety

Every row carries `raw_source_material_excluded`, `secrets_excluded`,
and `ambient_authority_excluded`. The validator emits
`raw_source_material_present`, `secrets_present`, or
`ambient_authority_present` as a blocker for any row that flips one of
those booleans to false. The packet never admits raw server source
bodies, environment variable values, `.env` secrets, ambient
credentials, npm tokens, or provider payloads. Launch-profile parity
rows bind only the *surface* (dev/start/debug/test launch profile
shapes, `node --inspect` / nodemon / ts-node-dev hot-reload entry
points) — never the secret values themselves.

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
- a row narrowed below `expert_grade` drops its disclosure ref,
- a row declares a non-`none_declared` known limit and drops its
  disclosure ref,
- a row binds a non-`none` downgrade automation and drops its
  disclosure ref,
- any of the eight required consumer projections is missing or
  collapses one of the closed vocabularies (pack, row class, support
  class, workflow loop, known limit, downgrade automation, or evidence
  class),
- raw source bodies, secrets, or ambient credentials slip past the
  boundary,
- the stored promotion state disagrees with the derived findings.

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

## How to read the packet

Consumers materialize the packet through
`NodeAndExpressAdvancedWorkflowPackTruthPacket::materialize` and then
read the projection that matches their surface. The packet is
metadata-only and suitable for inclusion in any support export or
release proof bundle.

## Where the packet lives

- Schema: [`schemas/language/node_and_express_advanced_workflow_pack_truth.schema.json`](../../../schemas/language/node_and_express_advanced_workflow_pack_truth.schema.json)
- Reviewer artifact: [`artifacts/language/m4/stabilize-the-node-and-express-advanced-workflow-pack.md`](../../../artifacts/language/m4/stabilize-the-node-and-express-advanced-workflow-pack.md)
- Checked-in packet: [`artifacts/language/m4/node_and_express_advanced_workflow_pack_truth_packet.json`](../../../artifacts/language/m4/node_and_express_advanced_workflow_pack_truth_packet.json)
- Fixture corpus: [`fixtures/language/m4/node_and_express_advanced_workflow_pack_truth_packet/`](../../../fixtures/language/m4/node_and_express_advanced_workflow_pack_truth_packet/)
- Rust module: [`crates/aureline-language/src/node_and_express_advanced_workflow_pack_truth_packet/mod.rs`](../../../crates/aureline-language/src/node_and_express_advanced_workflow_pack_truth_packet/mod.rs)
- Replay tests: [`crates/aureline-language/tests/node_and_express_advanced_workflow_pack_truth_packet.rs`](../../../crates/aureline-language/tests/node_and_express_advanced_workflow_pack_truth_packet.rs)
