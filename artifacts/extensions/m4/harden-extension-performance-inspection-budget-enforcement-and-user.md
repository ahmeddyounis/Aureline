# Artifact: Harden extension performance inspection, budget enforcement, and user-visible cost explanation

**Task:** Make an extension's runtime cost inspectable, budget-enforced, and explained to the user on the stable line — binding the inspected worst-case measurement (budget axis, measured p50 / p95, benchmark-lab trace, corpus metadata, freshness, attested flag), the budget enforcement against the published p50 / p95 ceilings (status, enforcement mode, threshold-adjustment posture), the waiver hook for any intentional threshold tightening / narrowing / relaxation, the user-visible cost explanation, the permission posture, compatibility, and install/revocation/mirror posture into one validated packet, and derive the stability qualification with automatic narrowing below Stable.

**Status:** Implemented
**Verification class:** Conformance / interoperability suite + Security / privacy review + Docs validation + Release evidence review

## Summary

This lane binds, for a stable ecosystem performance row, the identity (performance-profile descriptor ref, row identity, package identity, pinned performance-budget profile version, publisher namespace, pinned benchmark-evidence ref, publisher trust tier, lifecycle state), the **performance measurement** (worst-case budget axis `activation_cold_start` / `activation_warm_start` / `main_thread_block` / `background_cpu` / `memory_footprint` / `io_throughput` / `host_rpc_latency` / `render_frame`, measured p50 / p95, sample count, benchmark-lab trace and corpus refs, measurement freshness, trace-attested flag), the **budget enforcement** against the published p50 / p95 ceilings (budget status `within_budget` / `over_budget` / `unbounded` / `not_measured`, enforcement mode `enforced` / `advisory` / `unenforced`, threshold-adjustment posture `unchanged` / `tightened` / `narrowed` / `relaxed`), the **waiver hook** (state, ref, authority) that backs any intentional threshold move, the **user-visible cost explanation** (cost class, dominant factor, explained flag, explanation ref), the permission posture (declared / effective refs, widened flag), the compatibility label, and the install posture into one validated packet, and derives the stability qualification it may claim. A `stable` performance-budget claim is only allowed when the row pins the published profile version, is evidence-backed, keeps its trust tier out of quarantine, stays runnable, keeps its cost bounded and within the published p50 / p95 budget, keeps the budget enforced, keeps a fresh and attested benchmark measurement, carries an active waiver for any relaxed threshold and a recorded waiver hook for any tightened / narrowed threshold, explains its cost to the user, never widens permissions, keeps verified non-parity-limited compatibility, discloses its install scope, keeps a clean revocation posture, stays mirrorable, and is fully attributed.

When any condition fails the visible tier is automatically narrowed below Stable with machine-readable reasons. An unbounded budget or cost, a non-runnable lifecycle, a permission widened beyond the declared manifest, an unsupported compatibility, or a quarantined / revoked revocation posture withdraws the row; an unenforced budget, a relaxed threshold without a waiver, a missing waiver hook, an unexplained cost, an undisclosed install scope, an expired / not-measured measurement, or an unattested trace narrows to `preview`; an over-budget cost, a stale measurement, an advisory enforcement mode, an expired waiver, a parity-limited compatibility, an advisory revocation posture, or a not-mirrorable row narrows to `beta`. The measured p50 / p95 and the budget status are cross-checked numerically so the inspected cost and the budget status can never contradict. The checked-in packet is canonical: marketplace result / detail rows, install review, the extension detail view, the performance inspector, diagnostics, support exports, the CLI inspector, and release packets ingest it instead of cloning a "Fast" badge.

## What changed

- New Rust module: `crates/aureline-extensions/src/harden_extension_performance_inspection_budget_enforcement_and_user/mod.rs` (+ `tests.rs`)
- Re-exported from `crates/aureline-extensions/src/lib.rs`
- New schema: `schemas/extensions/stable_performance_budget.schema.json`
- New fixtures: `fixtures/extensions/m4/harden-extension-performance-inspection-budget-enforcement-and-user/`
  - `within_budget_language_tools_stable.json` — cold-start within the enforced published budget; holds Stable.
  - `tightened_threshold_with_waiver_stable.json` — an intentionally tightened, waiver-backed threshold; holds Stable.
  - `over_budget_narrows_to_beta.json` — measured cold-start over the published budget; narrows to `beta`.
  - `unenforced_budget_narrows_to_preview.json` — a published budget with no host enforcement; narrows to `preview`.
  - `unexplained_cost_narrows_to_preview.json` — within budget but no user-visible cost explanation; narrows to `preview`.
  - `relaxed_threshold_without_waiver_narrows_to_preview.json` — a relaxed budget with no active waiver; `preview` with a banner.
  - `unbounded_budget_withdrawn.json` — an unbounded background-CPU cost; `withdrawn` with a banner.
  - `widened_permission_withdrawn.json` — permissions widened beyond the declared manifest; `withdrawn` with a banner.
- New dump example: `crates/aureline-extensions/examples/dump_stable_performance_budget_records.rs`
- New docs: `docs/extensions/m4/harden-extension-performance-inspection-budget-enforcement-and-user.md`

## Acceptance criteria

- [x] The checked-in implementation, fixtures, and proof packet are current and self-describing (schema, fixtures, and docs reference one another) rather than ad hoc notes. (`stable_performance_budget.schema.json`, fixtures dir, this packet)
- [x] Any surface still lacking stable qualification is automatically narrowed below Stable, with machine-readable reasons, instead of inheriting an adjacent green row. (`over_budget_narrows_to_beta.json`, `unenforced_budget_narrows_to_preview.json`, `unexplained_cost_narrows_to_preview.json`, `relaxed_threshold_without_waiver_narrows_to_preview.json`, `unbounded_budget_withdrawn.json`, `widened_permission_withdrawn.json`)
- [x] Users and admins can inspect permissions (permission posture, declared/effective refs), compatibility range (compatibility label + scorecard ref), activation cost (measured p50 / p95 vs published budget, cost class), lifecycle label, publisher provenance (trust tier + performance profile), and rollback/revocation state (revocation posture + rollback support) for the touched ecosystem row. (`stable_performance_budget_inspection`, `stable_performance_measurement`, `stable_performance_budget_enforcement`, `stable_performance_cost_explanation`, `stable_performance_permission_posture`, `stable_performance_compatibility`, `stable_performance_install_posture`)
- [x] Conformance fixtures, activation-budget instrumentation, and publisher continuity packets make the ecosystem claims supportable and mirrorable on the M4 line. (`stable_performance_measurement` with benchmark trace + corpus refs, `stable_performance_budget_enforcement`, `mirrorability_class`, all eight fixtures)
- [x] Published p50/p95 budgets are protected with benchmark-lab traces, corpus metadata, and waiver hooks where thresholds are intentionally tightened or narrowed. (`measured_p50` / `measured_p95` vs `published_p50_budget` / `published_p95_budget` with a numeric cross-check; `benchmark_trace_ref` + `corpus_metadata_ref` + `trace_attested`; `stable_performance_budget_waiver` required for any `tightened` / `narrowed` / `relaxed` threshold)
- [x] One stable manifest / permission / lifecycle / compatibility vocabulary is shared across install review, runtime, mirror/manual import, disable/rollback, and revocation paths. (trust-tier, lifecycle, compatibility-label, install-scope, revocation-posture, mirrorability, stability-tier, and claim-basis vocabularies are the same closed string sets shared with the catalog-truth, lifecycle-flow, bridge-certification, and mirror-import stable lanes.)

## Guardrails honored

- No unbounded activation cost: an `unbounded` budget status or an `unbounded` cost class withdraws the row (`unbounded_budget_withdraws_the_row`, `unbounded_cost_withdraws_the_row`); `allows_unbounded_activation_cost` is pinned false. `over_budget` narrows to `beta`.
- No catalog-only trust: a `catalog_asserted_only` basis can never back a stable claim (`catalog_asserted_basis_cannot_back_stable`, `no_catalog_only_stable_claim`), and an `unenforced` published budget — a number with nothing behind it — narrows to `preview`; `allows_catalog_only_trust` is pinned false.
- No ambient extension privilege: a permission set widened beyond the declared manifest withdraws the row (`widened_permission_withdraws_the_row`); `allows_ambient_extension_privilege` is pinned false.
- No widened public scope from this row alone: the packet only narrows; it never promotes a row to a wider claim than the posture supports.
- A narrower stable claim is published rather than papered over: the effective tier, downgrade flag, reasons, and banner are re-derived from the posture at validation time, and the measured-vs-published budget is cross-checked, so the packet cannot drift.

## How to verify

```bash
cargo test -p aureline-extensions harden_extension_performance
cargo run -q -p aureline-extensions --example dump_stable_performance_budget_records -- validate
```

Materialized packets for every fixture validate against `schemas/extensions/stable_performance_budget.schema.json` (checked with a Draft 2020-12 validator).

## Risks / follow-ups

- The `budget_axis_class` vocabulary here is finer-grained than the frozen axis register in `artifacts/extensions/runtime_budget_rows.yaml` (`cold_activation`, `warm_activation`, `idle_polling`, `memory`, `egress`, …); a later revision should source the axis directly from that register's `axis_id` set rather than re-declaring a parallel closed string set, so the stable performance lane reads one budget register instead of minting per-lane axis names.
- The budget axis, measured p50 / p95, and published ceilings are summarized for a single worst-case surface; a later revision should carry a per-surface activation row so a reviewer can see exactly which surface drove the worst case.
- Numeric budgets are integers in the axis unit (ms or unit count); when the benchmark-lab registry exposes a typed unit + distribution model, these should be sourced from it directly rather than re-declared as bare integers here.
- The cost class is a producer-supplied closed string; a later revision could derive it from the measured-vs-published ratio so a "negligible" claim cannot be asserted over a heavy measurement.
- Trust-tier, lifecycle, compatibility-label, install-scope, revocation-posture, mirrorability, stability-tier, and claim-basis vocabularies are closed string sets shared with the catalog-truth, lifecycle-flow, bridge-certification, and mirror-import stable lanes; when those crates stabilize typed enums, these should be narrowed to share them rather than re-declared as strings.
- The waiver hook is carried by state + ref + authority; a later revision should resolve the bound waiver's own expiry and scope rather than accepting a producer-supplied `waiver_state_class`.
