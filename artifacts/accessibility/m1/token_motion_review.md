# Early accessibility review: token / state / reduced-motion audit

This packet is what the early accessibility review cites at M1 exit when
checking that Aureline's protected shell surfaces consume the tokenized
appearance, focus, and motion contracts the design system promises —
not ad hoc styling islands.

## Why this matters for accessibility

Three of the most common accessibility failures in shell UIs come from
surfaces drifting away from the shared design contracts:

1. **Lost focus visibility under reduced motion.** A surface that
   stops consulting the shared motion module can swap a transition for
   an instant cut without preserving the focus ring or selection
   marker. Keyboard-only and AT users lose the only signal that a row
   is now active.
2. **Color-only state cues.** A surface that stops calling
   `TokenRegistry::require_color("status.warning.*" / "status.danger.*")`
   collapses warning/error/restricted postures to hue alone. High
   contrast and forced-colors users no longer perceive the state.
3. **Hand-coded motion durations.** A surface that hand-codes a
   `Duration::from_millis(200)` motion duration silently bypasses the
   `motion_reduced` / `motion_low_motion` / `motion_critical_hot_path`
   substitution rules.

The token / state / reduced-motion audit catches all three classes
before late-stage design review.

## Protected walk (what was confirmed)

| Surface         | Token adoption | State vocabulary       | Motion preset reuse                                                                                       |
| --------------- | -------------- | ---------------------- | --------------------------------------------------------------------------------------------------------- |
| shell_chrome    | OK             | FOCUS_VISIBLE, SELECTED | overlay_dialog_enter, overlay_dialog_exit (preserves_state_conveyance + preserves_focus_visibility)       |
| start_center    | OK             | FOCUS_VISIBLE, SELECTED | overlay_dialog_enter, overlay_dialog_exit                                                                  |
| search_palette  | OK             | FOCUS_VISIBLE          | overlay_dialog_enter, overlay_dialog_exit                                                                  |
| trust_surface   | OK             | FOCUS_VISIBLE          | overlay_dialog_enter, overlay_dialog_exit                                                                  |

Latest capture (raw observed sets per surface):
[`artifacts/milestones/m1/captures/token_motion_audit_validation_capture.json`](../../milestones/m1/captures/token_motion_audit_validation_capture.json).

## Failure drill (what would break the lane)

The audit's named drills cover one regression class per surface:

- **`shell_chrome_drop_focus_ring_token`** — drop the
  `al.color.focus.ring` token call; lane reports
  `token_state_audit.required_token.missing` so the focus ring cannot
  silently disappear from the shell.
- **`start_center_drop_selected_state`** — drop the
  `ComponentStates::SELECTED` symbol; lane reports
  `token_state_audit.required_state.missing` so a chosen Start Center
  row cannot collapse into "looks the same as focused".
- **`search_palette_drop_overlay_motion_preset`** — drop the
  `overlay_dialog_enter.yaml` motion-preset reference; lane reports
  `token_state_audit.required_motion_preset.missing` so the palette
  cannot regress to a per-surface timer.
- **`trust_surface_drop_warning_token`** — drop the
  `status.warning.border` token call; lane reports
  `token_state_audit.required_token.missing` so the trust chip cannot
  collapse to color-alone hue.

Replay any drill with:

```bash
python3 tests/ux/token_state_audit/run_token_state_audit.py \
  --repo-root . \
  --force-drill <drill_id>
```

## Citation pointers for the M1 exit review

When citing this audit during M1 exit:

- Reviewer entrypoint: [`artifacts/ux/m1_token_and_motion_audit.md`](../../ux/m1_token_and_motion_audit.md)
- Proof packet: [`artifacts/milestones/m1/proof_packets/token_motion_audit.md`](../../milestones/m1/proof_packets/token_motion_audit.md)
- Latest capture: [`artifacts/milestones/m1/captures/token_motion_audit_validation_capture.json`](../../milestones/m1/captures/token_motion_audit_validation_capture.json)
- Lane registration: [`artifacts/milestones/m1/artifact_index.yaml`](../../milestones/m1/artifact_index.yaml) (`token_motion_audit`)

## Refresh trigger

Refresh this packet when:

- a new protected M1 surface joins the shell (extend the case fixture
  set under `fixtures/ux/reduced_motion_cases/`);
- the shared token / state / motion contracts shift (re-run the audit
  and capture the new observed sets);
- an accessibility regression on one of these surfaces lands and is
  fixed (cite the drill class that would catch it next time, and add
  a row if a new class is required).
