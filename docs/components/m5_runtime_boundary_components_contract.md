# M5 Terminal-Tab, Remote-Target-Pill, Environment-Status-Strip, Toolchain-Pin-Row, Presence-Avatar-Stack, and Repair-Action-Card Component Contract

> Task: M05-852 · Batch B100 · Delivery class: component implementation +
> cross-surface boundary/status parity + repair preview hardening.

This contract freezes the checked-in matrix for Aureline's reusable
runtime-boundary and repair components — the ones that still drift too easily by
feature lane: terminal tabs/headers, remote target pills, environment status
strips, toolchain pin rows, presence avatar stacks, and repair action cards. It
names the controlled anatomy, state vocabulary, and supportability hooks M5 will
honor for each component family, so later M5 rows can no longer invent private
host/runtime/repair status semantics without changing the matrix.

- **Boundary schema:** [`schemas/ui/m5-runtime-boundary-components.schema.json`](../../schemas/ui/m5-runtime-boundary-components.schema.json)
- **Rust source of truth:** `crates/aureline-shell/src/freeze_the_m5_terminal_tab_remote_target_pill_environment_status_strip_toolchain_pin_row_presence_avatar_stack_and_repair_action_card_component_matrix/`
- **Headless emitter:** `aureline_shell_m5_runtime_boundary_components`
- **Checked support export:** [`artifacts/release/m5-runtime-boundary-proof/support_export.json`](../../artifacts/release/m5-runtime-boundary-proof/support_export.json)
- **Matrix CSV:** [`artifacts/release/m5-runtime-boundary-proof/matrix.csv`](../../artifacts/release/m5-runtime-boundary-proof/matrix.csv)
- **Report:** [`artifacts/components/m5-runtime-boundary-components.md`](../../artifacts/components/m5-runtime-boundary-components.md)
- **Design matrix:** [`artifacts/design/m5-runtime-boundary-component-matrix.md`](../../artifacts/design/m5-runtime-boundary-component-matrix.md)
- **Narrowed fixtures:** [`fixtures/ui/m5-runtime-boundary-components/`](../../fixtures/ui/m5-runtime-boundary-components/)

The shell topology this matrix binds against — the eight canonical shell zones,
the compact/standard/expanded responsive classes, the window classes, the
consumer surfaces, and the ten claimed M5 surface families — is reused verbatim
from the frozen
[M5 shell-zone matrix](../../schemas/shell/m5-shell-zone.schema.json). This lane
mints no parallel slot, layout, window, surface-family, or consumer vocabulary; it
adds only the vocabulary for the runtime-boundary and repair components themselves.

## Track invariant

Session title, host boundary, shell-integration quality, resolved
runtime/toolchain, winning scope/source, collaboration role/follow state, repair
blast radius, and reversibility class remain explicit everywhere a user runs,
shares, switches, repairs, or exports execution state. No M5 lane invents a second
status grammar, masks a host/runtime boundary, conflates a live session with a
restored one, or overstates reversibility.

## Component families (rows)

Each row binds one component family to its canonical shell zone, the responsive
classes it must survive, the window classes it keeps continuity across, the
claimed M5 surface families that render it, its mandatory labels, its
family-specific controlled vocabulary, its non-visual accessibility routes, its
consumer surfaces, and the downgrade triggers that narrow it below its claim.

| Component family | Zone | Family-specific vocabulary |
| --- | --- | --- |
| `terminal_tab` | `bottom_panel` | shell-integration qualities + session-liveness states |
| `remote_target_pill` | `title_context_bar` | host-boundary classes + connection states |
| `environment_status_strip` | `status_bar` | runtime source classes |
| `toolchain_pin_row` | `right_inspector` | toolchain source classes + pin states |
| `presence_avatar_stack` | `title_context_bar` | collaboration roles + follow states |
| `repair_action_card` | `transient_overlay` | repair blast radii + reversibility classes |

The `component_family` predicates drive per-family lints: the terminal family must
declare shell-integration qualities and session-liveness states; the remote-target
family must declare host-boundary classes and connection states; the environment
family must declare runtime source classes; the toolchain family must declare
toolchain source classes and pin states; the presence family must declare
collaboration roles and follow states; the repair family must declare blast radii
and reversibility classes. Vocabulary a family does not carry stays empty.

## Controlled vocabularies

- **Shell-integration qualities:** `fully_integrated`, `command_marks_only`,
  `cwd_reporting_only`, `basic_pty_no_integration`, `integration_degraded`.
- **Session-liveness states:** `live_attached`, `live_detached_running`,
  `restored_from_transcript`, `reconnecting`, `closed_exited`.
- **Host-boundary classes:** `local_host`, `remote_ssh_host`, `container_host`,
  `managed_workspace_host`, `virtual_machine_host`, `wasm_sandbox_host`.
- **Connection states:** `connected`, `connecting`, `reconnecting`,
  `disconnected`, `offline_cached`.
- **Runtime source classes:** `project_pinned`, `workspace_configured`,
  `tool_manager_resolved`, `system_default`, `container_provided`,
  `session_override`.
- **Toolchain source classes:** `pin_file`, `workspace_setting`,
  `version_manager`, `system_installed`, `container_image`, `session_override`.
- **Toolchain pin states:** `pinned_resolved`, `pinned_missing_fallback`,
  `unpinned`, `pin_conflict`, `pin_overridden`.
- **Collaboration roles:** `session_host`, `collaborator`, `presenter`,
  `observer`, `control_holder`.
- **Follow states:** `following_presenter`, `being_followed`, `not_following`,
  `presenting_to_others`, `follow_paused`.
- **Repair blast radii:** `no_writes_preview`, `workspace_scoped`,
  `toolchain_scoped`, `host_environment_scoped`, `multi_target_scoped`.
- **Reversibility classes:** `fully_reversible_checkpoint`,
  `reversible_with_backup`, `partially_reversible`, `irreversible_confirmed`,
  `reversal_requires_manual_steps`.
- **Accessibility routes:** `keyboard_focusable`, `screen_reader_announced`,
  `non_hover_reachable`, `pointer_optional`, `high_contrast_safe`,
  `support_exportable`.
- **Required labels:** `identity`, `state`, `keyboard_route` (mandatory on every
  component) plus `boundary`, `resolved_source`, `reversibility`.

## Hard invariants

Every row asserts four booleans that MUST be `false`; any `true` value is a
`component_invariant_violated` blocker:

- `masks_host_or_runtime_boundary` — no component masks a remote/container/managed
  boundary as local.
- `conflates_live_and_restored_session` — a terminal tab never shows a restored
  transcript as a live session.
- `invents_private_status_grammar` — no component invents a second status grammar.
- `overstates_reversibility_or_drops_audit_truth` — a repair card never overstates
  reversibility, and audit/support truth is never lost off the primary surface.

## Downgrade triggers

`shell_integration_quality_hidden`, `session_liveness_ambiguous`,
`host_boundary_masked`, `connection_state_stale`, `runtime_source_unexplained`,
`toolchain_pin_conflict_hidden`, `collaboration_role_masked`,
`follow_state_ambiguous`, `repair_blast_radius_understated`,
`reversibility_overstated`, `audit_truth_lost_off_primary_surface`, `proof_stale`.

## First consumers bound to the matrix

- Terminal / session surfaces consume the shell-integration / session-liveness
  vocabulary.
- Remote and environment surfaces consume the host-boundary / connection / runtime
  source vocabulary.
- Collaboration surfaces consume the role / follow vocabulary.
- Repair surfaces consume the blast-radius / reversibility vocabulary.
- Support/export and the accessibility bridge each read one canonical
  runtime-boundary source.

## Regenerating the artifacts

```sh
BIN=aureline_shell_m5_runtime_boundary_components
cargo run -q -p aureline-shell --bin $BIN -- support-export > artifacts/release/m5-runtime-boundary-proof/support_export.json
cargo run -q -p aureline-shell --bin $BIN -- csv            > artifacts/release/m5-runtime-boundary-proof/matrix.csv
cargo run -q -p aureline-shell --bin $BIN -- report         > artifacts/components/m5-runtime-boundary-components.md
cargo run -q -p aureline-shell --bin $BIN -- fixture-presence-avatar-stack-beta-narrowed > fixtures/ui/m5-runtime-boundary-components/presence_avatar_stack_beta_narrowed.json
cargo run -q -p aureline-shell --bin $BIN -- fixture-repair-action-card-preview-narrowed > fixtures/ui/m5-runtime-boundary-components/repair_action_card_preview_narrowed.json
cargo run -q -p aureline-shell --bin $BIN -- validate
```

The inline test `checked_support_export_matches_seed` and the fixture round-trip
test assert the checked-in JSON is bit-for-bit identical to the seed builder, so
the artifacts can never silently drift from the in-code matrix.
