# UI Copy Contract Fixtures

Worked YAML fixtures for:

- [`/docs/copy/ui_copy_contract.md`](../../../docs/copy/ui_copy_contract.md)
- [`/artifacts/copy/ui_copy_lint_rules.yaml`](../../../artifacts/copy/ui_copy_lint_rules.yaml)
- [`/schemas/copy/error_message.schema.json`](../../../schemas/copy/error_message.schema.json)

Each file is a `ui_copy_case_record` spanning at least one of:

- action-label vagueness fail gates,
- four-part error-message structure requirements, and
- AI copy guardrails against unsupported certainty or validation claims.

The fixtures are written so a reviewer (or future lint tooling) can reject copy
mechanically, without style debates.

## Cases

- [`trust_prompt_action_labels.yaml`](./trust_prompt_action_labels.yaml)
  - Trust prompt avoids standalone vague actions.
- [`settings_row_action_labels.yaml`](./settings_row_action_labels.yaml)
  - Settings row primary action label names the outcome.
- [`banner_error_message_four_part.yaml`](./banner_error_message_four_part.yaml)
  - Banner error message demonstrates the four required parts.
- [`toast_action_and_error_copy.yaml`](./toast_action_and_error_copy.yaml)
  - Toast uses outcome-specific action labels and avoids generic “OK”.
- [`ai_card_copy_guardrails.yaml`](./ai_card_copy_guardrails.yaml)
  - AI card copy uses proposal language and avoids unsupported certainty.
- [`help_error_surface_error_message.yaml`](./help_error_surface_error_message.yaml)
  - Help/error surface uses explicit scope and next safe action.

