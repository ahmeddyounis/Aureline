# M5 min-width guards: editor minimum, compare fallback, no unusable narrow pane

Generated from the seeded packet in
[`crate::m5_min_width_guards`](../../crates/aureline-shell/src/m5_min_width_guards/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_min_width_guards -- markdown > \
  artifacts/shell/m5-min-width-guards.md
```

- Packet id: `m5-min-width-guards:stable:0001`
- Source schema ref: `schemas/shell/m5-min-width-guards.schema.json`
- Certifies matrix packet: `m5-shell-zone-matrix:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Absolute min useful size: `320x200` px
- Standard min useful size: `480x320` px
- Rows certified: 10
- Green (fully-guarded): 6
- Yellow (auto-narrowed): 4
- Red (blocked): 0
- All rows publishable: `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-30T00:00:00Z`

## Guard rows

| Surface | Status | Qualification | Min size | Enforcement | Fallback | Strategy | Status continuity | Waiver |
| ------- | ------ | ------------- | -------- | ----------- | -------- | -------- | ----------------- | ------ |
| Notebook editor / cell surface | `green` | `stable` | `560x360` | `min_useful_size_enforced` | `safe_fallback_before_unusable_split` | `tabbed_compare` | `identity_breadcrumbs_status_preserved` | — |
| Tabular data grid surface | `green` | `stable` | `560x360` | `min_useful_size_enforced` | `safe_fallback_before_unusable_split` | `tabbed_compare` | `identity_breadcrumbs_status_preserved` | — |
| Profiler / performance surface | `yellow` | `stable` | `360x240` | `disclosed_reduced_minimum` | `safe_fallback_before_unusable_split` | `staged_peek` | `identity_breadcrumbs_status_preserved` | — |
| Pipeline / workflow graph surface | `green` | `stable` | `560x360` | `min_useful_size_enforced` | `safe_fallback_before_unusable_split` | `tabbed_compare` | `identity_breadcrumbs_status_preserved` | — |
| Documentation reader surface | `green` | `stable` | `560x360` | `min_useful_size_enforced` | `safe_fallback_before_unusable_split` | `tabbed_compare` | `identity_breadcrumbs_status_preserved` | — |
| Preview surface (render, diff, media) | `green` | `stable` | `560x360` | `min_useful_size_enforced` | `safe_fallback_before_unusable_split` | `staged_peek` | `identity_breadcrumbs_status_preserved` | — |
| Review / change-request surface | `green` | `stable` | `560x360` | `min_useful_size_enforced` | `safe_fallback_before_unusable_split` | `tabbed_compare` | `identity_breadcrumbs_status_preserved` | — |
| Incident / operations-response surface | `yellow` | `beta` | `520x340` | `min_useful_size_enforced` | `safe_fallback_before_unusable_split` | `tabbed_compare` | `identity_breadcrumbs_status_preserved` | — |
| Companion assistant surface | `yellow` | `beta` | `520x340` | `min_useful_size_enforced` | `safe_fallback_before_unusable_split` | `sequential_disclosure` | `disclosed_status_relocation` | `waiver:companion-status-relocation:0001` |
| Operator / control-plane surface | `yellow` | `beta` | `520x340` | `min_useful_size_enforced` | `disclosed_fallback_narrowing` | `staged_peek` | `identity_breadcrumbs_status_preserved` | — |

## Per-class compare plan

| Surface | Compact | Standard | Expanded |
| ------- | ------- | -------- | -------- |
| Notebook editor / cell surface | `tabbed_compare` | `side_by_side_split` | `side_by_side_split` |
| Tabular data grid surface | `tabbed_compare` | `side_by_side_split` | `side_by_side_split` |
| Profiler / performance surface | `staged_peek` | `tabbed_compare` | `tabbed_compare` |
| Pipeline / workflow graph surface | `tabbed_compare` | `side_by_side_split` | `side_by_side_split` |
| Documentation reader surface | `tabbed_compare` | `side_by_side_split` | `side_by_side_split` |
| Preview surface (render, diff, media) | `staged_peek` | `side_by_side_split` | `side_by_side_split` |
| Review / change-request surface | `tabbed_compare` | `side_by_side_split` | `side_by_side_split` |
| Incident / operations-response surface | `tabbed_compare` | `side_by_side_split` | `side_by_side_split` |
| Companion assistant surface | `sequential_disclosure` | `staged_peek` | `staged_peek` |
| Operator / control-plane surface | `staged_peek` | `tabbed_compare` | `tabbed_compare` |

## Auto-narrowed rows

- `profiler` (`yellow`) — Under compact width the profiler discloses a reduced-but-still-usable minimum useful size for its capture readout, staying above the absolute floor; the row is narrowed below green while the pane stays usable.
- `incident` (`yellow`) — The incident surface is qualified at Beta in the frozen shell-zone matrix; its min-size enforcement and compare fallback are fully guarded but the claim is narrowed below Stable and disclosed.
- `companion` (`yellow`) — The companion surface is qualified at Beta; under compact width it cannot host a side-by-side compare and relocates its recovery-critical connection status to a disclosed, waivered still-visible affordance while preserving breadcrumbs and the active object identity.
- `operator` (`yellow`) — The operator surface is qualified at Beta; under compact width its compare fallback trims a secondary control panel's optional detail in a disclosed way before an unusable split could occur, so the claim is narrowed and disclosed.

## Exact guard causes

- `profiler` — `upstream_dependency_narrowed` (disclosed: `true`) — Under compact width the editor discloses a reduced-but-still-usable minimum useful size while staying above the absolute floor.
- `incident` — `upstream_dependency_narrowed` (disclosed: `true`) — Frozen shell-zone matrix qualifies this family at `beta`, below a Stable shell claim.
- `companion` — `upstream_dependency_narrowed` (disclosed: `true`) — Frozen shell-zone matrix qualifies this family at `beta`, below a Stable shell claim.
- `companion` — `upstream_dependency_narrowed` (disclosed: `true`) — Recovery-critical status is relocated to a disclosed, waivered still-visible affordance while the compare fallback is active.
- `operator` — `upstream_dependency_narrowed` (disclosed: `true`) — Frozen shell-zone matrix qualifies this family at `beta`, below a Stable shell claim.
- `operator` — `upstream_dependency_narrowed` (disclosed: `true`) — The compare fallback trims a secondary pane's optional detail in a disclosed way before an unusable split could occur.

## Active waivers

- `waiver:companion-status-relocation:0001` (`companion`, owner: Companion surface owner, expires `2026-09-30T00:00:00Z`) — Under compact width the companion assistant cannot host a side-by-side compare and falls back to sequential disclosure; its recovery-critical connection status relocates from the inline header to a disclosed, still-visible status affordance while the shared status-strip contract is unified in the next sync. Breadcrumbs and the active object identity stay visible and the relocation is disclosed, never silent.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_min_width_guards -- validate
cargo test -p aureline-shell --test m5_min_width_guards_fixtures
```
