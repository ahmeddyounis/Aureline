# Notification privacy, quiet-hours, badges, and cross-client dedupe (beta)

The beta notification-privacy projection promotes the existing
notification primitives — typed envelope, dedupe-aware router,
quiet-hours posture, privacy-safe external payload, badge
reconciliation, suppression audit — to a page-level projection a
reviewer can inspect on every claimed attention surface. It replaces
per-surface ad-hoc privacy and dedupe decisions with one stable record
per claimed scenario, agreeing with the badge mirror and the
support-export row on classes, posture, and identity.

The projection lives in
[`crates/aureline-shell/src/notifications/beta.rs`](../../../crates/aureline-shell/src/notifications/beta.rs).
It does not re-derive the per-envelope privacy or routing truth — that
still comes from
[`crate::notifications::envelope`](../../../crates/aureline-shell/src/notifications/envelope.rs),
[`crate::notifications::router`](../../../crates/aureline-shell/src/notifications/router.rs),
[`crate::notifications::quiet_hours`](../../../crates/aureline-shell/src/notifications/quiet_hours.rs),
[`crate::notifications::external`](../../../crates/aureline-shell/src/notifications/external.rs),
[`crate::notifications::actions`](../../../crates/aureline-shell/src/notifications/actions.rs),
and
[`crate::notifications::audit`](../../../crates/aureline-shell/src/notifications/audit.rs).
The beta page projects the acceptance promises a daily-beta reviewer
needs to inspect on every claimed row.

## Contract surface

The beta projection ships five record kinds, all under the shared
contract ref `shell:notification_privacy_beta:v1`:

- `shell_notification_privacy_beta_row_record` — one attention row per
  claimed scenario. Each row carries a stable `row_id`, `case_id`,
  `canonical_event_id`, source subsystem, severity, privacy class,
  privacy payload class, lock-screen posture, redaction class, badge
  class, dedupe scheme, reopen target, observed quiet-hours posture,
  cross-client fanout posture, expected coalescing posture, the closed
  thirteen forbidden-shortcut classes, the routed occurrence count,
  and a reviewer-facing narrative.
- `shell_notification_privacy_beta_badge_record` — row-aligned status
  badge mirror. The badge echoes the row's `badge_class`,
  `severity_class`, `privacy_class`, OS app-icon visibility, and held /
  active count posture. The validator rejects any drift between the
  badge and the row.
- `shell_notification_privacy_beta_page_record` — page record that
  aggregates the rows, the badge mirror, an embedded
  [`DurableBadgeProjection`](../../../crates/aureline-shell/src/notifications/quiet_hours.rs)
  derived from the live router, and a summary banner with lock-screen
  posture counts, coalescing counts, cross-client counts, quiet-hours
  suppression counts, and critical-bypass counts.
- `shell_notification_privacy_beta_support_export_row_record` —
  per-row export row that quotes the row's identity, scenario class,
  privacy posture, lock-screen posture, redaction class, badge class,
  dedupe scheme, and occurrence count. Raw private material is
  excluded by construction.
- `shell_notification_privacy_beta_support_export_record` — support
  export wrapper. Embeds the page, every per-row export row, and
  every `case_id` in stable page order so support reviewers can pivot
  from a row to the page without separate query plumbing.

## Acceptance posture

The beta projection delivers the four M3 acceptance gates from the task:

- **Stable classes.** Every row pins a stable
  `(privacy_class, privacy_payload_class, severity_class, badge_class,
  dedupe_key_scheme)` tuple. The lock-screen posture mirrors the
  payload class verbatim (`generic_summary_only`,
  `scoped_workspace_safe`, `in_product_only`,
  `redacted_metadata_only`, `policy_forbidden_on_lock_screen`), and
  the validator rejects drift. Notification classes are no longer ad
  hoc per surface; they are inspectable as one row per scenario.
- **Lock-screen privacy.** Sensitive details stay out of the
  lock-screen / OS / companion payload unless the privacy class
  explicitly permits it. `summary_safe` and `workspace_sensitive`
  envelopes may render `lock_screen_safe_generic` or
  `lock_screen_safe_scoped`; `security_critical` and
  `managed_sensitive` envelopes are restricted to
  `redacted_metadata_only`, `policy_forbidden_on_lock_screen`, or
  `in_product_only`. The validator rejects any row whose payload
  class is strictly more permissive than the privacy class allows.
- **Repeated-failure coalescing.** Retry storms collapse into one
  durable item. Coalescing rows reject `canonical_event_id` and
  `canonical_object_target_plus_event_class`; they must use
  `grouped_burst_id` or `subsystem_plus_object_plus_phase`. The row
  records the live router's `occurrence_count` and `is_dedupe_repeat`,
  and the badge class is bound to `failed_runs`,
  `durable_running_count`, or another retry-friendly class — never
  `completion_unread`.
- **Cross-client dedupe.** Companion / remote-agent / managed-admin
  fanout collapses under `cross_client_canonical_event_id`. The row
  enumerates the sibling client scopes, asserts
  `cross_client_dedupe_in_effect = true`,
  `payload_class_not_widened_across_clients = true`, and
  `cross_client_dismissal_collapses = true`, so a desktop dismissal
  collapses the companion row instead of stranding it.

In addition, the closed thirteen
[`ForbiddenShortcutActionClass`](../../../crates/aureline-shell/src/notifications/external.rs)
set must be enumerated on every row, so the chrome cannot silently
grow its list of mutation classes per surface; critical-safety
severity must never admit a quiet-hours hold; durable truth (durable
job row, status item, status strip, activity-center digest card) is
preserved on every routed envelope.

The page must additionally cover the ten claimed beta row classes:
`delivered_summary_safe`, `coalesced_repeated_failure`,
`lock_screen_safe_generic_payload`, `lock_screen_safe_scoped_payload`,
`lock_screen_forbidden_security_critical`, `quiet_hours_held`,
`admin_policy_suppressed`, `critical_safety_escalation`,
`companion_cross_client_fanout`, `forbidden_shortcut_bypass_refused`.
A coverage gap is treated as an acceptance failure.

## Headless inspector

The beta projection is exercised through the
`aureline_shell_notification_privacy` binary. The bin is the only
mint-from-truth path for the JSON checked in under
`fixtures/ux/m3/notification_privacy/`, so live shell records, CLI
rows, and support-export rows cannot drift.

```sh
cargo run -q -p aureline-shell --bin aureline_shell_notification_privacy -- page
cargo run -q -p aureline-shell --bin aureline_shell_notification_privacy -- rows
cargo run -q -p aureline-shell --bin aureline_shell_notification_privacy -- badges
cargo run -q -p aureline-shell --bin aureline_shell_notification_privacy -- badge-projection
cargo run -q -p aureline-shell --bin aureline_shell_notification_privacy -- support-export
cargo run -q -p aureline-shell --bin aureline_shell_notification_privacy -- validate
```

The `validate` subcommand prints `ok` on success or a list of typed
[`NotificationPrivacyBetaValidationError`](../../../crates/aureline-shell/src/notifications/beta.rs)
diagnostics on failure (privacy posture drift, lock-screen posture
drift, coalescing scheme incompatibility, cross-client dedupe missing,
critical-safety hold admitted, forbidden-shortcut list incomplete,
badge parity drift, support-export drift, row-class coverage
incomplete, durable badge projection drift).

## Schema-of-record posture

The beta projection composes upstream schemas; it carries no parallel
notification vocabulary. Every typed axis is re-exported verbatim from
[`/schemas/ux/notification_envelope.schema.json`](../../../schemas/ux/notification_envelope.schema.json)
and the existing notification primitives. The companion review-side
audit checklist lives in
[`docs/ux/notification_privacy_dedupe_audit.md`](../notification_privacy_dedupe_audit.md);
this document specializes the audit to the beta acceptance gates and
binds the rows to the shell's Rust types.

Adding a new beta row class is additive-minor and requires bumping
`NOTIFICATION_PRIVACY_BETA_SCHEMA_VERSION`. Repurposing an existing
class is breaking and requires a new decision row.

## Companion artifacts

- [`crates/aureline-shell/src/notifications/`](../../../crates/aureline-shell/src/notifications/)
  — typed envelope, dedupe-aware router, quiet-hours posture,
  privacy-safe external payload, badge reconciliation, suppression
  audit, and this beta projection.
- [`crates/aureline-shell/src/bin/aureline_shell_notification_privacy.rs`](../../../crates/aureline-shell/src/bin/aureline_shell_notification_privacy.rs)
  — headless inspector.
- [`fixtures/ux/m3/notification_privacy/`](../../../fixtures/ux/m3/notification_privacy/)
  — beta fixture corpus (rows, badges, page, badge projection,
  support export).
- [`schemas/ux/notification_envelope.schema.json`](../../../schemas/ux/notification_envelope.schema.json)
  — frozen boundary schema for envelopes and privacy-class rules.
- [`docs/ux/notification_privacy_dedupe_audit.md`](../notification_privacy_dedupe_audit.md)
  — review-side audit checklist for grouped-burst dedupe,
  repeated-failure coalescing, lock-screen-safe summaries, companion
  fanout, and forbidden shortcut actions.
- [`docs/ux/os_notification_and_quiet_hours_contract.md`](../os_notification_and_quiet_hours_contract.md)
  — upstream contract for suppression, the privacy-safe payload rule
  record, the desktop-summary affordance record, and the closed
  thirteen forbidden-shortcut classes.

## Verification

```sh
cargo test -p aureline-shell --lib notifications::beta
cargo test -p aureline-shell --test notification_privacy_beta_fixtures
cargo run -q -p aureline-shell --bin aureline_shell_notification_privacy -- validate
```
