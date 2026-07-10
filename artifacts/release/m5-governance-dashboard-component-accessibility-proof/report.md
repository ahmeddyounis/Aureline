# M5 Governance-Dashboard Component Accessibility & Auto-Narrowing

- Packet: `m5-governance-dashboard-component-accessibility-parity:stable:0001`
- As of: `2026-07-10T00:00:00Z`
- Families: 9 certified across 9 / 9 frozen families
- Status: 2 green / 7 yellow / 0 red

## Rows

- **a11y:fitness-dashboard-tile** (fitness_dashboard_tile) — family=fitness_dashboard_tile keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=governed_pass effective_claim=governed_pass status=parity
- **a11y:governance-report-row** (governance_report_row) — family=governance_report_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=governed_pass effective_claim=provisional status=narrowed_disclosed
  - Auto-narrow: governed_pass → provisional (dimension=evidence_freshness, trigger=evidence_stale_hidden) — Evidence stale — lane shown from last-known governance evidence until re-verification lands, not a fresh pass
- **a11y:waiver-expiry-queue-item** (waiver_expiry_queue_item) — family=waiver_expiry_queue_item keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=governed_pass effective_claim=waiver_gated status=narrowed_disclosed
  - Auto-narrow: governed_pass → waiver_gated (dimension=waiver_expiry, trigger=waiver_expiry_hidden) — Waiver expiring — lane held by an exception waiver and shown waiver-gated, not a clean pass, until the waiver is renewed or cleared
- **a11y:release-gate-banner** (release_gate_banner) — family=release_gate_banner keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=governed_resolved effective_claim=blocked status=narrowed_disclosed
  - Auto-narrow: governed_resolved → blocked (dimension=decision_right_truth, trigger=decision_forum_masked) — Decision forum unresolved — no authoritative forum can approve this move yet, so the gate is shown blocked, not ready to ship
- **a11y:mitigation-note-card** (mitigation_note_card) — family=mitigation_note_card keyboard=reachable_and_labeled screen_reader=disclosed_reduced_but_reachable cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=governed_pass effective_claim=degraded status=narrowed_disclosed
  - Auto-narrow: governed_pass → degraded (dimension=support_class, trigger=mitigation_hidden_behind_jargon) — Support class partial — mitigation shown degraded until the plain-language note replaces the internal jargon support consumers cannot reuse
- **a11y:service-ownership-card** (service_ownership_card) — family=service_ownership_card keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=governed_resolved effective_claim=governed_resolved status=parity
- **a11y:on-call-strip** (on_call_strip) — family=on_call_strip keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=governed_pass effective_claim=degraded status=narrowed_disclosed
  - Auto-narrow: governed_pass → degraded (dimension=owner_coverage, trigger=owner_coverage_overstated) — On-call coverage partial — rotation shown degraded with an unfilled backup slot, never as a fully covered on-call route
- **a11y:decision-right-card** (decision_right_card) — family=decision_right_card keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=governed_pass effective_claim=provisional status=narrowed_disclosed
  - Auto-narrow: governed_pass → provisional (dimension=decision_right_truth, trigger=decision_forum_masked) — Decision-right record stale — forum authority shown from last-known state until re-confirmation lands, not a fresh authoritative reading
- **a11y:milestone-dashboard-row** (milestone_dashboard_row) — family=milestone_dashboard_row keyboard=reachable_and_labeled screen_reader=disclosed_reduced_but_reachable cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=governed_pass effective_claim=blocked status=narrowed_disclosed
  - Auto-narrow: governed_pass → blocked (dimension=decision_right_truth, trigger=decision_forum_masked) — Milestone gate forum unresolved — no authoritative forum can clear the next exit gate, so the milestone is shown blocked, not on-track
