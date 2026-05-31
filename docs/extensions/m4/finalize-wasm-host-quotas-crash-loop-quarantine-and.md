# Finalize Wasm host quotas, crash-loop quarantine, and restart-budget governance

**Status:** Stable extension-runtime governance lane — implemented in `crates/aureline-extensions`.

## Goal

Promote the extension host *supervision* contract (quota pressure, crash-loop quarantine, restart budget) into the **stable line**. Every claimed stable ecosystem row carries one canonical, checked-in governance truth: the published Wasm host quota axes (with their declared bound, observed peak, fail-closed enforcement state, and pressure), the crash-loop window with its disable/quarantine thresholds and trip state, the restart-budget snapshot, a derived quarantine posture, and an active-contribution inspector. The **stability qualification** that truth is allowed to claim is derived, not asserted: when the host can no longer back a `stable` governance claim, the visible tier is **automatically narrowed below Stable** (`beta`, `preview`, or `withdrawn`) with machine-readable reasons. Install review, the runtime inspector, the quarantine flow, diagnostics, and support exports ingest this packet instead of inventing a generic "extension stopped working" string.

## Design principles

1. **No unbounded quota** — Every Wasm host quota axis publishes a declared bound, an observed peak, and an enforcement state. An axis that is `bounded == false` (or whose enforcement is `unenforceable_refused`) is an unbounded quota and is **withdrawn** from the stable line — it can never ride a stable claim.
2. **No unbounded restart** — A `restart_budget_unbounded_refused` restart posture is withdrawn. A stable claim must use a bounded posture (`no_restart_attempted`, `one_warm_restart_under_budget`, `exponential_backoff_bounded`) and keep its attempts under budget.
3. **Crash-loop quarantine is governed, not silent** — The crash-loop window names its `disable_threshold` and `quarantine_threshold`; an open window or a tripped disable narrows to `preview`, and a tripped quarantine is `withdrawn`. The state is validated against the distinct-failure counters so it cannot drift.
4. **No prose/catalog-only stable claim** — A `stable` governance tier must be `enforcement_backed`; a `catalog_asserted_only` basis narrows below Stable.
5. **Quarantine posture is derived** — The quarantine state (`none_nominal`, `throttled`, `disabled_until_next_session`, `quarantined`) is re-derived from the narrowing reasons, so a stored packet cannot claim it is healthy while its evidence says otherwise. A non-nominal posture must surface on a user-facing surface and, when disabling/quarantining, cite a `quarantine_rule:` trigger.
6. **Active-contribution inspector stays attributable** — Each contribution always carries source package, runtime class, execution locus, trust tier, used permissions, and last-known-good host — even when quarantined, bridged, or downgraded.
7. **Downgraded-host banner** — Any physical host degradation (unbounded/fail-closed/breached quota, crash-loop trip, restart exhaustion, quarantined trust tier, non-runnable lifecycle, non-nominal contribution) raises a banner that names the most-severe reason and the last-known-good host.

## Record kinds

| Record kind | Purpose |
|---|---|
| `stable_wasm_host_governance_packet` | Top-level packet consumed by install review, the runtime inspector, the quarantine flow, diagnostics, support export, docs/help, and release packets. |
| `stable_wasm_host_governance_identity` | Governed runtime contract ref, pinned governance contract version, source package, publisher trust tier, lifecycle state. |
| `stable_wasm_host_governance_runtime_class_declaration` | Published runtime class and execution locus. |
| `stable_wasm_host_quota_axis` | One quota axis: declared bound ref, observed peak ref, enforcement state, pressure, bounded flag. |
| `stable_crash_loop_governance` | Crash-loop window, distinct failures, disable/quarantine thresholds, state, trigger rule. |
| `stable_restart_budget_governance` | Restart posture, attempts used, attempts remaining. |
| `stable_quarantine_posture` | Derived quarantine state, recovery precondition, trigger rule, visibility surface, blocks-activation flag. |
| `stable_governed_contribution_entry` | Per-contribution attribution that survives quarantine/bridge/downgrade. |
| `stable_wasm_host_governance_qualification_claim` | Claimed tier, effective tier after the posture is applied, support claim, narrowing reasons. |
| `stable_wasm_host_governance_downgraded_host_banner` | Whether a downgraded-host banner must display, why, and the last-known-good host. |
| `stable_wasm_host_governance_inspection` | Compact projection for CLI and inspector surfaces. |
| `stable_wasm_host_governance_support_export` | Metadata-safe support/partner export row. |

## Closed vocabularies

### Quota axes
`linear_memory`, `execution_fuel`, `table_elements`, `instance_count`, `epoch_deadline`, `host_call_egress`

### Quota enforcement states
`enforced_as_published`, `fail_closed_downgraded`, `unenforceable_refused`

### Quota pressure classes
`nominal`, `approaching_limit`, `soft_breach`, `hard_breach`, `not_applicable`

### Crash-loop states
`nominal`, `window_open`, `disable_tripped`, `quarantine_tripped`

### Restart postures
`no_restart_attempted`, `one_warm_restart_under_budget`, `exponential_backoff_bounded`, `restart_budget_unbounded_refused`

### Quarantine states (derived)
`none_nominal`, `throttled`, `disabled_until_next_session`, `quarantined`

### Stability tiers
`stable`, `beta`, `preview`, `withdrawn` (only `stable` is a stable governance claim)

### Narrowing reasons
`governance_version_mismatch`, `catalog_only_trust_not_enforcement_backed`, `quota_enforcement_unverified`, `quota_axis_soft_breached`, `quota_axis_hard_breached`, `quota_unbounded_refused`, `crash_loop_window_breached`, `crash_loop_quarantine_active`, `restart_budget_exhausted`, `restart_posture_unbounded`, `contribution_not_nominal`, `trust_tier_quarantined`, `lifecycle_not_runnable`, `attribution_incomplete`

### Tier buckets
- **withdrawn**: `quota_unbounded_refused`, `crash_loop_quarantine_active`, `restart_posture_unbounded`, `lifecycle_not_runnable`
- **preview**: `governance_version_mismatch`, `catalog_only_trust_not_enforcement_backed`, `quota_axis_hard_breached`, `crash_loop_window_breached`, `restart_budget_exhausted`, `contribution_not_nominal`, `trust_tier_quarantined`, `attribution_incomplete`
- **beta**: `quota_enforcement_unverified`, `quota_axis_soft_breached`

## Key invariants

- A `stable` effective tier requires the published governance version, an `enforcement_backed` basis, every quota axis bounded and `enforced_as_published` with no soft/hard breach, a `nominal` crash-loop window, a bounded and non-exhausted restart budget, a `none_nominal` quarantine posture, a non-quarantined trust tier, a runnable lifecycle, every contribution `running_nominal`, and complete attribution.
- The runtime-class → execution-locus mapping is enforced.
- The crash-loop state is validated against the distinct-failure counters and the disable/quarantine thresholds (`disable_threshold <= quarantine_threshold`, both ≥ 1).
- The effective tier, support claim, downgrade reasons, quarantine state, and downgraded-host banner are re-derived from the posture at validation time, so a stored packet cannot drift from its evidence.
- `allows_unbounded_quota`, `allows_unbounded_restart`, `allows_catalog_only_trust`, and `allows_silent_quarantine` are forced `false` and validated.

## File locations

| Artifact | Path |
|---|---|
| Implementation | `crates/aureline-extensions/src/finalize_wasm_host_quotas_crash_loop_quarantine_and/mod.rs` |
| Schema | `schemas/extensions/stable_wasm_host_governance.schema.json` |
| Fixtures | `fixtures/extensions/m4/finalize-wasm-host-quotas-crash-loop-quarantine-and/` |
| Tests | `crates/aureline-extensions/src/finalize_wasm_host_quotas_crash_loop_quarantine_and/tests.rs` |
| Dump example | `crates/aureline-extensions/examples/dump_stable_wasm_host_governance_records.rs` |
| Proof packet | `artifacts/extensions/m4/finalize-wasm-host-quotas-crash-loop-quarantine-and.md` |

## Integration with existing lanes

- Sits above the extension host *supervision* contract (`crates/aureline-extensions/src/supervision/`): that module owns the per-evaluation beta decision (axis pressure → throttle/disable/quarantine, restart-budget snapshot, crash-loop window counters); this module owns the published, stable governance truth and its stability qualification. The `identity.runtime_contract_ref` points back at the runtime v1 beta admission contract (`runtime_v1_beta:` prefix).
- Reuses the runtime-class, execution-locus, trust-tier, lifecycle, and contribution-host-state vocabularies that the stable runtime-ABI lane carries, so support and review surfaces share one vocabulary. Crash-loop / quarantine-rule semantics mirror `artifacts/extensions/quarantine_rules.yaml` and `runtime_budget_rows.yaml`.

## Verification

```bash
cargo test -p aureline-extensions finalize_wasm_host
cargo run -q -p aureline-extensions --example dump_stable_wasm_host_governance_records -- validate
```

Materialized packets for every fixture validate against `schemas/extensions/stable_wasm_host_governance.schema.json` (checked with a Draft 2020-12 validator).
