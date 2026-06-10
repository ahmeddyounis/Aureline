# AI Review Evidence, Finding Cards, and Review-Pack Change-Object Integration

- Packet: `ai-review-evidence-cards:stable:0001`
- Schema: `schemas/review/ship-ai-review-evidence-finding-cards-and-review-pack-integration-with-change-objects.schema.json`
- Support export: `artifacts/review/m5/ship_ai_review_evidence_finding_cards_and_review_pack_integration_with_change_objects/support_export.json`
- Contract doc: `docs/review/m5/ship_ai_review_evidence_finding_cards_and_review_pack_integration_with_change_objects.md`
- Fixtures: `fixtures/review/m5/ship_ai_review_evidence_finding_cards_and_review_pack_integration_with_change_objects/`
- Producer: `aureline_review::current_evidence_card_export`

## Coverage

- **AI review evidence** records, per evidence row, an explicit source class
  (`model_generated`, `static_analysis`, `local_check`, `provider_imported`), a
  freshness class (`fresh`, `stale_diff`, `superseded`, `unavailable`), the
  provenance label of what produced it, and a summary. Stale and superseded
  evidence is labeled rather than hidden, and evidence ids are unique so cards can
  cite them unambiguously.
- **Finding cards** are what a reviewer reads: each carries a severity, a status,
  the durable anchor it attaches to, the evidence it cites, the apply posture for
  any suggestion, and the change object it is scoped to. Every card cites evidence
  that exists in the packet and points at a change object a binding covers, and a
  blocked apply carries an explicit block-reason label so no suggestion is
  silently withheld or applied.
- **Review-pack integration with change objects** binds a review pack to an
  explicit change object, records required-check coverage, and carries an
  attribution label. A detached binding is relabeled with a non-empty detach label
  rather than being silently dropped.

## Trust guardrails

The `trust_review` block encodes the hard invariants — all must hold for the
packet to validate: AI evidence names its source and provenance; evidence
freshness and binding detachment are labeled rather than hidden; every finding
card cites real evidence and shows explicit severity and status; apply posture is
explicit and never triggers a silent write; review packs bind to explicit change
objects with explicit attribution; no finding-card or binding surface creates
hidden write scope; downgrade narrows the claim instead of hiding the lane; and
stale or underqualified rows block promotion.

Proof freshness SLO is 168 hours with automatic narrowing on stale proof. The
supported downgrade triggers are `proof_stale`, `policy_blocked`,
`evidence_stale`, `finding_evidence_missing`, `binding_detach_unlabeled`,
`trust_narrowing`, `scope_expansion_unqualified`, and
`upstream_dependency_narrowed`.

## Boundary

Raw diff bodies, raw model prompts or completions, raw build logs, raw provider
payloads, credentials, and live provider responses never cross this boundary. The
packet carries only metadata, qualification truth, and contract references. Every
finding-card and binding action stays read-only or attributable and reviewable.
