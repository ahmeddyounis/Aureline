# Extension host isolation, restart budgets, resource limits, and quarantine behavior

This page is the reviewer-facing entrypoint for the extension host
supervision lane. It is the typed contract that finalizes how the
extension platform isolates hosts, enforces restart budgets, measures
resource limits, and quarantines extensions whose runtime behavior
crosses a typed trigger rule.

The contract is governed. The canonical Rust source of truth lives in
[`crates/aureline-extensions/src/supervision/`](../../../crates/aureline-extensions/src/supervision/);
the cross-tool boundary schema is
[`schemas/extensions/host_isolation.schema.json`](../../../schemas/extensions/host_isolation.schema.json);
the checked-in fixtures live under
[`fixtures/extensions/m3/isolation_and_quarantine/`](../../../fixtures/extensions/m3/isolation_and_quarantine/).

## Why a supervision record on top of the runtime contract

The runtime v1 beta admission contract
([`docs/extensions/m3/runtime_v1_beta.md`](runtime_v1_beta.md)) answers
"is this extension host admitted at all?" It does not by itself answer
"is the host running under budget, throttled, disabled, awaiting an
explicit reenable, quarantined, or recovering?". Without one typed
supervision record, every later surface (install / review chrome, the
permission inspector, the runtime status pill, support exports, the
partner packet template, the CLI / headless lanes) is free to invent a
local "extension stopped working" string. That is the gap this lane
closes.

One `ExtensionHostSupervisionRecord` binds:

- the already-evaluated `RuntimeV1BetaContractRecord` (extension
  identity, host placement / supervision class, lifecycle, restart
  posture, restart attempt count, degraded state, admission decision);
- the per-axis runtime-budget evidence (one
  [`SupervisionAxisClass`](../../../crates/aureline-extensions/src/supervision/mod.rs) ->
  [`BudgetPressureClass`](../../../crates/aureline-extensions/src/supervision/mod.rs)
  entry per discovery / cold-activation / warm-activation /
  idle-polling / memory / egress / crash-loop axis, mirrored from
  [`artifacts/extensions/runtime_budget_rows.yaml`](../../../artifacts/extensions/runtime_budget_rows.yaml));
- the restart-budget snapshot (`restart_posture_class`, attempts used,
  attempts remaining, crash-loop window distinct failures, and the
  trip thresholds for disable and quarantine);
- the supervisor's `response_class` (`none_nominal`,
  `throttle_background_work`, `disable_until_next_session`,
  `disable_until_user_explicit_reenable`, `quarantine`), mirrored from
  [`artifacts/extensions/quarantine_rules.yaml`](../../../artifacts/extensions/quarantine_rules.yaml);
- the `visibility_posture_class` (where chrome surfaces the response,
  with a typed `not_visible_nominal_row` sentinel reserved for nominal
  rows only);
- the `discovery_ranking_posture_class` so a quarantined or disabled
  row cannot quietly keep its installed-in-many-workspaces signal;
- the `maintainer_coverage_class` (`required_quorum_recorded` is
  mandatory for `quarantine` and `disable_until_user_explicit_reenable`
  decisions; a missing quorum on a quorum-bearing decision refuses
  the supervision row);
- the `recovery_precondition_class` and the
  `recovery_visible_projection_class` so reading the row alone tells a
  reviewer whether recovery is pending and what the visible state will
  look like once it lands;
- the optional `trigger_rule_ref` (a `quarantine_rule:` ref) plus the
  paired audit-event refs and a repair-affordance label.

The record always emits `RedactionClass::MetadataSafeDefault`. Raw
manifest bytes, raw signing-key material, raw policy bodies, raw
paths, raw tokens, raw helper-binary invocation bodies, raw bridge-
shim payloads, raw runtime payload bytes, raw memory snapshots, raw
core dumps, raw log bodies, and raw network payload bytes MUST NOT
appear anywhere in the record set; every field is an opaque ref or a
closed vocabulary value.

## Supervision flow

[`evaluate_extension_host_supervision`](../../../crates/aureline-extensions/src/supervision/mod.rs)
is deterministic and fails closed. In strict precedence order:

1. If the runtime contract is not in an `admitted`, `admitted_narrowed`,
   or `quarantined` posture, refuse with
   `refused_runtime_contract_not_admitted` — the supervision lane is
   not the place to mask a refused admission.
2. If the contract's lifecycle is `publisher_blocked`, hold with
   `publisher_block_active`. Recovery is upstream (the publisher
   continuity row clears it); the supervisor records the hold without
   inventing a quarantine for it.
3. If the contract reports an active runtime-budget quarantine OR the
   lifecycle is `quarantined` but no axis is at
   `crash_loop_window_breach` pressure, refuse with
   `refused_axis_pressure_inconsistent_with_response`. Likewise, if an
   axis reports `crash_loop_window_breach` pressure but the contract is
   not quarantined, refuse with the same reason.
4. If any axis is at `crash_loop_window_breach`, response =
   `quarantine`. Requires a typed `trigger_rule_ref`, maintainer
   coverage `required_quorum_recorded`, discovery ranking
   `removed_from_ranking`, and visibility on `install_review` or
   `permission_inspector`. Failure of any of these guardrails refuses
   the supervision row with a typed reason.
5. If `memory` is at `hard_breach`, response =
   `disable_until_next_session` with reason
   `memory_hard_cap_breach_disables_until_next_session`.
6. If `idle_polling` is at `hard_breach`, response =
   `disable_until_next_session` with reason
   `idle_polling_hard_breach_disables_until_next_session`.
7. If `egress` is at `hard_breach`, response =
   `disable_until_user_explicit_reenable`. Requires maintainer coverage
   `required_quorum_recorded` and visibility on `install_review` or
   `permission_inspector`. Reason is
   `egress_hard_breach_requires_user_reenable`.
8. If any sustained axis is at `soft_breach`, response =
   `throttle_background_work` with reason
   `sustained_soft_breach_throttles_background`.
9. Otherwise, if a recovery precondition is pending, return
   `recovery_in_progress` with reason
   `recovery_in_progress_returning_to_nominal`. A `recovery_in_progress`
   decision requires a non-`not_recovering` visible projection.
10. Otherwise return `continue_admitted` with reason
    `nominal_under_budget`.

## First consumer: support / partner export

[`project_extension_host_supervision_support_export`](../../../crates/aureline-extensions/src/supervision/mod.rs)
is the first reading consumer. It emits a
`ExtensionHostSupervisionSupportExportRecord` that quotes the same
closed tokens visible to install / review and to the permission
inspector, plus:

- the worst-axis class and pressure (so support reviewers see which
  axis drove the response without re-deriving it from the entries),
- the restart-budget snapshot (`attempts_used`, `attempts_remaining`,
  `crash_loop_window_distinct_failures`) so the support packet states
  the restart budget plainly,
- the `blocks_activation` flag (set whenever the decision is
  `disable_until_next_session`, `disable_until_user_explicit_reenable`,
  `quarantine_pending_review`, `hold_publisher_blocked`, or
  `refuse_inconsistent_input`),
- the optional `trigger_rule_ref` and a `repair_affordance_label` that
  chrome and CLI surfaces can render verbatim.

The support export is what the partner packet template, the
support-export bundle, the install review chrome's runtime sub-panel,
and any later CLI / headless review consumer read. They join through
`supervision_ref` and quote `supervision_decision_class` /
`supervision_reason_class` verbatim instead of inventing a local
"extension stopped working" string.

## Fixtures

The checked-in fixtures replay every reserved decision class through
the support export bundle:

- [`wasm_in_process_nominal_continue.json`](../../../fixtures/extensions/m3/isolation_and_quarantine/wasm_in_process_nominal_continue.json)
  — a capability-bounded Wasm component-model row with every axis
  nominal; `continue_admitted` / `nominal_under_budget`.
- [`wasm_subprocess_soft_breach_throttled.json`](../../../fixtures/extensions/m3/isolation_and_quarantine/wasm_subprocess_soft_breach_throttled.json)
  — sustained soft breach on `warm_activation`;
  `throttle_background_work` /
  `sustained_soft_breach_throttles_background`.
- [`external_host_memory_hard_cap_disabled.json`](../../../fixtures/extensions/m3/isolation_and_quarantine/external_host_memory_hard_cap_disabled.json)
  — supervised external host process at memory hard-cap;
  `disable_until_next_session` /
  `memory_hard_cap_breach_disables_until_next_session`.
- [`external_host_crash_loop_quarantined.json`](../../../fixtures/extensions/m3/isolation_and_quarantine/external_host_crash_loop_quarantined.json)
  — supervised external host at crash-loop quarantine;
  `quarantine_pending_review` /
  `crash_loop_window_breach_trips_quarantine`. Cites
  `quarantine_rule:crash_loop_window_breach_trips_quarantine`, records
  maintainer-coverage quorum, removes from discovery ranking, and
  surfaces on install_review + permission_inspector + runtime status
  pill.
- [`wasm_subprocess_publisher_blocked_hold.json`](../../../fixtures/extensions/m3/isolation_and_quarantine/wasm_subprocess_publisher_blocked_hold.json)
  — a denial drill where the runtime contract was refused for opaque
  publisher identity; the supervisor refuses to evaluate with
  `refused_runtime_contract_not_admitted`.

## Guardrails

The supervision contract refuses to widen on:

- a runtime contract that is not in an admitted, admitted-narrowed, or
  quarantined posture (the upstream refusal is preserved verbatim);
- a quarantine or `disable_until_user_explicit_reenable` decision that
  did not record maintainer-coverage quorum;
- a quarantine without a typed `trigger_rule_ref`;
- a quarantine that did not remove the row from discovery ranking;
- a disable or quarantine response whose visibility posture omits
  `install_review` and `permission_inspector`; and
- a runtime contract reporting an active quarantine or quarantined
  lifecycle while no axis is at `crash_loop_window_breach`, or
  vice-versa.

These map directly to the spec's "no beta widening on opaque publisher
identity, missing diff reports, or unbounded host authority" rule and
to `quarantine_rules.yaml`'s "silent quarantine is forbidden" /
"single-owner egress disable is forbidden" invariants.

## How to verify

```
cargo test -p aureline-extensions supervision::
```

The supervision tests replay every fixture above end-to-end through
`evaluate_extension_host_supervision`,
`validate_extension_host_supervision`, and
`project_extension_host_supervision_support_export`, and exercise every
refuse / quarantine / hold guardrail.
