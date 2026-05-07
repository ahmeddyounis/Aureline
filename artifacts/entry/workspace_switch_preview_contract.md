# Workspace-switch Preview and Unsaved-Buffer Carry Contract (Entry Surfaces)

This artifact publishes the canonical **workspace-switch preview** and **unsaved-buffer carry** vocabulary used by the Start Center, workspace switcher, recent-work reentry, and protocol/deep-link entry surfaces.

Its goal is cross-surface honesty: switching workspaces MUST NOT silently change **execution behavior**, **trust posture**, **policy restrictions**, or **unsaved local work**.

This file does **not** define final UI composition. It freezes **what must be previewed before commit**, which deltas are **material**, and which **carry / preserve** behaviors are required so every surface renders the same truthful switch story.

## 1. Canonical sources (quoted by reference)

This artifact binds existing entry/switching contracts into one switch-preview corpus:

- Entry and route preview (pre-commit invariants and fallback parity):
  - `docs/ux/project_entry_contract.md`
  - `docs/ux/workspace_entry_route_matrix.md`
  - `docs/ux/preview_apply_revert_contract.md`
- Switcher row anatomy and cross-window consequence disclosure:
  - `docs/ux/recent_work_and_restore_card_contract.md`
  - `schemas/ux/recent_work_row.schema.json`
- Trust / restricted-mode ownership:
  - `docs/ux/trust_prompt_contract.md`
  - `docs/adr/0018-workspace-trust-and-restricted-mode.md`
- Dirty-buffer, pinning, and close/reopen semantics:
  - `docs/ux/tab_state_contract.md`
  - `docs/ux/editor_document_state_contract.md`
  - `docs/ux/shell_close_reopen_contract.md`
  - `docs/ux/cross_window_transfer_contract.md`
- System-level requirement sources for switching (authoritative):
  - `.t2/docs/Aureline_Technical_Design_Document.md` (Start center + switching rules)
  - `.t2/docs/Aureline_UI_UX_Spec_Document.md` (Start center and switching)
  - `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` (switcher and unsaved-buffer expectations)

Machine-readable switch delta + audit records:

- `schemas/entry/workspace_switch_delta.schema.json`
- `fixtures/entry/workspace_switch_cases/`

## 2. Switch preview boundary (must happen before commit)

Every workspace-switch activation that could change execution behavior, trust, policy, target kind, or unsaved-buffer posture MUST produce a **switch preview** before any durable state changes or authority changes occur.

Switch-preview invariants:

1. **No switch after-the-fact surprises.** Material deltas MUST be shown before the user commits the switch. Revealing deltas only after the new workspace is already active is non-conforming.
2. **Cancel is always real.** Until commit, `cancel_switch` MUST leave the current workspace untouched.
3. **Fallback is always named.** A preview MUST include at least one safe fallback action (for example `cancel_switch`, `open_in_new_window`, `reopen_previous_workspace`, or `open_read_only_cached_view` when applicable).
4. **Unsaved buffers are preserved by default.** A switch MUST NOT discard or close unsaved buffers without an explicit, reviewable user choice.
5. **Audit is emitted.** The user’s switch decision and (when applicable) unsaved-buffer choice MUST be exportable as a decision/outcome record.

## 3. Required delta rows (the “switch preview table”)

Every switch preview MUST include the six rows below, even when a row is “No change”, so users can scan the same stable table across surfaces:

1. **Root delta** — what workspace boundary is being switched (single-root vs multi-root, local vs remote-root class).
2. **Profile delta** — which profile becomes active (including lock/override posture).
3. **Target delta** — what target kind/environment mode the workspace binds to (local/remote/container/managed).
4. **Trust delta** — trust state change and whether a trust review is required.
5. **Capability delta** — which capability classes materially change (especially run/debug/test, AI assist, extension operations, remote attach).
6. **Policy delta** — policy epoch/authority-delta changes, required reviews, and any policy-blocked capability classes.

### 3.1 Materiality rules (what “matters”)

A delta row MUST be marked material when it changes either:

- **Execution behavior** (what can run, where it runs, what is attached, what is gated, what becomes evidence-only), or
- **Support / safety expectations** (trust boundary, policy boundary, managed-vs-local responsibility, audit/export requirements).

When a delta is material, the preview MUST:

- require explicit commit (and MAY require explicit confirmation); and
- include a short reason summary that does not rely on color, iconography, or implied knowledge.

## 4. Unsaved-buffer carry contract (switching with dirty work)

Switching workspaces can change identity, trust, and execution posture; unsaved buffers therefore have an explicit carry policy.

### 4.1 Invariants (non-negotiable)

1. **No silent discard.** Dirty buffers and recovered drafts MUST NOT be silently closed, overwritten, or dropped during a switch.
2. **Pinned buffers are anchors.** Pinned + dirty buffers MUST be treated as deliberate anchors: a switch that would close them MUST require an explicit reviewed choice (and MUST offer a preserve path).
3. **Preserve-first default.** When dirty buffers exist, the default switch posture MUST preserve them without requiring the user to make them safe first (for example by opening the target in a new window or suspending the previous workspace as reopenable).
4. **Cross-window truth is explicit.** The preview MUST state whether activation focuses an existing window, opens a new window, replaces the current workspace, or is blocked pending a decision.
5. **Failure does not destroy unsaved work.** If the new workspace fails to open/attach/admit, the user MUST still be able to return to the previous workspace with unsaved buffers intact via `reopen_previous_workspace` (or an equivalent preserved-work path).

### 4.2 Carry choices (stable vocabulary)

Surfaces MUST express unsaved-buffer handling using one of these choice classes:

- `open_target_in_new_window_keep_current_open` — preferred default when dirty buffers exist.
- `focus_existing_window_keep_current_open` — preferred when the target is already open elsewhere.
- `suspend_current_workspace_reopenable` — allowed when a new window is not possible, but the prior workspace remains reopenable.
- `discard_unsaved_buffers` — allowed only behind an explicit reviewed confirmation; never the default.

Surfaces MAY offer fewer choices when policy or window constraints require it, but they MUST still provide at least one preserve-first choice plus `cancel_switch`.

### 4.3 Switching across local/remote/managed/container/multi-root

When switching between materially different target classes (local ⇄ remote ⇄ container ⇄ managed, or single-root ⇄ multi-root):

- The switch preview MUST treat **target** and **capability** deltas as material by default.
- If the switch would change where execution occurs (local → remote attach, managed → local fallback, container attach → local-only), the preview MUST say so before commit.
- Any downgrade to restricted mode, policy-blocked execution, or authority rebind MUST surface as a material trust/policy delta (not a passive badge that appears after the switch completes).

## 5. Exportable records (preview, decision, outcome)

The switch preview and the resulting decision/outcome are exportable records governed by:

- `schemas/entry/workspace_switch_delta.schema.json`

Required export behavior:

- A preview record is emitted before commit.
- A decision record is emitted when the user commits/cancels, including the chosen unsaved-buffer handling choice when relevant.
- An outcome record is emitted after the switch resolves (success/failure/cancel), including whether unsaved buffers were preserved.

## 6. Non-conforming examples (for reviewers)

The following behaviors are contract violations:

- a switch that activates the new workspace and only then reveals it is restricted, policy-blocked, or uses a different execution target;
- a switch that closes dirty buffers without an explicit reviewed discard choice;
- a switch whose “Cancel” path already destroyed the current workspace context;
- a switcher entry that hides whether it focuses another window vs opening a new one vs replacing the current workspace;
- a switch that fails to open the target and leaves no `reopen_previous_workspace` path.

