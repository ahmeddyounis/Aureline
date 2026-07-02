# M5 splitter & resizable-pane control precision, persistence, restore & export contract

This lane is the **pane-control certification capstone** on top of the frozen
[M5 status-bar, transient-inspect, pane-control, and durable-progress-component
matrix](m5_shell_primitives_contract.md). Where the matrix *freezes* the pane-control
primitives — the splitter handle and the named pane-resize preset, with their
pane-resize states, accessibility routes, and mandatory labels — this lane *certifies*
that, in every claimed M5 multi-pane layout, a splitter or resizable pane can be resized
precisely with both pointer and keyboard inputs (enlarged logical hit targets,
hover/focus strengthen states, keyboard step-size controls, and double-click /
default-size restore); that resize intent persists as proportions or named presets
rather than brittle pixel positions, and that compact/expanded or monitor-topology
changes preserve that intent safely; that reset-to-default and restore after crash or
display/topology changes stay lossless and non-destructive; and that current pane
proportions and recent resize actions are reconstructable from a support export without
screenshots or manual reproduction.

The lane exists so that M5 can honestly claim mature shell quality: users never resize
panes through brittle pixel-only hit targets, never lose their layout to a compact or
monitor-topology change, and never have to reproduce a layout bug by hand because the
resize state is diagnosable from the support export.

## Governed pane layouts

The certification proof covers exactly six claimed M5 multi-pane layouts, and refuses to
ship if any is missing. Detached windows, compact-width sheets, and restoration after
crash or display changes are certified within each layout's row rather than as separate
layouts:

- `notebook` — Notebook cell / editor / output splitters
- `data` — Data grid query / grid / inspector splitters
- `review` — Review tree / diff / comment splitters
- `docs` — Docs nav / article / preview splitters
- `profiler` — Profiler timeline / flame-graph / detail splitters
- `incident` — Incident signal / log / action splitters

## Per-layout certification row

Each row names the pane-control primitives it drives (`splitter_handle` and
`pane_resize_preset`) and — pulled straight from the union of the frozen matrix's two
pane rows — the pane-resize states, required labels, accessibility routes, consumer
surfaces, and downgrade triggers. Because pane controls carry no source/provider or
freshness truth, this lane certifies neither freshness labels nor representation /
promotion truth, and its required-label set is the four pane-control labels — `identity`,
`state`, `keyboard_route`, and `reopen_path` — not the full six. It is certified across
four posture axes:

- **resize-control precision** — `precise_pointer_and_keyboard_resize` (green),
  `disclosed_reduced_hit_target_or_step` (yellow: a compact-width splitter's enlarged
  hit target shrinks or a coarse keyboard step is used while both pointer and keyboard
  resize still resolve and the default-size restore stays reachable), or
  `pointer_only_or_brittle_hit_target` (red: a pane is pointer-only or its hit target is
  too brittle to grab).
- **proportion persistence** — `proportions_or_presets_persisted` (green),
  `disclosed_reduced_persistence_fidelity` (yellow: a preset snaps to the nearest safe
  ratio under one topology while intent stays serialized as proportions rather than
  pixels), or `brittle_pixel_only_persistence` (red: resize intent persists only as
  brittle pixels so a topology change loses or corrupts the layout).
- **reset / restore** — `default_reset_and_topology_restore` (green),
  `disclosed_reduced_restore_fidelity` (yellow: a waivered fallback to a safe default
  layout after a display change while reset-to-default and a non-destructive restore
  still resolve), or `restore_lost_or_destructive` (red: restore after crash or display
  change is lost or collapses the layout to an unusable state).
- **resize-state export** — `proportions_and_actions_reconstructable` (green),
  `disclosed_partial_capture` (yellow: current proportions reconstruct while the recent
  resize-action log is a disclosed partial capture), or
  `resize_state_absent_from_capture` (red: current proportions or the resize-action log
  are absent from the support-export capture).

Each row also carries the hard invariant `pane_never_pointer_only_resizable`; `false` is
a blocker (a pane resizable by pointer only, with no keyboard step-size route).

## Derived status and the structural lints

The green/yellow/red status is **derived, never asserted**. A row drops to `yellow` when
it discloses a reduced hit target or keyboard step, a reduced persistence fidelity, a
reduced restore fidelity (backed by a waiver), or a partial support-export capture. It
drops to `red` when any axis reaches its blocked state, a pane is resizable by pointer
only, or its pane-resize states / required labels are incomplete. Those structural
lints — `pane_resize_states_complete` and `required_labels_complete` — are what prevent
a later pane control from shipping without its full idle / dragging / keyboard-step /
snapped / reset / clamped / collapsed transition set or its
identity/state/keyboard-route/reopen-path labels. The Rust validator in
`crates/aureline-shell/src/m5_pane_control_certification` is the authoritative gate.

A narrowed (non-green) row must disclose a reason; a
`disclosed_reduced_restore_fidelity` narrowing must additionally carry an active,
matching, unexpired waiver.

## Records

- **Certification packet** — the full set of rows with derived per-row status, aggregate
  green/yellow/red counts, active waivers, the exact certification causes, and the
  blocking findings the lane refuses to ship with.
- **Certification dashboard** — a light projection the shell / layout engine / release
  automation reads to auto-narrow a claimed layout when its pane-control proof falls out
  of policy.
- **Support export** — the packet plus dashboard plus stable case ids (packet id, matrix
  ref, build id, each layout, each waiver id).

The records carry only stable ids, closed vocabulary, counts, refs, and short labels —
never raw URLs, raw local paths, raw usernames, raw hostnames, tokens, or credentials.

## Artifacts

The headless emitter
(`cargo run -q -p aureline-shell --bin aureline_shell_m5_pane_control_certification`) is
the only mint-from-truth path for:

- `artifacts/release/m5-pane-control-certification-proof/packet.json`
- `artifacts/release/m5-pane-control-certification-proof/dashboard.json`
- `artifacts/release/m5-pane-control-certification-proof/support_export.json`
- `artifacts/release/m5-pane-control-certification-proof/matrix.csv`
- `artifacts/shell/m5-pane-control-certification.md` (this report's rendered companion)
- `fixtures/ui/m5-pane-control-certification/packet.json` (and `dashboard.json`,
  `support_export.json`, `compact.txt`)

The boundary schema is
[`schemas/shell/m5-pane-control-certification.schema.json`](../../schemas/shell/m5-pane-control-certification.schema.json).

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_pane_control_certification -- validate
cargo test -p aureline-shell --test m5_pane_control_certification_fixtures
cargo test -p aureline-shell m5_pane_control_certification
```

Regenerate the artifacts after any change to the seed:

```sh
BIN="cargo run -q -p aureline-shell --bin aureline_shell_m5_pane_control_certification --"
$BIN packet         > artifacts/release/m5-pane-control-certification-proof/packet.json
$BIN dashboard      > artifacts/release/m5-pane-control-certification-proof/dashboard.json
$BIN support-export > artifacts/release/m5-pane-control-certification-proof/support_export.json
$BIN csv            > artifacts/release/m5-pane-control-certification-proof/matrix.csv
$BIN markdown       > artifacts/shell/m5-pane-control-certification.md
$BIN packet         > fixtures/ui/m5-pane-control-certification/packet.json
$BIN dashboard      > fixtures/ui/m5-pane-control-certification/dashboard.json
$BIN support-export > fixtures/ui/m5-pane-control-certification/support_export.json
$BIN compact        > fixtures/ui/m5-pane-control-certification/compact.txt
```
