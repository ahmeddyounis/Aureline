# M5 Docs-Source, Docs-Result, Docs-Pack-Manifest, Version-Match, Citation-Set, and Browser-Handoff Matrix

This document is the contract for the frozen M5 matrix that names the canonical
Aureline documentation object model. The matrix is the single M5 source of truth
for documentation: docs browser, docs/code search, AI explain, onboarding,
Help/About, support exports, the extension API, and the release center ingest the
checked-in packet rather than re-expressing docs truth ad hoc.

- Record kind: `freeze_m5_docs_source_result_pack_version_match_citation_set_and_browser_handoff_matrix`
- Schema: [`schemas/docs/freeze-the-m5-docs-source-result-pack-version-match-citation-set-and-browser-handoff-matrix.schema.json`](../../../schemas/docs/freeze-the-m5-docs-source-result-pack-version-match-citation-set-and-browser-handoff-matrix.schema.json)
- Canonical support export: [`artifacts/docs/m5/freeze_the_m5_docs_source_result_pack_version_match_citation_set_and_browser_handoff_matrix/support_export.json`](../../../artifacts/docs/m5/freeze_the_m5_docs_source_result_pack_version_match_citation_set_and_browser_handoff_matrix/support_export.json)
- Summary artifact: [`artifacts/docs/m5/freeze_the_m5_docs_source_result_pack_version_match_citation_set_and_browser_handoff_matrix.md`](../../../artifacts/docs/m5/freeze_the_m5_docs_source_result_pack_version_match_citation_set_and_browser_handoff_matrix.md)
- Fixtures: [`fixtures/docs/m5/freeze_the_m5_docs_source_result_pack_version_match_citation_set_and_browser_handoff_matrix/`](../../../fixtures/docs/m5/freeze_the_m5_docs_source_result_pack_version_match_citation_set_and_browser_handoff_matrix/)
- Producer: `aureline_docs::current_stable_m5_docs_contracts_matrix_export`

## Governed objects

| Object | Qualification | State vocabularies | Source contract |
| --- | --- | --- | --- |
| `docs_source_descriptor` | Stable | source_class / trust_class / locale_match / mirror_offline_posture | [`schemas/docs/stable_docs_source_result_pack_and_citation.schema.json`](../../../schemas/docs/stable_docs_source_result_pack_and_citation.schema.json) |
| `docs_result_object` | Stable | source_class / trust_class / version_match_state / freshness_state | [`schemas/docs/stable_docs_source_result_pack_and_citation.schema.json`](../../../schemas/docs/stable_docs_source_result_pack_and_citation.schema.json) |
| `docs_pack_manifest` | Stable | source_class / version_match_state / mirror_offline_posture / locale_match | [`schemas/docs/docs_pack_manifest.schema.json`](../../../schemas/docs/docs_pack_manifest.schema.json) |
| `derived_explanation_citation_set` | Stable | source_class / trust_class / freshness_state | [`schemas/docs/derived_explanation_descriptor.schema.json`](../../../schemas/docs/derived_explanation_descriptor.schema.json) |
| `version_match_state` | Stable | version_match_state / freshness_state | [`schemas/docs/stable_docs_source_result_pack_and_citation.schema.json`](../../../schemas/docs/stable_docs_source_result_pack_and_citation.schema.json) |
| `stale_example_finding` | Stable | version_match_state / freshness_state | [`schemas/docs/stable_docs_source_result_pack_and_citation.schema.json`](../../../schemas/docs/stable_docs_source_result_pack_and_citation.schema.json) |
| `browser_handoff_object` | Beta | browser_handoff_reason / browser_handoff_privacy_consequence | [`schemas/integration/browser_handoff_packet.schema.json`](../../../schemas/integration/browser_handoff_packet.schema.json) |

Each object row binds a qualification class to its required fields, the controlled
state vocabularies it carries, the concrete vocabulary tokens it admits, its
evidence requirement, the proof packet refs that keep it current, its downgrade
triggers, its rollback posture, its source contracts, and the consumer surfaces
that must project its qualification truth. An object kind's required state
vocabularies must appear in `state_vocabularies`, and a declared vocabulary must
carry concrete tokens while an undeclared vocabulary must carry none — so the
matrix is exact about which truth each object speaks.

## Controlled vocabulary

The matrix freezes one self-describing `vocabulary_set` block, mapped onto the
canonical tokens already owned by the docs-browser, docs-pack, derived-explanation,
locale-overlay, and scoped browser-handoff runtimes rather than minting parallel
tokens:

- **Source class** — `project_docs`, `mirrored_official_docs`, `extension_docs_pack`,
  `live_external_docs`, `curated_knowledge_pack`, `generated_reference`,
  `derived_explanation`. Project docs never masquerade as vendor docs.
- **Version-match state** — `exact_build_match`, `compatible_minor_drift`,
  `incompatible_drift_detected`, `pre_release_unverified`, `unknown_target_build`.
  A non-exact match never silently upgrades to an exact build match.
- **Freshness** — `authoritative_live`, `warm_cached`, `degraded_cached`, `stale`,
  `unverified`. A cached or stale source never claims live authority.
- **Trust class** — `first_party_authoritative`, `signed_mirror_verified`,
  `extension_pack_signed`, `live_provider_handoff`, `derived_inference_only`.
  Derived inference is never primary authority.
- **Locale match** — `source_language_original`, `translated_complete`,
  `translated_partial`, `translated_stale`, `source_language_fallback`,
  `locale_not_installed`. Localized prose never implies reviewed parity it lacks.
- **Mirror/offline posture** — `live_online`, `local_project_pack`,
  `generated_local`, `mirrored_pack`, `offline_pinned_pack`, `cached_local`,
  `not_installed`, `support_pack`.
- **Browser-handoff reason** — `exact_anchor_unavailable_locally`,
  `live_version_newer_than_mirror`, `source_not_mirrored`,
  `review_thread_requires_hosted_view`, `user_requested_open_in_browser`.
- **Browser-handoff privacy consequence** — `no_context_shared`, `scoped_url_only`,
  `query_terms_disclosed`, `isolated_session`, `shared_context_blocked`. A
  context-sharing handoff that exceeds the qualified scope is blocked, not
  performed silently.

The `vocabulary_set` block must match these canonical token lists exactly; any
drift fails validation with `vocabulary_set_drift`.

## Track invariant

Documentation truth stays typed and inspectable. The `trust_review` block encodes
the lane invariants as hard flags — all must hold for the matrix to validate:

- `source_class_locale_version_freshness_visible` — source class, locale, version
  match, and freshness stay visible.
- `project_docs_never_masquerade_as_vendor` — project docs never claim a
  higher-trust vendor identity.
- `derived_explanations_never_outlive_citation_sets` and
  `citations_bound_to_source_and_version` — a derived explanation expires with its
  citation set, which stays bound to source identity and version.
- `version_match_and_freshness_never_silently_upgraded` — version-match and
  freshness truth never silently upgrades.
- `mirror_offline_state_disclosed` — mirror/offline posture stays disclosed.
- `handoff_never_silently_shares_context` and
  `handoff_never_impersonates_governed_docs` — browser handoff cannot silently
  share context or impersonate a governed docs surface.
- `stale_examples_surfaced_not_hidden` — stale examples are surfaced, not hidden.
- `no_speculative_knowledge_platform_or_hosted_search` — no speculative
  knowledge-platform or hosted-search product is in scope.
- `downgrade_narrows_instead_of_hides` and
  `stale_or_underqualified_blocks_promotion`.

## Consumer projection and release posture

`consumer_projection` binds every consumer surface to the shared object model:
docs browser, docs search, AI explain, onboarding, support export, the extension
API, the release center, Help/About, and the browser companion all read the same
packet, and Preview/Labs surfaces are visibly labeled when not covered. The
`release_posture` block binds the supporting release packet
(`evidence:docs-contracts-release-packet:m5`) and the mirror/offline packet
(`evidence:docs-contracts-mirror-offline-packet:m5`) and requires support/export
and mirror/offline parity for every object.

## Downgrade and freshness

`proof_freshness` carries the SLO (168 hours) and the last-refresh timestamp; when
proof goes stale `auto_narrow_on_stale` narrows the affected object. The supported
downgrade triggers are `proof_stale`, `policy_blocked`, `mirror_offline`,
`source_version_mismatch`, `freshness_expired`, `trust_narrowing`,
`citation_set_expired`, `source_class_unverified`, `handoff_context_leak_risk`,
`locale_skew_detected`, and `upstream_dependency_narrowed`. The
[fixtures](../../../fixtures/docs/m5/freeze_the_m5_docs_source_result_pack_version_match_citation_set_and_browser_handoff_matrix/)
show a held browser-handoff object and a mirror-offline-narrowed docs-pack
manifest; both remain valid packets because narrowing is explicit, not hidden.

Stable promotion of any claimed docs/help/onboarding/AI row that maps to a governed
object fails while that object lacks a current matrix entry and mapped proof packet:
`current_stable_m5_docs_contracts_matrix_export` revalidates the checked-in packet,
and a missing object, drifted vocabulary, missing proof ref, or unsatisfied trust
invariant blocks the packet.

## Boundary

Raw document bodies, raw source files, rendered HTML, raw URLs, raw provider
payloads, credentials, and live vendor-doc snapshots never cross this boundary. The
packet carries only metadata, qualification truth, controlled-vocabulary tokens,
and contract references.
