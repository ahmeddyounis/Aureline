# First-run task-success packet (beta)

This page describes the bounded task-success packet that backs the
two claimed beta switching flows in M3:

- the no-account first-run path that opens a local folder through the
  Start Center; and
- the imported-profile path that reviews a VS Code settings packet,
  mints a rollback checkpoint, and commits per-item outcomes.

The packet lives in the shell projection at
[`crate::onboarding_metrics`](../../../crates/aureline-shell/src/onboarding_metrics/mod.rs).
The shared contract ref is `shell:first_run_task_success_packet_beta:v1`
and the schema is published at
[`/schemas/ux/first_run_metrics.schema.json`](../../../schemas/ux/first_run_metrics.schema.json).

## What the packet pins

The packet enumerates one classified row per `(flow, state)` pair so
M3 reviewers can prove task success and not only ship the UI. Two
flow families and four task-success states are required:

| Flow | Completion | Fallback | Abandonment | Repair required |
|---|---|---|---|---|
| `first_run` | first-useful-edit reached on no-account local folder | managed sign-in declined at opt-in boundary | dropped before admission | forced sign-in before useful local work |
| `imported_profile` | per-item outcomes committed with rollback checkpoint | managed sync declined after import | dry-run dismissed before commit | rollback checkpoint missing |

Each row carries:

- a stable `row_id` quoted across surfaces;
- the bound `measurement_surface`, `entry_route_id`, `entry_verb`, and
  `target_kind` from the onboarding measurement plan;
- the typed `completion_class`, `completion_checkpoint_class`,
  `failure_category`, and `outcome_class` consumed by dashboards;
- a `repair_action_token` whenever the row reports
  `state = repair_required`;
- a `no_raw_sensitive_user_content = true` declaration so imported
  profiles are measurable without collecting raw user content;
- refs into the embedded onboarding telemetry capture; and
- partner scorecard, beta readiness review, docs/help, and support
  export refs.

## Privacy posture

The embedded onboarding telemetry capture is composed through
[`aureline_telemetry::onboarding`](../../../crates/aureline-telemetry/src/onboarding/mod.rs)
and inherits the metadata-safe-default privacy envelope:

- `privacy_class = privacy_local_only_no_emission` and the default
  consent state remains
  `off_by_default_no_emission_until_consent`;
- `export_posture = support_export_on_request` so the packet is only
  bundled into a support export when explicitly requested;
- raw project content, raw repo or project names, file paths, raw
  URLs, prompt or terminal text, clipboard content, and credentials
  are prohibited and never present.

A row that declares `no_raw_sensitive_user_content = false` is a
contract bug — the validator rejects it.

## Consumers

The same projection is consumed by:

- the live shell (planned wiring; the shell-only record is the
  contract surface today);
- the headless inspector
  (`aureline_shell_onboarding_metrics`);
- the support-export wrapper
  (`shell_first_run_task_success_packet_beta_support_export_record`);
- the markdown packet checked in at
  [`artifacts/ux/m3/first_run_task_success_packet.md`](../../../artifacts/ux/m3/first_run_task_success_packet.md);
- the deterministic fixtures under
  [`fixtures/ux/first_run_task_success_packet/`](../../../fixtures/ux/first_run_task_success_packet/);
  and
- the partner scorecards and beta readiness reviews quoted in the
  packet's `partner_scorecard_refs` and `readiness_review_refs`.

## Inspecting the packet

The headless inspector emits the same packet records the live shell
projects, the support-export wrapper retains, and the integration
tests replay against the checked-in fixtures:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_onboarding_metrics -- packet
cargo run -q -p aureline-shell --bin aureline_shell_onboarding_metrics -- rows
cargo run -q -p aureline-shell --bin aureline_shell_onboarding_metrics -- summary
cargo run -q -p aureline-shell --bin aureline_shell_onboarding_metrics -- telemetry
cargo run -q -p aureline-shell --bin aureline_shell_onboarding_metrics -- support-export
cargo run -q -p aureline-shell --bin aureline_shell_onboarding_metrics -- validate
cargo run -q -p aureline-shell --bin aureline_shell_onboarding_metrics -- compact
cargo run -q -p aureline-shell --bin aureline_shell_onboarding_metrics -- markdown
```

## Acceptance invariants

The validator (`validate_first_run_task_success_packet`) enforces:

1. every required `(flow, state)` cell has at least one row;
2. completion rows declare a typed `completion_checkpoint_class`;
3. abandonment and repair-required rows declare a typed
   `failure_category`;
4. repair-required rows declare a typed `repair_action_token`;
5. every row declares `no_raw_sensitive_user_content = true`;
6. the embedded telemetry capture's privacy envelope prohibits raw
   project content and reports `contains_raw_project_content = false`;
7. the telemetry capture's privacy class is local-only-no-emission or
   opt-in aggregate-only;
8. every event name referenced by a row resolves in the capture;
9. the `state_summary` block matches the rows; and
10. the packet declares at least one partner scorecard ref and one
    beta readiness review ref.

The validator returns a machine-readable list of detected violations
when an invariant breaks, so support evidence and partner scorecards
quote one source of truth.
