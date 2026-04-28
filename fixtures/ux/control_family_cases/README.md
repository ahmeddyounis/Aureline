# Core-control family cases

Worked fixtures for [`docs/ux/control_family_contract.md`](../../../docs/ux/control_family_contract.md)
and [`schemas/ux/control_state.schema.json`](../../../schemas/ux/control_state.schema.json).

The corpus covers:

- `busy_submit.json` - provider-backed submit action in a pending busy
  state.
- `policy_lock.json` - policy-owned settings value rendered as locked,
  not disabled.
- `imported_value.json` - editable onboarding input with imported value
  provenance.
- `live_toggle.json` - immediate, reversible local toggle.
- `staged_apply.json` - settings apply action that commits staged
  values after validation.
- `destructive_action.json` - destructive managed-admin action that
  opens review before commit.

Fixtures carry opaque refs and redaction-aware labels only.
