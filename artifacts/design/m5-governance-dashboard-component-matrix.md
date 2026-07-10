# M5 governance-dashboard component matrix — design record

Batch B125 · Wave W125 · Readiness R125. This design record accompanies the frozen
M5 fitness-dashboard-tile, governance-report-row, waiver-expiry-queue-item,
release-gate-banner, mitigation-note-card, service-ownership-card, on-call-strip,
decision-right-card, and milestone-dashboard-row component matrix.

## Goal

Freeze the reusable governance-dashboard component matrix so assurance, operator,
and shiproom surfaces stop drifting on freshness, waivers, ownership, and
decision-right truth. Later implementation rows reuse this matrix without inventing
dashboard-local status vocabulary or hidden shiproom logic.

## Where it lives

- Validator + seed: `crates/aureline-release/src/freeze_the_m5_fitness_dashboard_tile_governance_report_row_waiver_expiry_queue_item_release_gate_banner_mitigation_note_card_service_ownership_card_on_call_strip_decision_right_card_and_milestone_dashboard_row_component_matrix`
- Matrix schema: `schemas/ui/m5-governance-dashboard-component-matrix.schema.json`
- Per-component schemas: `schemas/ui/m5-{fitness-dashboard-tile,governance-report-row,waiver-expiry-queue-item,release-gate-banner,mitigation-note-card,service-ownership-card,on-call-strip,decision-right-card,milestone-dashboard-row}.schema.json`
- Contract doc: `docs/help/m5_governance_dashboard_components_contract.md`
- Support export / CSV / summary: `artifacts/release/m5-governance-dashboard-proof/`
- Fixtures: `fixtures/ui/m5-governance-dashboard-components/`

## Design decisions

1. **One shared readiness vocabulary is the acceptance-criteria deliverable.** The
   nine-token `M5GovernanceReadinessState` (`passing`, `warning`, `blocked`,
   `waived`, `expired_waiver`, `evidence_stale`, `owner_unresolved`,
   `forum_unresolved`, `not_evaluated`) is pinned by a frozen test and carried by
   every component row. Only `passing` is a clean pass.

2. **Family-specific vocabularies live beside the shared vocabulary, not instead of
   it.** Each component adds only the vocabulary it owns: fitness provenance,
   report scopes, waiver expiry states, release-gate decisions, mitigation
   postures, ownership coverage states, on-call coverage + escalation routes,
   decision forum classes + decision-right states, and milestone gate states.

3. **Guardrails are hard invariants, not prose.** The four guardrails from the spec
   — never render waived/stale as a clean pass, never let an ownerless/forumless
   blocker read resolved, never hide mitigation behind jargon, never invent a
   private status grammar — are boolean row fields the validator forces to `false`.

4. **The matrix layers on top of existing feeds.** It does not re-architect the
   fitness feeds, waiver ledgers, ownership maps, or decision-forum manifests; it is
   the shared component contract over them, bound through eleven source-contract
   refs (the matrix schema, the contract doc, and the nine per-component schemas).

5. **Narrowing keeps every component visible.** Stale proof auto-narrows; the two
   narrowed fixtures (service-ownership card → Beta, release-gate banner → Preview)
   keep all nine components present.

## Verification

- `cargo test -p aureline-release --lib <module>` — 32 passing, 1 gated generator
  ignored.
- Schemas are Draft 2020-12 meta-valid; the support export and both narrowed
  fixtures validate against the matrix schema; the nine per-component instances
  validate against their per-component schemas.
