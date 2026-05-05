# Input, search-field, and combobox cases

Worked fixtures for:

- [`docs/ux/input_and_combobox_contract.md`](../../../docs/ux/input_and_combobox_contract.md)
- [`schemas/ux/input_control_state.schema.json`](../../../schemas/ux/input_control_state.schema.json)

The corpus covers:

- `settings_search_stale_suggestions.json` — settings search field with stale
  completion/suggestion state and explicit refresh/clear recovery actions.
- `filtered_target_picker_loading.json` — filtered target-picker combobox whose
  option list is loading and explicitly labeled as such.
- `secret_reveal_row_policy_locked.json` — secret-bearing input row with
  reveal-on-demand posture, policy-gated reveal/copy/export semantics, and no
  hidden value mutation from clear/reveal.
- `request_environment_selector_no_match.json` — request environment combobox
  showing a no-match state that offers clear/recover actions without auto-commit.
- `repair_card_inline_validation_error.json` — repair-card text input with
  consistent inline validation timing and a blocking error message after
  meaningful input.

Fixtures carry opaque refs and redaction-aware labels only.

