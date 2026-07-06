# M5 Toolchain-Pin / Switch-Review Primitive — Design Matrix

Design QA companion for task **M05-855** (batch B100). This is the third `implement_`
lane narrowing the runtime-boundary component matrix (M05-852) — after the terminal-tab
(M05-853) and remote-target / environment-strip (M05-854) primitives. It implements the
`toolchain_pin_row` family plus the precedence inspector and switch-review card the
acceptance criteria require.

## Claimed environment selectors (matrix rows)

| Selector surface | Shell zone | Target kind focus | Worked cases |
| ---------------- | ---------- | ----------------- | ------------ |
| Status-Bar Selector | Status Bar | interpreter | pinned-resolved; policy override shadowing a project pin |
| Command-Palette Switcher | Transient Overlay | sdk | session override + reviewed switch; unpinned default |
| Settings Toolchain Row | Main Workspace | shell | workspace shadows user (AC1); lone user pin |
| Interpreter Picker | Transient Overlay | interpreter | missing interpreter + repair; container-image pin |
| SDK Selector | Right Inspector | sdk | stale SDK + repair; mismatched session override + host switch |
| Shell-Profile Picker | Status Bar | shell | host default (unpinned); session override + fully reversible switch |
| Kernel Picker | Main Workspace | kernel | project shadows user (conflict); reconnect-required switch |
| Runtime-Target Switcher | Title Context Bar | runtime | restart switch; policy shadows two durable pins |
| Repair-Panel Selector | Right Inspector | runtime | missing runtime + repair; mismatched runtime + host switch |

## Winning-scope precedence ladder

`policy > session > project > workspace > user > host > global_default`

The winning layer is the lowest-rank present layer; overshadowed layers are ranked below
it and each carries an explicit `shadow_reason`.

## Derived-state coverage

- **Target kinds (5):** interpreter, sdk, shell, kernel, runtime.
- **Pin states (5):** pinned_resolved, pinned_missing_fallback, unpinned, pin_conflict,
  pin_overridden.
- **Winning scopes (7):** all of policy, session, project, workspace, user, host,
  global_default appear as a winning scope across the worked resolutions.
- **Selection health (4):** healthy, degraded_stale, mismatched_version,
  missing_unavailable.
- **Toolchain sources (6):** pin_file, workspace_setting, version_manager,
  system_installed, container_image, session_override — each appears as a winning source.
- **Pin actions (4):** review_precedence, clear_override, revert_to_shadowed_pin,
  repair_selection.
- **Switch blast radii:** workspace_scoped, toolchain_scoped, host_environment_scoped,
  multi_target_scoped.
- **Switch reversibility:** fully_reversible_checkpoint, reversible_with_backup,
  partially_reversible, reversal_requires_manual_steps.

## Narrowed variants

- `repair_panel_beta_narrowed` — repair-panel selector held at Beta pending
  switch-blast-radius rendering parity on every profile; every surface stays visible.
- `runtime_target_preview_narrowed` — runtime-target switcher narrowed to Preview pending
  reversibility parity proof across every export path; every surface stays visible.

## Acceptance-criteria witnesses

- **AC1** — `settings-shell-workspace-shadows-user` and `status-py-policy-override` prove
  a higher layer shadowing a durable pin with the shadow disclosed.
- **AC2** — `palette-sdk-session-override`, `kernel-workspace-reconnect-switch`, and
  peers prove the predicted blast radius and reversibility shown before switching.
- **AC3** — `picker-py-missing`, `sdk-project-stale`, and `repair-*` prove a degraded /
  mismatched / missing selection keeping an explicit repair action.
