# Artifact: Finalize Wasm host quotas, crash-loop quarantine, and restart-budget governance

**Task:** Promote the extension host supervision contract into the stable line — publish the Wasm host quota axes (declared bound, observed peak, fail-closed enforcement state, pressure, bounded flag), the crash-loop quarantine window with its disable/quarantine thresholds and trip state, and the restart-budget governance snapshot — derive the quarantine posture and the stability qualification with automatic narrowing below Stable, and expose a downgraded-host banner and an active-contribution inspector.
**Status:** Implemented
**Verification class:** Conformance / interoperability suite + Security / privacy review + Docs validation + Release evidence review

## Summary

This lane binds the governed runtime-contract identity, the published runtime class, the Wasm host quota axes, the crash-loop governance window, the restart-budget snapshot, the derived quarantine posture, and the active-contribution inspector entries into one validated packet, and derives the stability qualification it may claim. A `stable` governance claim is only allowed when the row pins the published governance contract version, is enforcement-backed, holds every quota axis bounded and enforced as published with no breach, keeps its crash-loop window nominal, keeps its restart budget bounded and not exhausted, keeps its quarantine posture nominal, keeps its publisher trust tier out of quarantine, stays on a runnable lifecycle, keeps every contribution nominal, and is fully attributed. An unbounded quota or an unbounded restart posture can never ride the stable line — both are withdrawn — and any physical host degradation raises a downgraded-host banner. When any condition fails the visible tier is automatically narrowed below Stable (to `beta`, `preview`, or `withdrawn`) with machine-readable reasons. The checked-in packet is canonical: install review, the runtime inspector, the quarantine flow, diagnostics, and support exports ingest it instead of inventing a generic "extension stopped working" string.

## What changed

- New Rust module: `crates/aureline-extensions/src/finalize_wasm_host_quotas_crash_loop_quarantine_and/mod.rs` (+ `tests.rs`)
- Re-exported from `crates/aureline-extensions/src/lib.rs`
- New schema: `schemas/extensions/stable_wasm_host_governance.schema.json`
- New fixtures: `fixtures/extensions/m4/finalize-wasm-host-quotas-crash-loop-quarantine-and/`
  - `wasm_capability_sandbox_stable_current.json` — a Wasm capability-sandbox formatter that bounds and enforces every quota axis, keeps its crash-loop window and restart budget clear, and holds Stable.
  - `quota_soft_breach_and_fail_closed_narrows_to_beta.json` — a linter whose memory soft-breaches and whose fuel meter fails closed; it narrows to `beta`, throttles, and raises a downgraded-host banner.
  - `crash_loop_window_breach_narrows_to_preview.json` — a debugger with an open crash-loop window; it narrows to `preview`, disables until the next session, and cites a quarantine rule.
  - `crash_loop_quarantine_tripped_withdraws_the_claim.json` — a sync extension whose crash loop tripped a quarantine (and whose lifecycle is quarantined and restart budget exhausted); the claim is `withdrawn` while the contribution stays attributable.
  - `unbounded_quota_withdraws_the_claim.json` — an external-host compiler whose host-call egress quota cannot be bounded; the unbounded quota withdraws the claim and raises a banner.
  - `catalog_asserted_restart_exhausted_narrows_to_preview.json` — a packer claiming Stable on catalog assertion with an exhausted restart budget; it narrows to `preview`.
- New dump example: `crates/aureline-extensions/examples/dump_stable_wasm_host_governance_records.rs`
- New docs: `docs/extensions/m4/finalize-wasm-host-quotas-crash-loop-quarantine-and.md`

## Acceptance criteria

- [x] The checked-in implementation, fixtures, and proof packet are current and self-describing (schema, fixtures, and docs reference one another) rather than ad hoc notes. (`stable_wasm_host_governance.schema.json`, fixtures dir, this packet)
- [x] Any surface still lacking stable qualification is automatically narrowed below Stable, with machine-readable reasons, instead of inheriting an adjacent green row. (`quota_soft_breach_and_fail_closed_narrows_to_beta.json`, `crash_loop_window_breach_narrows_to_preview.json`, `crash_loop_quarantine_tripped_withdraws_the_claim.json`, `unbounded_quota_withdraws_the_claim.json`, `catalog_asserted_restart_exhausted_narrows_to_preview.json`)
- [x] Users and admins can inspect permissions, compatibility (governance version + runtime class), activation cost (quota axes + restart budget), lifecycle label, publisher provenance, and rollback/revocation state (quarantine posture + lifecycle) for the touched ecosystem row. (`stable_wasm_host_governance_inspection`, `stable_governed_contribution_entry`, `stable_quarantine_posture`)
- [x] Install review, runtime inspector, quarantine flow, diagnostics, and support export all name the enforced quota/crash-loop/restart posture or a narrower-than-stable downgrade. (`consumer_surfaces`, `stable_wasm_host_governance_support_export`)
- [x] Audits prove stable surfaces, diagnostics, and support exports preserve contribution attribution and downgraded-host truth instead of collapsing into a generic extension badge. (`every_fixture_builds_validates_and_matches_expectations`, `support_export_quotes_governance_truth`)

## Guardrails honored

- No unbounded activation cost / quota: a quota axis with `bounded == false` or `unenforceable_refused` enforcement withdraws the claim (`unbounded_quota_withdraws_and_raises_banner`), and `allows_unbounded_quota` is forced `false` and surfaced on the inspection row.
- No unbounded restart: a `restart_budget_unbounded_refused` posture withdraws the claim (`unbounded_restart_posture_is_withdrawn`); `allows_unbounded_restart` is forced `false`.
- No catalog-only trust: a `catalog_asserted_only` basis can never back a stable claim (`catalog_asserted_restart_exhausted_narrows_to_preview.json`, `no_catalog_only_stable_claim`).
- Crash-loop quarantine is never silent: a tripped quarantine is withdrawn and a disabled/quarantined posture must surface and cite a `quarantine_rule:` trigger (`crash_loop_quarantine_withdraws_the_claim`, `crash_loop_window_open_narrows_to_preview_and_disables`).
- A narrower stable claim is published rather than papered over: the effective tier, support claim, downgrade reasons, quarantine state, and banner are re-derived from the posture at validation time, so the packet cannot drift (`tampered_effective_tier_is_rejected_on_validate`).

## How to verify

```bash
cargo test -p aureline-extensions finalize_wasm_host
cargo run -q -p aureline-extensions --example dump_stable_wasm_host_governance_records -- validate
```

Materialized packets for every fixture validate against `schemas/extensions/stable_wasm_host_governance.schema.json` (checked with a Draft 2020-12 validator). 19 module tests pass (208 total in the crate).

## Risks / follow-ups

- The quota enforcement states, pressure classes, crash-loop state, and restart-budget counters are supplied by the producing host. When a wall-clock quota meter and a live crash-loop probe land, the narrowing should be derived from the live probe versus the published bound rather than a producer-supplied class.
- Runtime-class, trust, lifecycle, and contribution-host-state vocabularies are closed string sets shared with the beta supervision/runtime lanes and the stable runtime-ABI lane; when those crates stabilize typed enums, these should be narrowed to share them rather than re-declared as strings.
- The crash-loop window and restart budget consume producer-supplied counters; a later revision should source them directly from the supervision contract this packet governs instead of accepting a parallel copy.
