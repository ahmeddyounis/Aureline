# OS appearance live-change matrix and restart/reload disclosure rules

This document publishes Aureline’s **desktop appearance change matrix**: how
the product reacts when the operating system changes **system theme**,
**contrast / forced-colors posture**, **system accent**, **system text scale**,
or **reduced-motion** settings.

The goal is to stop appearance changes from becoming **implicit** or
**inconsistent** across:

- the shell chrome and first-party surfaces;
- dialogs / capability sheets; and
- embedded or extension-hosted surfaces.

If this document and
[`/artifacts/design/appearance_live_change_matrix.yaml`](../../artifacts/design/appearance_live_change_matrix.yaml)
ever disagree, the YAML wins for tooling and this document must be updated in
the same change.

## Companion contracts and artifacts

- [`/docs/design/appearance_session_contract.md`](./appearance_session_contract.md)
  — canonical appearance-session state, live follow-system policy, and revision
  event vocabulary.
- [`/docs/ux/appearance_import_and_checkpoint_contract.md`](../ux/appearance_import_and_checkpoint_contract.md)
  and
  [`/schemas/ux/appearance_checkpoint.schema.json`](../../schemas/ux/appearance_checkpoint.schema.json)
  — checkpoint/rollback atomicity vocabulary, including `surface_reload_*` and
  `full_restart_*` apply paths.
- [`/docs/ux/theme_and_visual_asset_contract.md`](../ux/theme_and_visual_asset_contract.md)
  — the `appearance_change_posture` vocabulary reused for theme/asset changes
  (`live_apply`, `surface_reload_required`, `full_restart_required`).
- [`/artifacts/platform/claimed_desktop_profiles.yaml`](../../artifacts/platform/claimed_desktop_profiles.yaml)
  — the **only** claimed desktop profile roster this matrix applies to.
- [`/fixtures/design/os_appearance_transition_cases/`](../../fixtures/design/os_appearance_transition_cases/)
  — worked transition cases used by QA and support exports.

Normative sources projected here:

- `.t2/docs/Aureline_UI_UX_Spec_Document.md` “Platform conventions” (`Theme/contrast changes`)
  and the “Native desktop integration…” contract.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` “System theme and contrast”
  row (platform conventions table).
- `.t2/docs/Aureline_PRD.md` “Theme engine” acceptance criterion (“Theme change
  applies without restart”).

## 1. Definitions

### 1.1 Appearance axes (OS-triggered)

This matrix covers five OS-triggered appearance axes (names match the
appearance-session contract vocabulary):

- `mode_theme_class` (OS light/dark preference → resolved theme class)
- `contrast_mode` (standard/high contrast/forced colors posture)
- `accent_source` (system accent / highlight)
- `text_scale` (system text scale / accessibility text size where available)
- `reduced_motion_posture` (OS reduced-motion preference)

### 1.2 Surface families (where the change must land)

OS-triggered appearance changes are evaluated per surface family:

- **First-party shell surfaces**: shell chrome and first-party dialogs/sheets.
- **Embedded / extension-hosted surfaces**: webview-like or extension-rendered
  UI that appears inside host chrome and may have independent rendering stacks.
- **Native platform dialogs**: OS-provided dialogs (open/save/print) where the
  OS owns appearance; Aureline may only control the surrounding framing and
  disclosure.

### 1.3 “Live apply”, “surface reload”, and “full restart”

The appearance contracts treat “apply” as an atomic operation:

- **Live apply**: the surface updates in-place without destroying the surface.
- **Surface reload required**: a specific embedded/extension surface must be
  reloaded/recreated to guarantee consistent tokens and protected cues.
- **Full restart required**: the whole app must restart to guarantee consistent
  tokens and protected cues (used sparingly; never silently).

When a change is not fully live-applicable, the user must see a precise posture
(`Reload required` vs `Restart required`) and a bounded action that restores
consistency.

## 2. Non-negotiable invariants

1. **No half-truth about protected cues.** Trust, policy-lock, severity, and
   source-integrity cues must not end up visually contradictory across surfaces
   after an OS-triggered change.
2. **State is attributable.** Users must be able to tell whether Aureline:
   - followed the OS signal,
   - held a local override, or
   - deferred the change behind an explicit checkpoint + reload/restart posture.
3. **Apply/revert is atomic.** If any surface cannot participate safely in a
   live apply, the system must either:
   - reload/recreate that surface, or
   - explicitly defer the change for that surface behind a disclosure gate,
     without leaving stale content pretending it is current.
4. **No silent restarts.** OS-triggered appearance changes may require a restart
   on some profiles/surfaces, but the restart is always an explicit user action.

## 3. How OS-triggered changes are processed (pipeline)

1. **Detect OS signal** and map it to one or more appearance axes.
2. **Resolve current appearance-session posture**:
   - follow-system vs manual override vs managed-policy override vs
     platform-unavailable.
3. **Consult the live follow-system policy** (`live_follow_system_policy_record`)
   to determine, per axis, whether the signal:
   - applies live,
   - applies live with a revertable checkpoint,
   - requires explicit confirm/review, or
   - is policy-blocked.
4. **Consult the platform/surface matrix**
   (`artifacts/design/appearance_live_change_matrix.yaml`) to determine whether
   the target surfaces can actually apply the change live, or require surface
   reload / full restart for atomicity.
5. **Apply with disclosure**:
   - If every affected surface family can apply live: apply per policy.
   - If any affected embedded/extension surface requires reload: the host must
     immediately enter a **disclosed pending** posture for that surface (badge +
     in-surface banner) and offer `Reload surface`.
   - If the matrix declares full restart: show an explicit `Restart required`
     disclosure and a bounded `Restart now` action (plus `Later`).

## 4. Follow-system vs override transitions (what users must be able to tell)

### 4.1 Follow-system posture

When `follow_system_posture = follow_system`:

- OS signals may revise the appearance session (`cause_class = os_signal_change`)
  and must be recorded as appearance-session revision events.
- Each revision must be explainable as “followed OS signal X on axis Y”, and
  must identify whether the axis applied live, required a checkpoint, or was
  deferred behind reload/restart disclosure.

### 4.2 Manual (local) override

When `follow_system_posture = manual_override`:

- OS theme/contrast/accent changes **do not** silently change the effective
  appearance for that axis.
- Aureline must still disclose the OS change in a bounded, non-modal way
  (e.g., status item or toast) so users can tell why the app did not change:
  `System appearance changed; Aureline is using a local override.`
- The disclosure must offer a single action that re-enters follow-system mode
  (subject to the live policy’s confirm/checkpoint requirements).

### 4.3 Managed-policy override

When `follow_system_posture = managed_policy_override`:

- The UI must treat OS signals as non-authoritative for policy-blocked axes and
  must surface the lock source (policy epoch / admin source) via the appearance
  session inspector.
- A policy-blocked axis must never appear to “sometimes follow” the OS.

### 4.4 Preview/checkpoint interaction

If `preview_state ∈ {preview_pending_validation, preview_live}`:

- OS-triggered changes must not produce an untracked second rollback handle.
- The system must either:
  - **defer** the OS-triggered change until preview commit/revert completes, or
  - fold the OS-triggered change into the active checkpoint scope and disclose
    that the preview now includes an OS-triggered delta.

The transition-case fixtures include worked examples of these interactions.

## 5. Disclosure rules (reload/restart)

When a surface cannot fully live-update:

1. **Say exactly what is required**:
   - `Reload required` for a specific embedded/extension surface.
   - `Restart required` only when the whole app must restart to guarantee
     consistent tokens and protected cues.
2. **Keep the mismatch local and obvious**:
   - The affected surface must show an in-surface banner explaining the mismatch
     and offering the action.
   - The shell must also show a single global indicator (“1 surface needs
     reload”) so users can find and resolve pending mismatches.
3. **Never leave stale surfaces unmarked.**
4. **Never auto-reload destructive surfaces.** A reload that might discard
   in-memory state must always be user-directed, or must route through a
   checkpoint-backed “Reload with restore” path.

## 6. Worked transition cases

The fixture corpus under
[`/fixtures/design/os_appearance_transition_cases/`](../../fixtures/design/os_appearance_transition_cases/)
includes:

- OS theme flip applied live on a claimed macOS profile.
- OS forced-colors / high-contrast flip that requires embedded surface reload on
  a claimed Windows profile.
- OS reduced-motion flip applied live with a revertable checkpoint.
- OS text-scale change held behind an explicit confirm step.
- OS theme flip ignored under manual override, with a disclosure offering
  re-entering follow-system posture.

## 7. Out of scope

- Native OS integration code and signal-watcher implementation.
- Theme engine implementation details (token resolver, renderer plumbing).
- Final platform-specific UI strings; this document defines behavior and
  disclosure *posture*, not localization-ready copy.

