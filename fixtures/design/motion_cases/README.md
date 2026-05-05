# Motion timing and semantic-equivalence fixtures

Worked fixtures for the motion timing / easing token ledger and motion-preset
semantic-equivalence contract frozen in:

- [`/docs/design/motion_timing_contract.md`](../../../docs/design/motion_timing_contract.md)

and validated by:

- [`/schemas/design/motion_transition.schema.json`](../../../schemas/design/motion_transition.schema.json)

Each case is a single `motion_preset_record` describing the default motion tokens
plus the reduced-motion, low-motion, power-saver, and critical-hot-path
fallbacks that preserve meaning when motion is simplified or removed.

## Fixtures

- [`overlay_dialog_enter.yaml`](./overlay_dialog_enter.yaml)
  — Dialog/sheet entry preset with reduced-motion crossfade/instant fallbacks.
- [`overlay_dialog_exit.yaml`](./overlay_dialog_exit.yaml)
  — Dialog/sheet exit preset with reduced-motion crossfade/instant fallbacks.
- [`banner_state_change.yaml`](./banner_state_change.yaml)
  — Banner show/update preset with no-layout-shift expectations under reduced motion.
- [`toast_enter.yaml`](./toast_enter.yaml)
  — Toast entry preset that never steals focus and preserves static labels.
- [`toast_exit.yaml`](./toast_exit.yaml)
  — Toast dismissal preset with interruption/cancellation expectations.
- [`durable_job_progress.yaml`](./durable_job_progress.yaml)
  — Progress-indicator preset with loop-permitted default and static reduced-motion fallbacks.
- [`guided_tour_advance.yaml`](./guided_tour_advance.yaml)
  — Guided-overlay step advance preset with semantic-equivalent low-motion behavior.
- [`focus_follow_advance.yaml`](./focus_follow_advance.yaml)
  — Focus-advance preset that avoids gliding focus under reduced motion.

