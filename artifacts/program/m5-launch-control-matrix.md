# M5 Dogfood-Ring, Certification-Cohort, ORR, Rehearsal, Freeze-Exception, and Go/No-Go Control Matrix

- Packet: `m5-launch-control:stable:0001`
- Label: `M5 dogfood-ring, certification-cohort, ORR, rehearsal, freeze-exception, and go/no-go control matrix`
- Cohorts: 5 (5 stable)
- Launch-control roles: cohort_membership, readiness_event, rehearsal_currency, freeze_exception_authority, go_no_go_authority, rollback_stop, regression_asset
- Widening stages: alpha, beta, release_candidate, stable, long_term_support
- Proof freshness SLO: 720 hours (last refresh: 2026-07-14T00:00:00Z)

## Cohorts

- **core_team_canary**: `stable`
  - Owner: Core-team canary owner
  - Canonical schema: `schemas/program/m5-cohort-descriptor.schema.json`
  - Scope: One core-team canary cohort naming the internal dogfood ring entered, the known limits published before widening, the armed rollback-stop rule, and the reviewed dogfood telemetry so no stable claim skips the canary cohort and no ring widens on tribal memory
  - Required labels: identity, control_role, registry_reference, cohort_membership
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **design_partner_preview**: `stable`
  - Owner: Design-partner preview owner
  - Canonical schema: `schemas/program/m5-cohort-descriptor.schema.json`
  - Scope: One design-partner preview cohort naming the partners enrolled under NDA, the preview feedback triaged to requirements, the partner support language matched to cohort proof, and the ring widening gated on known limits so partner support language never outruns current cohort proof
  - Required labels: identity, control_role, registry_reference, cohort_membership
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **extension_author**: `stable`
  - Owner: Extension-author cohort owner
  - Canonical schema: `schemas/program/m5-freeze-exception-packet.schema.json`
  - Scope: One extension-author cohort naming the cohort admitted, the compatibility rehearsal kept current, the freeze exception documented not implicit, and the mixed-version drill passed so a freeze exception never becomes undocumented scope widening
  - Required labels: identity, control_role, registry_reference, readiness_state
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **public_preview**: `stable`
  - Owner: Public preview owner
  - Canonical schema: `schemas/program/m5-freeze-exception-packet.schema.json`
  - Scope: One public preview cohort naming the public preview ring opened, the publish/rollback drill kept current, the advisory/revocation rehearsal kept current, and the public support-handoff drill kept current so public proof never outruns cohort evidence and rehearsals stay current
  - Required labels: identity, control_role, registry_reference, readiness_state
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **certified_archetype**: `stable`
  - Owner: Certified-archetype owner
  - Canonical schema: `schemas/program/m5-go-no-go-decision.schema.json`
  - Scope: One certified-archetype cohort naming the cohort validated, the operational-readiness review signed, the go/no-go decision recorded, and the evidence snapshot and on-call roster preserved so a stable claim never widens without a go/no-go decision and shiproom never implies green while go/no-go or ORR state is stale
  - Required labels: identity, control_role, registry_reference, go_no_go_state
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
