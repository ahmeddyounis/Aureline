# M5 notification / activity-center handoff routing primitive contract

One reusable notification / activity-center handoff routing primitive for Aureline.
Whenever a security-affecting advisory or revocation event needs to reach a user, admin, or
support, this primitive routes the same event so it lands in a durable activity-center row,
a privacy-safe native OS notification where policy allows, a Help/About summary, and a
support-bundle export field — never collapsing to a bare badge, a transient toast that
disappears, or a link to an external page, and always reopening onto the authoritative
affected-install or disclosure surface.

- **Module:** `crates/aureline-shell/src/implement_the_m5_notification_and_activity_center_handoff_routing_primitive`
- **Boundary schema:** `schemas/security/m5-notification-activity-handoff.schema.json`
- **Support export:** `artifacts/release/m5-notification-activity-handoff-proof/support_export.json`
- **Matrix CSV:** `artifacts/release/m5-notification-activity-handoff-proof/matrix.csv`
- **Markdown report:** `artifacts/security/m5-notification-activity-handoff-primitive.md`
- **Narrowed fixtures:** `fixtures/security/m5-notification-activity-handoff-primitive/`
- **Emitter:** `cargo run -p aureline-shell --bin aureline_shell_m5_notification_activity_handoff_primitive -- <subcommand>`

This primitive *narrows* the native-notification-handoff family of the frozen M5
advisory-component matrix (`schemas/security/m5-advisory-component-matrix.schema.json`,
minted by
`freeze_the_m5_security_advisory_emergency_notice_affected_install_and_disclosure_link_matrix`)
into a working notification / activity handoff, and aligns its vocabulary to the frozen
advisory-identity record (`schemas/security/advisory_identity.schema.json`, whose Aureline
advisory id family the copy-safe advisory id mirrors), the OS-notification / quiet-hours
contract (`docs/ux/os_notification_and_quiet_hours_contract.md`, whose quiet-hours,
do-not-disturb, emergency-bypass, and no-sensitive-body rules the delivery posture and
notification behaviors honor), and the attention-routing schema
(`schemas/activity/m5-attention-routing.schema.json`, whose durable activity-center routing
and reopen continuity the handoff aligns to). It reuses the matrix's severity classes,
action states, required actions, continuity claims, delivery profiles, mirror-freshness
states, native-notification behaviors, export fields, accessibility routes, qualification
classes, and downgrade triggers verbatim; and reuses the frozen shell-zone matrix's zones,
responsive classes, window classes, and consumer surfaces. The derived delivery posture and
reopen surface are resolver-side vocabularies, kept out of the frozen set.

## Resolver

`resolve_notification_handoff(&M5NotificationHandoffResolutionInput) -> Result<M5ResolvedNotificationHandoff, M5NotificationHandoffResolutionError>`
takes one advisory / revocation event on one notification-delivery lane — its copy-safe
advisory id, severity, event kind, affected scope, current status, authoritative reopen
surface, reopen target, signer / source state, delivery profile, mirror freshness, action
state, primary action, and local-continuity claim — and produces one resolved handoff. The
resolver:

- derives the **delivery posture** from the delivery lane and severity (`foreground_focused`
  / `background_unfocused` → `native_notification_plus_activity_row`; `quiet_hours_active` /
  `do_not_disturb_enforced` / `managed_policy_restricted` → `activity_center_durable_only`
  for a non-emergency event, or `emergency_bypass_delivered` for a critical / operational
  emergency; `offline_or_mirror_deferred` → `deferred_then_durable`), so a suppressed OS
  notification still lands durably in the activity center instead of collapsing to a badge,
  and an emergency-grade severity bypasses quiet hours / do-not-disturb. `remains_durable`,
  `collapses_to_badge_only`, `collapses_to_toast_only`, and `collapses_to_website_only` are
  fixed (true / false / false / false) by construction;
- keeps the **reopen target** pointed at the authoritative surface (`affected_install_panel`,
  `disclosure_block`, `advisory_card`, or `emergency_notice`) so a notification or activity
  row always lands on the authoritative affected-install or disclosure surface.
  `reopens_to_authoritative_surface` is true and `is_dead_end` is false by construction;
- derives the **privacy-safe native-notification behaviors** — the OS payload always carries
  a compact summary with no sensitive body (`no_sensitive_body_in_payload`), click-through
  to the in-product advisory, and in-app dismissal sync; a non-emergency event respects
  quiet hours while an emergency-grade event bypasses them (`payload_is_privacy_safe`);
- keeps `remains_visible` and `shares_advisory_vocabulary` true by construction — the native
  notification and the activity row read from one advisory vocabulary;
- projects the same advisory id, severity, affected scope, delivery posture, and reopen
  surface into every channel (`activity_center`, `native_notification`, `help_about`,
  `support_bundle`) so the event stays in parity across surfaces; and
- emits a copy-safe, export-safe summary carrying the mandatory export columns
  (`advisory_id`, `severity`, `action_state`, `affected_surface`, `mitigation_state`,
  `continuity_note`) so Help/About and support bundles can explain the current advisory
  state without separate manual prose.

Resolution rejects an empty advisory id, empty affected scope, empty current status, empty
reopen target, empty signer / source state, and any representation carrying forbidden
material.

## Parity matrix

`M5NotificationHandoffPacket` binds one row per notification-delivery lane
(`foreground_focused`, `background_unfocused`, `quiet_hours_active`,
`do_not_disturb_enforced`, `offline_or_mirror_deferred`, `managed_policy_restricted`) to the
shared handoff anatomy, the severity vocabulary, every channel, the notification behaviors,
the event kinds, the export fields, and the accessibility routes. Every lane carries worked
resolution cases whose stored resolution must equal a fresh resolve of its input
(`example_handoff_drift`).

### Handoff anatomy (all mandatory — carried by every route, no detail drawer)

`event_identity`, `severity`, `affected_scope`, `current_status`, `delivery_state`,
`reopen_target`, `primary_action`.

### Hard invariants (every row)

- never collapses an event to a badge-only, toast-only, or website-only state,
- never hides handoff truth behind a detail drawer,
- never drops an event out of the durable activity history,
- never splits the native-notification and activity-row vocabulary,
- never drops the copy-safe id or export summary.

### Acceptance-criterion lints (packet)

- **`channel_parity_unproven`** — every worked resolution must project all four channels with
  identical advisory id, severity, affected scope, delivery posture, and reopen surface, and
  some worked resolution must carry a full copy-safe export summary (AC2 / AC3: native
  notifications, activity rows, Help/About, and support share the same advisory vocabulary
  and one export that explains the current advisory state).
- **`durable_routing_unproven`** — every worked resolution must stay durable without
  collapsing to a badge / toast / website-only state, and some worked resolution must
  exercise a suppressed OS-notification lane that still lands durably in the activity center
  (AC1: advisory and revocation events remain durable and never degrade to badge-only,
  toast-only, or website-only).
- **`reopen_continuity_unproven`** — every worked resolution must reopen onto its
  authoritative surface without a dead-end, and the worked resolutions together must reopen
  onto both the affected-install panel and the disclosure block (AC1: a notification or
  activity row lands on the authoritative affected-install or disclosure surface).
- **`event_kind_coverage_unproven`** / **`severity_coverage_unproven`** /
  **`delivery_posture_coverage_unproven`** — the worked resolutions must exercise every event
  kind, every severity class, and every delivery posture (including the emergency-bypass and
  deferred-then-durable postures).

## Governance

Stale proof auto-narrows the primitive (`proof_freshness.auto_narrow_on_stale`). Narrowed
variants (quiet hours → Beta, offline / mirror deferred → Preview) hold a single lane below
Stable while keeping every lane visible. Raw hostnames, absolute paths, exploit payloads,
signatures, private registry URLs, credentials, and raw notification bodies never cross the
boundary; only opaque, export-safe reprs and a copy-safe advisory id are carried. The Rust
validator and resolver in `crates/aureline-shell` are the authoritative gate.
