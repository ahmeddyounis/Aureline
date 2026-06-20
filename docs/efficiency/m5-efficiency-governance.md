# M5 efficiency-state governance

This is the normative policy companion for the canonical M5 efficiency-state,
battery-or-thermal, and hidden-pane render-suppression matrix. The matrix is the
**single source of truth** for low-power and thermal behaviour across every M5
surface; later status, help, support, and release surfaces consume it instead of
cloning low-power prose.

- Machine-readable matrix: [`artifacts/efficiency/m5-efficiency-governance.json`](../../artifacts/efficiency/m5-efficiency-governance.json)
- Human-readable rendering: [`artifacts/efficiency/m5-efficiency-governance.md`](../../artifacts/efficiency/m5-efficiency-governance.md)
- Boundary schema: [`schemas/efficiency/m5-efficiency-governance.schema.json`](../../schemas/efficiency/m5-efficiency-governance.schema.json)
- Recompute fixtures: [`fixtures/efficiency/m5-efficiency-governance/`](../../fixtures/efficiency/m5-efficiency-governance/)
- Enforcing gate: [`ci/check_m5_efficiency_governance.py`](../../ci/check_m5_efficiency_governance.py)
- Regenerator: [`tools/regenerate_m5_efficiency_governance.py`](../../tools/regenerate_m5_efficiency_governance.py)
- Shell binding: [`crates/aureline-shell/src/efficiency/governance/`](../../crates/aureline-shell/src/efficiency/governance/)

## Why this matrix exists

M5 ships notebooks, traces, previews, docs/browser panes, pipelines, remote
sessions, support exports, and companion-adjacent views. Without one typed
efficiency-state contract, each surface could invent its own low-power wording,
keep a hidden pane polling or rendering alive off-screen, or shed the wrong work
first under battery or thermal pressure. The matrix freezes the contract so:

- efficiency states are **canonical and inspectable**;
- hidden or off-screen panes **suppress rendering, polling, and nonessential
  animation** before any protected path degrades;
- active tasks, debug correctness, local save, navigation, and review authority
  stay **protected**;
- overrides are **explicit and policy-aware**;
- diagnostics, docs, and release packets all derive from the **same**
  efficiency-state vocabulary and evidence.

## Closed vocabulary

The matrix is the frozen registry of seven closed vocabularies. They mirror the
shell efficiency runtime tokens and MUST NOT drift from them; the conformance
test in the shell binding fails closed if they do.

| Vocabulary | Owns |
| --- | --- |
| `efficiency_state` | `Nominal`, `EfficiencyAware`, `ThermalConstrained`, `ProtectCore`, `Recovery` |
| `source_of_change` | the battery, thermal, low-power, frame-miss, policy, and pressure-cleared signals that drive a state |
| `throttled_subsystem` | the shed-work families (AI warmup, prefetch, uploads, animation, indexing, extension polling, preview refresh, graph enrichment, remote helpers) |
| `hidden_pane_behavior` | `render_suppressed`, `animation_suppressed`, `polling_paused`, `correctness_poll_only`, `fully_quiescent` |
| `visibility_state` | the focused/background/occluded/hidden/collapsed/detached pane states |
| `override_posture` | `not_overridable`, `user_override_session_only`, `user_override_persistent`, `policy_blocked`, `admin_controlled` |
| `recovery_state` | `not_in_recovery`, `staged_resume`, `awaiting_user_restore_power`, `awaiting_reconnect`, `awaiting_admin_policy`, `recovered` |

## Postures and narrowing

Each surface row declares a **posture** (its published low-power claim ceiling)
and carries inline evidence. The gate recomputes, per row, which **narrowing
reasons** fire and narrows the effective posture to the lowest-ranked target.

A posture is **claim-bearing** at `qualified_low_power` and `certified_low_power`.
The two lower postures (`undeclared_badge`, `state_declared`) assert no
publishable low-power claim.

Claim narrowing MUST bind to the following, all mechanically detected:

- **`missing_efficiency_state_evidence`** — no materialized efficiency-state
  evidence, state, or source-of-change → quarantine to `undeclared_badge`.
- **`vague_low_power_badge`** — a low-power state with no declared behaviour
  change (the guardrail: a `battery saver` or `thermal mode` badge MUST declare
  what changes) → quarantine to `undeclared_badge`.
- **`unqualified_hidden_work_suppression`** — a hidden or off-screen pane that
  cannot prove qualified render/poll suppression → narrow to `state_declared`.
- **`protected_path_regression_under_pressure`** — a protected interaction
  regressed under pressure → narrow to `state_declared`.
- **`override_not_policy_aware`** — a user-overridable posture without an
  explicit, policy-aware override reference → narrow to `qualified_low_power`.
- **`recovery_not_staged`** — recovery applies but is not staged → narrow to
  `qualified_low_power`.
- **`missing_consumer_propagation`** — the posture does not reach every required
  release, support, docs, and help surface → narrow to `qualified_low_power`.

## Certification states and the promotion gate

| State | Meaning |
| --- | --- |
| `certified` | every dimension is clean; effective posture equals the published ceiling |
| `narrowed` | at least one reason fired; effective posture is below the ceiling |
| `quarantined` | narrowed to the undeclared-badge floor; asserts no claim, retained for diagnosis |

The gate holds promotion (`hold`) when any **claim-bearing** row's effective
posture is below the posture it asserts. A `narrowed` or `quarantined`
non-claim-bearing row never holds promotion — it is the honest record of a
surface that has not yet earned a low-power claim. The current matrix resolves to
**proceed**.

## Consumers (one source of truth)

Release promotion, release packets, support export, and docs/help all bind to the
matrix:

- **release_promotion** ingests the certification states, fired reasons, effective
  postures, and the promotion verdict.
- **release_packet** ingests each row's declared certification state and effective
  posture, which must equal the recompute.
- **support_export** ingests states, reasons, labels, and bound refs only — never
  raw logs, machine labels, or provider payloads.
- **docs_help** ingests the closed efficiency vocabulary and each surface's
  effective posture and certification label.

Later M5 low-power copy MUST quote this matrix's vocabulary and per-surface
postures rather than re-describing low-power behaviour locally. The
[publication-ingestion register](publication-ingestion.md) makes that mechanical:
it binds docs, in-product help, the About surface, service-health, the
policy-or-admin surface, and support exports to each matrix row and fails closed
when a surface clones prose or renders a stronger claim than the row's narrowed
effective posture.

## Indexing and enforcement

The matrix cites the canonical M5 evidence index
(`artifacts/release/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json`)
via its `evidence_index_ref`, and the gate
[`ci/check_m5_efficiency_governance.py`](../../ci/check_m5_efficiency_governance.py)
— wired through
[`.github/workflows/check_m5_efficiency_governance.yml`](../../.github/workflows/check_m5_efficiency_governance.yml)
— enforces it on every change so promotion tooling can mechanically detect any
underqualified low-power row.

## Changing the matrix

Edit the row inputs and metadata in
[`tools/regenerate_m5_efficiency_governance.py`](../../tools/regenerate_m5_efficiency_governance.py),
then run:

```sh
python3 tools/regenerate_m5_efficiency_governance.py
python3 ci/check_m5_efficiency_governance.py --repo-root .
cargo test -p aureline-shell --lib efficiency::governance
```

The regenerator recomputes every derived field with the **same** engine the gate
uses, so the checked-in matrix can never disagree with the validator. When the
shell efficiency vocabulary changes, update the closed vocabulary and re-run the
conformance test so the matrix stays bound to what ships.
