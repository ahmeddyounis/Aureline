# M5 emergency-notice banner primitive contract

One reusable emergency banner primitive for Aureline. Whenever a capability kill
switch, a trust-root rotation, a channel freeze, a forced-disable action, or a signed
emergency bundle changes what is safe to do next, this primitive renders the same
banner so the reason, affected capability, blast radius, local-work continuity,
deadline, and the next / recovery actions are visible inline — never behind a generic
red banner that implies broader loss of local work than the evidence supports, and
never behind one generic close button that ignores the event class.

- **Module:** `crates/aureline-shell/src/implement_the_m5_emergency_notice_banner_primitive`
- **Boundary schema:** `schemas/security/m5-emergency-notice-banner.schema.json`
- **Support export:** `artifacts/release/m5-emergency-notice-banner-proof/support_export.json`
- **Matrix CSV:** `artifacts/release/m5-emergency-notice-banner-proof/matrix.csv`
- **Markdown report:** `artifacts/security/m5-emergency-notice-banner-primitive.md`
- **Narrowed fixtures:** `fixtures/security/m5-emergency-notice-banner-primitive/`
- **Emitter:** `cargo run -p aureline-shell --bin aureline_shell_m5_emergency_notice_banner_primitive -- <subcommand>`

This primitive *narrows* the emergency-notice family of the frozen M5
advisory-component matrix
(`schemas/security/m5-advisory-component-matrix.schema.json`, minted by
`freeze_the_m5_security_advisory_emergency_notice_affected_install_and_disclosure_link_matrix`)
into a working emergency banner, and aligns its reason, continuity, and dismissal
vocabulary to the frozen emergency-action object model
(`docs/security/emergency_action_model.md`), the emergency-disable bundle
(`schemas/security/emergency_disable_bundle.schema.json`), and the local-continuity
card (`schemas/security/local_continuity_card.schema.json`). It reuses the matrix's
severity classes, action states, required actions, dismissal states, continuity claims,
export fields, accessibility routes, qualification classes, and downgrade triggers
verbatim, and reuses the frozen shell-zone matrix's zones, responsive classes, window
classes, and consumer surfaces. The local-work state, the derived continuity posture,
and the dismissal policy are resolver-side vocabularies, kept out of the frozen set.

## Resolver

`resolve_emergency_banner(&M5EmergencyBannerResolutionInput) -> Result<M5ResolvedEmergencyBanner, M5EmergencyBannerResolutionError>`
takes one emergency on one reason-class lane — its copy-safe notice id, severity,
affected capability, blast radius, local-work state, deadline, recovery path, signer /
source state, action state, primary and recovery actions, local-continuity claim, and
dismissal policy — and produces one resolved banner. The resolver:

- derives the normalized **continuity posture** from the local-work state
  (`editing_review_export_safe` → `local_work_continues_safely`, `degraded_but_safe` →
  `local_work_continues_degraded`, `affected_capability_suspended` →
  `affected_capability_suspended_local_safe`, `blocked_pending_acknowledgement` →
  `blocked_pending_acknowledgement`, `data_loss_confirmed` → `data_loss_proven`,
  `continuity_not_yet_determined` → `continuity_assessment_pending`);
- sets `implies_data_loss` **only** for the `data_loss_confirmed` state, so an
  emergency never implies data loss or unsafe local work unless the event actually
  proves it, and sets `local_work_safe` when editing, review, and export can still
  continue (including while a capability is suspended or an action is blocked pending
  acknowledgement);
- derives the **dismissal state** and the allowed acknowledge / snooze / dismiss
  actions from the event's dismissal policy (`not_dismissable_blocked` →
  `blocked_until_remediated` + acknowledge only; `acknowledge_required` →
  `unacknowledged` + acknowledge only; `acknowledge_or_snooze` → `unacknowledged` +
  acknowledge / snooze; `fully_dismissible` → `unacknowledged` +
  acknowledge / snooze / dismiss; `informational_dismissible` → `not_acknowledgeable` +
  dismiss), so user agency matches the event class instead of one generic close button;
- keeps `remains_visible` true by construction — the primitive structurally cannot hide
  an emergency banner;
- projects the same severity, continuity posture, primary action, and dismissal state
  into every channel (`update_center`, `extension_host`, `native_notification`,
  `support_bundle`) so the banner stays in parity across surfaces; and
- emits a copy-safe, export-safe summary carrying the mandatory export columns
  (`advisory_id`, `severity`, `action_state`, `affected_surface`, `mitigation_state`,
  `continuity_note`) for support and admin flows.

Resolution rejects an empty notice id, empty affected capability, empty blast radius,
empty deadline, empty recovery path, empty signer / source state, and any representation
carrying forbidden material.

## Parity matrix

`M5EmergencyBannerPrimitivePacket` binds one row per reason-class lane
(`capability_kill_switch`, `trust_root_rotation`, `channel_freeze`, `forced_disable`,
`signed_emergency_bundle`) to the shared banner anatomy, the severity vocabulary, every
channel, the dismissal policies, the export fields, and the accessibility routes. Every
lane carries worked resolution cases whose stored resolution must equal a fresh resolve
of its input (`ExampleNoticeDrift`).

### Banner anatomy (all mandatory — visible inline, no detail drawer)

`reason_class`, `affected_capability`, `blast_radius`, `local_continuity_note`,
`deadline_urgency`, `primary_action`, `recovery_action`.

### Hard invariants (every row)

- never hides emergency truth behind a detail drawer,
- never implies data loss without proof,
- never collapses dismissal into one generic close button,
- never drops the copy-safe id or export summary.

### Acceptance-criterion lints (packet)

- **`channel_parity_unproven`** — some worked resolution must project every channel in
  parity (AC1: kill-switch, trust-root rotation, channel-freeze, and forced-disable all
  render one emergency banner model across update, extension-host, native-notification,
  and support surfaces).
- **`local_safe_continuity_unproven`** — some worked resolution must keep local work
  safe without implying data loss, with the full banner inline and a complete export
  summary (AC2: emergency notices no longer imply data loss or unsafe local work unless
  the event actually proves it).
- **`dismissal_rule_unproven`** — the worked resolutions must cover acknowledge,
  snooze, and dismiss across the matrix, and at least one worked emergency must forbid
  an outright dismiss (AC3: dismissal behavior is explicit and consistent, not one
  generic close button).
- **`severity_coverage_unproven`** / **`continuity_posture_coverage_unproven`** — the
  worked resolutions must exercise every severity class and every continuity posture,
  including the sole posture that proves data loss.

## Governance

Stale proof auto-narrows the primitive (`proof_freshness.auto_narrow_on_stale`).
Narrowed variants (forced disable → Beta, signed emergency bundle → Preview) hold a
single lane below Stable while keeping every lane visible. Raw reporter identities,
exploit payloads, signatures, hostnames, paths, private registry URLs, and credentials
never cross the boundary; only opaque, export-safe reprs are carried. The Rust validator
and resolver in `crates/aureline-shell` are the authoritative gate.
