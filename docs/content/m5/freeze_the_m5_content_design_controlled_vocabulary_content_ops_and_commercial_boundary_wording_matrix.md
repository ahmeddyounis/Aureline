# M5 Content-Design, Controlled-Vocabulary, Content-Ops, and Commercial-Boundary Wording Matrix

This document is the contract for the frozen M5 matrix that names the canonical
Aureline product-wording object model. The matrix is the single M5 source of truth
for user-facing wording: product UI, CLI/help, docs, support exports, release
notes, screenshots/demos, AI surfaces, onboarding, Help/About, and the marketplace
ingest the checked-in packet rather than maintaining parallel copy lists.

- Record kind: `freeze_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix`
- Schema: [`schemas/content/freeze-the-m5-content-design-controlled-vocabulary-content-ops-and-commercial-boundary-wording-matrix.schema.json`](../../../schemas/content/freeze-the-m5-content-design-controlled-vocabulary-content-ops-and-commercial-boundary-wording-matrix.schema.json)
- Canonical support export: [`artifacts/content/m5/freeze_the_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix/support_export.json`](../../../artifacts/content/m5/freeze_the_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix/support_export.json)
- Summary artifact: [`artifacts/content/m5/freeze_the_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix.md`](../../../artifacts/content/m5/freeze_the_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix.md)
- Fixtures: [`fixtures/content/m5/freeze_the_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix/`](../../../fixtures/content/m5/freeze_the_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix/)
- Producer: `aureline_shell::freeze_the_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix::current_stable_m5_content_wording_matrix_export`
- Headless emitter: `aureline_shell_content_wording_matrix`

## Governed objects

| Object | Qualification | Owner | State vocabularies | Source contract |
| --- | --- | --- | --- | --- |
| `safety_critical_ui_string` | Stable | Product copy owner | lifecycle_state / trust_class / policy_state / freshness_state | [`docs/copy/naming_and_state_label_contract.md`](../../copy/naming_and_state_label_contract.md) |
| `glossary_term` | Stable | Design systems owner | lifecycle_state / trust_class / client_scope | [`artifacts/copy/controlled_glossary.yaml`](../../../artifacts/copy/controlled_glossary.yaml) |
| `action_label_pattern` | Stable | Product copy owner | policy_state / client_scope | [`docs/copy/ui_copy_contract.md`](../../copy/ui_copy_contract.md) |
| `error_recovery_block` | Stable | Supportability owner | policy_state / freshness_state | [`docs/copy/ui_copy_contract.md`](../../copy/ui_copy_contract.md) |
| `ai_copy_guardrail` | Beta | AI product owner | trust_class / policy_state / freshness_state | [`docs/ai/ai_copy_guardrails_contract.md`](../../ai/ai_copy_guardrails_contract.md) |
| `count_scope_phrase_set` | Stable | Design systems owner | freshness_state / compatibility_state | [`docs/copy/count_scope_freshness_grammar.md`](../../copy/count_scope_freshness_grammar.md) |
| `content_ops_artifact` | Stable | Docs owner | compatibility_state / freshness_state | [`docs/copy/translation_safe_content_ops_contract.md`](../../copy/translation_safe_content_ops_contract.md) |
| `commercial_boundary_wording` | Beta | Commercial boundary owner | hosting_boundary / edition_label / client_scope | [`artifacts/governance/deployment_profiles.yaml`](../../../artifacts/governance/deployment_profiles.yaml) |

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
canonical tokens already owned by the controlled glossary, the count/scope/freshness
grammar, the AI copy guardrails contract, the product truth vocabulary, and the
deployment-profile register rather than minting parallel tokens:

- **Lifecycle** — `labs`, `preview`, `beta`, `stable`, `lts_facing`, `deprecated`,
  `disabled_by_policy`, `retired`. Mirrors the product truth vocabulary lifecycle
  class; a wording surface never invents a parallel release-stage synonym.
- **Trust class** — `official_public`, `official_private`, `community`,
  `third_party_vendor`. Mirrors the authority class; community or third-party
  sources are never relabeled as official.
- **Policy state** — `allowed`, `trust_required`, `restricted`, `requires_review`,
  `policy_blocked`. `Trust required`, `Restricted`, and `Policy blocked` are never
  softened into generic unavailability.
- **Compatibility** — `compatible`, `minor_skew_compatible`, `incompatible`,
  `unverified_compatibility`, `deprecated_path`.
- **Freshness** — `proven_current`, `cached`, `warming`, `stale`, `unverified`.
  `stale` keeps one reserved meaning across UI, CLI, docs, exports, and support
  packets; a cached or stale value never implies proven-current authority.
- **Client scope** — `desktop`, `browser_companion`,
  `desktop_plus_browser_companion`, `headless_only`, `local_only`. A browser
  companion never implies full desktop parity, and `local_only` is never claimed
  when managed recall, sync, or hosted evidence participated.
- **Hosting boundary** — `individual_local`, `self_hosted`, `enterprise_online`,
  `air_gapped`, `managed_cloud`. Mirrors the frozen deployment-profile vocabulary.
- **Edition label** — `open_source`, `local_independent`, `self_hosted`, `managed`,
  `commercial`. Open or local-independent language is never applied when managed
  services participated, and commercial language is never applied to an open-source
  capability.

The `vocabulary_set` block must match these canonical token lists exactly; any
drift fails validation with `vocabulary_set_drift`.

## Track invariant

Language stays truthful and machine-anchored. The `trust_review` block encodes the
lane invariants as hard flags — all must hold for the matrix to validate:

- `safety_critical_strings_use_stable_ids` and
  `safety_critical_strings_use_controlled_terms` — safety-critical strings keep
  stable message ids and controlled terms.
- `action_labels_and_counts_scope_honest` — action labels and counts stay
  scope-honest.
- `error_copy_explains_failure_remaining_capability_and_next_action` — error copy
  explains failure, remaining capability, and the next safe action.
- `ai_wording_never_overstates_confidence_or_autonomy` — AI wording never
  overstates confidence or autonomy.
- `content_ops_artifacts_keep_version_and_source_metadata` — docs/help/export and
  screenshot/demo artifacts keep version/source metadata.
- `commercial_boundary_wording_matches_product_boundary` — hosted/open/self-hosted/
  commercial language cannot drift from the actual product boundary.
- `controlled_terms_never_softened_for_tone` and
  `one_controlled_term_inventory_not_parallel_copy_lists` — controlled terms are
  never softened, and every surface resolves to one controlled-term inventory.
- `no_speculative_brand_or_marketing_campaign_scope` — no speculative brand-refresh
  or marketing-campaign work is in scope.
- `downgrade_narrows_instead_of_hides` and
  `stale_or_underqualified_blocks_promotion`.

## Consumer projection and release posture

`consumer_projection` binds every consumer surface to the shared object model:
product UI, CLI/help, docs, support export, release notes, screenshots/demos, AI
surfaces, onboarding, Help/About, and the marketplace all read the same packet, and
Preview/Labs surfaces are visibly labeled when not covered. The `release_posture`
block binds the supporting release packet
(`evidence:content-wording-release-packet:m5`) and the mirror/offline packet
(`evidence:content-wording-mirror-offline-packet:m5`) and requires support/export
and mirror/offline parity for every object.

## Downgrade and freshness

`proof_freshness` carries the SLO (168 hours) and the last-refresh timestamp; when
proof goes stale `auto_narrow_on_stale` narrows the affected object. The supported
downgrade triggers are `proof_stale`, `policy_blocked`, `controlled_term_drift`,
`message_id_unstable`, `overclaim_detected`, `scope_count_dishonest`,
`freshness_expired`, `commercial_boundary_drift`, `localization_parity_lost`,
`content_ops_metadata_missing`, and `upstream_dependency_narrowed`. The
[fixtures](../../../fixtures/content/m5/freeze_the_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix/)
show a held commercial-boundary wording object and a preview-narrowed AI copy
guardrail; both remain valid packets because narrowing is explicit, not hidden.

Stable promotion of any claimed M5 user-facing row that maps to a governed object
fails while that object lacks a current matrix entry and mapped proof packet:
`current_stable_m5_content_wording_matrix_export` revalidates the checked-in packet,
and a missing object, drifted vocabulary, missing proof ref, or unsatisfied trust
invariant blocks the packet.

## Boundary

Raw message bodies, raw provider payloads, credentials, secret material, and
untranslated free-text prose never cross this boundary. The packet carries only
metadata, qualification truth, controlled-vocabulary tokens, and contract
references.
