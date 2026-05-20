# Scaffold and generated-project safety conformance corpus

This corpus is the failure / recovery drill harness for the M3 scaffold and
generated-project safety beta projection (`ScaffoldSafetyBetaProjection` over
the `TemplateGeneratorDescriptor`, `ScaffoldPlanRecord`, and
`ScaffoldRunRecord` boundary records owned by
`aureline-workspace::scaffold`).

It converts the scaffold / generation UX promise into a regression-gated
proof system: each drill binds one signed template / generator descriptor,
one scaffold plan, and zero-or-one scaffold run, then pins the projected
scaffold-safety truth a claimed beta creation row must reproduce — provider
and signature identity, generation kind and verb, declared
hook / network / registry / remote-image / dependency side effects, the
create-empty / set-up-later handoffs and the rollback boundary, the honesty
labels a surface renders, the seven typed guardrails, the disclosure verdict,
and the reconstructable generated-project lineage.

Every drill is loaded by the conformance harness at
[`crates/aureline-qe/src/scaffold_safety/`](../../../../crates/aureline-qe/src/scaffold_safety/)
and replayed by
`cargo test -p aureline-qe --test scaffold_safety_conformance`.

## Single source of truth

`manifest.json` is authoritative. Positive drills MUST parse, project, and
match **every** `expected_*` field in the manifest. Negative drills MUST FAIL
projection with an error whose message contains `expected_failure_substring`.
The fixtures carry only the scenario records and a `__fixture__` prelude —
they do **not** restate the expectations, so there is exactly one place to
read and audit the pinned truth.

Boundary schemas:

- [`/schemas/workspace/template_generator_descriptor.schema.json`](../../../../schemas/workspace/template_generator_descriptor.schema.json)
- [`/schemas/workspace/scaffold_plan.schema.json`](../../../../schemas/workspace/scaffold_plan.schema.json)
- [`/schemas/workspace/scaffold_run.schema.json`](../../../../schemas/workspace/scaffold_run.schema.json)
- [`/schemas/workspace/scaffold_safety.schema.json`](../../../../schemas/workspace/scaffold_safety.schema.json)

Beta contract: [`docs/workspace/m3/scaffold_safety_beta.md`](../../../../docs/workspace/m3/scaffold_safety_beta.md).
Reviewer guidance: [`docs/workspace/m3/scaffold_safety_beta_audit.md`](../../../../docs/workspace/m3/scaffold_safety_beta_audit.md).
Conformance artifact: [`artifacts/workspace/m3/scaffold_safety_report.md`](../../../../artifacts/workspace/m3/scaffold_safety_report.md).
Lineage audit: [`artifacts/workspace/m3/generated_project_lineage_audit.md`](../../../../artifacts/workspace/m3/generated_project_lineage_audit.md).

## Coverage axes

| Axis | Drill ids |
| --- | --- |
| First-party signed template — create | `template.first_party_signed_create` |
| Extension-provided starter — governed | `template.extension_starter_governed` |
| AI-assisted suggestion — governed | `template.ai_assisted_governed` |
| Imported scaffold pack — signed offline bundle | `import.scaffold_pack_offline_signed` |
| Mirrored template — signed, registry restore | `mirror.template_signed_registry` |
| Policy-blocked generator — allowlist / fleet-pinned | `policy.generator_blocked_allowlist` |
| Create-empty parity handoff | `handoff.create_empty_parity` |
| Generated-diff review (generate-into-existing) | `diff_review.generate_into_existing` |
| Set-up-later deferred handoff | `handoff.setup_later_deferred` |
| Remote-image / devcontainer bootstrap | `remote_image.devcontainer_pull` |
| Update / regenerate managed zone — partial | `update.regenerate_managed_zone_partial` |
| Support export — run lineage | `support.export_run_lineage` |
| Failure — partial generation left in place | `failure.partial_generation_left_in_place` |
| Failure — missing toolchain, rolled back | `failure.missing_toolchain_rolled_back` |
| Failure — mirror outage, cancelled | `failure.mirror_outage_cancelled` |
| Failure — remote image unavailable, rolled back | `failure.remote_image_unavailable_rolled_back` |
| Caught — writes before review | `caught.writes_before_review` |
| Caught — hidden side effect | `caught.hidden_side_effect` |
| Caught — hidden project database | `caught.hidden_project_database` |
| Negative — run invokes an undeclared hook | `negative.run_invokes_undeclared_hook` |
| Negative — run invokes an undeclared task | `negative.run_invokes_undeclared_task` |
| Negative — plan binds a sibling descriptor | `negative.plan_binds_sibling_descriptor` |
| Negative — run binds a sibling plan | `negative.run_binds_sibling_plan` |
| Negative — plan plans an undeclared "declared" task | `negative.plan_plans_undeclared_task` |

## Transverse invariants

The conformance suite also pins, across the whole positive set:

- every distinct generation verb (`create_project`,
  `generate_into_existing`, `update_regenerate`) keeps a drill;
- every claimed provider class (`first_party`, `partner_signed`,
  `community_signed`, `extension_provided`, `ai_assisted`) keeps a drill;
- mirrored / offline / extension / AI distributions are all present, and the
  `mirror` and `offline_bundle` rows keep their `signed_verified` signature
  instead of flattening into generic local files;
- create-empty and set-up-later handoffs are both offered by at least one
  drill;
- writes-before-review, an undeclared (hidden) side effect, and a hidden
  project database are each caught as a **failing** guardrail (the individual
  guardrail predicate flips to `false`), not a tolerated detail;
- partial / failure / cleanup outcomes (`succeeded`, `partially_applied`,
  `failed_rolled_back`, `failed_left_in_place`, `cancelled`) and the
  `checkpoint` / `delete_generated_files` / `git_initial_commit` rollback
  boundaries are all covered;
- all five side-effect families (`hook`, `network`, `registry`,
  `remote_image`, `dependency`) are declared by at least one drill;
- AI-assisted and extension-provided generation pin
  `ai_extension_uses_governed_surface = true` and are always disclosed;
- the generated-project lineage stays reconstructable after a failed /
  partial run (the runner asserts the lineage ref is non-empty and the
  descriptor / plan / run chain reconstructs for every run-bearing drill);
- the five negative drills keep undeclared-hook and undeclared-task
  execution, sibling descriptor / plan binding, and smuggled "declared"
  tasks rejected before projection.

## Running the corpus

```
cargo test -p aureline-qe --test scaffold_safety_conformance
```

The crate also exposes the corpus loader + projection assertions as a library
(`aureline_qe::scaffold_safety::{load_corpus, run_corpus,
run_corpus_from_repo_root}`), so other harnesses (Start Center / palette /
generator-preview UI checks, CLI / headless mirrors, support-export parity
reviews, release evidence reviews) can quote the same drill matrix without
re-parsing the fixtures.

## Redaction guarantees

Every fixture is metadata-safe: only opaque refs and typed labels cross the
boundary. Raw absolute paths, raw template bytes, raw generated file content,
raw credentials, raw commands, and raw remote URLs never appear. The runner
additionally scans each positive fixture for forbidden raw-export tokens
(`raw_template_bytes_export_allowed`, `raw_generated_content_export_allowed`,
`raw_path_export_allowed`, `raw_credential_export_allowed`,
`raw_secret_export_allowed`, `raw_token_export_allowed`,
`raw_command_export_allowed`, `raw_remote_url_export_allowed`); any occurrence
fails the drill before projection. Removing any positive or negative drill
without a replacement is a breaking contract change for the
`workspace.scaffold_safety_corpus.beta` corpus.
