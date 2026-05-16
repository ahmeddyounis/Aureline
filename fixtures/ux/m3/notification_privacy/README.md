# Notification privacy / quiet-hours / badge / cross-client dedupe (beta) fixture corpus

Reviewable fixtures for the beta notification-privacy projection that
lives in
[`crates/aureline-shell/src/notifications/beta.rs`](../../../../crates/aureline-shell/src/notifications/beta.rs).

Each JSON file is a literal projection of the seeded
`NotificationPrivacyBetaPage` produced by the headless inspector
([`crates/aureline-shell/src/bin/aureline_shell_notification_privacy.rs`](../../../../crates/aureline-shell/src/bin/aureline_shell_notification_privacy.rs)).
The inspector is the only mint-from-truth path for these fixtures, so
the checked-in JSON cannot drift from the Rust types.

All records carry the shared contract ref
`shell:notification_privacy_beta:v1` so shell UI rows, headless CLI
rows, the row badges, and support-export rows pivot to the same
`row_id` and `case_id`.

## Index

| Fixture | Coverage |
| --- | --- |
| [`rows.json`](./rows.json) | Beta attention rows covering delivered-summary-safe, coalesced-retry-burst, lock-screen-safe-generic, lock-screen-safe-scoped, lock-screen-forbidden-security-critical, quiet-hours-held, admin-policy-suppressed, critical-safety-escalation, companion-cross-client-fanout, and forbidden-shortcut-bypass-refused scenarios. |
| [`badges.json`](./badges.json) | Row-aligned badge mirror that echoes the row's badge class, severity class, privacy class, and OS app-icon visibility. |
| [`badge_projection.json`](./badge_projection.json) | Durable badge projection across the seeded rows (deduped by canonical event id). |
| [`page.json`](./page.json) | Full beta page with aggregate summary banner (lock-screen posture, coalescing, cross-client, quiet-hours, critical-bypass counts) and the embedded badge projection. |
| [`support_export.json`](./support_export.json) | Support-export wrapper that quotes the page, per-row export rows, and every `case_id` in stable page order. Raw private material is excluded by construction. |

## Fixture rules

- Every record carries a stable `case_id`, `row_id`, and the shared
  contract ref `shell:notification_privacy_beta:v1`; record kinds are
  stable Rust constants.
- Privacy posture is stable: each row pins a
  `(privacy_class, privacy_payload_class, severity_class, badge_class,
  dedupe_key_scheme)` tuple, never an ad-hoc per-surface routing
  decision.
- Lock-screen posture mirrors the envelope's
  `privacy_payload_class` — `generic_summary_only`,
  `scoped_workspace_safe`, `in_product_only`, `redacted_metadata_only`,
  or `policy_forbidden_on_lock_screen`. The validator rejects drift.
- Retry coalescing rows reject `canonical_event_id` and
  `canonical_object_target_plus_event_class`; they must use
  `grouped_burst_id` or `subsystem_plus_object_plus_phase` so the
  badge counts one durable item, not one per delivery.
- Cross-client rows enumerate sibling client scopes
  (`companion_surface`, `remote_agent`, `managed_admin_surface`, …),
  carry `cross_client_dedupe_in_effect=true`, and use
  `cross_client_canonical_event_id`. The validator rejects siblings
  that duplicate the originating scope.
- Critical-severity rows record
  `bypassed_by_critical_severity=true`, never admit a hold, and
  preserve durable truth (`durable_truth_preserved=true`). The
  validator rejects a critical row that admits a hold.
- Every row enumerates the closed thirteen forbidden-shortcut classes
  on `forbidden_shortcut_posture.forbidden_classes`. The validator
  rejects a row whose list is missing a class.
- Badges, rows, and support-export rows must agree on `badge_class`,
  `severity_class`, `privacy_class`, `lock_screen_posture`,
  `dedupe_key_scheme`, and `canonical_event_id`. Drift is a contract
  bug the validator rejects.

## Regenerate

```sh
cargo run -q -p aureline-shell --bin aureline_shell_notification_privacy -- page              > fixtures/ux/m3/notification_privacy/page.json
cargo run -q -p aureline-shell --bin aureline_shell_notification_privacy -- rows              > fixtures/ux/m3/notification_privacy/rows.json
cargo run -q -p aureline-shell --bin aureline_shell_notification_privacy -- badges            > fixtures/ux/m3/notification_privacy/badges.json
cargo run -q -p aureline-shell --bin aureline_shell_notification_privacy -- badge-projection  > fixtures/ux/m3/notification_privacy/badge_projection.json
cargo run -q -p aureline-shell --bin aureline_shell_notification_privacy -- support-export    > fixtures/ux/m3/notification_privacy/support_export.json
```

## Verification

```sh
cargo test -p aureline-shell --test notification_privacy_beta_fixtures
cargo run -q -p aureline-shell --bin aureline_shell_notification_privacy -- validate
```
