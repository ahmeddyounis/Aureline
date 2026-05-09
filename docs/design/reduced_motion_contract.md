# Reduced-motion policy and protected-path animation guards

Aureline treats reduced motion as a first-class runtime posture. Protected UI
surfaces MUST consult the shared policy before running transitions so:

- motion never becomes the only carrier of state,
- typing, palette input, and decision-making are never delayed by decoration,
- reduced-motion and constrained postures collapse, suppress, or simplify motion
  while preserving focus visibility and state conveyance.

## Canonical sources

- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` (§10 Motion system)
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` (Performance-aware interaction standards → Motion)
- `docs/design/motion_timing_contract.md` (normative motion-token + substitution rules)
- `artifacts/design/motion_tokens.yaml` (duration + easing token ledger)
- `fixtures/design/motion_cases/` (worked motion presets)

## Runtime posture vocabulary

Reduced-motion is carried by the appearance-session record:

- `crates/aureline-ui/src/themes/session.rs`
  - [`AccessibilityPostureClass`] defines the posture ladder:
    `motion_standard` → `motion_reduced` → `motion_low_motion` →
    `motion_power_saver` → `motion_critical_hot_path`.

The shell persists the active posture in `appearance_session.json` alongside
theme/density state so the behavior is explicit and restart-stable.

## Shared motion contract (UI crate)

`crates/aureline-ui/src/motion/mod.rs` publishes the cross-surface contract:

- [`MotionPresetDefinition`] describes a preset in terms of token names.
- [`MotionPresetDefinition::plan_for`] selects the correct fallback row for the
  active posture.
- [`MotionPlan`] resolves duration tokens via the seeded [`TokenRegistry`].

Protected consumers should:

1. Choose a canonical preset (for overlays, use `OVERLAY_DIALOG_ENTER`).
2. Call `plan_for(appearance.reduced_motion_posture)`.
3. Treat missing/unknown tokens as `motion.instant` (do not block input).
4. Apply the substitution class:
   - `crossfade_only`: opacity-only (no translation/scale)
   - `collapse_to_instant` / `suppress_entirely`: no transition delay

## Shell integration (one protected consumer)

`crates/aureline-shell/src/bootstrap/native_shell.rs` wires the policy into the
transient-overlay consumer:

- Command palette overlay (search) uses the `overlay.dialog.enter` preset.
- Shell overlays (trust/help/inspector sheets) use the same guard.
- `Cmd/Ctrl+Alt+Shift+M` cycles the reduced-motion posture for live validation.

The failure drill is supported by construction: changing posture while an enter
transition is in-flight snaps the next rendered frame to the constrained plan
(typically instant).

