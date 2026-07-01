# M5 desktop-profile certification contract

This lane is the **desktop-profile capstone** on top of the frozen
[M5 shell-zone, responsive-class, and multi-window continuity matrix](m5_shell_zone_matrix_contract.md).
Six sibling capstones certify the matrix's promises one *dimension* at a time (slot occupancy,
responsive collapse, min-width guards, multi-window parity, owning-window routing, and
window-lifecycle safety). This lane certifies all four continuity truths **together** on every
claimed M5 **desktop profile**: for each profile it certifies that, across every claimed surface
family the matrix freezes, the live shell keeps four promises:

- **shell-zone integrity** — every claimed surface attaches only to a declared shell slot; no
  surface invents a private slot under this profile;
- **adaptive-layout continuity** — responsive collapse never changes task identity, hides
  critical state instead of overflowing it, or forces an unusable narrow pane;
- **multi-window truth** — every window preserves workspace-global trust, remote target,
  deployment profile, and recovery state while density/focus/layout stay local;
- **owning-window routing** — routed dialogs, notifications, and approvals return to the owning
  window and object without focus theft, orphaning, or a wrong-window reopen.

The lane exists so that M5 can honestly claim desktop maturity: the new notebook, data, review,
preview, docs, operator, and incident surfaces cannot invent their own slot, collapse, or
multi-window behavior under compact widths, zoom, mixed-DPI, secondary displays, or a
dependency-missing restore.

## Claimed desktop profiles

The certification covers exactly the six desktop profiles Aureline claims, and refuses to ship
if any is missing:

- `compact_desktop` — Compact desktop (narrow width / zoom)
- `standard_desktop` — Standard desktop (default width)
- `expanded_desktop` — Expanded desktop (wide display)
- `mixed_dpi` — Mixed-DPI (per-display scale factors)
- `multi_monitor` — Multi-monitor (secondary displays / topology change)
- `dependency_missing_restore` — Dependency-missing restore (crash / restart)

## Evaluated surface families

Every profile row evaluates all ten claimed surface families the matrix freezes — `notebook`,
`data_grid`, `profiler`, `pipeline`, `docs`, `preview`, `review`, `incident`, `companion`, and
`operator` — pulled straight from the frozen matrix. A row that evaluates fewer regresses into a
partial, single-surface view and **blocks**.

## Per-profile truth-dimension axes

Each row is certified across the four continuity truth dimensions:

- **shell-zone integrity** — `all_surfaces_in_declared_slots` (green), a disclosed
  `disclosed_slot_fallback_narrowing` where a surface falls back to its declared *fallback* slot
  because a dependency is unavailable (yellow), or `private_slot_drift_detected` (red: a surface
  attached outside any declared slot).
- **adaptive layout** — `identity_stable_no_unusable_pane` (green), a disclosed
  `disclosed_collapse_narrowing` where collapse takes a docked→sheet/overflow narrowing while
  preserving identity and the reopen path (yellow), or `identity_lost_or_unusable_pane` (red:
  collapse changed identity, hid critical state, or forced an unusable narrow pane).
- **multi-window truth** — `all_truths_preserved_layout_local` (green), a disclosed
  `disclosed_truth_projection_narrowing` where a workspace truth is projected in a reduced form
  until a dependency is restored while staying visible in every window (yellow), or
  `workspace_truth_diverged_across_windows` (red: a workspace-global truth diverged across
  windows).
- **owning-window routing** — `routes_to_owning_object_no_focus_theft` (green), a disclosed
  `disclosed_routing_relocation` where a routed action from a closed window is relocated to a
  disclosed, waivered still-visible prompt in the primary window (yellow), or
  `routing_lost_focus_theft_or_orphan` (red: a routed action was lost to focus theft, orphaning,
  or a wrong-window reopen).

## Derived status and the certification lint

The green/yellow/red status is **derived, never asserted**. A row drops to `yellow` when a
surface falls back to a disclosed but still-declared slot, collapse takes a disclosed
identity-preserving narrowing, a truth is projected in a disclosed reduced form, or a routed
action is deferred to a disclosed waivered relocation. It drops to `red` when a surface invents a
private slot, collapse changes identity or forces an unusable pane, workspace truth diverges
across windows, a routed action is lost, or the profile fails to evaluate every claimed surface
family. The evaluated-family completeness check is the lint that prevents a profile audit from
silently regressing into a partial view — the Rust validator in
`crates/aureline-shell/src/m5_desktop_profile_certification` is the authoritative gate.

A narrowed (non-green) row must disclose a reason; a `disclosed_routing_relocation` narrowing
must additionally carry an active, matching, unexpired waiver.

## Records

- **Certification packet** — the full set of rows with derived per-row status, aggregate
  green/yellow/red counts, active waivers, the exact profile causes, and the blocking findings
  the lane refuses to ship with.
- **Certification dashboard** — a light projection the shell / windowing / layout / status
  automation reads to auto-narrow a claimed surface's desktop profile when its certification
  falls out of policy.
- **Support export** — the packet plus dashboard plus stable case ids (packet id, matrix ref,
  build id, each profile, each waiver id).

The records carry only stable ids, closed vocabulary, counts, refs, and short labels — never raw
URLs, raw local paths, raw usernames, raw hostnames, tokens, or credentials.

## Artifacts

The headless emitter
(`cargo run -q -p aureline-shell --bin aureline_shell_m5_desktop_profile_certification`) is the
only mint-from-truth path for:

- `artifacts/release/m5-desktop-profile-certification-proof/packet.json`
- `artifacts/release/m5-desktop-profile-certification-proof/dashboard.json`
- `artifacts/release/m5-desktop-profile-certification-proof/support_export.json`
- `artifacts/release/m5-desktop-profile-certification-proof/matrix.csv`
- `artifacts/shell/m5-desktop-profile-certification.md` (this report's rendered companion)
- `fixtures/ui/m5-desktop-profile-certification/packet.json` (and `dashboard.json`,
  `support_export.json`, `compact.txt`)

The boundary schema is
[`schemas/shell/m5-desktop-profile-certification.schema.json`](../../schemas/shell/m5-desktop-profile-certification.schema.json).

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_desktop_profile_certification -- validate
cargo test -p aureline-shell --test m5_desktop_profile_certification_fixtures
cargo test -p aureline-shell m5_desktop_profile_certification
```

Regenerate the artifacts after any change to the seed:

```sh
BIN="cargo run -q -p aureline-shell --bin aureline_shell_m5_desktop_profile_certification --"
$BIN packet         > artifacts/release/m5-desktop-profile-certification-proof/packet.json
$BIN dashboard      > artifacts/release/m5-desktop-profile-certification-proof/dashboard.json
$BIN support-export > artifacts/release/m5-desktop-profile-certification-proof/support_export.json
$BIN csv            > artifacts/release/m5-desktop-profile-certification-proof/matrix.csv
$BIN markdown       > artifacts/shell/m5-desktop-profile-certification.md
$BIN packet         > fixtures/ui/m5-desktop-profile-certification/packet.json
$BIN dashboard      > fixtures/ui/m5-desktop-profile-certification/dashboard.json
$BIN support-export > fixtures/ui/m5-desktop-profile-certification/support_export.json
$BIN compact        > fixtures/ui/m5-desktop-profile-certification/compact.txt
```
