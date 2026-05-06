# TypeScript / JavaScript expert acceptance matrix

This document is the narrative companion to the TypeScript/JavaScript
expert acceptance matrix. It exists so launch-language bundles,
framework packs, compatibility reports, benchmark packets, migration
copy, and support exports can cite concrete acceptance rows instead of
relying on vague “TS/JS first” promises.

Companion artifacts:

- [`/artifacts/compat/ts_js_acceptance_rows.yaml`](../../artifacts/compat/ts_js_acceptance_rows.yaml)
  — machine-readable acceptance rows with stable ids, requirement
  classes (required launch / aspirational / community-only), and
  evidence hooks.
- [`/fixtures/compat/ts_js_reference_scenarios/`](../../fixtures/compat/ts_js_reference_scenarios)
  — named reference scenarios that bind acceptance rows to archetype
  rows, framework packs, and corpus/workflow reservations.
- [`/artifacts/product/language_bundle_rows.yaml`](../../artifacts/product/language_bundle_rows.yaml)
  — launch-language bundle and framework-pack inventory; the TS/JS
  launch wedge resolves through `launch_bundle:typescript_web_app.seed`.
- [`/artifacts/compat/reference_workspace_rows.yaml`](../../artifacts/compat/reference_workspace_rows.yaml)
  — reference-workspace archetype inventory; TS/JS work resolves
  through `archetype_row:ts_web_app_or_service`.
- [`/artifacts/compat/archetype_rubric.yaml`](../../artifacts/compat/archetype_rubric.yaml)
  — the authoritative support-class evidence-burden rules that control
  when a claim may move from experimental → supported → certified.

## Why this exists

The launch-language bundle rubric already prevents free-text “supports
TypeScript” claims by requiring a named bundle id and an archetype row.
What it still needs is a reviewer-facing definition of what “replacement
grade for TS/JS” means in practice.

This matrix provides that definition:

- required launch rows enumerate the minimum scenarios that must be
  evidenced before a certified TS/JS claim is admissible;
- aspirational rows document intended uplift without blocking launch
  copy; and
- community-only rows make it explicit which “works for me” workflows
  must not widen first-party claims.

## Row model

Every acceptance row in `artifacts/compat/ts_js_acceptance_rows.yaml`
includes:

- `acceptance_row_id` — stable id referenced by packets and docs.
- `requirement_class` — `required_launch`, `aspirational_first_party`,
  or `community_only`.
- `workflow_axis` — the primary axis the row exercises (monorepo
  navigation, test/debug/build, preview/source maps, package-manager
  safety, migration/import, and so on).
- `framework_pack_refs` — the pack(s) that must remain in scope for the
  row to be claim-bearing.
- `evidence_hooks` — concrete file/row/schema refs that must exist for
  the row to be promoted beyond prose.

Evidence hooks are intentionally specific. A claim cannot move to a
stronger class based only on an internal “we tried it” note; the hook
must point at an artifact a reviewer or tool can inspect.

## Required launch rows (TS/JS)

These are the acceptance rows that block replacement-grade TS/JS claims.
They map directly to the TS/JS launch wedge and to the TS/JS archetype
row.

| Acceptance row | Scope | Proof hooks (minimum) |
|---|---|---|
| `ts_js_acceptance_row:monorepo_first_useful_result` | Large monorepo navigation | reference scenario + corpus workflow reservations + project-graph contract |
| `ts_js_acceptance_row:rename_across_project_references` | Multi-package refactors | reference scenario + refactor preview/rollback contract |
| `ts_js_acceptance_row:framework_route_to_component_workflow` | Framework-aware routing | reference scenario + framework pack + project-graph contract |
| `ts_js_acceptance_row:test_discover_run_rerun_debug` | Test/debug/build loop | reference scenario + run/attempt + invocation result + package-plan contract |
| `ts_js_acceptance_row:node_debug_sourcemaps_stability` | Debug + source maps | reference scenario + artifact-resolution contracts |
| `ts_js_acceptance_row:browser_preview_and_hmr` | Browser preview runtime | reference scenario + route-truth + preview-route schema |
| `ts_js_acceptance_row:generated_artifacts_safe_handling` | Generated artifacts | reference scenario + generated-artifact posture policy + drift fixtures |
| `ts_js_acceptance_row:package_manager_safety_plans` | Package-manager safety | reference scenario + package-change-plan contract + fixture corpus |
| `ts_js_acceptance_row:mixed_js_ts_path_aliases` | Mixed module/typing states | reference scenario + project-graph/indexing contract |
| `ts_js_acceptance_row:remote_container_identity_and_attribution` | Remote/container mode | reference scenario + attach/forward/route contract + attach fixtures |
| `ts_js_acceptance_row:migration_import_vscode_expectations` | Migration/import | reference scenario + migration source row + import diff/rollback contract |
| `ts_js_acceptance_row:ai_patch_evidence_across_projects` | AI patch evidence | reference scenario + AI review/evidence replay contracts + refactor preview rules |

## Reference scenario set

The reference scenarios under `fixtures/compat/ts_js_reference_scenarios/`
are the reviewer-facing “worked” definitions of the rows above. Each
scenario:

- names which acceptance rows it covers;
- binds the scenario to the TS/JS archetype row and bundle/framework-pack
  scope;
- lists the benchmark/workflow reservation ids the benchmark lane must
  materialize (or keep reserved as explicit future work);
- lists the docs/schemas/contracts that must exist to make the scenario
  honest; and
- records known-limit notes the scenario expects to exist before a
  certified claim is admissible.

## Evidence hook map: supported → certified (TS/JS)

Promotion from supported to certified is not a copy change; it is an
evidence change. For the TS/JS launch wedge, the evidence hook map is:

| Evidence lane | Supported floor (must exist) | Certified add-on (must also exist) |
|---|---|---|
| Benchmark | Corpus/workflow scenario ids exist and are run often enough to remain current. | Certified-archetype workflows cover the required launch rows across the declared platform/mode matrix. |
| Compatibility | A current compatibility report row exists for the archetype/bundle scope, and it links to the same acceptance-row ids. | The certified-archetype report is current and names the same workflow rows the compatibility report cites. |
| Docs | Public-truth docs pages exist for the bundle and the acceptance matrix; contracts cited by the rows are published and version-matched. | Certified publication copy cites the acceptance rows and the certified-archetype report packet rather than summarizing results in prose. |
| Migration | Marketed migration source rows (for example VS Code) have current import diff/rollback evidence and scope caveats. | Any migration wording used in top-level TS/JS claims is backed by certified-archetype evidence and does not widen beyond the published source-row scope. |
| Support | Support bundles and Project Doctor packets can carry the same target/toolchain/package-plan/preview-route evidence required to reproduce failures. | Certified-class support claims name the exact report packet refs and known-limit refs needed to defend the claim during regressions. |

If any required evidence hook is missing or stale, the archetype rubric’s
demotion rules apply: the row narrows (typically certified → supported,
supported → community) rather than leaving the claim language unchanged.
