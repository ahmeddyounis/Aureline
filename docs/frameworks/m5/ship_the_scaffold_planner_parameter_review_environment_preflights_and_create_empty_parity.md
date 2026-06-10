# Scaffold planner, parameter review, environment preflights, and create-empty parity

This contract describes the export-safe packet that carries the **scaffold
planner**: the set of prepared scaffold plans the IDE may apply, each row
annotated with its parameter-review state, environment-preflight state, previewed
write impact and rollback boundary, create-empty parity posture, and readiness.
The packet is the canonical truth that the gallery, scaffold preflight,
parameter-review sheet, run and recovery surfaces, diagnostics, and support
exports ingest instead of re-describing plan, preflight, or parity state by hand.

- Boundary schema:
  `schemas/templates/ship-the-scaffold-planner-parameter-review-environment-preflights-and-create-empty-parity.schema.json`
- Implementation:
  `crates/aureline-scaffold/src/ship_the_scaffold_planner_parameter_review_environment_preflights_and_create_empty_parity/`
- Checked support export:
  `artifacts/templates/m5/ship_the_scaffold_planner_parameter_review_environment_preflights_and_create_empty_parity/support_export.json`
- Summary artifact:
  `artifacts/templates/m5/ship_the_scaffold_planner_parameter_review_environment_preflights_and_create_empty_parity.md`
- Fixtures:
  `fixtures/templates/m5/ship_the_scaffold_planner_parameter_review_environment_preflights_and_create_empty_parity/`
- Producer:
  `aureline_scaffold::ship_the_scaffold_planner_parameter_review_environment_preflights_and_create_empty_parity::current_scaffold_planner_export`

This packet **projects** the upstream scaffold-run, template-manifest, and
hook-policy contracts (`schemas/templates/scaffold_run_alpha.schema.json`,
`schemas/templates/template_manifest_alpha.schema.json`,
`schemas/templates/scaffold_hook_policy.schema.json`). It reuses their target,
parameter, preflight, write-impact, rollback, and create-empty vocabulary rather
than inventing parallel terms, and references each upstream record by an opaque
ref (`scaffold_run_ref`, `manifest_ref`) instead of embedding it.

## Boundary discipline

The packet is metadata only. Raw parameter values, secrets, absolute paths,
repository URLs, manifest bodies, hook bodies, command output, and user-authored
content never cross this boundary. Rows carry opaque refs, closed-vocabulary
class tokens, count summaries, and short reviewable summaries. `validate` rejects
any export that leaks obviously forbidden material.

## Plan truth

Each `plan_row` binds one prepared scaffold plan to:

- **Plan kind and provenance** — `plan_kind` (`template_scaffold`,
  `create_empty_workspace`, `create_empty_parity_bridge`), plus `template_id` and
  `manifest_ref` for template-backed plans and `scaffold_run_ref` for every plan.
- **Parameter review** — `parameter_review` with the review class and the
  total / resolved / required / unresolved-required / invalid counts, plus
  `no_mutation_during_review` so reviewing parameters never mutates the workspace.
- **Environment preflight** — `environment_preflight` with the preflight class
  and the total / passed / warning / blocking-failure counts and check refs.
- **Create-empty parity** — `create_empty_parity` records whether a create-empty
  workspace reaches the same safety pipeline (`shares_preflight_pipeline`,
  `shares_parameter_review`, `shares_rollback_boundary`) as a templated scaffold,
  so the empty flow is never presented as an unreviewed shortcut.
- **Write impact and rollback** — `write_impact_preview` with the created /
  modified file and directory counts, `no_writes_before_confirmation` (always
  true: no write happens before review), the preview ref, and `rollback_posture`.
- **Readiness and admission** — `readiness_state` and `admitted_for_apply`. A
  blocking parameter or preflight state, a non-ready readiness, a blocked parity
  posture, or an unavailable rollback boundary forces `admitted_for_apply` to
  `false`.
- **Downgrade and projection** — `downgrade_triggers` and `consumer_surfaces`.

## Create-empty parity

Parity is the property that a create-empty workspace reaches the same parameter
review, environment preflight, write-impact preview, and rollback boundary as a
templated scaffold. A create-empty plan that claims `full_parity_with_template_flow`
**must** share the preflight pipeline and rollback boundary; the validator
rejects a parity label asserted without the shared guarantees behind it. The
canonical export carries a create-empty workspace plan that reaches full parity
and is admitted for apply alongside the templated plans.

## Downgrade automation

`apply_downgrade_automation` narrows plans from observed runtime signals so a
stale or underqualified plan narrows before it is offered, instead of being
hidden:

- Unresolved required parameters mark the plan `awaiting_required_input`, set
  readiness to `blocked_awaiting_input`, and withdraw admission.
- A failed environment preflight marks it `preflight_failed_blocked`, sets
  readiness to `blocked_failed_preflight`, and withdraws admission.
- A missing write-impact preview or rollback boundary forces review and withdraws
  admission, setting `rollback_unavailable_review_required` for the latter.
- Broken create-empty parity marks a create-empty plan `parity_broken_blocked`,
  sets readiness to `blocked_parity_broken`, and withdraws admission.
- Stale proof or a narrowed upstream withdraws admission.

A narrowed plan stays a valid, export-safe packet, so the gallery and support
surfaces show a current, labeled state rather than an optimistic placeholder.

## Consumers

`current_scaffold_planner_export()` reads and validates the checked support
export. It is the first real consumer: a gallery, preflight, parameter-review,
diagnostics, or support-export surface ingests the canonical packet through it.
The two checked fixtures (`parameter_unresolved_blocked.json`,
`create_empty_parity_broken.json`) are valid, narrowed packets that exercise the
downgrade behavior the canonical export keeps green.

The artifact and fixtures are regenerated deterministically from the canonical
builder:

```text
cargo run -p aureline-scaffold --example dump_scaffold_planner -- canonical
cargo run -p aureline-scaffold --example dump_scaffold_planner -- markdown
cargo run -p aureline-scaffold --example dump_scaffold_planner -- parameter_unresolved
cargo run -p aureline-scaffold --example dump_scaffold_planner -- create_empty_parity_broken
```
