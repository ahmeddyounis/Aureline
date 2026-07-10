# M5 governance-dashboard component contract

This contract freezes Aureline's reusable governance-dashboard components so
assurance, operator, and shiproom surfaces stop drifting on freshness, waivers,
ownership, and decision-right truth. It is the human-readable companion to the
machine contract in
[`schemas/ui/m5-governance-dashboard-component-matrix.schema.json`](../../schemas/ui/m5-governance-dashboard-component-matrix.schema.json)
and the Rust validator in
`crates/aureline-release/src/freeze_the_m5_fitness_dashboard_tile_governance_report_row_waiver_expiry_queue_item_release_gate_banner_mitigation_note_card_service_ownership_card_on_call_strip_decision_right_card_and_milestone_dashboard_row_component_matrix`.

The Rust validator is authoritative. The checked-in support export lives at
[`artifacts/release/m5-governance-dashboard-proof/support_export.json`](../../artifacts/release/m5-governance-dashboard-proof/support_export.json).

## The nine governed component families

| Component | Purpose | Family-specific vocabulary |
| --- | --- | --- |
| `fitness_dashboard_tile` | A fitness-function reading with its corpus/profile provenance | fitness provenance classes |
| `governance_report_row` | A lane's readiness with evidence | report scopes |
| `waiver_expiry_queue_item` | When a waiver lapses | waiver expiry states |
| `release_gate_banner` | A ship/no-ship decision and its reason | release-gate decisions |
| `mitigation_note_card` | User-facing, jargon-free mitigation language | mitigation postures |
| `service_ownership_card` | Owner coverage and freshness | ownership coverage states |
| `on_call_strip` | On-call coverage and the escalation route | on-call coverage states, escalation route classes |
| `decision_right_card` | The forum authorized to approve the next move | decision forum classes, decision-right states |
| `milestone_dashboard_row` | A milestone's exit-gate state | milestone gate states |

## The frozen readiness-state vocabulary

Every component carries the same controlled readiness vocabulary. No surface may
invent a dashboard-local status word; later implementation rows reuse these tokens
directly:

`passing`, `warning`, `blocked`, `waived`, `expired_waiver`, `evidence_stale`,
`owner_unresolved`, `forum_unresolved`, `not_evaluated`.

Only `passing` is a clean pass. Every other state must be rendered as its own state
— a waived, expired, stale, ownerless, or forumless lane is never shown as clear.

## Hard invariants

Every row asserts these four invariants are false:

- `renders_waived_or_stale_as_clean_pass` — a waived or stale reading never reads as
  a clean pass.
- `lets_ownerless_or_forumless_blocker_read_resolved` — an ownerless or forumless
  blocker never reads as resolved.
- `hides_mitigation_behind_internal_jargon` — mitigation text stays reusable by
  support and export consumers.
- `invents_private_governance_status_grammar` — no component invents a second
  status grammar.

## Downgrade triggers

A component narrows below its claim when any of its downgrade triggers fires:
`fitness_provenance_unstated`, `evidence_stale_hidden`, `waiver_expiry_hidden`,
`release_gate_reason_generic`, `mitigation_hidden_behind_jargon`,
`owner_coverage_overstated`, `on_call_gap_hidden`, `escalation_route_unstated`,
`decision_forum_masked`, `advisory_forum_reads_authoritative`,
`milestone_gate_overstated`, and `proof_stale`. Stale proof auto-narrows every
component.

## Consumers

Assurance-center, operator-overview, release-center, shiproom, service-health,
support, docs, and admin surfaces consume this single matrix so one fitness tile
carries its provenance, one report row states its readiness, one waiver-expiry item
names when a waiver lapses, one release-gate banner states its reason, one
mitigation card carries reusable language, one service-ownership card names owner
coverage, one on-call strip names the escalation route, one decision-right card
names the authorized forum, and one milestone row states its exit gate.

## Boundary and redaction

The packet is metadata-only and export-safe. Raw URLs, raw tokens, credentials, and
user text bodies never cross this boundary.
