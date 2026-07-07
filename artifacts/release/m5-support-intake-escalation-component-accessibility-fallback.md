# M5 Support-Intake / Escalation Component Accessibility & Auto-Narrowing

- Packet: `m5-support-intake-escalation-component-accessibility-fallback:stable:0001`
- As of: `2026-07-07T00:00:00Z`
- Families: 5 certified across 5 / 5 frozen families
- Status: 2 green / 4 yellow / 0 red

## Rows

- **a11y:support-scenario-picker-row** (support_scenario_picker_row) — family=support_scenario_picker_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=ready_to_escalate effective_claim=unclassified_scenario status=narrowed_disclosed
  - Auto-narrow: ready_to_escalate → unclassified_scenario (dimension=scenario_classification, trigger=scenario_or_scope_unstated) — Symptom could not be mapped to a stable scenario family with confidence — shown as an unclassified scenario with its scope and bound finding family still preserved, starting a local diagnosis
- **a11y:issue-report-builder-step-evidence-omitted** (issue_report_builder_step) — family=issue_report_builder_step keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=ready_to_escalate effective_claim=evidence_incomplete_case status=narrowed_disclosed
  - Auto-narrow: ready_to_escalate → evidence_incomplete_case (dimension=evidence_completeness, trigger=evidence_class_masked) — One or more evidence classes were left out of the report — shown as an evidence-incomplete case that names the included and excluded classes, never as a full report
- **a11y:issue-report-builder-step-reviewable** (issue_report_builder_step) — family=issue_report_builder_step keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=reviewable_case effective_claim=reviewable_case status=parity
- **a11y:escalation-packet-summary** (escalation_packet_summary) — family=escalation_packet_summary keyboard=reachable_and_labeled screen_reader=disclosed_reduced_but_reachable cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=ready_to_escalate effective_claim=local_only_diagnosis status=narrowed_disclosed
  - Auto-narrow: ready_to_escalate → local_only_diagnosis (dimension=destination_reach, trigger=packet_destination_unstated) — Packet destination is restricted to a local-only bundle — shown as a local-only diagnosis with its packet id and finding / repair lineage preserved, never as a shared escalation
- **a11y:handoff-timeline-row** (handoff_timeline_row) — family=handoff_timeline_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=ready_to_escalate effective_claim=ready_to_escalate status=parity
- **a11y:unsafe-fix-blocked-note** (unsafe_fix_blocked_note) — family=unsafe_fix_blocked_note keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=ready_to_escalate effective_claim=policy_blocked_repair status=narrowed_disclosed
  - Auto-narrow: ready_to_escalate → policy_blocked_repair (dimension=repair_guidance, trigger=approved_repair_class_masked) — Suggested repair is held by policy and cannot be applied — shown as a policy-blocked repair that names the block reason and safer-repair guidance, never an approved fix
