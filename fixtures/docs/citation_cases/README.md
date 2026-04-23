# Docs citation-anchor and symbol-linked-reference cases

Worked fixtures for the docs citation-anchor and symbol-linked-reference
contract frozen in
[`/docs/docs_integrity/citation_and_reference_contract.md`](../../../docs/docs_integrity/citation_and_reference_contract.md).
Every `docs_citation_anchor_record` fixture validates against
[`/schemas/docs/citation_anchor.schema.json`](../../../schemas/docs/citation_anchor.schema.json);
every `symbol_linked_reference_record` fixture validates against
[`/schemas/docs/symbol_linked_reference.schema.json`](../../../schemas/docs/symbol_linked_reference.schema.json).

The fixtures exist so docs search, guided help, glossary cards,
onboarding / guided tours, the AI-explanation overlay, hosted-review
evidence, repair packets, and portability / offboarding exports can
write against a shared corpus without inventing their own citation
dialect. Each file carries a `__fixture__` section summarizing the
scenario, the axes it exercises, and the contract sections it
illustrates.

## Intended usage

- **Schema conformance.** Every fixture MUST validate against the
  corresponding schema. A fixture that fails validation is a bug in
  the fixture, not in the schema.
- **Reuse-rule corpus.** A later parity audit comparing how docs
  panes, the docs browser, Help / About, the service-health view,
  the support summary, onboarding, the AI-explanation overlay,
  glossary cards, hosted-review evidence rows, repair packets, and
  portability / offboarding exports attach citations runs against
  these fixtures.
- **Offline / export reconstructability corpus.** Fixtures capture
  the five fields (`pack_id`, `pack_revision_ref`, `target_ref`,
  `target_build_identity_ref`, `locale`) a reviewer needs to
  reconstruct the row a user saw in an offline or exported context.

## Required fields (per the contract)

A `docs_citation_anchor_record` MUST carry:

- `anchor_id`, `anchor_kind`, `source_class`, `pack_id`,
  `pack_revision_ref`, `target_ref` (typed discriminator), `locale`,
  `target_build_identity_ref`, `freshness_class_at_mint`,
  `version_match_state_at_mint`, `client_scopes`, `derivation`,
  `upstream_citation_anchor_refs`, `reuse_surfaces`,
  `export_posture`, `policy_context`, `redaction_class`,
  `minted_at`.
- When `derivation` is `derived` or `derived_with_upstream_anchors`,
  at least one `upstream_citation_anchor_refs` entry MUST resolve
  to an authoritative source class; citing only
  `vendor_provider_anchor` or `external_status_feed_anchor` is
  non-conforming.
- When `browser_handoff_reason` is non-null,
  `browser_handoff_packet_ref` MUST be non-null.

A `symbol_linked_reference_record` MUST carry:

- `reference_id`, `subject_kind`, `subject_ref`, `display_label`,
  `binding_kind`, `symbol_link_resolution_class`,
  `symbol_link_resolution_fallback_chain`, `pack_id`,
  `pack_revision_ref`, `source_class`, `locale`,
  `target_build_identity_ref`, `freshness_class_at_mint`,
  `version_match_state_at_mint`, `client_scopes`,
  `project_vs_vendor_truth_cue`,
  `derived_explanation_reuse_state`, `citation_anchor_refs`,
  `reuse_surfaces`, `export_posture`, `policy_context`,
  `redaction_class`, `minted_at`.
- When `symbol_link_resolution_class` is
  `unresolved_requires_refresh` or `no_claim_yet_support_routed`,
  `repair_hook_ref` MUST be non-null.
- When `binding_kind` is `ai_derived_explanation_entry` or
  `derived_explanation_reuse_state` is
  `reusable_with_citation_anchor`, `citation_anchor_refs` MUST be
  non-empty.
- When `binding_kind` is `vendor_overlay_inspect_only`,
  `browser_handoff_reason` + `browser_handoff_packet_ref` MUST be
  non-null.

## Fixtures

### `docs_citation_anchor_record` cases

- [`project_docs_overrides_vendor_citation.json`](./project_docs_overrides_vendor_citation.json) —
  project docs outrank the vendor overlay on an in-scope symbol;
  `derivation=primary`, no browser handoff, `retain_full` export
  posture.
- [`mirrored_official_docs_citation.json`](./mirrored_official_docs_citation.json) —
  mirrored language standard-library pack with a typed browser
  handoff to the upstream portal (`external_docs_or_runbook`);
  `warm_cached` freshness with the mirror snapshot pinned on
  `pack_revision_ref`.
- [`extension_curated_pack_citation.json`](./extension_curated_pack_citation.json) —
  glossary card for an extension-contributed curated knowledge
  pack; `glossary_term_anchor`, `derivation=primary`, reusable on
  the AI-explanation overlay.
- [`live_external_docs_citation.json`](./live_external_docs_citation.json) —
  live vendor-provider docs row with a typed browser handoff and
  `retain_opaque_only` export posture; a derived AI-explanation
  overlay MUST cite additional authoritative anchors alongside
  this one to avoid `derived_explanation_uncited` refusal.
- [`derived_explanation_citation.json`](./derived_explanation_citation.json) —
  AI-explanation overlay paragraph with
  `derivation=derived_with_upstream_anchors` and two retained
  upstream anchors (project symbol + generated-reference setting);
  satisfies the 'derived explanations never appear as uncited
  primary authority' rule.
- [`localized_guided_tour_citation.json`](./localized_guided_tour_citation.json) —
  guided-tour onboarding step localized to `pt-BR`;
  `onboarding_step_anchor` preserves the locale so exports render
  the typed partial-locale-coverage disclosure alongside the
  step.
- [`hosted_review_repair_packet_citation.json`](./hosted_review_repair_packet_citation.json) —
  support-runbook step referenced in hosted-review evidence and
  repair-packet evidence; `internal_support_restricted` redaction
  class forces `strip_for_redaction_class` on metadata-safe
  exports; anchor survives hosted-review and repair-packet
  handoff.

### `symbol_linked_reference_record` cases

- [`symbol_linked_reference_exact_match.json`](./symbol_linked_reference_exact_match.json) —
  docs search row for a project symbol with
  `exact_symbol_match` resolution, empty fallback chain, and one
  attached citation anchor; `derived_explanation_reuse_state =
  reusable_with_citation_anchor`.
- [`symbol_linked_reference_unresolved_refresh.json`](./symbol_linked_reference_unresolved_refresh.json) —
  docs search row whose pack index is stale;
  `unresolved_requires_refresh` with the full fallback chain,
  empty `citation_anchor_refs`, `derived_explanation_reuse_state =
  refused_uncited`, and a `refresh_freshness` repair hook.

## Related schemas and artifacts

- [`/schemas/docs/citation_anchor.schema.json`](../../../schemas/docs/citation_anchor.schema.json)
  — the `docs_citation_anchor_record` shape.
- [`/schemas/docs/symbol_linked_reference.schema.json`](../../../schemas/docs/symbol_linked_reference.schema.json)
  — the `symbol_linked_reference_record` shape.
- [`/schemas/docs/help_status_badge.schema.json`](../../../schemas/docs/help_status_badge.schema.json)
  — the minimal `citation_anchor_record` reserved in ADR 0013 for
  badge-row inlining; `help_status_badge_record.citation_anchor_refs`
  resolve into the richer records in this corpus.
- [`/schemas/docs/docs_pack_manifest.schema.json`](../../../schemas/docs/docs_pack_manifest.schema.json)
  — `pack_id` and `pack_revision_ref` targets every citation
  anchor and symbol-linked reference pins into.
- [`/fixtures/docs/docs_pack_examples/`](../docs_pack_examples/) —
  pack manifests the anchors and references in this corpus pin
  into.
- [`/fixtures/docs/symbol_link_validation_manifest.yaml`](../symbol_link_validation_manifest.yaml)
  — symbol-link validation corpus that re-exports the frozen
  `symbol_link_resolution_class`,
  `project_vs_vendor_truth_cue`, and
  `derived_explanation_reuse_state` vocabularies these fixtures
  carry.
