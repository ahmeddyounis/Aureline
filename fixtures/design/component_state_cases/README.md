# Component-state screenshot, honesty-review, and diff packet fixtures

Worked fixtures for the component-state corpus and review-packet
template frozen in
[`/artifacts/design/component_state_screenshot_corpus.yaml`](../../../artifacts/design/component_state_screenshot_corpus.yaml)
and
[`/docs/design/component_state_diff_packet_template.md`](../../../docs/design/component_state_diff_packet_template.md).

Every YAML file in this directory pins an addressable row shape
that conforms to the closed vocabularies in the corpus and the
template. The fixtures exist so design review, accessibility,
high-contrast, reduced-motion, extension-parity, and release-
evidence reviewers can all cite one row shape instead of
inventing local checklists.

The fixtures resolve refs verbatim against:

- [`/artifacts/design/component_state_screenshot_corpus.yaml`](../../../artifacts/design/component_state_screenshot_corpus.yaml)
  — for `corpus_row_id`, `corpus_state_class`,
  `taxonomy_state_class`, honesty-review rule ids, and per-state
  capture / cue requirements.
- [`/artifacts/design/theme_support_rows.yaml`](../../../artifacts/design/theme_support_rows.yaml)
  and
  [`/artifacts/design/layer_and_scrim_tokens.yaml`](../../../artifacts/design/layer_and_scrim_tokens.yaml)
  — for the four first-party theme rows, the five accessibility
  postures, the seven layer tokens, and the four scrim tokens.
- [`/artifacts/design/token_drift_rules.yaml`](../../../artifacts/design/token_drift_rules.yaml)
  — for the rule_id values each non-pass packet row routes
  through.
- [`/schemas/design/component_state_machine.schema.json`](../../../schemas/design/component_state_machine.schema.json)
  — for `component_surface_class`, `component_taxonomy_state_class`,
  and the seven evidence-hook ref families.

## Fixture index

### Baselines (one per `corpus_state_class`)

- [`baseline_command_palette_empty_state.yaml`](./baseline_command_palette_empty_state.yaml)
  — `empty` on `command_palette_and_search`.
- [`baseline_command_palette_loading_state.yaml`](./baseline_command_palette_loading_state.yaml)
  — `loading` on `command_palette_and_search` with reduced-motion
  and low-motion captures.
- [`baseline_dialog_action_pending_state.yaml`](./baseline_dialog_action_pending_state.yaml)
  — `pending` on `dialog_or_capability_sheet`.
- [`baseline_inspector_degraded_state.yaml`](./baseline_inspector_degraded_state.yaml)
  — `degraded` on `inspector_surface` with forced-colors capture.
- [`baseline_settings_blocked_state.yaml`](./baseline_settings_blocked_state.yaml)
  — `blocked` on `settings_canvas` (locked posture with policy
  source).
- [`baseline_command_palette_error_state.yaml`](./baseline_command_palette_error_state.yaml)
  — `error` on `command_palette_and_search`.
- [`baseline_dialog_action_completed_state.yaml`](./baseline_dialog_action_completed_state.yaml)
  — `completed` on `dialog_or_capability_sheet`.
- [`baseline_start_center_restored_state.yaml`](./baseline_start_center_restored_state.yaml)
  — `restored` on `start_center_canvas` after crash recovery.
- [`baseline_trust_prompt_restricted_state.yaml`](./baseline_trust_prompt_restricted_state.yaml)
  — `restricted` on `trust_prompt_canvas`.
- [`baseline_settings_policy_blocked_state.yaml`](./baseline_settings_policy_blocked_state.yaml)
  — `policy_blocked` on `settings_canvas`.
- [`baseline_notification_quiet_hours_held_state.yaml`](./baseline_notification_quiet_hours_held_state.yaml)
  — `quiet_hours_held` on `notification_canvas`.
- [`baseline_status_strip_reconnecting_state.yaml`](./baseline_status_strip_reconnecting_state.yaml)
  — `reconnecting` on `status_strip` with motion-reduced and
  low-motion captures.

### Honesty-review violations

- [`honesty_violation_settings_lock_reason_hover_only.yaml`](./honesty_violation_settings_lock_reason_hover_only.yaml)
  — `lock_reason_revealed_only_on_hover` on a settings row;
  routes to `block`.
- [`honesty_violation_durable_job_color_alone_error.yaml`](./honesty_violation_durable_job_color_alone_error.yaml)
  — `color_alone_conveys_state` on a durable job row error
  posture; routes to `block_release`.

### State-semantic diff packets

- [`diff_packet_build_to_build_pending_collapsed_into_loading.yaml`](./diff_packet_build_to_build_pending_collapsed_into_loading.yaml)
  — build-to-build packet that detects
  `pending_collapsed_into_loading` on a dialog action; routes
  through `component_state_repurposed_without_decision_row`.
- [`diff_packet_revision_to_revision_high_contrast_narrowed.yaml`](./diff_packet_revision_to_revision_high_contrast_narrowed.yaml)
  — revision-to-revision packet that records a
  `theme_support_narrowed` change with a decision-row ref;
  resolves to `pass_with_disclosed_gap`.

## Intended usage

- **Schema and corpus conformance.** The YAML shape is the
  contract of record. Every fixture cites
  `component_state_corpus_schema_version: 1` and resolves refs
  against the corpus YAML.
- **Reviewers.** Design, accessibility, high-contrast, reduced-
  motion, support, and extension-parity reviewers can walk a
  fixture from `corpus_row_id` to per-state capture, honesty-
  review axis, and diff verdict without negotiating field
  names.
- **Conformance gates.** A later runner diffs implementation
  against these records (and the canonical token export) without
  reinterpretation.
