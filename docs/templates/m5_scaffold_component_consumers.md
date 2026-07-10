# M5 scaffold component consumers

Status: Stable · Schema `schemas/ui/m5-scaffold-component-consumer.schema.json` · Record kind `add_shared_start_center_workspace_admission_template_registry_framework_pack_workflow_bundle_and_support_consumers_so_scaffold_components_keep_source_side_effect_and_health_language_aligned_across_claimed_m5_profiles`

This is the **adoption lane** over the frozen M5 scaffold-component matrix
(`docs/templates/m5_scaffold_component_matrix.md`). The matrix freezes six governed component
families and three sibling implement lanes narrow them into working primitives:

| Component family | Narrowed primitive | Canonical schema |
| --- | --- | --- |
| `scaffold_template_card` | scaffold template card / starter parameter row | `schemas/ui/m5-scaffold-template-card-starter-parameter-row-controls.schema.json` |
| `starter_parameter_row` | scaffold template card / starter parameter row | `schemas/ui/m5-scaffold-template-card-starter-parameter-row-controls.schema.json` |
| `scaffold_preflight_card` | scaffold preflight card / template health row | `schemas/ui/m5-scaffold-preflight-card-template-health-row-controls.schema.json` |
| `template_health_row` | scaffold preflight card / template health row | `schemas/ui/m5-scaffold-preflight-card-template-health-row-controls.schema.json` |
| `generated_project_diff_card` | generated-project diff card / scaffold handoff banner | `schemas/ui/m5-generated-project-diff-card-scaffold-handoff-banner-controls.schema.json` |
| `scaffold_handoff_banner` | generated-project diff card / scaffold handoff banner | `schemas/ui/m5-generated-project-diff-card-scaffold-handoff-banner-controls.schema.json` |

This lane proves those six families are **reusable components** — not one start-center page plus a
few isolated bootstrap objects — by binding every claimed M5 scaffold consumer to the same canonical
component schemas and the same descriptor vocabulary.

## Consumers

| Consumer | Token | Role |
| --- | --- | --- |
| Start Center | `start_center` | offers gallery cards and starter parameters with source / support truth |
| Workspace Admission | `workspace_admission` | admits a bootstrapped workspace with preflight and handoff truth |
| Template Registry | `template_registry` | lists signed templates with source / support and health-freshness truth |
| Framework Pack | `framework_pack` | discloses pack starter side effects before any create |
| Workflow Bundle | `workflow_bundle` | reads parameter-source and generated-versus-user-owned truth |
| Help / Support | `help_support` | keeps the generated boundary and recovery path explicit in support flows |
| Safe Handoff / Export Packet | `safe_handoff_export` | the authoritative rendering; references the canonical schemas so its prose can never drift |

Every family is adopted by **at least two** distinct consumers, and the safe handoff / export packet
references the canonical schema for every family it adopts.

## Shared descriptor vocabulary

The acceptance criterion is one truth for **starter source and support, side-effect disclosure,
health freshness, and the generated-versus-user-owned / recovery boundary** across every scaffold
surface. Those four descriptors (`source_and_support`, `side_effect_disclosure`, `health_freshness`,
`recovery_and_ownership_boundary`) are required on every binding, so starter-selection and bootstrap
language no longer drifts between gallery cards, entry review sheets, workflow-bundle surfaces, or
support artifacts.

## Auto-narrowing

A consumer that cannot preserve full parity **auto-narrows** its claim language and always discloses
a self-contained banner naming the exact reason and the recovery action — never a generic
"degraded" note:

| Parity-health mode | Narrowing reason | Recovery action | Export caveat |
| --- | --- | --- | --- |
| `source_or_support_unverified_narrowed` | `source_or_support_unverified` | `inspect_starter_source_and_support` | `source_or_support_unverified` |
| `side_effect_pending_narrowed` | `side_effect_disclosure_pending` | `review_side_effects_before_create` | `side_effect_disclosed_not_silent` |
| `health_stale_narrowed` | `health_freshness_stale` | `rerun_health_check_before_trusting` | `health_signal_stale_not_fresh` |
| `recovery_required_narrowed` | `recovery_required_after_partial_generation` | `delete_generated_or_continue_without_starter` | `generated_output_recoverable_not_final` |

### A generic Create never hides a side effect

`side_effect_disclosure_pending` reflects a starter that carries a network, dependency-install,
remote-provisioning, trust, or managed-workspace side effect. The resolver marks such a binding
`reflects_undisclosed_side_effect_risk = true`, always narrows it, and always resolves
`presents_ready_starter_without_caveat = false`. Only a full-parity binding may present a
ready-to-create starter without a caveat. This is the acceptance criterion that a generic Create
never hides a side effect on any claimed M5 scaffold consumer, and that Continue without starter,
Create empty, and delete-generated recovery paths stay explicit.

## Resolver

`resolve_scaffold_component_binding` takes one consumer's adoption of one component family, the
descriptor set it surfaces, the parity-health mode, and any export caveats, and produces one
`M5ScaffoldComponentResolvedBinding`. It rejects an empty or incomplete descriptor set and any
forbidden binding material, keeps the descriptor vocabulary aligned at full parity, auto-narrows
under any weakened mode, and — when narrowed — emits a self-contained banner.

## Governance & proof

The checked support export, matrix CSV, and Markdown report live under
`artifacts/release/m5-scaffold-component-consumer-proof/`, and the two narrowed fixtures
(framework pack → Beta, workspace admission → Preview) live under
`fixtures/ui/m5-scaffold-component-consumers/`. All are minted only by the
`dump_scaffold_component_consumers` example emitter so the in-code matrix, the artifact, the worked
bindings, and the fixtures never drift. Raw file bodies, raw diffs, raw local paths, repository
URLs, credentials, and secrets never cross this boundary.
