# M5 Security-Advisory, Emergency-Notice, Affected-Install, and Disclosure-Link Component Matrix

- Packet: `m5-advisory-components:stable:0001`
- Label: `M5 security-advisory, emergency-notice, affected-install, and disclosure-link component matrix`
- Component families: 6 (6 stable)
- Severity classes: informational, low, moderate, high, critical, operational_emergency
- Projection surfaces: update_center, marketplace, help_about, support_bundle, native_notification, mirror_offline_drill, activity_center, release_packet
- Proof freshness SLO: 720 hours (last refresh: 2026-07-01T00:00:00Z)

## Component families

- **advisory_card**: `stable`
  - Owner: Security advisory component owner
  - Scope: One security-advisory card model that names the affected object, its severity, current exposure, the fixed version or mitigation, the signer/source continuity state, and the primary actions, and always states what still works locally — never a generic update banner
  - Shell zone: `main_workspace`
  - Required labels: identity, severity, keyboard_route, provenance, primary_action, continuity_note
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **emergency_notice**: `stable`
  - Owner: Emergency response component owner
  - Scope: One emergency-notice model that stays explicit about blast radius, the acknowledge/snooze/dismiss rules, and the forced-disable scope, and that cannot be silently dismissed while an exposure is unremediated
  - Shell zone: `title_context_bar`
  - Required labels: identity, severity, keyboard_route, provenance, primary_action, continuity_note
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **affected_install_panel**: `stable`
  - Owner: Install/update component owner
  - Scope: One affected-install panel model that assesses which install lanes are affected from the same install-profile, exact-build, delivery-profile, and mirror-freshness vocabulary, and that discloses mirror lag and the local-continuity claim instead of staying green
  - Shell zone: `right_inspector`
  - Required labels: identity, severity, keyboard_route, provenance, primary_action, continuity_note
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **disclosure_block**: `stable`
  - Owner: Disclosure/history component owner
  - Scope: One disclosure/history block model that carries the copy-safe Aureline advisory id with CVE and GHSA aliases, the disclosure timing and visibility posture, and the resolved-versus-active history — so disclosure lives in the product and is never flattened into a single link to an external page
  - Shell zone: `bottom_panel`
  - Required labels: identity, severity, keyboard_route, provenance, primary_action
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **advisory_activity_row**: `stable`
  - Owner: Activity/history component owner
  - Scope: One advisory activity-row model that projects each advisory event into the activity center and the support export with the advisory id, severity, action state, affected surface, mitigation state, delivery profile, freshness state, continuity note, disclosure visibility, and history state — so a support bundle reconstructs advisory truth without a screenshot
  - Shell zone: `bottom_panel`
  - Required labels: identity, severity, keyboard_route, provenance, continuity_note
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **native_notification_handoff**: `stable`
  - Owner: Notification routing component owner
  - Scope: One native-notification handoff model that surfaces a compact OS summary with no sensitive body, clicks through to the in-product advisory, respects quiet hours for non-emergency severities while letting an emergency bypass them, and syncs OS dismissal to the in-app dismissal state
  - Shell zone: `transient_overlay`
  - Required labels: identity, severity, keyboard_route, provenance, primary_action
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
