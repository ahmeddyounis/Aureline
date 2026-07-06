# M5 Runtime-Boundary & Repair Component Matrix (Design QA)

> Task: M05-852 · Batch B100 · Wave W100 / R100.
> One shared matrix for Design, schema, QA, and release owners covering the
> reusable terminal-tab, remote-target-pill, environment-status-strip,
> toolchain-pin-row, presence-avatar-stack, and repair-action-card components.

This is the design-owner face of the frozen matrix. The authoritative gate is the
Rust validator and the checked support export; this document is the human-readable
agreement so no surface reinvents host/runtime/repair status language locally.

- **Contract doc:** [`docs/components/m5_runtime_boundary_components_contract.md`](../../docs/components/m5_runtime_boundary_components_contract.md)
- **Boundary schema:** [`schemas/ui/m5-runtime-boundary-components.schema.json`](../../schemas/ui/m5-runtime-boundary-components.schema.json)
- **Support export (canonical):** [`artifacts/release/m5-runtime-boundary-proof/support_export.json`](../release/m5-runtime-boundary-proof/support_export.json)
- **Matrix CSV:** [`artifacts/release/m5-runtime-boundary-proof/matrix.csv`](../release/m5-runtime-boundary-proof/matrix.csv)

## Track invariant

Session title, host boundary, shell-integration quality, resolved
runtime/toolchain, winning scope/source, collaboration role/follow state, repair
blast radius, and reversibility class remain **explicit everywhere** a user runs,
shares, switches, repairs, or exports execution state.

## The six governed components

| Component | Canonical zone | What it must always show | Family-specific vocabulary |
| --- | --- | --- | --- |
| Terminal tab / header | Bottom panel | Session title, host boundary, shell-integration quality, live-vs-restored | shell-integration qualities (5), session-liveness states (5) |
| Remote target pill | Title / context bar | Host boundary class + live connection state | host-boundary classes (6), connection states (5) |
| Environment status strip | Status bar | Which runtime source won and why | runtime source classes (6) |
| Toolchain pin row | Right inspector | Why the toolchain won + its pin state | toolchain source classes (6), pin states (5) |
| Presence avatar stack | Title / context bar | Each participant's role + follow state | collaboration roles (5), follow states (5) |
| Repair action card | Transient overlay | Blast radius + reversibility before approval | blast radii (5), reversibility classes (5) |

Each component also binds: the responsive classes it must survive
(compact/standard/expanded desktop), the window classes it keeps continuity across
(primary/secondary/floating/companion), the ten claimed M5 surface families, its
mandatory labels, its non-visual accessibility routes, its consumer surfaces, and
the downgrade triggers that narrow it below its claim.

## Mandatory labels (every component)

`identity` (session title / what it is), `state` (typed state), `keyboard_route`
(non-visual reach). Components additionally carry `boundary`, `resolved_source`,
and/or `reversibility` where relevant.

## Hard invariants (all MUST be false)

- **No masked boundary** — remote/container/managed never shown as local.
- **No live/restored conflation** — a restored transcript never reads as a live
  session.
- **No private status grammar** — every component reuses this vocabulary.
- **No overstated reversibility / dropped audit truth** — a repair card never
  claims more reversibility than it has, and audit/support truth stays on the
  primary surface.

## Auto-narrowing (downgrade triggers)

If a component hides shell-integration quality, leaves live-vs-restored ambiguous,
masks a host boundary, shows a stale connection, leaves the winning runtime source
unexplained, hides a toolchain pin conflict, masks a collaboration role, leaves the
follow state ambiguous, understates repair blast radius, overstates reversibility,
loses audit truth off the primary surface, or ships stale proof — it narrows below
its Stable claim rather than presenting a full-truth label.

## QA / release checklist

1. Every claimed consumer points to **one** canonical component contract (this
   matrix), not a locally reworded host/runtime/repair status.
2. `cargo test -p aureline-shell --lib freeze_the_m5_terminal_tab` is green
   (31 tests): family coverage, per-family lints, invariant blockers, checked
   support export bit-for-bit matches the seed, and narrowed fixtures round-trip.
3. Support export carries no raw URLs, paths, hostnames, tokens, or credentials.
4. Narrowed fixtures keep every component visible while narrowing one family
   (presence stack → Beta; repair card → Preview).
