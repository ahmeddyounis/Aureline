# M5 open/save/reveal path truth

Generated from the seeded report in
[`crate::m5_open_save_reveal`](../../crates/aureline-workspace/src/m5_open_save_reveal/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-workspace --bin aureline_workspace_m5_open_save_reveal -- report-md > \
  artifacts/platform/m5-open-save-reveal.md
```

- Report id: `workspace:m5_open_save_reveal:report:v1`
- Source schema ref: `schemas/platform/m5-path-boundary.schema.json`
- Claimed platforms: `macos`, `windows`, `linux`
- Registered flows: `9`
- Marketed flows: `9`
- Write flows: `6`
- Blocking findings: `0`
- Narrowable marketed flows: `0`
- Status: **clean**
- Generated at: `2026-06-16T00:00:00Z`

## Cross-links

| Upstream packet | Ref |
| --------------- | --- |
| `filesystem_identity_ref` | `schemas/workspace/canonical_identity_lineage.schema.json` |
| `save_coordination_ref` | `schemas/state/artifact_save_truth.schema.json` |
| `restore_continuity_ref` | `docs/workspace/entry_restore_object_model.md` |
| `native_desktop_matrix_ref` | `artifacts/platform/m5-native-desktop-matrix.md` |
| `system_entry_intake_ref` | `artifacts/platform/m5-system-open-and-file-association.md` |
| `help_about_ref` | `docs/help/open_save_reveal_path_truth.md` |

## Per-flow-kind coverage

| Flow kind | Registered flows |
| --------- | ---------------: |
| Open | 1 |
| Save | 4 |
| Save As | 2 |
| Reveal in system shell | 1 |
| Open in default browser | 1 |

## Per-boundary coverage

| Boundary | Flows | Write-protected |
| -------- | ----: | --------------: |
| Local (writable) | 5 | 0 |
| Remote / network share | 1 | 0 |
| Generated artifact | 2 | 1 |
| Read-only destination | 1 | 1 |

## Per-path-condition coverage

| Path condition | Flows | With recovery |
| -------------- | ----: | ------------: |
| Exact / available | 5 | 0 |
| Missing canonical target | 1 | 1 |
| Network-share alias | 1 | 1 |
| Generated output | 1 | 1 |
| Read-only destination | 1 | 1 |

## Path-truth index

| Flow | Kind | Path truth | Boundary | Overwrite posture | Condition |
| ---- | ---- | ---------- | -------- | ----------------- | --------- |
| `flow:case.generated_output` | Save | `boundary_labeled_artifact` | `generated` | `export_not_in_place_save` | `generated_output` |
| `flow:case.missing_canonical_target` | Save As | `canonical_target_missing` | `local_writable` | `overwrite_review_required` | `missing_canonical_target` |
| `flow:case.network_share_alias` | Save | `canonical_alias_resolved` | `remote_adjacent` | `overwrite_review_required` | `network_share_alias` |
| `flow:case.read_only_destination` | Save | `boundary_labeled_artifact` | `read_only` | `write_blocked_read_only` | `read_only_destination` |
| `flow:open.local_file` | Open | `literal_is_canonical` | `local_writable` | `no_write_action` | `exact_available` |
| `flow:open_in_browser.generated_preview` | Open in default browser | `boundary_labeled_artifact` | `generated` | `no_write_action` | `exact_available` |
| `flow:reveal.local_file` | Reveal in system shell | `literal_is_canonical` | `local_writable` | `no_write_action` | `exact_available` |
| `flow:save.local_file` | Save | `literal_is_canonical` | `local_writable` | `overwrite_with_checkpoint` | `exact_available` |
| `flow:save_as.local_file` | Save As | `literal_is_canonical` | `local_writable` | `create_new_file` | `exact_available` |

## Findings summary

| Class | Count |
| ----- | ----: |
| _(none)_ | 0 |

## Per-flow rows

### `flow:case.generated_output` (save)

- Descriptor revision: `flow:case.generated_output:rev:2026.06.01-01`
- Literal target: `literal:case.generated_output:captured` (`posix_path`)
- Canonical target: `canonical:case.generated_output:generated_file` (`boundary_labeled_artifact`)
- Detected target kind: `local_file`
- Boundary: `generated` (`boundary:flow:case.generated_output:generated`)
- Overwrite posture: `export_not_in_place_save` (checkpoint: `not_applicable`)
- Overwrite review: `save:overwrite_review:checkpoint_aware:v1`
- Reveal side effect: `no_external_side_effect`
- Filesystem identity: `filesystem-identity:flow:case.generated_output`
- Save coordination: `save-coordination:flow:case.generated_output`
- Active profile owner: `profile-owner:flow:case.generated_output`
- Trust checkpoint: `trust:flow:case.generated_output:profile_policy`
- Canonical command: `cmd:workspace.save.target`
- Path condition: `generated_output`
- Recovery actions: `export_instead_of_save`, `regenerate_from_source`, `show_canonical_path`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:open_save_reveal:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: Saving a generated artifact is presented as an export, not an in-place save, with a path to regenerate from the canonical source, so a generated output is never mistaken for an editable file.
- Degraded-state vocabulary:
  - This is a generated file
  - Export a copy instead of editing it in place
  - Regenerate it from its source

Findings: none.

### `flow:case.missing_canonical_target` (save_as)

- Descriptor revision: `flow:case.missing_canonical_target:rev:2026.06.01-01`
- Literal target: `literal:case.missing_canonical_target:captured` (`posix_path`)
- Canonical target: `canonical:case.missing_canonical_target:unresolved` (`canonical_target_missing`)
- Detected target kind: `local_file`
- Boundary: `local_writable` (`boundary:flow:case.missing_canonical_target:local_writable`)
- Overwrite posture: `overwrite_review_required` (checkpoint: `not_applicable`)
- Overwrite review: `save:overwrite_review:checkpoint_aware:v1`
- Reveal side effect: `no_external_side_effect`
- Filesystem identity: `filesystem-identity:flow:case.missing_canonical_target`
- Save coordination: `save-coordination:flow:case.missing_canonical_target`
- Active profile owner: `profile-owner:flow:case.missing_canonical_target`
- Trust checkpoint: `trust:flow:case.missing_canonical_target:profile_policy`
- Canonical command: `cmd:workspace.save_as.target`
- Path condition: `missing_canonical_target`
- Recovery actions: `choose_different_target`, `show_canonical_path`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:open_save_reveal:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: When the canonical target cannot be resolved from the literal the user selected, the save is held for explicit review instead of writing to a guessed path, with a target picker and the canonical-path detail offered.
- Degraded-state vocabulary:
  - The file you selected no longer resolves to a known location
  - Show where this path points
  - Choose a different file to save to

Findings: none.

### `flow:case.network_share_alias` (save)

- Descriptor revision: `flow:case.network_share_alias:rev:2026.06.01-01`
- Literal target: `literal:case.network_share_alias:captured` (`windows_unc_path`)
- Canonical target: `canonical:case.network_share_alias:share_target` (`canonical_alias_resolved`)
- Detected target kind: `local_file`
- Boundary: `remote_adjacent` (`boundary:flow:case.network_share_alias:remote_adjacent`)
- Overwrite posture: `overwrite_review_required` (checkpoint: `unavailable`)
- Overwrite review: `save:overwrite_review:checkpoint_aware:v1`
- Reveal side effect: `no_external_side_effect`
- Filesystem identity: `filesystem-identity:flow:case.network_share_alias`
- Save coordination: `save-coordination:flow:case.network_share_alias`
- Active profile owner: `profile-owner:flow:case.network_share_alias`
- Trust checkpoint: `trust:flow:case.network_share_alias:profile_policy`
- Canonical command: `cmd:workspace.save.target`
- Path condition: `network_share_alias`
- Recovery actions: `resolve_share_alias`, `reconnect_share`, `show_canonical_path`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:open_save_reveal:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A save through a network-share alias discloses the canonical share target the alias resolves to and holds the write for review, so an alias-path confusion can never silently land on the wrong remote target.
- Degraded-state vocabulary:
  - This path is a network-share alias
  - Show the share target it points to
  - Reconnect the network share to continue

Findings: none.

### `flow:case.read_only_destination` (save)

- Descriptor revision: `flow:case.read_only_destination:rev:2026.06.01-01`
- Literal target: `literal:case.read_only_destination:captured` (`posix_path`)
- Canonical target: `canonical:case.read_only_destination:read_only_file` (`boundary_labeled_artifact`)
- Detected target kind: `local_file`
- Boundary: `read_only` (`boundary:flow:case.read_only_destination:read_only`)
- Overwrite posture: `write_blocked_read_only` (checkpoint: `not_applicable`)
- Overwrite review: `save:overwrite_review:checkpoint_aware:v1`
- Reveal side effect: `no_external_side_effect`
- Filesystem identity: `filesystem-identity:flow:case.read_only_destination`
- Save coordination: `save-coordination:flow:case.read_only_destination`
- Active profile owner: `profile-owner:flow:case.read_only_destination`
- Trust checkpoint: `trust:flow:case.read_only_destination:profile_policy`
- Canonical command: `cmd:workspace.save.target`
- Path condition: `read_only_destination`
- Recovery actions: `save_writable_copy_elsewhere`, `open_read_only`, `show_canonical_path`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:open_save_reveal:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A save to a read-only destination blocks the in-place write and offers a writable copy elsewhere, so platform-native dialog convenience never lets a read-only target appear writable.
- Degraded-state vocabulary:
  - This destination is read-only
  - Save a writable copy somewhere else
  - Open it read-only instead

Findings: none.

### `flow:open.local_file` (open)

- Descriptor revision: `flow:open.local_file:rev:2026.06.01-01`
- Literal target: `literal:open.local_file:captured` (`posix_path`)
- Canonical target: `canonical:open.local_file:single_file` (`literal_is_canonical`)
- Detected target kind: `local_file`
- Boundary: `local_writable` (`boundary:flow:open.local_file:local_writable`)
- Overwrite posture: `no_write_action` (checkpoint: `not_applicable`)
- Overwrite review: `save:overwrite_review:checkpoint_aware:v1`
- Reveal side effect: `no_external_side_effect`
- Filesystem identity: `filesystem-identity:flow:open.local_file`
- Save coordination: `save-coordination:flow:open.local_file`
- Active profile owner: `profile-owner:flow:open.local_file`
- Trust checkpoint: `trust:flow:open.local_file:profile_policy`
- Canonical command: `cmd:workspace.open.target`
- Path condition: `exact_available`
- Recovery actions: _(none required)_
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:open_save_reveal:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A system open of a local file targets the literal file the user selected, which is its own canonical target, and reads it without widening scope.
- Degraded-state vocabulary:
  - Open this file
  - This file is no longer at the path you selected
  - Choose a different file

Findings: none.

### `flow:open_in_browser.generated_preview` (open_in_default_browser)

- Descriptor revision: `flow:open_in_browser.generated_preview:rev:2026.06.01-01`
- Literal target: `literal:open_in_browser.generated_preview:captured` (`url`)
- Canonical target: `canonical:open_in_browser.generated_preview:generated_html` (`boundary_labeled_artifact`)
- Detected target kind: `local_file`
- Boundary: `generated` (`boundary:flow:open_in_browser.generated_preview:generated`)
- Overwrite posture: `no_write_action` (checkpoint: `not_applicable`)
- Overwrite review: `save:overwrite_review:checkpoint_aware:v1`
- Reveal side effect: `opens_default_browser`
- Reveal action label: `action:open_in_browser.generated_preview:open_in_default_browser`
- Filesystem identity: `filesystem-identity:flow:open_in_browser.generated_preview`
- Save coordination: `save-coordination:flow:open_in_browser.generated_preview`
- Active profile owner: `profile-owner:flow:open_in_browser.generated_preview`
- Trust checkpoint: `trust:flow:open_in_browser.generated_preview:profile_policy`
- Canonical command: `cmd:workspace.open_in_default_browser`
- Path condition: `exact_available`
- Recovery actions: _(none required)_
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:open_save_reveal:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: Open in default browser is a stable, explicit action that hands a generated preview artifact to the default browser; the target is labeled generated so it is never mistaken for an editable source.
- Degraded-state vocabulary:
  - Open in default browser
  - This is a generated preview, not the source file
  - Open the source that generated this instead

Findings: none.

### `flow:reveal.local_file` (reveal_in_system_shell)

- Descriptor revision: `flow:reveal.local_file:rev:2026.06.01-01`
- Literal target: `literal:reveal.local_file:captured` (`posix_path`)
- Canonical target: `canonical:reveal.local_file:single_file` (`literal_is_canonical`)
- Detected target kind: `local_file`
- Boundary: `local_writable` (`boundary:flow:reveal.local_file:local_writable`)
- Overwrite posture: `no_write_action` (checkpoint: `not_applicable`)
- Overwrite review: `save:overwrite_review:checkpoint_aware:v1`
- Reveal side effect: `selects_target_in_file_manager`
- Reveal action label: `action:reveal.local_file:reveal_in_system_shell`
- Filesystem identity: `filesystem-identity:flow:reveal.local_file`
- Save coordination: `save-coordination:flow:reveal.local_file`
- Active profile owner: `profile-owner:flow:reveal.local_file`
- Trust checkpoint: `trust:flow:reveal.local_file:profile_policy`
- Canonical command: `cmd:workspace.reveal_in_system_shell`
- Path condition: `exact_available`
- Recovery actions: _(none required)_
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:open_save_reveal:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: Reveal in system shell is a stable, explicit action: it opens the OS file manager and selects the canonical target, and discloses that external side effect rather than hiding it.
- Degraded-state vocabulary:
  - Reveal in system shell
  - This opens your file manager and selects the file
  - Reveal the parent folder instead

Findings: none.

### `flow:save.local_file` (save)

- Descriptor revision: `flow:save.local_file:rev:2026.06.01-01`
- Literal target: `literal:save.local_file:captured` (`posix_path`)
- Canonical target: `canonical:save.local_file:single_file` (`literal_is_canonical`)
- Detected target kind: `local_file`
- Boundary: `local_writable` (`boundary:flow:save.local_file:local_writable`)
- Overwrite posture: `overwrite_with_checkpoint` (checkpoint: `pinned`)
- Checkpoint: `checkpoint:save.local_file:pre_overwrite`
- Overwrite review: `save:overwrite_review:checkpoint_aware:v1`
- Reveal side effect: `no_external_side_effect`
- Filesystem identity: `filesystem-identity:flow:save.local_file`
- Save coordination: `save-coordination:flow:save.local_file`
- Active profile owner: `profile-owner:flow:save.local_file`
- Trust checkpoint: `trust:flow:save.local_file:profile_policy`
- Canonical command: `cmd:workspace.save.target`
- Path condition: `exact_available`
- Recovery actions: _(none required)_
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:open_save_reveal:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: An in-place save overwrites the canonical target only after pinning a checkpoint, using the same checkpoint-aware overwrite review the in-product save flow performs.
- Degraded-state vocabulary:
  - Save changes to this file
  - This will overwrite the file on disk
  - Restore from the checkpoint taken before this save

Findings: none.

### `flow:save_as.local_file` (save_as)

- Descriptor revision: `flow:save_as.local_file:rev:2026.06.01-01`
- Literal target: `literal:save_as.local_file:captured` (`posix_path`)
- Canonical target: `canonical:save_as.local_file:new_file` (`literal_is_canonical`)
- Detected target kind: `local_file`
- Boundary: `local_writable` (`boundary:flow:save_as.local_file:local_writable`)
- Overwrite posture: `create_new_file` (checkpoint: `not_applicable`)
- Overwrite review: `save:overwrite_review:checkpoint_aware:v1`
- Reveal side effect: `no_external_side_effect`
- Filesystem identity: `filesystem-identity:flow:save_as.local_file`
- Save coordination: `save-coordination:flow:save_as.local_file`
- Active profile owner: `profile-owner:flow:save_as.local_file`
- Trust checkpoint: `trust:flow:save_as.local_file:profile_policy`
- Canonical command: `cmd:workspace.save_as.target`
- Path condition: `exact_available`
- Recovery actions: _(none required)_
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:open_save_reveal:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A save-as writes to the new literal target the user named; because no file exists there yet it creates a new file rather than overwriting, so no checkpoint is required.
- Degraded-state vocabulary:
  - Save a copy to a new file
  - A file with this name already exists here
  - Choose a different name or location

Findings: none.

## Verification

```sh
cargo run -q -p aureline-workspace --bin aureline_workspace_m5_open_save_reveal -- validate
cargo test -p aureline-workspace --test m5_open_save_reveal_fixtures
python3 tools/ci/m5/open_save_reveal_check.py
```
