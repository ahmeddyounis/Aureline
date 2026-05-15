# Beta command-parity diff report

Generated from the seeded parity projection in
[`crate::command_parity`](../../../crates/aureline-shell/src/command_parity/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_command_parity -- report-md > \
  artifacts/ux/m3/command_parity_diff_report.md
```

- Report id: `shell:command_parity_beta:diff:v1`
- Descriptor schema ref: `schemas/commands/command_descriptor.schema.json`
- Claimed beta commands: `5`
- High-risk beta commands: `3`
- Surface rows checked: `25`
- Blocking findings: `0`
- Status: **clean**
- Generated at: `2026-05-15T00:00:00Z`

## Per-surface coverage

| Surface | Claimed | Narrowed | Unknown high-risk |
| ------- | ------: | -------: | ----------------: |
| Command palette | 5 | 0 | 0 |
| Menus and buttons | 3 | 2 | 0 |
| Keybinding help | 2 | 3 | 0 |
| CLI / headless | 4 | 1 | 0 |
| AI tool surface | 2 | 3 | 0 |

## Findings summary

| Class | Count |
| ----- | ----: |
| `unknown_high_risk_gap` | 0 |
| `command_id_drift` | 0 |
| `label_drift` | 0 |
| `lifecycle_label_drift` | 0 |
| `preview_class_drift` | 0 |
| `disabled_reason_drift` | 0 |
| `missing_docs_help_anchor` | 0 |
| `alias_drift` | 0 |
| `missing_narrowing_reason` | 0 |
| `missing_projection` | 0 |

## Per-command rows

### `cmd:command_palette.open` (stable)

- Descriptor revision: `cmd-rev:command_palette.open:2026.04.22-01`
- Preview class: `no_preview_required`
- Capability scope: `inert_metadata_only`
- Disabled reason mode: `always_invokable`
- Docs/help anchor: `docs:anchor:command_palette:open_overview`
- High-risk: `no`

| Surface | Status | Projected preview | Disabled reason mode | Narrowing reason |
| ------- | ------ | ----------------- | -------------------- | ---------------- |
| Command palette | `claimed` | `no_preview_required` | `always_invokable` | - |
| Menus and buttons | `explicitly_narrowed` | `-` | `-` | learnability_only_route |
| Keybinding help | `claimed` | `no_preview_required` | `always_invokable` | - |
| CLI / headless | `not_surfaced_on_this_client` | `-` | `-` | client_scope_excludes_surface |
| AI tool surface | `explicitly_narrowed` | `-` | `-` | ui_only_route |

Findings: none.

### `cmd:workspace.clone_repository` (stable)

- Descriptor revision: `cmd-rev:workspace.clone_repository:2026.04.22-01`
- Preview class: `structured_diff_preview`
- Capability scope: `recoverable_durable_mutation`
- Disabled reason mode: `typed_reason_required_when_unavailable`
- Docs/help anchor: `docs:anchor:workspace:clone_repository_overview`
- High-risk: `yes`

| Surface | Status | Projected preview | Disabled reason mode | Narrowing reason |
| ------- | ------ | ----------------- | -------------------- | ---------------- |
| Command palette | `claimed` | `structured_diff_preview` | `typed_reason_required_when_unavailable` | - |
| Menus and buttons | `claimed` | `structured_diff_preview` | `typed_reason_required_when_unavailable` | - |
| Keybinding help | `explicitly_narrowed` | `-` | `-` | keybinding_unassigned_at_beta |
| CLI / headless | `claimed` | `structured_diff_preview` | `typed_reason_required_when_unavailable` | - |
| AI tool surface | `claimed` | `structured_diff_preview` | `typed_reason_required_when_unavailable` | - |

Findings: none.

### `cmd:workspace.import_profile` (beta)

- Descriptor revision: `cmd-rev:workspace.import_profile:2026.04.22-01`
- Preview class: `structured_diff_preview`
- Capability scope: `recoverable_durable_mutation`
- Disabled reason mode: `typed_reason_required_when_unavailable`
- Docs/help anchor: `docs:anchor:migration:import_profile_overview`
- High-risk: `yes`

| Surface | Status | Projected preview | Disabled reason mode | Narrowing reason |
| ------- | ------ | ----------------- | -------------------- | ---------------- |
| Command palette | `claimed` | `structured_diff_preview` | `typed_reason_required_when_unavailable` | - |
| Menus and buttons | `claimed` | `structured_diff_preview` | `typed_reason_required_when_unavailable` | - |
| Keybinding help | `explicitly_narrowed` | `-` | `-` | keybinding_unassigned_at_beta |
| CLI / headless | `claimed` | `structured_diff_preview` | `typed_reason_required_when_unavailable` | - |
| AI tool surface | `explicitly_narrowed` | `-` | `-` | approval_required |

Findings: none.

### `cmd:workspace.open_folder` (stable)

- Descriptor revision: `cmd-rev:workspace.open_folder:2026.04.21-01`
- Preview class: `no_preview_required`
- Capability scope: `reversible_local_mutation`
- Disabled reason mode: `typed_reason_required_when_unavailable`
- Docs/help anchor: `docs:anchor:workspace:open_folder_overview`
- High-risk: `no`

| Surface | Status | Projected preview | Disabled reason mode | Narrowing reason |
| ------- | ------ | ----------------- | -------------------- | ---------------- |
| Command palette | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | - |
| Menus and buttons | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | - |
| Keybinding help | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | - |
| CLI / headless | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | - |
| AI tool surface | `claimed` | `no_preview_required` | `typed_reason_required_when_unavailable` | - |

Findings: none.

### `cmd:workspace.restore_from_checkpoint` (beta)

- Descriptor revision: `cmd-rev:workspace.restore_from_checkpoint:2026.04.22-01`
- Preview class: `destructive_bulk_mutation_preview`
- Capability scope: `destructive_bulk_mutation`
- Disabled reason mode: `typed_reason_required_when_unavailable`
- Docs/help anchor: `docs:anchor:workspace:restore_from_checkpoint_overview`
- High-risk: `yes`

| Surface | Status | Projected preview | Disabled reason mode | Narrowing reason |
| ------- | ------ | ----------------- | -------------------- | ---------------- |
| Command palette | `claimed` | `destructive_bulk_mutation_preview` | `typed_reason_required_when_unavailable` | - |
| Menus and buttons | `explicitly_narrowed` | `-` | `-` | approval_required |
| Keybinding help | `explicitly_narrowed` | `-` | `-` | keybinding_unassigned_at_beta |
| CLI / headless | `claimed` | `destructive_bulk_mutation_preview` | `typed_reason_required_when_unavailable` | - |
| AI tool surface | `explicitly_narrowed` | `-` | `-` | approval_required |

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_command_parity -- validate
cargo test -p aureline-shell --test command_parity_fixtures
python3 tools/ci/m3/command_parity_check.py
```
