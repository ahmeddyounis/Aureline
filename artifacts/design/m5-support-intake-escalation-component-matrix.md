# M5 Support-Scenario-Picker-Row, Issue-Report-Builder-Step, Escalation-Packet-Summary, Handoff-Timeline-Row, and Unsafe-Fix-Blocked-Note Component Matrix

- Packet: `m5-support-intake-escalation-components:stable:0001`
- Label: `M5 support-scenario-picker-row, issue-report-builder-step, escalation-packet-summary, handoff-timeline-row, and unsafe-fix-blocked-note component matrix`
- Component families: 5 (5 stable)
- Scenario families: crash_recovery, performance_health, extension_conflict, data_integrity, connectivity_sync, uncategorized_scenario
- Case dispositions: local_only, vendor_case, uncategorized, unsafe_fix_blocked, resolved_locally
- Proof freshness SLO: 720 hours (last refresh: 2026-07-07T00:00:00Z)

## Component families

- **support_scenario_picker_row**: `stable`
  - Owner: Support-scenario picker row owner
  - Scope: One support-scenario-picker-row model naming which class of problem a user is starting from — crash recovery, performance health, extension conflict, data integrity, connectivity sync, or an uncategorized scenario — how wide the incident reaches, and which Doctor finding family the scenario binds to, so a user never has to assemble a case from generic logs or guess which diagnosis path applies
  - Required labels: identity, state, keyboard_route, scenario_and_scope
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **issue_report_builder_step**: `stable`
  - Owner: Issue-report builder step owner
  - Scope: One issue-report-builder-step model naming which step of the report the user is on — choose scenario, describe symptom, attach evidence, review redaction, confirm scope, or submit / export — and which evidence classes it selects and omits, so selected and omitted evidence is explicit and a user never ships a case without knowing what it carries
  - Required labels: identity, state, keyboard_route, scenario_and_scope, evidence_and_redaction
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **escalation_packet_summary**: `stable`
  - Owner: Escalation-packet summary owner
  - Scope: One escalation-packet-summary model naming where a case is bound — a local-only bundle, a self-serve export, a vendor support case, an enterprise admin queue, a community forum, or a blocked destination — and how it redacts on export, so a local-only bundle is never mislabelled as a shared case and a redacted packet is never shown as a full export
  - Required labels: identity, state, keyboard_route, evidence_and_redaction, destination_and_next_step
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **handoff_timeline_row**: `stable`
  - Owner: Handoff-timeline row owner
  - Scope: One handoff-timeline-row model naming where in the diagnosis-to-handoff timeline a case sits — diagnosis started, repair suggested, repair attempted, case built, handed off, or awaiting a human — and the next human step, so scenario, finding, and packet lineage is never lost between local diagnosis and human handoff and the next step is always explicit
  - Required labels: identity, state, keyboard_route, destination_and_next_step
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **unsafe_fix_blocked_note**: `stable`
  - Owner: Unsafe-fix blocked note owner
  - Scope: One unsafe-fix-blocked-note model naming why a suggested fix is blocked — approval required, irreversible change, out-of-scope repair, insufficient evidence, policy blocked, or unsupported scenario — and which repair class is approved instead, so a user never guesses which repair is safe and an unsafe fix is never applied without saying why it is blocked
  - Required labels: identity, state, keyboard_route, destination_and_next_step
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
