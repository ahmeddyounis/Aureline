# Targeted test/run/debug loop (reference scenario)

## Covers acceptance rows

- `ts_js_acceptance_row:test_discover_run_rerun_debug`

## Binding

- Launch bundle: `launch_bundle:typescript_web_app.seed`
- Archetype row: `archetype_row:ts_web_app_or_service`
- Framework packs (in-scope for this scenario):
  - `framework_pack:typescript_web.vitest` (certified-path test runner)
  - `framework_pack:typescript_web.jest` (supported alternate)

## Scenario goal

Prove that test discovery and targeted execution are:

- attributable (execution context, target identity, toolchain identity);
- repeatable (rerun-failed paths are stable and do not drift into a
  different target silently); and
- debuggable (breakpoints/stack inspection behave consistently under the
  same execution context).

## Required truth and disclosures

- Test execution is described through run/attempt and invocation-result
  packets, not raw terminal text:
  - `docs/execution/run_and_attempt_contract.md`
  - `docs/commands/invocation_result_and_parity_contract.md`
- Any package-manager work required by the test run (install/update/fix)
  is guarded by a reviewable package-change plan before writes or network:
  - `docs/execution/package_manager_and_lockfile_safety_contract.md`
  - `docs/package/package_action_contract.md`

## Benchmark/workflow reservations (must be materialised before certification)

- `workflow.ts_js_targeted_test_run_and_debug`

## Evidence hooks

- Project Doctor probes for missing toolchains, wrong targets, and trust
  posture: `docs/support/project_doctor_packet.md`

## Known-limit expectations

- If Jest remains “supported alternate” rather than certified-path, the
  TS/JS launch bundle must carry a known-limit note that narrows certified
  wording to the Vitest pack scope.

