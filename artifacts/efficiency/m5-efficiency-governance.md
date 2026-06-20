# M5 efficiency governance — human-readable rendering

Human-readable rendering of the canonical M5 efficiency-state governance matrix.
The machine-readable truth is at
[`artifacts/efficiency/m5-efficiency-governance.json`](./m5-efficiency-governance.json),
validated by
[`schemas/efficiency/m5-efficiency-governance.schema.json`](../../schemas/efficiency/m5-efficiency-governance.schema.json).
The normative policy companion is
[`docs/efficiency/m5-efficiency-governance.md`](../../docs/efficiency/m5-efficiency-governance.md).
This lane is registered under the canonical M5 evidence index
(`artifacts/release/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json`).

The matrix freezes **one typed efficiency-state contract** for every M5 surface
that adapts under battery or thermal pressure — notebooks, previews, docs/browser
panes, traces, pipelines, remote sessions, support exports, and
companion-adjacent views. It binds each surface to the evidence that backs its
low-power claim and recomputes, for each row, the narrowing reasons that fire,
the narrowed effective posture, and the certification state. The shell efficiency
runtime ([`crates/aureline-shell/src/efficiency/`](../../crates/aureline-shell/src/efficiency/))
owns the closed vocabulary; the matrix is a downstream projection bound back to
it by a conformance test so the two can never drift.

## Closed vocabulary

The matrix freezes seven closed vocabularies, mirrored from the shell efficiency
runtime: **efficiency state** (`Nominal`, `EfficiencyAware`, `ThermalConstrained`,
`ProtectCore`, `Recovery`), **source-of-change** (the battery/thermal/policy
pressure signals), **throttled subsystem** (the shed-work families), **hidden-pane
behaviour** (`render_suppressed`, `animation_suppressed`, `polling_paused`,
`correctness_poll_only`, `fully_quiescent`), **visibility state**, **override
posture** (`not_overridable`, `user_override_session_only`,
`user_override_persistent`, `policy_blocked`, `admin_controlled`), and **recovery
state** (`not_in_recovery`, `staged_resume`, `awaiting_user_restore_power`,
`awaiting_reconnect`, `awaiting_admin_policy`, `recovered`).

## Postures (ordered)

| Rank | Posture | Claim-bearing |
| --- | --- | --- |
| 0 | `undeclared_badge` | no |
| 1 | `state_declared` | no |
| 2 | `qualified_low_power` | yes |
| 3 | `certified_low_power` | yes |

The effective posture of any row is the **lowest-ranked** of its published
ceiling and every fired narrowing reason's target.

## Dimensions (pillars)

Each row must prove all seven dimensions.

| Pillar | Proves |
| --- | --- |
| `efficiency_state_evidence` | The surface materializes an inspectable efficiency-state record with a named state and source-of-change. |
| `behavior_declaration` | The low-power state declares a concrete behaviour change rather than remaining a vague badge. |
| `hidden_work_suppression` | Hidden, occluded, or off-screen panes commit no render work and pause nonessential polling and animation. |
| `protected_path_preservation` | Active tasks, debug correctness, local save, navigation, and review authority stay protected under pressure. |
| `override_policy_awareness` | A user-overridable posture exposes an explicit, policy-aware override reference. |
| `recovery_staging` | When pressure clears, deferred work resumes in staged order rather than thrashing back at once. |
| `consumer_propagation` | The posture reaches every required publication surface so later low-power copy derives from one source of truth. |

## Narrowing reasons (claim-narrowing vocabulary)

Every reason is mechanically detectable; the effective posture is recomputed from
the firing reasons.

| Reason | Pillar | Narrows to |
| --- | --- | --- |
| `missing_efficiency_state_evidence` | efficiency_state_evidence | `undeclared_badge` |
| `vague_low_power_badge` | behavior_declaration | `undeclared_badge` |
| `unqualified_hidden_work_suppression` | hidden_work_suppression | `state_declared` |
| `protected_path_regression_under_pressure` | protected_path_preservation | `state_declared` |
| `override_not_policy_aware` | override_policy_awareness | `qualified_low_power` |
| `recovery_not_staged` | recovery_staging | `qualified_low_power` |
| `missing_consumer_propagation` | consumer_propagation | `qualified_low_power` |

## Certification states

| State | Certified | Holds promotion when claim-bearing |
| --- | --- | --- |
| `certified` | yes | no |
| `narrowed` | no | yes |
| `quarantined` | no | yes |

## Governance rows

| Row | Surface | Posture | Effective | State | Blocks |
| --- | --- | --- | --- | --- | --- |
| `eff.notebooks.thermal` | notebooks | `certified_low_power` | `certified_low_power` | certified | no |
| `eff.previews.battery_saver` | previews | `certified_low_power` | `certified_low_power` | certified | no |
| `eff.docs_browser.critical_battery` | docs_browser_panes | `qualified_low_power` | `qualified_low_power` | certified | no |
| `eff.traces.thermal` | traces | `qualified_low_power` | `qualified_low_power` | certified | no |
| `eff.pipelines.low_battery` | pipelines | `certified_low_power` | `certified_low_power` | certified | no |
| `eff.remote_sessions.protect_core` | remote_sessions | `qualified_low_power` | `qualified_low_power` | certified | no |
| `eff.support_exports.recovery` | support_exports | `certified_low_power` | `certified_low_power` | certified | no |
| `eff.companion_adjacent.badge` | companion_adjacent | `undeclared_badge` | `undeclared_badge` | quarantined | no |

The companion-adjacent badge row is a worked example of the guardrail: it shows a
"battery saver" badge with no materialized efficiency-state evidence and no
declared behaviour change, so it quarantines to `undeclared_badge`. It asserts no
low-power claim and is retained only for diagnosis; it does not hold promotion.

## Promotion gate

- **Recompute:** for each row, fire every narrowing reason whose condition holds
  over its inline evidence, set the effective posture to the lowest-ranked of the
  published ceiling and each fired reason's target, then derive the state.
- **Promotion gate:** promotion holds when any claim-bearing row's effective
  posture is below the posture it asserts. The current matrix resolves to
  **proceed**: no claim-bearing row is narrowed below its posture.

## Consumer bindings

| Consumer | Projection | What it ingests |
| --- | --- | --- |
| `release_promotion` | `promotion_gate` | Certification states, fired reasons, effective postures, and the promotion verdict. |
| `release_packet` | `release_binding` | Each row's declared certification state and effective posture, which must equal the recompute. |
| `support_export` | `redaction_safe_projection` | States, reasons, labels, and bound refs only — never raw logs or provider payloads. |
| `docs_help` | `low_power_vocabulary_projection` | The closed efficiency vocabulary and each surface's effective posture and certification label. |

The recompute fixtures under
[`fixtures/efficiency/m5-efficiency-governance/`](../../fixtures/efficiency/m5-efficiency-governance/)
prove each fail-closed narrowing path is detected automatically.
