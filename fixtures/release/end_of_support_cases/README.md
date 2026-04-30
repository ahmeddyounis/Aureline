# End-of-support and migration acceptance cases

These fixtures are the seed cases the end-of-support and migration
contract at
[`docs/release/end_of_support_and_migration_contract.md`](../../../docs/release/end_of_support_and_migration_contract.md)
defines. Each case pairs:

- one `end_of_support_notice_record` validated against
  [`schemas/release/end_of_support_notice.schema.json`](../../../schemas/release/end_of_support_notice.schema.json);
- where the notice's affected scope includes one of the seven
  category scopes admitted by the compatibility-window report,
  one companion `compatibility_window_report_record` validated
  against
  [`schemas/release/compatibility_window_report.schema.json`](../../../schemas/release/compatibility_window_report.schema.json).

Every case:

- names one stable `notice_id` (and where present, one stable
  `report_id`);
- binds at least one factual `what_still_works` projection, at least
  one typed `risk_after_cutoff` projection, and exactly one
  `safest_next_step`;
- declares the four floor surfaces (`update_center_banner`,
  `about_panel_banner`, `compatibility_report_banner`,
  `migration_or_import_workflow_banner`).

## Case list

- `approaching_end_of_support_extension_compatibility_drift_notice.yaml`
  + `approaching_end_of_support_extension_compatibility_drift_report.yaml`
  — approaching the end of the rolling support window with extension
  compatibility-drift risk; safest next step routes through the
  migration-helper handoff with a one-click checkpoint required
  before apply.
- `past_end_of_support_security_only_window_closed_notice.yaml` —
  past the security-only window on a stable build; safest next step
  is to switch to the LTS train.
- `active_deprecation_macro_replacement_named_notice.yaml`
  + `active_deprecation_macro_replacement_named_report.yaml` —
  actively deprecated automation macro family with a named replacement
  workflow bundle; safest next step routes through the
  compatibility-window report.
- `breaking_compatibility_ahead_command_schema_v3_notice.yaml`
  + `breaking_compatibility_ahead_command_schema_v3_report.yaml` —
  breaking command-schema change ahead before stable; safest next step
  is to export the environment summary and review the
  compatibility-window report BEFORE the change ships.

Every case cites its companion compatibility-window report (when
applicable), affected support-window badges, channel rows, exact-build
identities, version-skew register rows, and migration-helper handoff
checkpoint refs by stable ref so release, support, About, update
center, compatibility report, and migration / import surfaces can pivot
in O(1) from one notice to the canonical evidence.
