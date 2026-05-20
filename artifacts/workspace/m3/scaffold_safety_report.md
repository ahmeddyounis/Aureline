# Scaffold and generated-project safety conformance evidence

This artifact is the release-consumable conformance evidence for the M3
scaffold and generated-project safety beta lane. Every claimed beta
template / generator creation row reads exactly one
`ScaffoldSafetyBetaProjection` assembled from a
`TemplateGeneratorDescriptor`, a `ScaffoldPlanRecord`, and an optional
`ScaffoldRunRecord`. Every projection is exercised by at least one drill in
[`fixtures/workspace/m3/scaffold_safety_corpus/`](../../../fixtures/workspace/m3/scaffold_safety_corpus/);
the drills are executed by
[`crates/aureline-qe/src/scaffold_safety/`](../../../crates/aureline-qe/src/scaffold_safety/)
and replayed by
`cargo test -p aureline-qe --test scaffold_safety_conformance`.

The corpus is owned by the QE crate so the same fixture matrix can gate Start
Center starter rows, command-palette and generator-preview projections, AI /
extension generation, CLI / headless mirrors, support-export parity reviews,
and release evidence reviews from one shared truth.

The exit-gate condition the corpus guards is the M3 scaffold anchor:

> Claimed beta template / generator rows have current proof for signed
> manifest truth, preflight diff review, undeclared-hook rejection,
> cleanup / rollback behavior, and generated-project lineage across local,
> mirrored, and policy-constrained creation paths.

## Result

`cargo test -p aureline-qe --test scaffold_safety_conformance` — **13
tests, all passing** (1 corpus replay + 12 transverse invariants). The
corpus publishes **19 positive** drills and **5 negative** drills. The
in-library replay `aureline_qe::scaffold_safety::run_corpus_from_repo_root`
returns a `CorpusReport` with no `failures()`.

## Coverage matrix

| Axis | Drill id | Outcome anchored |
| --- | --- | --- |
| First-party signed template — create | `template.first_party_signed_create` | `first_party` / `signed_verified`, deferred restore, checkpoint rollback, succeeding run, lineage bound. |
| Extension-provided starter — governed | `template.extension_starter_governed` | `extension_provided` / `signed_unverified`, generate-into-existing, extension actor run reuses governed surface, delete-generated rollback. |
| AI-assisted suggestion — governed | `template.ai_assisted_governed` | `ai_assisted` / `unsigned`, AI actor run blocks undeclared actions, `ai_extension_uses_governed_surface` holds, create-empty offered. |
| Imported scaffold pack — offline bundle | `import.scaffold_pack_offline_signed` | `community_signed` / `signed_verified`, `offline_bundle` distribution preserved, no egress, `offline_only` policy. |
| Mirrored template — signed registry | `mirror.template_signed_registry` | `partner_signed` / `signed_verified`, `mirror` distribution preserved, hook / registry / dependency declared. |
| Policy-blocked generator | `policy.generator_blocked_allowlist` | `policy_constrained` honesty, allowlist + fleet-pinned, restore narrowed to manifest entry, preflight only. |
| Create-empty parity | `handoff.create_empty_parity` | `create_empty_available` + `set_up_later_available`, no template files, no disclosure needed. |
| Generated-diff review | `diff_review.generate_into_existing` | Preflight-only generate-into-existing, create + modify impact, `writes_into_existing_project`. |
| Set-up-later deferred | `handoff.setup_later_deferred` | `scaffold_without_dependency_restore` + `set_up_later`, deferred restore, lineage bound. |
| Remote-image / devcontainer | `remote_image.devcontainer_pull` | `remote_image_pull` egress, devcontainer bootstrap declared, hook / network / remote-image / dependency disclosed. |
| Update / regenerate managed zone | `update.regenerate_managed_zone_partial` | `update_regenerate`, `partially_applied`, `partial_rollback` honesty, replay-safe lineage. |
| Support export — run lineage | `support.export_run_lineage` | Support surface, export-safe lineage capture ties run to artifacts / checkpoint / lineage ref. |
| Failure — partial generation left in place | `failure.partial_generation_left_in_place` | `failed_left_in_place`, rollback `unavailable_manual` (named, not `not_needed`), partial output attributable. |
| Failure — missing toolchain | `failure.missing_toolchain_rolled_back` | `failed_rolled_back`, rollback `performed`, no stray artifacts, lineage reconstructable for retry. |
| Failure — mirror outage | `failure.mirror_outage_cancelled` | `cancelled` before any write, rollback `not_needed`, mirror signature preserved. |
| Failure — remote image unavailable | `failure.remote_image_unavailable_rolled_back` | `failed_rolled_back`, rollback `performed`, remote-image / network side effects stay attributable. |
| Caught — writes before review | `caught.writes_before_review` | Projection assembles but `no_writes_before_review` guardrail flips false → `guardrails_all_hold = false`. |
| Caught — hidden side effect | `caught.hidden_side_effect` | `side_effects_declared_before_execution` guardrail flips false → `guardrails_all_hold = false`. |
| Caught — hidden project database | `caught.hidden_project_database` | `generated_output_is_plain_workspace_content` guardrail flips false; lineage still reconstructable. |
| Negative — run invokes undeclared hook | `negative.run_invokes_undeclared_hook` | Projection rejects with `… invoked undeclared hook …`. |
| Negative — run invokes undeclared task | `negative.run_invokes_undeclared_task` | Projection rejects with `… invoked undeclared task …`. |
| Negative — plan binds sibling descriptor | `negative.plan_binds_sibling_descriptor` | Projection rejects with `scaffold plan references descriptor …`. |
| Negative — run binds sibling plan | `negative.run_binds_sibling_plan` | Projection rejects with `scaffold run references plan …`. |
| Negative — plan plans undeclared "declared" task | `negative.plan_plans_undeclared_task` | Projection rejects with `… as declared, but it is not on the descriptor`. |

## Acceptance-criteria mapping

- **The corpus fails any creation flow that executes undeclared hooks, writes
  before preflight review, or hides dependency / network side effects.**
  Undeclared-hook (and undeclared-task) execution are rejected at projection
  time by the `negative.run_invokes_undeclared_hook` /
  `negative.run_invokes_undeclared_task` drills. Writes-before-review and a
  hidden side effect are caught as failing guardrails by
  `caught.writes_before_review` and `caught.hidden_side_effect`
  (`guardrails_all_hold = false`).
- **Generated-project lineage remains reconstructable after rollback, retry,
  import / export, and support-safe capture.** The runner reconstructs the
  descriptor / plan / run chain and asserts a non-empty generated lineage ref
  for every run-bearing drill — including the rolled-back, left-in-place,
  cancelled, partial, hidden-database, and support-surface rows.
- **Partial generation and cleanup states are explicit and do not strand
  users.** The four failure drills pin distinct explicit states:
  `failed_left_in_place` with a named manual rollback, `failed_rolled_back`
  with a performed rollback, `cancelled` with `not_needed`, and
  `partially_applied` with the `partial_rollback` honesty label.
- **Mirrored / offline template rows preserve signature / provenance truth
  instead of flattening into generic local files.** The mirror and offline
  drills pin `mirror` / `offline_bundle` source distribution **and**
  `signed_verified` signature; the conformance suite asserts the signature is
  not flattened.

## Cross-surface lineage check

For every run-bearing positive drill the harness asserts that:

- the projection's `descriptor_ref` and `scaffold_plan_ref` reconstruct the
  fixture descriptor and plan ids,
- the projection echoes the fixture run id in `scaffold_run_ref`, and
- the run summary's `generated_lineage_ref` is non-empty.

This holds on the failure, partial, cancelled, hidden-database, and
support-export drills, proving the lineage survives rollback / retry /
import-export / support capture.

## Replay

```
cargo test -p aureline-qe --test scaffold_safety_conformance
```

The corpus manifest at
`fixtures/workspace/m3/scaffold_safety_corpus/manifest.json` is the canonical
pass / fail input; CI consumers SHOULD treat any `failures()` returned by
`run_corpus_from_repo_root` as a beta release blocker for the workspace
scaffold / generation lane.

## Redaction guarantees

Every fixture is metadata-safe: only opaque refs and typed labels cross the
boundary. The runner scans each positive fixture for forbidden raw-export
tokens before projection, so the redaction contract lives on the corpus
itself, not only on individual surface read paths.
