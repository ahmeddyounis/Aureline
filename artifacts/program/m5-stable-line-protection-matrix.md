# M5 Stable-Line Protection, Evidence-Refresh, Correction-Line, and LTS-Readiness Matrix

- Packet: `m5-stable-line-protection:stable:0001`
- Label: `M5 stable-line protection, evidence-refresh, correction-line, and LTS-readiness matrix`
- Lines: 5 (5 stable)
- Stable-line-protection roles: support_window, correction_ownership, evidence_refresh, backport_decision, lts_eligibility, bundle_currentness, defect_ledger
- Widening stages: alpha, beta, release_candidate, stable, long_term_support
- Proof freshness SLO: 720 hours (last refresh: 2026-07-14T00:00:00Z)

## Lines

- **fresh_stable_line**: `stable`
  - Owner: Stable-line release owner
  - Canonical schema: `schemas/program/m5-stable-line-refresh-policy.schema.json`
  - Scope: One fresh stable line naming the crash/rollback flow protected, the support-export flow protected, the migration flow protected, and the first-thirty-day watch active so the just-shipped stable line never drifts on stale evidence in its first month
  - Required labels: identity, protection_role, registry_reference, support_window
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **evidence_refresh_line**: `stable`
  - Owner: Evidence-refresh cadence owner
  - Canonical schema: `schemas/program/m5-stable-line-refresh-policy.schema.json`
  - Scope: One evidence-refresh line naming the certified-archetype evidence refreshed, the compatibility evidence refreshed, the known-limits evidence refreshed, and the refresh cadence kept ordinary release ops so support language never outruns current refresh proof
  - Required labels: identity, protection_role, registry_reference, support_window
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **correction_backport_line**: `stable`
  - Owner: Correction-line owner
  - Canonical schema: `schemas/program/m5-supported-line-defect-ledger.schema.json`
  - Scope: One correction/backport line naming the correction path exercised, the backport decision recorded within SLA, the may-slip item shipped or narrowed, and the post-launch correction report published so no backport rests on tribal memory instead of a documented correction packet
  - Required labels: identity, protection_role, registry_reference, refresh_state
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **bundle_currentness_line**: `stable`
  - Owner: Bundle-currentness owner
  - Canonical schema: `schemas/program/m5-supported-line-defect-ledger.schema.json`
  - Scope: One launch-bundle-currentness line naming the launch-bundle freshness rechecked, the bundle-refresh obligation met, the shipping-line bundle audited, and any frozen-bundle drift detected so the shipping line never ships a stale launch bundle without a currentness audit
  - Required labels: identity, protection_role, registry_reference, refresh_state
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **lts_candidate_line**: `stable`
  - Owner: LTS-readiness decision owner
  - Canonical schema: `schemas/program/m5-lts-readiness-decision.schema.json`
  - Scope: One LTS-candidate line naming the backport discipline demonstrated, the rollback discipline demonstrated, the LTS decision packet recorded, and the support-evidence snapshot preserved so LTS is never claimed without current rollback and support evidence and never reads as green while refresh or ledger state is stale
  - Required labels: identity, protection_role, registry_reference, lts_posture
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
