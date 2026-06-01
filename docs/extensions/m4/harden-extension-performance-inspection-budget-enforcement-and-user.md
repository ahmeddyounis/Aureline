# Harden extension performance inspection, budget enforcement, and user-visible cost explanation

**Status:** Stable performance-budget lane — implemented in `crates/aureline-extensions`.

## Goal

Make an extension's runtime cost **inspectable**, **budget-enforced**, and **explained to the user** on the stable ecosystem line. Every claimed stable row carries one canonical, checked-in performance truth: the inspected worst-case measurement (budget axis, measured p50 / p95, sample count, benchmark-lab trace and corpus refs, freshness, attested flag), the budget enforcement against the published p50 / p95 ceilings (status, enforcement mode, threshold-adjustment posture), the waiver hook that backs any intentional threshold tightening / narrowing / relaxation, the user-visible cost explanation (cost class, dominant factor, explained flag), the permission posture, compatibility, and install/revocation/mirror posture. The **stability qualification** that truth is allowed to claim is derived, not asserted: when the evidence can no longer back a `stable` performance claim, the visible tier is **automatically narrowed below Stable** (`beta`, `preview`, or `withdrawn`) with machine-readable reasons. Marketplace result and detail rows, install review, the extension detail view, the performance inspector, diagnostics, support exports, the CLI inspector, and release packets ingest this packet instead of cloning a "Fast" badge.

## Design principles

1. **No unbounded activation cost** — The budget status carries `within_budget` / `over_budget` / `unbounded` / `not_measured`. An `unbounded` status withdraws the row (`allows_unbounded_activation_cost == false`); `over_budget` narrows to `beta`; `not_measured` narrows to `preview`. An `unbounded` user-visible cost also withdraws the row.
2. **The published p50/p95 budget is protected and enforced** — The enforcement record pins the published p50 / p95 ceilings and the enforcement mode. A budget published without host enforcement (`unenforced`) narrows to `preview` — a number with nothing behind it is catalog-only trust; an `advisory` mode narrows to `beta`. The measured p50 / p95 and the budget status are cross-checked numerically at validation time, so a "within budget" claim whose measurement exceeds the ceiling is rejected outright.
3. **Inspection is from attested benchmark traces** — The measurement carries the benchmark-lab trace ref, corpus metadata ref, sample count, freshness, and an attested flag. A `stale` measurement narrows to `beta`; an `expired` or `not_measured` measurement narrows to `preview`; an unattested trace fails attribution and narrows to `preview`.
4. **Threshold moves require a waiver hook** — When a publisher or admin intentionally tightens, narrows, or relaxes a threshold, a waiver hook must be recorded. A `relaxed` threshold without an active waiver narrows to `preview` (with a banner); a `tightened` / `narrowed` threshold with no recorded waiver hook narrows to `preview`; an `expired` waiver narrows to `beta`; a `revoked` waiver narrows to `preview` (with a banner).
5. **The cost is explained to the user** — The cost explanation carries a headline cost class, the dominant cost factor, and whether a user-readable explanation is actually attached. An unexplained cost narrows to `preview` — a stable row must let a user understand what they are paying for.
6. **No ambient extension privilege** — The permission posture carries declared and effective refs plus a `widened` flag. A permission set widened beyond the declared manifest withdraws the row (`allows_ambient_extension_privilege == false`).
7. **No catalog-only trust** — A `stable` tier must be `evidence_backed`; a `catalog_asserted_only` basis narrows below Stable (`allows_catalog_only_trust == false`).
8. **Revocation, mirrorability, and install scope are visible** — A quarantined / revoked revocation posture withdraws; an advisory posture or a not-mirrorable row narrows to `beta`; an undisclosed install scope narrows to `preview`.
9. **No drift** — The effective tier, downgrade verdict, narrowing reasons, and banner are re-derived from the posture at validation time, so a stored packet cannot drift from its truth.

## Record kinds

| Record kind | Purpose |
|---|---|
| `stable_performance_budget_packet` | Top-level packet consumed by marketplace result / detail rows, install review, the extension detail view, the performance inspector, diagnostics, support export, docs/help, release packets, and the CLI inspector. |
| `stable_performance_budget_identity` | Performance-profile ref, row identity, package identity, pinned profile version, publisher namespace, benchmark-evidence ref, publisher trust tier, lifecycle state. |
| `stable_performance_measurement` | Worst-case budget axis, measured p50 / p95, sample count, benchmark-lab trace and corpus refs, freshness, attested flag. |
| `stable_performance_budget_enforcement` | Budget status, published p50 / p95 ceilings, enforcement mode, threshold-adjustment posture, budget-profile ref. |
| `stable_performance_budget_waiver` | Waiver state, ref, and authority backing any intentional threshold adjustment. |
| `stable_performance_cost_explanation` | Headline cost class, dominant cost factor, explained flag, explanation ref. |
| `stable_performance_permission_posture` | Declared / effective permission refs, no-widening flag, re-consent flag. |
| `stable_performance_compatibility` | Compatibility label, scorecard ref, verified flag. |
| `stable_performance_install_posture` | Install scope + disclosure, revocation posture, mirrorability, rollback support. |
| `stable_performance_qualification_claim` | Claimed tier, effective tier after the posture is applied, support claim, narrowing reasons. |
| `stable_performance_downgraded_banner` | Whether a row-review banner must display and why. |
| `stable_performance_budget_inspection` | Compact projection (with the numeric p50 / p95 measured vs published) for CLI and dashboard surfaces. |
| `stable_performance_budget_support_export` | Metadata-safe support / partner / mirror export row that preserves the measured-vs-published cost so a reviewer can see why a row is or is not within budget. |

## Narrowing buckets

| Tier | Triggered by (examples) |
|---|---|
| **withdrawn** | non-runnable lifecycle, unbounded budget, unbounded cost, permission widened beyond the declared manifest, unsupported compatibility, quarantined/revoked revocation posture |
| **preview** | unpublished profile version, catalog-asserted basis, quarantined trust tier, not-measured budget, expired / not-measured measurement, unenforced budget, relaxed threshold without active waiver, tightened/narrowed threshold with no waiver hook, revoked waiver, unexplained cost, unverified compatibility, undisclosed install scope, incomplete attribution |
| **beta** | over-budget cost, stale measurement, advisory enforcement, expired waiver, parity-limited compatibility, advisory revocation posture, not mirrorable |

## Canonical fixtures

Under `fixtures/extensions/m4/harden-extension-performance-inspection-budget-enforcement-and-user/`:

- `within_budget_language_tools_stable.json` — cold-start within the enforced published budget, fresh attested trace, explained cost. Holds **Stable**.
- `tightened_threshold_with_waiver_stable.json` — an intentionally tightened threshold backed by a recorded waiver hook; still **Stable**.
- `over_budget_narrows_to_beta.json` — measured cold-start over the published budget; narrows to `beta`.
- `unenforced_budget_narrows_to_preview.json` — a published budget with no host enforcement; narrows to `preview`.
- `unexplained_cost_narrows_to_preview.json` — within budget but no user-visible cost explanation attached; narrows to `preview`.
- `relaxed_threshold_without_waiver_narrows_to_preview.json` — a relaxed budget with no active waiver; narrows to `preview` with a review banner.
- `unbounded_budget_withdrawn.json` — an unbounded background-CPU cost; the row is `withdrawn` with a banner.
- `widened_permission_withdrawn.json` — permissions widened beyond the declared manifest; the row is `withdrawn` with a banner.

## How to verify

```bash
cargo test -p aureline-extensions harden_extension_performance
cargo run -q -p aureline-extensions --example dump_stable_performance_budget_records -- validate
```

Materialized packets for every fixture validate against
[`schemas/extensions/stable_performance_budget.schema.json`](../../../schemas/extensions/stable_performance_budget.schema.json)
(checked with a Draft 2020-12 validator).
