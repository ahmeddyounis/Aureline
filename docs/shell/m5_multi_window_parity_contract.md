# M5 multi-window truth-parity contract

This lane is the **multi-window-truth capstone** on top of the frozen
[M5 shell-zone, responsive-class, and multi-window continuity matrix](m5_shell_zone_matrix_contract.md).
Where the matrix *declares* the window classes each claimed M5 surface family may live in
(primary workspace, secondary detached, floating utility, companion overlay), the
workspace-global continuity truths it must preserve, and the owning-window routing
expectations it must honor, this lane *certifies* that the live surface keeps those
promises across windows: **every window carrying its work preserves the same workspace
identity, trust, remote/host, profile, and recovery-critical truth** while keeping layout,
density, and focus **local**, dialogs, notifications, and approvals **route back to the
owning window and object** without focus theft or orphaning, and crash-restore,
dependency-loss, and monitor-topology drills stay **predictable and non-destructive**.

The lane exists so that M5 can honestly claim desktop maturity: a detached docs, review,
notebook, or preview window is not a second-class view that silently drops workspace-global
trust, its remote/host, its deployment profile, or its recovery state; a per-window layout
or density choice never hides workspace-global risk or policy state; a routed approval
always finds its owning window and object; and a crash restore or a monitor unplug never
drops work or orphans a window.

## Governed families

The parity proof covers exactly the ten families the matrix freezes, and refuses to ship
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

## Workspace-global continuity truths

Every row must declare all four workspace-global continuity truths — `workspace_global_trust`,
`remote_target`, `deployment_profile`, and `recovery_state` — and every per-window plan
must preserve all four. A row that fails to declare the full set, or a per-window plan that
drops one, blocks: some window could otherwise omit trust, remote, profile, or
recovery-critical truth.

## Per-window continuity plan

Each row carries one **per-window continuity plan** for each declared window class, naming
which workspace-global truths that window preserves, whether layout stays local, and
whether routed actions return to the owning window. The plans must:

- cover exactly the declared window classes — no window the family may live in is left
  uncertified and none is invented;
- preserve exactly the declared continuity truths in every window;
- keep layout / density / focus **local** to each window;
- **route** dialogs, notifications, and approvals back to the owning window and object.

## Per-family posture axes

Each row is certified across four posture axes:

- **continuity parity** — `all_truths_preserved_in_every_window` (green), a disclosed
  `disclosed_truth_projection_narrowing` where a detached or utility window shows a
  narrowed-but-still-visible projection of a truth (yellow), or
  `workspace_truth_diverged_across_windows` (red: a window shows different or missing
  workspace-global truth than its peers).
- **layout locality** — `layout_density_focus_local_risk_global` (green), a disclosed
  `disclosed_local_only_state` where a window discloses a purely-local view state that
  never hides global risk (yellow), or `workspace_global_risk_hidden_locally` (red: a
  local layout choice hid workspace-global risk or policy state).
- **owning-window routing** — `routes_to_owning_window_object` (green),
  `disclosed_routing_relocation` where a routed action relocates to a disclosed, waivered
  still-visible affordance when its owning window is absent (yellow), or
  `routing_lost_focus_theft_or_orphan` (red: a routed action stole focus or was orphaned).
- **recovery drill** — `restore_dependency_topology_predictable` (green), a disclosed
  `disclosed_recovery_narrowing` where a crash-restore, dependency-loss, or
  monitor-topology drill discloses a narrowed but non-destructive recovery (yellow), or
  `restore_destructive_or_orphaned` (red: a drill dropped work, orphaned a window, or
  diverged truth on restore).

## Derived status and the parity lint

The green/yellow/red status is **derived, never asserted**. A row drops to `yellow` when
its frozen qualification is below Stable, a window discloses a narrowed-but-visible truth
projection, a window discloses a purely-local state, a routed action relocates to a
disclosed waivered affordance, or a recovery drill discloses a non-destructive narrowing.
It drops to `red` when workspace-global truth diverges across windows, a window hides
workspace-global risk behind a local layout choice, a routed action is lost to focus theft
or orphaning, a recovery drill is destructive or orphaning, a required continuity truth or
routing expectation is undeclared, or a per-window plan drops a truth, layout locality, or
owning-window routing. The truth-completeness and per-window plan checks are the lint that
prevents a later cross-window regression from shipping as stable — the Rust validator in
`crates/aureline-shell/src/m5_multi_window_parity` is the authoritative gate.

A narrowed (non-green) row must disclose a reason; a `disclosed_routing_relocation`
narrowing must additionally carry an active, matching, unexpired waiver.

## Records

- **Parity packet** — the full set of rows with derived per-row status, aggregate
  green/yellow/red counts, active waivers, the exact parity causes, and the blocking
  findings the lane refuses to ship with.
- **Parity dashboard** — a light projection the shell / windowing / layout / release
  automation reads to auto-narrow a claimed surface when its multi-window proof falls out
  of policy.
- **Support export** — the packet plus dashboard plus stable case ids (packet id, matrix
  ref, build id, each family, each waiver id).

The records carry only stable ids, closed vocabulary, counts, refs, and short labels —
never raw URLs, raw local paths, raw usernames, raw hostnames, tokens, or credentials.

## Artifacts

The headless emitter
(`cargo run -q -p aureline-shell --bin aureline_shell_m5_multi_window_parity`) is the only
mint-from-truth path for:

- `artifacts/release/m5-multi-window-parity-proof/packet.json`
- `artifacts/release/m5-multi-window-parity-proof/dashboard.json`
- `artifacts/release/m5-multi-window-parity-proof/support_export.json`
- `artifacts/release/m5-multi-window-parity-proof/matrix.csv`
- `artifacts/shell/m5-multi-window-parity.md` (this report's rendered companion)
- `fixtures/ui/m5-multi-window-parity/packet.json` (and `dashboard.json`,
  `support_export.json`, `compact.txt`)

The boundary schema is
[`schemas/shell/m5-multi-window-parity.schema.json`](../../schemas/shell/m5-multi-window-parity.schema.json).

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_multi_window_parity -- validate
cargo test -p aureline-shell --test m5_multi_window_parity_fixtures
cargo test -p aureline-shell m5_multi_window_parity
```

Regenerate the artifacts after any change to the seed:

```sh
BIN="cargo run -q -p aureline-shell --bin aureline_shell_m5_multi_window_parity --"
$BIN packet         > artifacts/release/m5-multi-window-parity-proof/packet.json
$BIN dashboard      > artifacts/release/m5-multi-window-parity-proof/dashboard.json
$BIN support-export > artifacts/release/m5-multi-window-parity-proof/support_export.json
$BIN csv            > artifacts/release/m5-multi-window-parity-proof/matrix.csv
$BIN markdown       > artifacts/shell/m5-multi-window-parity.md
$BIN packet         > fixtures/ui/m5-multi-window-parity/packet.json
$BIN dashboard      > fixtures/ui/m5-multi-window-parity/dashboard.json
$BIN support-export > fixtures/ui/m5-multi-window-parity/support_export.json
$BIN compact        > fixtures/ui/m5-multi-window-parity/compact.txt
```
