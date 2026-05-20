# Scaffold and generated-project safety beta audit

Reviewer guidance for the M3 scaffold and generated-project safety
conformance lane. This document is the human-readable companion to the
regression-gated corpus that proves Aureline's scaffold / generation promise
across every claimed beta creation row.

The corpus sits on top of the beta scaffold-safety projection introduced with
the template / generator descriptor, scaffold plan, and scaffold run records
([`scaffold_safety_beta.md`](scaffold_safety_beta.md)). Where that layer
defines *what* the projection means, this lane proves the projection keeps
meaning it on every row — turning the scaffold UX promise into a proof system
that catches undeclared-hook execution, writes before review, hidden
dependency / network side effects, hidden project databases, and ambiguous
post-create half-trust states before beta claims harden.

The exit-gate condition the corpus guards is the M3 scaffold anchor:

> Claimed beta template / generator rows have current proof for signed
> manifest truth, preflight diff review, undeclared-hook rejection,
> cleanup / rollback behavior, and generated-project lineage across local,
> mirrored, and policy-constrained creation paths.

## Where the corpus lives

- Fixtures + manifest:
  [`/fixtures/workspace/m3/scaffold_safety_corpus/`](../../../fixtures/workspace/m3/scaffold_safety_corpus/)
  (`manifest.json` is the single source of truth).
- Harness:
  [`/crates/aureline-qe/src/scaffold_safety/`](../../../crates/aureline-qe/src/scaffold_safety/).
- Replay: `cargo test -p aureline-qe --test scaffold_safety_conformance`.
- Release evidence:
  [`/artifacts/workspace/m3/scaffold_safety_report.md`](../../../artifacts/workspace/m3/scaffold_safety_report.md).
- Lineage evidence:
  [`/artifacts/workspace/m3/generated_project_lineage_audit.md`](../../../artifacts/workspace/m3/generated_project_lineage_audit.md).

## What a reviewer checks

Each positive drill binds one `TemplateGeneratorDescriptor`, one
`ScaffoldPlanRecord`, and zero-or-one `ScaffoldRunRecord`, then the harness
projects them and compares against the manifest's `expected_*` fields. A
reviewer auditing a claimed beta creation row confirms:

1. **Template provenance is reconstructable and not flattened.**
   `expected_provider_class`, `expected_signature_state`,
   `expected_generation_kind`, and `expected_source_distribution_class`
   reproduce the template the row claims. First-party, partner, community,
   extension, and AI providers stay distinct; mirrored and offline rows keep
   their `signed_verified` signature rather than reading as generic local
   files.
2. **The generation verb and impact are disclosed before any write.**
   `expected_generation_verb` (create-project / generate-into-existing /
   update-regenerate) and the declared side-effect set are pinned, and the
   `no_writes_before_review` guardrail must hold for any clean row.
3. **Side effects are declared before execution.**
   `expected_declared_side_effect_classes` names the hook / network /
   registry / remote-image / dependency families the plan declares;
   `expected_egress_posture` pins the network posture. A side effect that is
   not declared before execution flips `side_effects_declared_before_execution`
   to `false` (see `caught.hidden_side_effect`).
4. **Create-empty / set-up-later / rollback handoffs are visible.**
   `expected_create_empty_available`, `expected_set_up_later_available`,
   `expected_rollback_boundary`, and `expected_rollback_automatic` pin the
   handoffs the source artifact supports, so a user always has a same-weight
   bypass and a visible cleanup path.
5. **The run keeps generated output plain and attributable.** When
   `expected_has_run` is true, the run summary's lineage ref is non-empty and
   the descriptor / plan / run chain reconstructs; `expected_run_outcome` pins
   the explicit outcome. A run whose authoritative result is a hidden project
   database flips `generated_output_is_plain_workspace_content` to `false`
   (see `caught.hidden_project_database`).
6. **The disclosure verdict is honest.** `expected_surface_must_disclose`
   pins whether the surface MUST render the row as something other than a
   plain trusted local create (unsigned / mismatched, AI / extension,
   network-bearing, writes-into-existing, declares a side effect, or a failed
   run).

## How the corpus catches the three "must fail" conditions

The acceptance criteria require the corpus to **fail** any creation flow that
executes undeclared hooks, writes before preflight review, or hides
dependency / network side effects. The corpus splits these into projection
rejections (negative drills) and failing guardrails (caught positive drills):

| Condition | How it is caught | Drill |
| --- | --- | --- |
| Executes an undeclared hook | Projection refuses to assemble | `negative.run_invokes_undeclared_hook` |
| Executes an undeclared setup / validation task | Projection refuses to assemble | `negative.run_invokes_undeclared_task` |
| Smuggles a "declared" task not on the descriptor | Projection refuses to assemble | `negative.plan_plans_undeclared_task` |
| Binds a sibling / stale descriptor or plan | Projection refuses to assemble | `negative.plan_binds_sibling_descriptor`, `negative.run_binds_sibling_plan` |
| Writes before preflight review | `no_writes_before_review` guardrail false → `guardrails_all_hold` false | `caught.writes_before_review` |
| Hides a dependency / network side effect | `side_effects_declared_before_execution` guardrail false | `caught.hidden_side_effect` |
| Makes a hidden project database authoritative | `generated_output_is_plain_workspace_content` guardrail false | `caught.hidden_project_database` |

## Partial generation, cleanup, and recovery

Four failure drills keep the post-create states explicit so a user is never
left uncertain whether partial output is safe, broken, or still owned by the
generator:

- `failure.partial_generation_left_in_place` — `failed_left_in_place` with a
  **named manual cleanup** (`unavailable_manual`, not `not_needed`).
- `failure.missing_toolchain_rolled_back` — `failed_rolled_back` with a
  **performed** rollback and no stray artifacts.
- `failure.mirror_outage_cancelled` — `cancelled` before any write, rollback
  `not_needed`, mirror signature preserved.
- `failure.remote_image_unavailable_rolled_back` — `failed_rolled_back` on a
  remote-image outage with the remote-image / network side effects still
  attributable.

The generated-project lineage stays reconstructable across all of these; see
[`generated_project_lineage_audit.md`](../../../artifacts/workspace/m3/generated_project_lineage_audit.md).

## Transverse invariants

Beyond the per-drill assertions, the conformance suite pins, across the whole
positive set: every distinct generation verb; every claimed provider class;
the mirror / offline / extension / AI distributions (with mirror + offline
signature truth preserved); the create-empty and set-up-later handoffs; the
three caught-guardrail conditions; the partial / failure / cleanup outcomes
and rollback boundaries; all five side-effect families; the AI / extension
governed-surface rows; and the lineage-survives-failure proof. The negative
set keeps undeclared-action execution and sibling binding rejected.

## When a drill changes

`manifest.json` is the single source of truth. Adding a creation row to a beta
surface means adding a drill (fixture + manifest entry) before the row can be
claimed. Removing or weakening a positive or negative drill without a
replacement is a breaking contract change for the
`workspace.scaffold_safety_corpus.beta` corpus and a beta release blocker for
the workspace scaffold / generation lane.

## Out of scope

This lane proves the existing beta projection on every claimed creation row.
It does not expand into long-tail framework registry publication or post-M3
marketplace growth work, and it keeps verdicts keyed to the canonical
scaffold-plan / scaffold-run objects rather than screenshot-only review.
