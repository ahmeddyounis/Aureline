# Badge/chip/pill fixtures

Worked examples for the compact status-token contract at
[`docs/ux/badge_pill_contract.md`](../../../docs/ux/badge_pill_contract.md) and the
schema at
[`schemas/ux/status_pill.schema.json`](../../../schemas/ux/status_pill.schema.json).

Each JSON file is a `status_pill_case_record` that exercises:

- the **three-inline** overflow budget for interactive surfaces,
- keyboard-recoverable overflow summaries,
- export/CLI text parity with UI labels, and
- mixed support-class + freshness downgrade expectations.

## Cases

| Fixture | Scenario axis |
| --- | --- |
| [`crowded_row_overflow_summary.json`](./crowded_row_overflow_summary.json) | Dense row with many families; overflows to a keyboard-reachable summary. |
| [`export_rendered_text_parity.json`](./export_rendered_text_parity.json) | UI and export share the same controlled labels and order. |
| [`high_contrast_overflow_is_not_hue_only.json`](./high_contrast_overflow_is_not_hue_only.json) | High-contrast/forced-colors preserves non-hue cues and overflow recoverability. |
| [`mixed_support_class_and_freshness_downgrade.json`](./mixed_support_class_and_freshness_downgrade.json) | Strong support claims stay paired with freshness+scope and downgrade when stale. |

