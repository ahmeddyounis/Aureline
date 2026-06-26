# M5 Documentation-Claim Certification

This document is the contract for the M5 certification packet that qualifies
every claimed documentation-facing profile — the documentation browser,
Help/About/service-health, onboarding/learning, AI explanation, and
support/export surfaces — against the frozen docs-source/result/pack/version-
match/citation-set/browser-handoff matrix and the checked-in evidence corpus
those contracts produce. The packet is the canonical M5 control source for this
lane: release gates, the claim-publication pipeline, support exports,
onboarding, and About/help/service-health surfaces ingest the checked-in packet
rather than cloning status text. **No profile may stay greener than this packet,
and no profile may stay greener than the frozen matrix.**

- Record kind: `certify_docs_source_pack_version_citation_and_browser_handoff_truth_and_narrow_stale_claims`
- Schema: [`schemas/docs/certify-docs-source-pack-version-citation-and-browser-handoff-truth-and-narrow-stale-claims.schema.json`](../../../schemas/docs/certify-docs-source-pack-version-citation-and-browser-handoff-truth-and-narrow-stale-claims.schema.json)
- Canonical support export: [`artifacts/docs/m5/certify_docs_source_pack_version_citation_and_browser_handoff_truth_and_narrow_stale_claims/support_export.json`](../../../artifacts/docs/m5/certify_docs_source_pack_version_citation_and_browser_handoff_truth_and_narrow_stale_claims/support_export.json)
- Summary artifact: [`artifacts/docs/m5/certify_docs_source_pack_version_citation_and_browser_handoff_truth_and_narrow_stale_claims.md`](../../../artifacts/docs/m5/certify_docs_source_pack_version_citation_and_browser_handoff_truth_and_narrow_stale_claims.md)
- Fixtures: [`fixtures/docs/m5/certify_docs_source_pack_version_citation_and_browser_handoff_truth_and_narrow_stale_claims/`](../../../fixtures/docs/m5/certify_docs_source_pack_version_citation_and_browser_handoff_truth_and_narrow_stale_claims/)
- Producer: `aureline_docs::current_stable_docs_claim_certification_export`
- Emitter: `cargo run -p aureline-docs --bin aureline_docs_claim_certification -- packet`

## Certified profiles

Each row binds one claimed profile to the documentation-evidence classes it
depends on, the upstream schemas and support exports that form its evidence, the
qualification class it earned, a certification verdict, downgrade triggers, and a
`not_greener_than_matrix` flag. Every row also records that source class is
disclosed (so project docs never masquerade as vendor docs), and — where the
profile depends on the matching evidence class — that derived explanations keep a
citation basis and that browser handoff keeps context isolated.

| Profile | Qualification | Evidence classes |
| --- | --- | --- |
| `docs_browser` | Stable | source_class, docs_pack_lifecycle, version_match, browser_handoff |
| `help_about` | Stable | source_class, version_match, browser_handoff |
| `onboarding_learning` | Beta | source_class, version_match, citation_set |
| `ai_explanation` | Beta | source_class, version_match, citation_set, browser_handoff |
| `support_export` | Stable | source_class, docs_pack_lifecycle, version_match, citation_set, browser_handoff |

A certified-and-promoted profile (Stable or Beta with a `certified` or
`narrowed_to_qualified` verdict) must carry at least one evidence packet ref.
Every row's `evidence_schema_refs` and `evidence_artifact_refs` must match the
canonical refs of the evidence classes it lists, so the certification can never
drift from the real documentation contracts it certifies against.

## Documentation-evidence classes

The five evidence classes map each profile onto the upstream B-batch
documentation contracts and their checked-in support exports — the evidence
corpus this certification is tied to:

| Evidence class | Upstream contract |
| --- | --- |
| `source_class` | [`stable-docs-source-and-result-object-reuse-across-consumer-surfaces`](../../../schemas/docs/stable-docs-source-and-result-object-reuse-across-consumer-surfaces.schema.json) + [`add-docs-source-precedence-and-ranking-parity-across-search-hover-onboarding-and-ai-context`](../../../schemas/docs/add-docs-source-precedence-and-ranking-parity-across-search-hover-onboarding-and-ai-context.schema.json) |
| `docs_pack_lifecycle` | [`ship-docs-pack-manager-rows-and-manifest-parity-with-import-export-continuity`](../../../schemas/docs/ship-docs-pack-manager-rows-and-manifest-parity-with-import-export-continuity.schema.json) |
| `version_match` | [`add-version-freshness-vocabulary-and-stale-example-broken-link-findings`](../../../schemas/docs/add-version-freshness-vocabulary-and-stale-example-broken-link-findings.schema.json) |
| `citation_set` | [`implement-derived-explanation-citation-sets-binding-docs-ai-glossary-tours-and-support-exports`](../../../schemas/docs/implement-derived-explanation-citation-sets-binding-docs-ai-glossary-tours-and-support-exports.schema.json) |
| `browser_handoff` | [`implement-browser-provider-console-handoff-objects-with-destination-reason-privacy-consequence-and-return-anchor`](../../../schemas/docs/implement-browser-provider-console-handoff-objects-with-destination-reason-privacy-consequence-and-return-anchor.schema.json) |

The `compatibility_report` asserts that every evidence class is covered by at
least one profile, so no documentation contract in this batch is left
unqualified.

## Compatibility report

The `compatibility_report` binds this certification to the frozen docs-contracts
matrix by support-export and schema ref, pins the matrix schema version, and
asserts that every profile is present, that every evidence class is covered, that
no profile is greener than the matrix, that every profile carries evidence, and
that the downgrade rules are auto-enforced. Release tooling reads these flags
directly.

## Downgrade rules and automation

The `downgrade_rules` set is machine-readable and auto-enforced. Each rule binds
a trigger to a narrowing action over the profiles and evidence classes it applies
to. There is one rule per evidence-class staleness trigger, so stale evidence in
any class narrows or holds the affected profiles:

- `source_class_evidence_stale` → **mark retest-pending** every profile that
  discloses source class, so project docs never silently masquerade as vendor
  docs.
- `docs_pack_lifecycle_evidence_stale` → **narrow to Beta** the docs browser and
  support export with explicit pack-state labels.
- `version_match_evidence_stale` → **mark retest-pending** so no answer keeps
  exact-current confidence on a possibly-drifted version.
- `citation_set_evidence_stale` → **hold** the explaining profiles; derived
  explanations never outlive their citation sets.
- `browser_handoff_evidence_stale` → **block publication** of any handoff-bearing
  profile; a handoff must not silently share context or impersonate a docs
  surface.
- `proof_freshness_expired` → **hold** every profile until re-proven.
- `greener_than_matrix` → **block publication** of any profile that drifts
  greener than the frozen matrix; this packet is canonical.

`DocsClaimCertificationPacket::narrowed_for_stale_evidence` applies the auto-
narrow behavior directly: given the set of stale evidence classes, every profile
that depends on a stale class is narrowed to Preview and marked `retest_pending`.
`narrowed_profiles`, `retest_pending_profiles`, and `publication_blockers` expose
the result to release and claim-publication tooling. This is how claim
publication narrows automatically when documentation evidence is stale, partial,
or failing, instead of leaving old green language in product and docs.

## Track invariant

The `trust_review` block encodes the lane invariants as hard constraints — all
must hold for the certification to validate: source class stays visible and
project docs never masquerade as vendor docs; version match and freshness stay
visible; derived explanations keep a citation basis and never outlive their
citation sets; browser handoff never silently shares context or impersonates a
docs surface; mirror/offline posture stays visible; no profile stays greener than
this packet; downgrade narrows rather than hides; and stale, partial, or failing
evidence narrows or blocks claim publication.

The `consumer_projection` block names the surfaces that consume this packet
rather than re-deriving docs/help truth: the release gate, the claim-publication
pipeline, About/help/service-health, support export, onboarding, and AI context.
Narrowed, retest-pending, held, and blocked profiles are visibly labeled, not
hidden, so the product, docs, release, support, and onboarding surfaces all
explain the current documentation truth from one packet set.

## Boundary

The packet references upstream schemas, support exports, and contracts by id.
Raw document bodies, raw source files, rendered HTML, raw URLs, raw query text,
raw provider payloads, credentials, and live vendor-doc snapshots stay outside
the support boundary. The `redaction_class_token` records the redaction posture,
and `validate` rejects any export that carries forbidden boundary material.
