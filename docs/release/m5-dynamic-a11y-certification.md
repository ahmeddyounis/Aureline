# M5 dynamic-surface assistive-tech certification

This page is the human entry point for the **dynamic-surface assistive-tech
certification**: the capstone lane that certifies, for every claimed M5
custom-rendered dynamic surface, that its assistive-technology contract is
currently proven across the six governed proof dimensions, and that any surface
whose screen-reader, focus-return, live-announcement, or non-visual proof has
gone stale, missing, or non-conformant is **auto-narrowed** before it can keep a
public Stable claim.

The product treats OS accessibility bridges, screen-reader label models,
live-announcement grammar, focus-return rules, dense-surface non-visual
summaries, and visual-adaptation (zoom/contrast/motion) parity as governed
contracts, not QA polish. Each of those concepts already has its own frozen
contract and proof lane; the [frozen dynamic-surface matrix][matrix] enumerates
them as the canonical accessibility object model, and the [AT diagnostics][diag]
materialize per-surface health. This certification is the final row over all of
them: it consumes the diagnostics report and certifies whether each claimed
surface may keep a Stable assistive-tech claim.

## What it certifies

One certification row per claimed dynamic surface, each carrying a derived
green/yellow/red traffic-light status across the six governed proof dimensions:

| Surface family | Custom-rendered surface |
| -------------- | ----------------------- |
| `shell_region` | Custom-rendered shell zones and landmark regions |
| `editor_canvas` | Custom-rendered editor content canvas |
| `terminal_canvas` | Terminal / log canvas |
| `dense_collection` | Dense list / table / data-grid collection |
| `notebook_cell` | Notebook cell (input + output) |
| `data_cell` | Data-surface cell |
| `review_diff` | Review / diff hunk surface |
| `overlay_sheet` | Durable overlay / sheet / modal surface |

| Proof dimension | What it certifies | Backing proof |
| --------------- | ----------------- | ------------- |
| `bridge_health` | OS accessibility-bridge health and semantic-node coverage | bridge / surface-descriptor proof |
| `announcement_coverage` | Dynamic live-announcement coverage and coalescing discipline | live-announcement proof |
| `focus_return` | Focus-return safety across async updates and overlay teardown | focus-return proof |
| `non_visual_summaries` | Dense-surface non-visual summaries and label/role fidelity | non-visual summary proof |
| `visual_adaptation_parity` | High-zoom / high-contrast / reduced-motion parity | AT diagnostics |
| `stale_proof_downgrade` | The stale-proof downgrade rules that auto-narrow on stale evidence | dynamic-event coverage proof |

## How the status is derived (auto-narrowing)

The status is **derived, never asserted**. The builder recomputes it from each
surface's six dimensions and its active waivers:

- **green — certified** — every dimension is current and conformant; the surface
  keeps its Stable claim and the release gate is `certified_promote`.
- **yellow — limited / retest-pending** — a *disclosed* narrowing. A dimension
  carries a disclosed conformance narrowing (`limited`), or a dimension's proof
  has fallen out of its freshness SLO (`retest_pending`). The surface
  auto-narrows below Stable (gate `auto_narrowed`) but keeps shipping at the
  reduced claim, with the exact stale-proof cause named per dimension.
- **red — degraded** — an unhandled blocking regression (a bridge, announcement,
  focus, non-visual, or zoom/contrast regression) or missing proof. Without a
  waiver the surface is **blocked** from Stable promotion (gate `blocked`,
  effective claim `held`) and named in the release packet rather than left
  invisible.

A blocking problem can be accepted under an **active waiver** scoped to a single
dimension. The waiver is disclosed with its accountable owner, expiry, and the
reduced claim it accepts; the surface then ships `auto_narrowed` at the waived
claim while its true status stays red. Waivers never re-grant Stable.

## The release gate

The release/public-truth automation reads the packet-level release gate:

- `blocks_stable_promotion` is `true` when **any** surface is blocked, so a
  screen-reader / focus-return / live-announcement regression on a custom dynamic
  surface can never keep Stable green silently.
- A surface that lacks current proof for any dimension **auto-narrows before
  Stable promotion** rather than implying silent screen-reader completeness.
- The gate names the blocked, auto-narrowed, certified, and waived surfaces.

## The dashboard

The compact [dashboard][dashboard] is the published green/yellow/red scoreboard.
It names the certified, limited, retest-pending, and degraded surfaces, the
surfaces that auto-narrowed or are blocked, the surfaces carrying active waivers,
the active waiver ids, and the **exact stale-proof / regression causes** across
all surfaces. It carries the same packet id as the certification packet so
consumers can resolve the full rows.

## Consumers

Release center, support exports, docs/help, onboarding, presentation, the
stable-claim matrix, and the shell / editor / notebook / data / review surfaces
consume this certification directly rather than reproducing assistive-tech
qualification by hand.

## Where the truth lives

- Boundary schema: [`schemas/a11y/m5-dynamic-a11y-certification.schema.json`][schema]
- Dashboard schema: [`schemas/a11y/m5-dynamic-a11y-dashboard.schema.json`][dashboard-schema]
- Certification support export: `artifacts/release/m5-dynamic-a11y-certification/support_export.json`
- Certification Markdown proof: `artifacts/release/m5-dynamic-a11y-certification/certification-proof.md`
- Published dashboard: `artifacts/a11y/m5-dynamic-a11y-dashboard.json`
- Drill fixtures: `fixtures/a11y/m5-dynamic-a11y-certification/`

## How to regenerate

The headless emitter is the only mint-from-truth path:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_dynamic_a11y_certification -- support-export
cargo run -q -p aureline-shell --bin aureline_shell_m5_dynamic_a11y_certification -- dashboard
cargo run -q -p aureline-shell --bin aureline_shell_m5_dynamic_a11y_certification -- markdown
cargo run -q -p aureline-shell --bin aureline_shell_m5_dynamic_a11y_certification -- validate
```

The inline tests assert the checked-in support export and dashboard match the
seed builder, so any drift fails `cargo test -p aureline-shell certification`.

[matrix]: ../a11y/m5-dynamic-surface-a11y.md
[diag]: ../a11y/m5-dynamic-a11y-diagnostics.md
[schema]: ../../schemas/a11y/m5-dynamic-a11y-certification.schema.json
[dashboard-schema]: ../../schemas/a11y/m5-dynamic-a11y-dashboard.schema.json
[dashboard]: ../../artifacts/a11y/m5-dynamic-a11y-dashboard.json
