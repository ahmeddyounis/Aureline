# AI Review Evidence, Finding Cards, and Review-Pack Integration with Change Objects

This document is the contract for the M5 packet that binds AI-produced review
evidence to the finding cards a reviewer reads and to the review pack and explicit
change objects those cards are scoped against. The packet is the canonical M5
control source for this lane: the review-workspace finding surface, the
review-pack panel, CLI/headless output, diagnostics, Help/About, and support
exports ingest the checked-in packet rather than cloning status text.

- Record kind: `ai_review_evidence_finding_cards_and_review_pack_change_object_integration`
- Schema: [`schemas/review/ship-ai-review-evidence-finding-cards-and-review-pack-integration-with-change-objects.schema.json`](../../../schemas/review/ship-ai-review-evidence-finding-cards-and-review-pack-integration-with-change-objects.schema.json)
- Canonical support export: [`artifacts/review/m5/ship_ai_review_evidence_finding_cards_and_review_pack_integration_with_change_objects/support_export.json`](../../../artifacts/review/m5/ship_ai_review_evidence_finding_cards_and_review_pack_integration_with_change_objects/support_export.json)
- Summary artifact: [`artifacts/review/m5/ship_ai_review_evidence_finding_cards_and_review_pack_integration_with_change_objects.md`](../../../artifacts/review/m5/ship_ai_review_evidence_finding_cards_and_review_pack_integration_with_change_objects.md)
- Fixtures: [`fixtures/review/m5/ship_ai_review_evidence_finding_cards_and_review_pack_integration_with_change_objects/`](../../../fixtures/review/m5/ship_ai_review_evidence_finding_cards_and_review_pack_integration_with_change_objects/)
- Producer: `aureline_review::current_evidence_card_export`

## Pillars

### AI review evidence

Each `evidence_rows[]` row records an `evidence_id`, a `source` class
(`model_generated`, `static_analysis`, `local_check`, `provider_imported`), a
`freshness` class (`fresh`, `stale_diff`, `superseded`, `unavailable`), a
`provenance_label` describing what produced the evidence, and a `summary_label`.
Stale and superseded evidence is labeled, never silently hidden, and evidence ids
are unique so finding cards can cite them unambiguously. Evidence binds the
AI-evidence attachment contract at
[`schemas/review/ai_review_evidence.schema.json`](../../../schemas/review/ai_review_evidence.schema.json).

### Finding cards

Each `finding_cards[]` row is the card a reviewer reads. It carries a
`finding_id`, a `severity` (`blocking`, `high`, `medium`, `low`, `info`), a
`status` (`open`, `acknowledged`, `resolved`, `dismissed`, `superseded`), the
durable `anchor_id` it attaches to, the `evidence_ref` it cites, an
`apply_posture` (`no_suggestion`, `safe_apply_previewed`, `manual_only`,
`apply_blocked`), the `change_object_ref` it is scoped to, and a `title_label`. A
card's `evidence_ref` must name an evidence row that exists in the packet, and its
`change_object_ref` must name a change object that a review-pack binding covers —
so no card cites missing evidence or points at an unbound change object. An
`apply_blocked` posture must carry a non-empty `apply_block_label`, so a
suggestion is never silently withheld or silently applied.

### Review-pack integration with change objects

Each `review_pack_bindings[]` row binds a `review_pack_id` to an explicit
`change_object_id`, records a `binding_state` (`bound_current`,
`bound_stale_base`, `unbound_pending`, `detached_relabeled`), a
`required_check_coverage` class (`all_required_present`, `required_missing_labeled`,
`advisory_only`, `unavailable_offline`), and an `attribution_label` naming who or
what produced the binding. A `detached_relabeled` state must carry a non-empty
`detach_label`. Bindings mirror the review-pack contract at
[`schemas/review/review_pack.schema.json`](../../../schemas/review/review_pack.schema.json)
and the change-object contract at
[`schemas/review/change_object_orchestration.schema.json`](../../../schemas/review/change_object_orchestration.schema.json).

## Track invariant

The `trust_review` block encodes the hard invariants — all must hold for the
packet to validate:

- `ai_evidence_provenance_explicit` and `evidence_freshness_labeled_not_hidden` —
  every piece of evidence names its source and its freshness, and stale evidence
  is labeled rather than hidden.
- `finding_cards_cite_real_evidence` and
  `finding_severity_and_status_explicit` — every card cites evidence that exists
  and shows an explicit severity and status.
- `apply_posture_explicit_no_silent_write` — apply posture is explicit and never
  triggers a silent write.
- `review_pack_binds_to_change_object`, `binding_detach_labeled_not_hidden`, and
  `change_object_attribution_explicit` — review packs bind to explicit change
  objects, detachment is labeled, and binding attribution is explicit.
- `no_hidden_write_scope`, `downgrade_narrows_instead_of_hides`, and
  `stale_or_underqualified_blocks_promotion`.

## Downgrade and freshness

`proof_freshness` carries the SLO (168 hours) and last-refresh timestamp; when
proof goes stale `auto_narrow_on_stale` narrows the lane. The supported downgrade
triggers are `proof_stale`, `policy_blocked`, `evidence_stale`,
`finding_evidence_missing`, `binding_detach_unlabeled`, `trust_narrowing`,
`scope_expansion_unqualified`, and `upstream_dependency_narrowed`. The
[fixtures](../../../fixtures/review/m5/ship_ai_review_evidence_finding_cards_and_review_pack_integration_with_change_objects/)
show a superseded-evidence packet with a blocked apply and an offline packet with
a detached binding; both remain valid because narrowing is explicit, not hidden.

## Boundary

Raw diff bodies, raw model prompts or completions, raw build logs, raw provider
payloads, credentials, and live provider responses never cross this boundary. The
packet carries only metadata, evidence provenance, finding verdicts, apply
postures, binding outcomes, and contract references. Every finding-card and
binding action stays read-only or attributable and reviewable.
