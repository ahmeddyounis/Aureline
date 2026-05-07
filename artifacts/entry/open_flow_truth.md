# Open-flow disambiguation, multi-root action truth, and stable-command parity (Entry Surfaces)

This artifact publishes the canonical **open-flow disambiguation** rules for every entry surface that can open or acquire a target (Start Center, menus, command palette, OS handoff, protocol/deep-link handlers, and CLI/headless previews).

Its goal is supportable truth: the product MUST NOT guess whether the user meant **open a file**, **open a folder**, **open a workspace/workset**, **add a root (multi-root)**, **open in a new window**, **remote attach/resume**, **clone**, or **import** when the consequences differ materially.

This file does **not** define final UI composition. It freezes **what must be explicit before commit** so later surfaces cannot rename routes or hide scope changes behind vague “Open …” language.

## 1. Canonical sources (quoted by reference)

Entry verbs, target kinds, and resulting modes (closed sets):

- `docs/workspace/entry_restore_object_model.md`
- `docs/ux/workspace_entry_route_matrix.md`

Chooser rows + open-flow sheets (pre-commit invariants and disclosure axes):

- `docs/ux/project_entry_contract.md`
- `schemas/ux/entry_chooser_row.schema.json`
- `schemas/ux/open_flow_sheet.schema.json`

Warm-start chooser honesty (resume live vs snapshot vs clone fresh vs open without starter):

- `artifacts/entry/warm_start_chooser_contract.md`
- `schemas/entry/freshness_revalidation.schema.json`
- `fixtures/entry/warm_start_cases/`

Multi-root and scope widening truth (evidence and diff record):

- `docs/workspace/scope_truth_packet.md`
- `docs/workspace/scope_widening_and_cross_repo_jump_contract.md`

Cross-window switching truth (new window vs replace vs focus existing):

- `artifacts/entry/workspace_switch_preview_contract.md`
- `schemas/entry/workspace_switch_delta.schema.json`

Stable command identity and parity expectations:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` (stable command IDs + one command graph)
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` (entry verbs and resulting modes stable across surfaces)
- `artifacts/commands/command_registry_seed.yaml` (seed command IDs and alias lifecycle)

Machine-readable parity packet for this artifact:

- `artifacts/entry/open_route_command_map.yaml`
- `fixtures/entry/open_flow_cases/`

## 2. Terms (what “open route” means here)

An **open route** is the combination of:

- the committed `entry_verb` (open/clone/import/add_root/restore/resume/start_from_snapshot),
- the resolved `target_kind`,
- the resolved `resulting_mode`,
- and the **scope consequence** (replace current workspace, open in new window while keeping current, or widen active workspace roots).

Two invocations that differ in scope consequence are **not** “the same open”, even when they originate from the same menu label.

## 3. Disambiguation rules (non-negotiable)

1. **No ambiguous “Open project” copy when consequences differ.**
   - If the choice could either replace the current workspace, open in a new window, or widen into multi-root, the surface MUST present those as distinct, named choices.
2. **The product never infers clone vs attach/resume.**
   - A remote repository clone (`entry_verb = clone`, `target_kind = remote_repository`) and a remote workspace attach/resume (`entry_verb = resume`, `target_kind = managed_cloud_workspace` / `ssh_workspace` / container variants) are different routes and MUST NOT be auto-selected from vague input.
3. **Multi-root widening is always explicit and evidenced.**
   - `add_root` emits a `scope_widen_diff_record` before commit; silent widening is forbidden.
4. **Cross-window consequences are explicit and preserved.**
   - If the route could displace unsaved work or replace the active workspace, the surface MUST follow the switch-preview contract (and preserve a keep-current-open path).
5. **Parity is command-backed.**
   - Every surface invocation resolves to the same stable command identity for the same route (see `artifacts/entry/open_route_command_map.yaml`).

## 4. Open-flow “truth states” (minimum set)

Each state below is the minimum pre-commit truth that must be reconstructable from exported artifacts (sheet records, switch previews, scope widen diffs, and invocation sessions).

### 4.1 Open local file (single file)

- **Entry verb**: `open`
- **Target kind**: `local_file`
- **Resulting mode**: `single_file`
- **Sheet class**: `open_local_target_no_review_required` when there is no trust/policy/profile/runtime/destination delta; otherwise `open_local_target`
- **Scope consequence**: does not widen roots; may replace current editor focus but does not redefine workspace roots
- **Recents**: record a `recent_work_entry_record` with `target_kind = local_file` (or update existing)

### 4.2 Open local folder (single-root workspace boundary)

- **Entry verb**: `open`
- **Target kind**: `local_folder` (or `local_repo_root` when explicitly chosen)
- **Resulting mode**: `folder` or `repo_root` (resolved before commit)
- **Sheet class**: `open_local_target`
- **Scope consequence** (must be explicit when a workspace is already active):
  - replace the current workspace, or
  - open in a new window while keeping the current workspace open
- **Recents**: record a `recent_work_entry_record` with `target_kind = local_folder` / `local_repo_root`

### 4.3 Open workspace / workset manifest (multi-root boundary by manifest)

- **Entry verb**: `open`
- **Target kind**: `workspace_manifest` or `workset_manifest`
- **Resulting mode**: `workspace_with_roots` or `workset_slice` (resolved before commit)
- **Sheet class**: `open_local_target`
- **Scope consequence**: replacing the active workspace boundary (or opening in a new window) is handled via the switch-preview contract when material
- **Recents**: record a `recent_work_entry_record` with `target_kind = workspace_manifest` / `workset_manifest`

### 4.4 Add root to active workspace (explicit multi-root widening)

- **Entry verb**: `add_root`
- **Target kind**: local or remote root kinds allowed by the entry object model
- **Resulting mode**: `workspace_with_roots` (or `workset_slice` when explicitly widening a slice)
- **Sheet class**: `add_root_to_active_workspace`
- **Scope consequence**: widens roots; never “opens a new workspace”
- **Evidence**: MUST emit a `scope_widen_diff_record` before commit
- **Recents**: update the active workspace’s recent entry (do not mint a second ambiguous “opened project” row for the newly added root)

### 4.5 Open folder in a new window (preserve current workspace)

This is not a separate entry verb; it is a **scope consequence** that must be captured explicitly:

- **Entry verb**: `open`
- **Target kind**: `local_folder` / `local_repo_root` / manifest kinds
- **Resulting mode**: the resolved open mode (`folder`, `repo_root`, `workspace_with_roots`, etc.)
- **Cross-window truth**: MUST be represented via a `workspace_switch_preview_record` choice
  (`unsaved_buffer_handling_class = open_target_in_new_window_keep_current_open`), so support can replay the decision.

### 4.6 Remote attach / resume (live workspace reattach)

- **Entry verb**: `resume`
- **Target kind**: `managed_cloud_workspace`, `ssh_workspace`, `container_workspace`, `devcontainer_workspace`
- **Resulting mode**: `resume_live_session`
- **Sheet class**: `restore_or_resume`
- **Scope consequence**: may replace current workspace or open in a new window; when material, must follow switch-preview rules before commit
- **Recents**: record a `recent_work_entry_record` with the corresponding remote `target_kind`

### 4.7 Clone remote repository (materialize bytes without implying trust)

- **Entry verb**: `clone`
- **Target kind**: `remote_repository`
- **Resulting mode**: one of `clone_then_review`, `clone_then_open`, `clone_then_add`, `clone_only` (resolved before commit)
- **Sheet class**: `clone_remote_target`
- **Destination truth**: destination disposition + collision class must be previewed before any writes
- **Recents**: after successful materialization, the chosen post-clone action determines the recent target (clone-only vs open/add)

### 4.8 Import artifact (portable state / handoff / competitor / support)

- **Entry verb**: `import`
- **Target kind**: `portable_state_package`, `handoff_packet`, `competitor_config_root` (and other importer kinds)
- **Resulting mode**: `inspect_only`, `extract_then_review`, `apply_to_active_workspace`, or `compare_before_restore` (resolved before commit)
- **Sheet class**: `import_artifact`
- **Preview truth**: import compare/dry-run must be available when mutation is possible
- **Recents**: import does not implicitly “open” a workspace unless the resulting mode explicitly does so

## 5. Stable-command parity (what cannot drift)

For every open route, every invocation surface MUST be able to point to:

- one stable command identity (`cmd:*`) for automation/support/audit, and
- one UX command ref (`command.*`) for chooser-row and sheet records.

The mapping is frozen in:

- `artifacts/entry/open_route_command_map.yaml`

Surfaces MUST NOT invent new route names, new command IDs, or new “Open …” labels for the same underlying command without updating that mapping (and the corresponding fixtures).

## 6. Fixture corpus (parity proofs)

The fixtures under `fixtures/entry/open_flow_cases/` are the seed parity corpus proving that:

- Start Center, main menu, command palette, protocol handlers, and deep links all resolve through the same open-route vocabulary; and
- multi-root widening and cross-window opening are explicit choices, not silent guesses.
