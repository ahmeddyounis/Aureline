# M5 Workspace-Window, Shared-Authority, Skeleton-Restore, and No-Rerun Session-Hydration Contract

Status: frozen (B141 opening matrix)

This contract freezes Aureline's concrete multi-window ownership and restore-orchestration behavior into
one export-safe matrix. It is the canonical source of window-restore truth for M5: later shell startup /
restore coordinators, workspace and session services, terminal / debug / notebook / collaboration
rehydration, display-topology recovery, shell / recovery / diagnostics / admin surfaces, docs/help,
support/export, and release-evidence tooling consume it directly rather than copying restore prose by hand.

- Matrix schema: `schemas/shell/m5-window-restore-matrix.schema.json`
- Window-topology domain schema (shared workspace authority / window-local topology / display recovery): `schemas/shell/m5-window-topology.schema.json`
- Restore-fidelity domain schema (skeleton-first rebuild / no-rerun hydration): `schemas/shell/m5-restore-fidelity.schema.json`
- Support export: `artifacts/release/m5-window-restore-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-window-restore-proof/matrix.csv`
- Design report: `artifacts/shell/m5-workspace-window-restore-matrix.md`
- Narrowed fixtures: `fixtures/ui/m5-window-restore/`
- Authoritative validator: `crates/aureline-ui` (`m5_window_restore_matrix`)
- Emitter (single mint-from-truth path): `cargo run -p aureline-ui --example dump_m5_window_restore_matrix`

## Governed workspace-restore families

The matrix freezes **five** workspace-restore families, each qualified independently and each pointing at
one canonical domain schema:

| Family | Restore concern | Owner | Domain schema |
| --- | --- | --- | --- |
| `shared_workspace_authority` | One authority backs multiple windows; selection and focus stay window-local | Workspace-authority owner | window-topology |
| `window_local_topology` | Versioned, attributable window-local pane trees | Window-topology owner | window-topology |
| `skeleton_first_restore` | Rebuild the layout skeleton before hydrating heavy dependencies | Restore-coordinator owner | restore-fidelity |
| `no_rerun_session_hydration` | Session-scoped tools never silently rerun or reacquire authority | Session-service owner | restore-fidelity |
| `display_topology_recovery` | Display-topology changes keep windows and dialogs reachable | Display-topology recovery owner | window-topology |

## Shared window-restore-role vocabulary

Every consumer binds to one controlled role vocabulary; no surface invents a parallel word:

`workspace_authority`, `window_topology`, `pane_role`, `layout_skeleton`, `session_hydration`,
`restore_fidelity`, `display_affinity`.

The authority / hydration / fidelity / affinity roles (`workspace_authority`, `session_hydration`,
`restore_fidelity`, `display_affinity`) must preserve window-local selection and no-rerun under shared
authority — a restore may never clobber a window-local selection, silently rerun session-scoped work,
overclaim restore fidelity, or strand a window off-screen. The descriptive structure roles
(`window_topology`, `pane_role`, `layout_skeleton`) are inspectable descriptors.

## Hard invariants

Every row carries five hard-invariant booleans that must be `false`, and the governance-review block
asserts the corresponding fleet-level guarantees:

1. Restore never reruns commands or reattaches privileged sessions implicitly.
2. A missing extension or remote target never deletes layout structure silently.
3. A display-topology remap never leaves windows or dialogs unreachable.
4. Workspace-authority state and window-topology state are never merged into one opaque blob.
5. Restore fidelity is never overclaimed when the system only reopened context or evidence.

## Automatic narrowing

Claim publication and support/export narrow window-restore claims automatically when the B141 registry is
missing, stale, or not yet qualified. Two narrowed fixtures demonstrate honest narrowing while keeping
every family visible:

- `no_rerun_session_hydration_beta_narrowed.json` — no-rerun session hydration held at **Beta**.
- `display_topology_recovery_preview_narrowed.json` — display-topology recovery narrowed to **Preview**
  pending complete multi-monitor remap-and-reachability evidence.

## Bound source contracts

The matrix binds back to already-landed truth so window-restore truth is never split across scattered
notes: the multi-window-parity schema (`schemas/shell/m5-multi-window-parity.schema.json`) and the
monitor-geometry-remap-and-restore-bounds matrix
(`schemas/shell/m5-monitor-geometry-remap-and-restore-bounds.schema.json`).
