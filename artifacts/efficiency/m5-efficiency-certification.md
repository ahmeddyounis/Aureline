# M5 efficiency certification — human-readable rendering

Human-readable rendering of the canonical M5 efficiency certification proof
packet. The machine-readable truth is at
[`artifacts/efficiency/m5-efficiency-proof-packet.json`](./m5-efficiency-proof-packet.json),
validated by
[`schemas/efficiency/m5-efficiency-certification.schema.json`](../../schemas/efficiency/m5-efficiency-certification.schema.json).
The normative policy companion is
[`docs/efficiency/m5-efficiency-certification.md`](../../docs/efficiency/m5-efficiency-certification.md).
The lane aligns every surface-family row with the
[M5 efficiency-state governance matrix](./m5-efficiency-governance.json).

The proof packet turns efficiency-state and hidden-work-shedding behaviour into a
**promotion-grade claim** instead of a best-effort runtime behaviour. For every
**claimed laptop-or-desktop profile** and every **long-running M5 surface family**
it runs a fixed drill set against current evidence — energy/thermal lab traces,
hidden-pane render audits, and active-session pressure postures — and recomputes,
per row, the narrowing reasons that fire, the narrowed effective posture, and the
certification state. The shell efficiency certification lane
([`crates/aureline-shell/src/efficiency/certification/`](../../crates/aureline-shell/src/efficiency/certification/))
owns the closed vocabulary and seeds the packet from the same efficiency-state
objects the rest of the contract uses; an integration test rebuilds the packet so
the artifact can never drift.

## Drill set (verification classes)

Each claimed subject must survive every required drill against current evidence.

| Drill | Proves | Evidence |
| --- | --- | --- |
| `efficiency_state_behavior` | The subject materializes inspectable efficiency-state transitions with a named state, source-of-change, and a recorded reason for every reduced surface. | energy/thermal trace |
| `hidden_work_suppression` | Hidden, occluded, and off-screen panes commit no render work and pause nonessential polling and animation. | hidden-pane audit |
| `protected_path_preservation` | Active tasks, debug correctness, local save, navigation, and review authority stay protected under pressure. | energy/thermal trace |
| `session_aware_shedding` | Optional assists shed before any live run's correctness or authority regresses, and a material downgrade is warned about first. | session-pressure posture |
| `staged_recovery` | When pressure clears, deferred work resumes in staged order rather than thrashing back at once. | energy/thermal trace |

## Evidence freshness rule

Bound evidence is **current** within 30 days of the packet `as_of`. Older evidence
is **stale**; an absent required-drill binding is **partial** when other evidence
exists for the subject and **missing** otherwise. Stale, partial, and missing
evidence each fire a narrowing reason — this is the guardrail that stops a claim
coasting on one good manual test while its current evidence is stale or
incomplete.

## Postures (ordered)

| Rank | Posture | Claim-bearing |
| --- | --- | --- |
| 0 | `undeclared_badge` | no |
| 1 | `state_declared` | no |
| 2 | `qualified_low_power` | yes |
| 3 | `certified_low_power` | yes |

A row's effective posture is the **lowest-ranked** of its published ceiling and
every fired narrowing reason's floor.

## Narrowing reasons (auto-detected)

| Reason | Narrows to |
| --- | --- |
| `missing_efficiency_evidence` | `undeclared_badge` |
| `stale_efficiency_evidence` | `state_declared` |
| `partial_evidence_coverage` | `state_declared` |
| `unqualified_hidden_work_suppression` | `state_declared` |
| `protected_path_regression_under_pressure` | `state_declared` |
| `session_shed_order_violation` | `state_declared` |
| `recovery_not_staged` | `qualified_low_power` |

## Certification states

| State | Certified | Holds promotion when claim-bearing |
| --- | --- | --- |
| `certified` | yes | no |
| `narrowed` | no | yes |
| `quarantined` | no | (asserts no claim) |

## Certification rows

| Subject | Axis | Claimed states | Ceiling | Effective | State | Blocks |
| --- | --- | --- | --- | --- | --- | --- |
| `battery_ultrabook` | laptop or desktop profile | EfficiencyAware, Recovery | `certified_low_power` | `certified_low_power` | certified | no |
| `thermal_workstation` | laptop or desktop profile | ThermalConstrained, Recovery | `certified_low_power` | `certified_low_power` | certified | no |
| `policy_managed_fleet` | laptop or desktop profile | EfficiencyAware | `qualified_low_power` | `qualified_low_power` | certified | no |
| `critical_battery_field` | laptop or desktop profile | EfficiencyAware, ProtectCore, Recovery | `certified_low_power` | `certified_low_power` | certified | no |
| `notebooks` | m5 surface family | ThermalConstrained, Recovery | `certified_low_power` | `certified_low_power` | certified | no |
| `previews` | m5 surface family | EfficiencyAware, Recovery | `certified_low_power` | `certified_low_power` | certified | no |
| `docs_browser_panes` | m5 surface family | ProtectCore, Recovery | `qualified_low_power` | `qualified_low_power` | certified | no |
| `traces` | m5 surface family | ThermalConstrained, Recovery | `qualified_low_power` | `qualified_low_power` | certified | no |
| `pipelines` | m5 surface family | EfficiencyAware, ProtectCore, Recovery | `certified_low_power` | `certified_low_power` | certified | no |
| `remote_sessions` | m5 surface family | ProtectCore, Recovery | `qualified_low_power` | `qualified_low_power` | certified | no |
| `support_exports` | m5 surface family | EfficiencyAware, Recovery | `certified_low_power` | `certified_low_power` | certified | no |
| `companion_adjacent` | m5 surface family | — | `undeclared_badge` | `undeclared_badge` | quarantined | no |

The companion-adjacent surface family is a worked example of the guardrail: it
shows a "battery saver" badge with no materialized efficiency-state evidence, so
its `missing_efficiency_evidence` reason quarantines it to `undeclared_badge`. It
asserts no low-power claim and is retained only for diagnosis; it does not hold
promotion.

## Promotion gate

- **Recompute:** for each row, grade every required drill's evidence freshness,
  run the drill predicate on current evidence, fire the implied narrowing reasons,
  set the effective posture to the lowest of the published ceiling and each fired
  reason's floor, then derive the certification state.
- **Promotion gate:** promotion holds when any row whose published ceiling is
  claim-bearing narrows below that ceiling because a required drill failed or its
  evidence is stale, partial, or missing. The current packet resolves to
  **proceed**: no claim-bearing row is narrowed below its ceiling.

## Consumers

| Consumer | Projection | What it ingests |
| --- | --- | --- |
| `release` | `promotion_gate` | The promotion decision, blocking rows, and each row's certification state and effective posture. |
| `support` | `redaction_safe_rows` | Drill outcomes, freshness grades, and effective postures only — never raw traces or content. |
| `docs` | `certified_claim_vocabulary` | Each subject's effective posture and certification label, derived from one packet. |
| `help` | `certified_claim_vocabulary` | The same effective postures and certification labels the docs surface renders. |

Recompute and the negative drills that prove each fail-closed narrowing path are
enforced by [`ci/check_efficiency_certification.py`](../../ci/check_efficiency_certification.py).
