# Package-manager + lockfile safety (reference scenario)

## Covers acceptance rows

- `ts_js_acceptance_row:package_manager_safety_plans`

## Binding

- Launch bundle: `launch_bundle:typescript_web_app.seed`
- Archetype row: `archetype_row:ts_web_app_or_service`
- Framework packs (in-scope for this scenario):
  - `framework_pack:typescript_web.npm_pnpm_toolchain`

## Scenario goal

Prove that any package-manager action required by TS/JS workflows is
reviewable and policy-bound before it writes to disk or opens the network.

This scenario is not about picking a canonical package manager. It is
about preserving:

- lockfile impact truth,
- lifecycle script/postinstall visibility,
- registry source + mirror/remapping truth,
- network egress class disclosure, and
- rollback/checkpoint posture.

## Required truth and disclosures

- Package changes emit a governed plan record before apply:
  - `docs/execution/package_manager_and_lockfile_safety_contract.md`
  - `schemas/execution/package_change_plan.schema.json`
- The plan composes with the package-action contract’s review-sheet and
  rollback vocabularies:
  - `docs/package/package_action_contract.md`

## Evidence hooks

- Worked example plan fixtures exist across local/container/CI/support
  projections:
  - `fixtures/execution/package_manager_cases/`

## Known-limit expectations

- If a package-manager adapter cannot meet the no-hidden-mutation guards,
  certification requires a known-limit note that blocks certified wording
  for workflows that depend on that adapter.

