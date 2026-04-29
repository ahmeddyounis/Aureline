# Visual adaptation fixtures

These fixtures exercise the visual adaptation contract:

- [`high_contrast_editor_gutter.yaml`](./high_contrast_editor_gutter.yaml)
  keeps breakpoint, diagnostic, fold, and hidden-state cues distinct in a
  high-contrast gutter.
- [`reduced_motion_durable_job_progress.yaml`](./reduced_motion_durable_job_progress.yaml)
  replaces animated progress with static labels, counts, and durable routes.
- [`color_safe_diff_diagnostic_combo.yaml`](./color_safe_diff_diagnostic_combo.yaml)
  preserves added, removed, modified, warning, error, trust, and blocked
  meaning in low-saturation review.
- [`compact_status_badges_grayscale.yaml`](./compact_status_badges_grayscale.yaml)
  proves status items, badges, settings locks, and inline severity cues remain
  distinguishable without hue or motion.

Schema:

- [`/schemas/ux/contrast_mode_state.schema.json`](../../../schemas/ux/contrast_mode_state.schema.json)

Contract:

- [`/docs/accessibility/visual_adaptation_contract.md`](../../../docs/accessibility/visual_adaptation_contract.md)
