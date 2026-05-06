# Contrast and color-substitution review fixtures

Worked fixtures for the contrast / color-substitution contract frozen in:

- [`/docs/design/contrast_and_color_substitution_contract.md`](../../../docs/design/contrast_and_color_substitution_contract.md)

Validated by:

- [`/schemas/design/contrast_thresholds.schema.json`](../../../schemas/design/contrast_thresholds.schema.json)

Each case is a `contrast_review_case_record` that:

- cites named threshold ids from
  [`/artifacts/design/contrast_threshold_rows.yaml`](../../../artifacts/design/contrast_threshold_rows.yaml); and
- demonstrates substitution behavior under high-contrast, forced-colors,
  reduced-chroma/low-color, print/export, and screenshot/publication contexts.

## Fixtures

- [`diff_surface_substitution.yaml`](./diff_surface_substitution.yaml)
  — Diff regions stay readable and distinguishable without hue-only cues.
- [`evidence_chart_substitution.yaml`](./evidence_chart_substitution.yaml)
  — Evidence chart marks remain distinguishable under reduced chroma and print/export.
- [`trust_warning_substitution.yaml`](./trust_warning_substitution.yaml)
  — Trust warning/banner remains explicit under forced colors and grayscale capture.
- [`notification_surface_substitution.yaml`](./notification_surface_substitution.yaml)
  — Notification surface severity remains legible without relying on red/green hue.

