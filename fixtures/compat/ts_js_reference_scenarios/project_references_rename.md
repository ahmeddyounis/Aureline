# TS project-references rename across packages (reference scenario)

## Covers acceptance rows

- `ts_js_acceptance_row:rename_across_project_references`

## Binding

- Launch bundle: `launch_bundle:typescript_web_app.seed`
- Archetype row: `archetype_row:ts_web_app_or_service`
- Framework packs (in-scope for this scenario):
  - `framework_pack:typescript_web.react`
  - `framework_pack:typescript_web.next_js`

## Scenario goal

Prove that rename and find-references work across a multi-package TS/JS
workspace, with explicit preview and rollback posture.

This scenario is intentionally strict about “silent” behavior: a rename
that cannot prove completeness must downgrade its preview completeness
class and disclose why, rather than returning a partial edit set as if it
were complete.

## Workspace shape (representative)

- A repo with multiple packages (apps + shared libraries).
- TypeScript project references or an equivalent multi-project setup.
- Path aliases that cross package boundaries.
- One or more generated artifacts that are semantically related to the
  renamed source (for example `.d.ts` output directories) and must be
  treated as generated/protected unless a declared regeneration intent is
  in scope.

## Required truth and disclosures

- The rename operation produces a governed preview packet and an explicit
  rollback path, per `docs/editor/refactor_and_replace_transaction_contract.md`.
- Generated artifacts must not be edited as if they are canonical source,
  per `docs/architecture/generated_artifact_safe_edit_policy.md`.

## Benchmark/workflow reservations (must be materialised before certification)

- `workflow.ts_js_project_references_rename_across_packages`

## Evidence hooks

- Refactor preview/apply/rollback contract:
  `docs/editor/refactor_and_replace_transaction_contract.md`
- Generated-artifact safe-edit posture:
  `docs/architecture/generated_artifact_safe_edit_policy.md`

## Known-limit expectations

- If rename completeness depends on index warm-up state, certification
  requires a known-limit note that narrows the claim to the declared
  completeness window and names a recovery route.

