# Policy chronology-bar cases

Reviewer-facing chronology-bar rows for policy previews, remembered-
decision expiry, legal-hold interception, and audit/export joins. These
cases complement the governed-record chronology fixtures by freezing the
compact bar-level field set that admin, support, and export surfaces can
quote consistently.

Required fields across every case:

- `case_id`
- `actor_ref`
- `event_ref`
- `effective_time.utc_instant`
- `effective_time.local_iso_with_offset`
- `effective_time.timezone_id`
- `effective_time.offset_at_instant`
- `source_clock_class`
- `skew_flag`
- `ordering.ordering_relation`
- `export_representation_rule`
- `rendering_representation`
- `support_export_fields`

Cases:

- [`policy_change_immediate_denial.yaml`](./policy_change_immediate_denial.yaml)
  — immediate deny replaces a remembered allow on the same subject.
- [`narrower_scope_carry_forward.yaml`](./narrower_scope_carry_forward.yaml)
  — remembered allow narrows to a root-scoped carry-forward.
- [`expired_remembered_decision_reprompt.yaml`](./expired_remembered_decision_reprompt.yaml)
  — remembered decision expires and a reprompt becomes mandatory.
- [`future_effective_policy_activation.yaml`](./future_effective_policy_activation.yaml)
  — future-effective deny preserves Berlin civil time and UTC.
- [`legal_hold_delete_block.yaml`](./legal_hold_delete_block.yaml)
  — legal hold blocks a retention delete without losing actor or order.
- [`audit_export_mixed_timezones.yaml`](./audit_export_mixed_timezones.yaml)
  — audit export uses mixed-timezone chronology while preserving one
  canonical order key.
