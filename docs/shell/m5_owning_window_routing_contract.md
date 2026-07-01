# M5 owning-window routing contract

This lane is the **owning-window routing-continuity capstone** on top of the frozen
[M5 shell-zone, responsive-class, and multi-window continuity matrix](m5_shell_zone_matrix_contract.md).
Where the matrix *declares* the window classes each claimed M5 surface family may live in
(primary workspace, secondary detached, floating utility, companion overlay) and the
owning-window routing expectations it must honor (route dialogs, notifications, and
approvals back to the owning window and object, preserve the exact object anchor on return,
never steal focus from an unrelated window, and never orphan a routed action on detach or
close), this lane *certifies* that the live surface keeps those promises for real routed
actions: **permission sheets, trust prompts, destructive confirmations, and
publish/approval dialogs bind to the window that owns the authoritative object**, durable
notification reopen paths **land on the exact object or a truthful placeholder rather than a
generic home screen**, routed actions **never steal focus from a protected typing surface**
(degrading to a badge or activity-center row instead), and privacy-safe
**OS-notification summaries preserve one exact reopen path without bypassing in-app
review**.

The lane exists so that M5 can honestly claim desktop maturity: a routed approval always
finds its owning window and object rather than stealing focus or orphaning; a durable
notification reopens onto the exact object it names (or a truthful placeholder that keeps
its identity and reopen path) instead of dropping the user on a generic home screen; a
routed action never yanks focus away from someone mid-type; and an OS notification never
leaks sensitive content or lets an approval be actioned outside in-app review.

## Governed families

The routing proof covers exactly the ten families the matrix freezes, and refuses to ship
if any is missing:

- `notebook` — Notebook editor / cell surface
- `data_grid` — Tabular data grid surface
- `profiler` — Profiler / performance surface
- `pipeline` — Pipeline / workflow graph surface
- `docs` — Documentation reader surface
- `preview` — Preview surface (render, diff, media)
- `review` — Review / change-request surface
- `incident` — Incident / operations-response surface
- `companion` — Companion assistant surface
- `operator` — Operator / control-plane surface

## Owning-window routing expectations

Every row must declare all four owning-window routing expectations —
`route_to_owning_window_object`, `preserve_object_anchor_on_return`, `no_focus_theft`, and
`no_orphan_on_detach`. A row that fails to declare the full set blocks: some routed action
could otherwise steal focus or be orphaned.

## Per-window routed-action plan

Each row carries one **per-window routed-action plan** for each declared window class,
naming whether a routed action originating in that window binds to the owning object,
preserves typing focus, and keeps a single exact reopen path. The plans must:

- cover exactly the declared window classes — no window the family may live in is left
  uncertified and none is invented;
- **bind** a routed dialog, notification, or approval to the owning window and
  authoritative object;
- **preserve** focus on protected typing surfaces;
- keep a **single exact reopen path**.

## Per-family posture axes

Each row is certified across four posture axes:

- **dialog binding** — `bound_to_owning_window_object` (green), a disclosed
  `disclosed_binding_relocation` where a routed dialog or approval relocates to a disclosed,
  waivered still-visible affordance when its owning window is absent (yellow), or
  `binding_lost_or_orphaned` (red: a routed dialog or approval stole focus or was orphaned).
- **reopen continuity** — `reopens_exact_object_or_truthful_placeholder` (green), a
  disclosed `disclosed_placeholder_narrowing` where a durable reopen lands on a truthful
  placeholder that discloses a narrowed context while preserving identity and the single
  reopen path (yellow), or `lands_on_generic_shell` (red: a reopen dropped to a generic
  home/shell, losing the object identity and reopen path).
- **focus retention** — `no_focus_steal_on_typing` (green), a disclosed
  `disclosed_deferral_to_badge_or_center` where a routed action defers to a badge or
  activity-center row rather than stealing focus while a protected typing path is active
  (yellow), or `focus_stolen_from_typing` (red: a routed action pulled focus away from a
  protected typing surface).
- **OS-notification privacy** — `privacy_safe_summary_preserves_reopen` (green), a disclosed
  `disclosed_minimal_summary` where the OS notification discloses a narrowed minimal summary
  while preserving the single reopen path (yellow), or `leaks_content_or_bypasses_review`
  (red: the OS notification leaked sensitive content or bypassed in-app review).

## Derived status and the routing lint

The green/yellow/red status is **derived, never asserted**. A row drops to `yellow` when
its frozen qualification is below Stable, a routed dialog/approval relocates to a disclosed
waivered affordance, a durable reopen lands on a disclosed truthful placeholder, a routed
action defers to a disclosed badge or activity-center row, or an OS-notification summary
discloses a narrowed minimal projection. It drops to `red` when a routed dialog/approval is
lost to focus theft or orphaning, a reopen lands on a generic shell, a routed action steals
focus from a protected typing surface, an OS notification leaks content or bypasses in-app
review, a routing expectation is undeclared, or a per-window plan drops owning-object
binding, focus preservation, or the single reopen path. The routing-completeness and
per-window plan checks are the lint that prevents a later routing regression from shipping
as stable — the Rust validator in `crates/aureline-shell/src/m5_owning_window_routing` is
the authoritative gate.

A narrowed (non-green) row must disclose a reason; a `disclosed_binding_relocation`
narrowing must additionally carry an active, matching, unexpired waiver.

## Records

- **Routing packet** — the full set of rows with derived per-row status, aggregate
  green/yellow/red counts, active waivers, the exact routing causes, and the blocking
  findings the lane refuses to ship with.
- **Routing dashboard** — a light projection the shell / windowing / notification / release
  automation reads to auto-narrow a claimed surface when its routing proof falls out of
  policy.
- **Support export** — the packet plus dashboard plus stable case ids (packet id, matrix
  ref, build id, each family, each waiver id).

The records carry only stable ids, closed vocabulary, counts, refs, and short labels —
never raw URLs, raw local paths, raw usernames, raw hostnames, tokens, or credentials.

## Artifacts

The headless emitter
(`cargo run -q -p aureline-shell --bin aureline_shell_m5_owning_window_routing`) is the only
mint-from-truth path for:

- `artifacts/release/m5-owning-window-routing-proof/packet.json`
- `artifacts/release/m5-owning-window-routing-proof/dashboard.json`
- `artifacts/release/m5-owning-window-routing-proof/support_export.json`
- `artifacts/release/m5-owning-window-routing-proof/matrix.csv`
- `artifacts/shell/m5-owning-window-routing.md` (this report's rendered companion)
- `fixtures/ui/m5-owning-window-routing/packet.json` (and `dashboard.json`,
  `support_export.json`, `compact.txt`)

The boundary schema is
[`schemas/shell/m5-owning-window-routing.schema.json`](../../schemas/shell/m5-owning-window-routing.schema.json).

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_owning_window_routing -- validate
cargo test -p aureline-shell --test m5_owning_window_routing_fixtures
cargo test -p aureline-shell m5_owning_window_routing
```

Regenerate the artifacts after any change to the seed:

```sh
BIN="cargo run -q -p aureline-shell --bin aureline_shell_m5_owning_window_routing --"
$BIN packet         > artifacts/release/m5-owning-window-routing-proof/packet.json
$BIN dashboard      > artifacts/release/m5-owning-window-routing-proof/dashboard.json
$BIN support-export > artifacts/release/m5-owning-window-routing-proof/support_export.json
$BIN csv            > artifacts/release/m5-owning-window-routing-proof/matrix.csv
$BIN markdown       > artifacts/shell/m5-owning-window-routing.md
$BIN packet         > fixtures/ui/m5-owning-window-routing/packet.json
$BIN dashboard      > fixtures/ui/m5-owning-window-routing/dashboard.json
$BIN support-export > fixtures/ui/m5-owning-window-routing/support_export.json
$BIN compact        > fixtures/ui/m5-owning-window-routing/compact.txt
```
