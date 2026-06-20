# Per-surface low-power disclosures

When battery, thermal, or policy pressure reduces optional work, the affected
product surfaces get less fresh: indexing lags, the assistant warms up more slowly, rich
previews and docs sync stop re-rendering, the marketplace catalog stops
refreshing, and optional uploads wait. Without a disclosure, a person can only
infer that from silence — and silence reads as "broken". The per-surface
disclosure makes the reduction explicit: it says what still works, what is
delayed, and how to inspect or override.

It builds on the canonical [efficiency state](./efficiency-state.md). Each
disclosure reuses the same typed `EfficiencyState`, `WorkloadBudgetDecision`,
override posture, recovery state, and frozen governance binding the status,
diagnostics, and support surfaces use, so the low-power story stays one object
rather than per-surface prose.

## The surfaces it covers

`aureline_shell::efficiency::disclosures` models one disclosure per affected
surface. Each [`DisclosureSurface`] maps to the canonical `WorkloadFamily` whose
budget decision governs its freshness, so the disclosed action and visible state
come from the frozen policy:

| Surface | Owner | Governing budget |
| --- | --- | --- |
| `paused_indexing` | Indexer | indexing refresh |
| `ai_warmups` | AI runtime | AI warmups |
| `rich_preview_refresh` | Preview | preview refresh |
| `docs_sync` | Docs/help | rich-content refresh |
| `marketplace_refresh` | Extensions | extension background refresh |
| `optional_uploads` | Sync/mirror | uploads and replication |

Docs sync shares the rich-content-refresh budget with preview, and marketplace
refresh shares the extension-background-refresh budget, so neither invents a
parallel low-power policy.

## What each disclosure says

Every [`SurfaceDisclosure`] answers the three questions the contract requires:

- **What still works now** — the protected edit, search, save, or review path the
  adaptation never narrows. Indexing reduces, but "editing, saving, and search
  across open files stay fully responsive". This is what keeps a reduced surface
  from reading as broken.
- **What is delayed** — the specific freshness or assist that is reduced, phrased
  so nothing protected is implied to have stopped.
- **How to inspect or override** — every disclosure carries the open-details
  command, and an explicit override affordance that is offered **only where
  policy allows it**.

A `freshness_class` (`reduced_cadence`, `deferred`, `paused`, `resuming`) lets a
surface render a stale/slow badge instead of an error, and `is_degraded_not_error`
is always true.

## Overrides are explicit and policy-aware

The override affordance derives from the active `OverridePosture`, so it never
silently collapses distinct causes into one:

| Cause | Posture | Override |
| --- | --- | --- |
| OS battery saver / low battery / user low-power mode | `user_override_session_only` | offered for the session |
| Thermal pressure | `not_overridable` | not offered (physical pressure) |
| Admin/local policy cap | `policy_blocked` | not offered; names the blocking policy |
| Critical-battery protect-core | `not_overridable` | not offered until pressure clears |
| Recovery | `not_in_recovery` → staged resume | resumes in stages automatically |

## Guardrails

- **No banner for unchanged behavior.** A disclosure is emitted only when a
  surface's behavior *materially changed*. Surfaces still running within budget
  are listed in `unaffected_surface_tokens` and show nothing. Under `Nominal`,
  the set is empty.
- **Not toast-only.** Each disclosure carries a [`DisclosurePlacement`] anchored
  to the surface's own persistent inline status affordance — never a dismissible
  toast that would lose long-lived low-power truth, and never the typing hot
  path.
- **Protected paths stay unblocked.** `protected_interactions_preserved` and
  `durability_preserved` ride along on the set, and
  `preserves_protected_path_truth()` asserts every disclosure keeps a protected
  path explicitly working.

## Consistency with the other surfaces

`EfficiencySurfaceDisclosures::from_snapshot` re-derives the set from a canonical
[`EfficiencyStateSnapshot`], so a disclosure, the status pill, the diagnostics
row, and the support export resolve the same state, cause, override posture, and
governance matrix. Wherever a disclosed surface's governing family also appears
in the snapshot's affected subsystems, the disclosed action and visible state
match the support export exactly.

## Sources of truth

- Code: `crates/aureline-shell/src/efficiency/disclosures/`
- Schema: `schemas/efficiency/low-power-disclosures.schema.json`
- Fixtures: `fixtures/efficiency/disclosures/`
- Conformance dump: `cargo run -p aureline-shell --example dump_efficiency_disclosures`

[`DisclosureSurface`]: ../../crates/aureline-shell/src/efficiency/disclosures/mod.rs
[`SurfaceDisclosure`]: ../../crates/aureline-shell/src/efficiency/disclosures/mod.rs
[`DisclosurePlacement`]: ../../crates/aureline-shell/src/efficiency/disclosures/mod.rs
[`EfficiencyStateSnapshot`]: ../../crates/aureline-shell/src/efficiency/mod.rs
