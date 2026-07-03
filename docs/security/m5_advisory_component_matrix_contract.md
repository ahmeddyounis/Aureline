# M5 Security-Advisory, Emergency-Notice, Affected-Install, and Disclosure-Link Component Contract

> Task: M05-764 · Batch B89 · Delivery class: security-notice contract +
> affected-install UI + release/help/support parity.

This contract freezes the checked-in matrix for Aureline's user-facing
security-advisory component model — the primitives that still drift too easily
into website copy or a generic update banner: security-advisory cards, emergency
notices, affected-install panels, disclosure/history blocks, advisory activity
rows, and native-notification handoff. It names the required anatomy, the
controlled severity/action/dismissal/continuity/delivery/freshness/disclosure/
export vocabulary, the projection surfaces, and the supportability hooks M5 will
honor for each advisory primitive family, so later M5 rows can no longer invent
generic advisory language or hide continuity rules outside the matrix.

- **Boundary schema:** [`schemas/security/m5-advisory-component-matrix.schema.json`](../../schemas/security/m5-advisory-component-matrix.schema.json)
- **Rust source of truth:** `crates/aureline-shell/src/freeze_the_m5_security_advisory_emergency_notice_affected_install_and_disclosure_link_matrix/`
- **Headless emitter:** `aureline_shell_m5_advisory_components`
- **Checked support export:** [`artifacts/release/m5-advisory-proof/support_export.json`](../../artifacts/release/m5-advisory-proof/support_export.json)
- **Matrix CSV:** [`artifacts/release/m5-advisory-proof/matrix.csv`](../../artifacts/release/m5-advisory-proof/matrix.csv)
- **Report:** [`artifacts/security/m5-advisory-component-matrix.md`](../../artifacts/security/m5-advisory-component-matrix.md)
- **Narrowed fixtures:** [`fixtures/security/m5-advisory-scenarios/`](../../fixtures/security/m5-advisory-scenarios/)

The shell topology this matrix binds against — the eight canonical shell zones,
the compact/standard/expanded responsive classes, the window classes, the
consumer surfaces, and the ten claimed M5 surface families — is reused verbatim
from the frozen
[M5 shell-zone matrix](../../schemas/shell/m5-shell-zone.schema.json). The
severity, action-state, dismissal, delivery, freshness, and disclosure
vocabularies are aligned field-for-field to the already-frozen advisory contracts:

- [`schemas/security/advisory_card.schema.json`](../../schemas/security/advisory_card.schema.json)
  — the advisory-surface projection contract for advisory cards, emergency
  banners, revocation notices, and disclosure links.
- [`schemas/security/affected_install_assessment.schema.json`](../../schemas/security/affected_install_assessment.schema.json)
  — the per-install affected-install assessment contract.
- [`docs/security/severity_matrix.md`](./severity_matrix.md) — the closed
  severity vocabulary and advisory discipline this matrix's severity classes
  project from.

This lane mints no parallel slot, layout, window, surface-family, consumer,
severity, or advisory-surface vocabulary; it adds only the vocabulary for the
user-facing advisory *components* themselves and binds them into shell topology.

## Track invariant

Advisories identify the affected object, severity, current exposure, fixed
version or mitigation, signer/source state, and primary actions without hiding
local continuity; emergency notices stay explicit about blast radius,
acknowledge/snooze/dismiss rules, and forced-disable scope; affected-install
panels, activity-center rows, native notifications, docs/help, and support
exports all describe the same advisory truth; and mirror lag, unsigned
distribution, or stale notice state auto-narrow claims instead of silently
staying green.

## Component families (rows)

Six governed advisory primitive families, one matrix row each. Every row binds a
canonical shell zone, the responsive/window classes it survives, the claimed M5
surface families that render it, the severity classes it shows, the projection
surfaces its truth reaches, its non-visual accessibility routes, its consumer
surfaces, and its downgrade triggers. The Rust validator narrows any row that is
incomplete.

| Family | Shell zone | Family-specific vocabulary it must declare |
|---|---|---|
| `advisory_card` | `main_workspace` | required advisory anatomy; action states; required actions |
| `emergency_notice` | `title_context_bar` | dismissal states; action states; required actions |
| `affected_install_panel` | `right_inspector` | continuity claims; delivery profiles; freshness states |
| `disclosure_block` | `bottom_panel` | disclosure fields |
| `advisory_activity_row` | `bottom_panel` | export fields |
| `native_notification_handoff` | `transient_overlay` | notification behaviors |

Every family additionally declares at least one severity class, at least one
projection surface, the three mandatory labels (`identity`, `severity`,
`keyboard_route`), a non-visual accessibility route, consumer surfaces, and
downgrade triggers. Family-specific vocabularies are declared **only** where the
family predicate holds — an affected-install panel declares no dismissal states,
an emergency notice declares no export fields, and so on.

## Controlled vocabulary

The frozen `vocabulary_set` is built from the typed Rust `ALL` arrays and must
match the canonical token lists exactly; any drift narrows the packet.

- **Required advisory anatomy** (`anatomy_field`): `affected_object`, `severity`,
  `current_exposure`, `fixed_version_or_mitigation`, `signer_source_state`,
  `primary_actions`, `local_continuity`.
- **Severity classes** (`severity_class`): `informational`, `low`, `moderate`,
  `high`, `critical`, `operational_emergency` — aligned to the advisory-surface
  `surface_severity_class` and the severity matrix.
- **Action states** (`action_state`): `informational`, `review_recommended`,
  `action_required`, `blocking`, `immediate_remediation`, `mitigation_complete`.
- **Required actions** (`required_action`): `none`, `review_notice`,
  `update_to_fixed_version`, `rollback_or_repin`, `disable_or_remove`,
  `import_signed_snapshot`, `rotate_trust_root`, `export_support_packet`,
  `contact_admin`, `wait_for_superseding_action`.
- **Dismissal states** (`dismissal_state`): `not_acknowledgeable`,
  `unacknowledged`, `acknowledged`, `snoozed_until_review`,
  `blocked_until_remediated` — acknowledgement is never mitigation.
- **Local-continuity claims** (`continuity_claim`): `local_use_unaffected`,
  `degraded_local_mode`, `requires_disabling_affected_profile`,
  `offline_mirror_lag_disclosed`, `no_safe_local_continuity`,
  `continuity_pending_fix`.
- **Delivery profiles** (`delivery_profile`): `local_only`, `managed`,
  `offline_mirror`, `manual_import`.
- **Freshness states** (`freshness_state`): `up_to_date`, `stale_within_grace`,
  `stale_past_grace`, `offline_expired`, `unknown`.
- **Disclosure fields** (`disclosure_field`): `aureline_advisory_id`,
  `cve_alias`, `ghsa_alias`, `disclosure_timing`, `visibility_posture`,
  `history_state`, `external_disclosure_link`.
- **Notification behaviors** (`notification_behavior`): `os_notification_summary`,
  `click_through_to_advisory`, `respects_quiet_hours`,
  `no_sensitive_body_in_payload`, `emergency_bypasses_quiet_hours`,
  `dismissal_syncs_to_in_app`.
- **Export fields** (`export_field`): `advisory_id`, `severity`, `action_state`,
  `affected_surface`, `mitigation_state`, `delivery_profile`, `freshness_state`,
  `continuity_note`, `disclosure_visibility`, `history_state`.
- **Projection surfaces** (`projection_surface`): `update_center`, `marketplace`,
  `help_about`, `support_bundle`, `native_notification`, `mirror_offline_drill`,
  `activity_center`, `release_packet`.
- **Accessibility routes** (`accessibility_route`): `keyboard_focusable`,
  `screen_reader_announced`, `non_hover_reachable`, `pointer_optional`,
  `high_contrast_safe`, `support_exportable`.
- **Required labels** (`required_label`): `identity`, `severity`,
  `keyboard_route` (mandatory), plus `provenance`, `primary_action`,
  `continuity_note`.

## Projection (first consumers)

The packet's `consumer_projection` block names how advisory truth projects from
one matrix into each downstream surface, so update, marketplace, Help/About,
support bundles, native notifications, and mirror/offline drills all read the same
source rather than re-deriving advisory language:

- `update_center_reads_advisory_matrix`
- `marketplace_reads_advisory_matrix`
- `help_about_reads_advisory_matrix`
- `support_bundle_reads_single_source`
- `native_notifications_read_single_source`
- `mirror_offline_drills_read_single_source`

## Hard invariants

Every row carries four hard-invariant booleans that MUST be `false`; any `true`
narrows the row on `advisory_invariant_violated`:

- `hides_affected_scope`
- `hides_local_continuity`
- `invents_generic_advisory_language`
- `stays_silent_on_stale_or_unsigned`

## Downgrade triggers

`affected_scope_hidden`, `exposure_hidden_behind_generic_banner`,
`fixed_version_or_mitigation_missing`, `signer_source_state_hidden`,
`local_continuity_hidden`, `dismissal_rule_violated`,
`forced_disable_scope_hidden`, `mirror_lag_undisclosed`,
`unsigned_distribution_undisclosed`, `stale_notice_state_silent`,
`external_disclosure_only`, `proof_stale`. Stale proof, mirror lag, and unsigned
distribution auto-narrow the claim instead of leaving a surface silently green.

## Release / help / support parity

The `release_posture` block requires support-export parity and accessibility
parity for every component and pins the release-packet and advisory-component
audit refs. Release, Help/About, and support packets reference this one
advisory-component contract source; downstream M5 rows cannot invent a private
advisory dialect without changing the matrix, its schema, and this contract in the
same change.

## Change control

Adding an advisory component family, a vocabulary value, a projection surface, or
a downgrade trigger is additive: extend the typed Rust `ALL` array, the schema
enum, and this document in the same change, then re-mint the checked support
export, CSV, report, and narrowed fixtures from the headless emitter so the
in-code matrix, the artifact, and the fixtures never drift. Repurposing an
existing value is breaking and requires a decision-register row co-signed by
`security_trust_review` and `release_council`, consistent with
[`docs/security/severity_matrix.md`](./severity_matrix.md).
