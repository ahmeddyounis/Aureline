# AI Copy Guardrails, Confidence Vocabulary, and Forbidden High-Trust Phrasing

This document is the contract for the AI copy guardrail catalog. The catalog is the
single source of truth for the controlled AI wording Aureline renders on
trust-sensitive assistant and review surfaces, and for the rejection register that
blocks high-trust overclaiming. The prompt composer, patch review, notebook help,
docs/help, and provider/account surfaces resolve AI wording through this catalog —
and route candidate copy through its lint — rather than inventing surface-local
phrasing that could imply a guarantee, autonomy, or review-free completion.

It materializes the product-wide
[AI copy guardrails contract](../../ai/ai_copy_guardrails_contract.md) and projects
the [approved](../../../artifacts/ai/approved_ai_terms.yaml) and
[forbidden](../../../artifacts/ai/forbidden_ai_terms.yaml) AI term registers into a
typed, validated truth packet.

- Record kind: `m5_ai_copy_guardrail_catalog`
- Schema: [`schemas/content/m5-ai-copy-guardrails.schema.json`](../../../schemas/content/m5-ai-copy-guardrails.schema.json)
- Canonical support export: [`artifacts/content/m5-ai-copy-proof/support_export.json`](../../../artifacts/content/m5-ai-copy-proof/support_export.json)
- Summary artifact: [`artifacts/content/m5-ai-copy-proof/m5_ai_copy_guardrails.md`](../../../artifacts/content/m5-ai-copy-proof/m5_ai_copy_guardrails.md)
- Fixtures: [`fixtures/content/m5-ai-copy-guardrails/`](../../../fixtures/content/m5-ai-copy-guardrails/)
- Producer: `aureline_shell::content::ai_copy_guardrails::current_ai_copy_guardrail_catalog_export`
- Headless emitter: `aureline_shell_m5_ai_copy_guardrails`

## Controlled AI terms

An `AiCopyTerm` is a controlled wording object: one reserved meaning, one
locale-neutral machine token, the protected surfaces it governs, and the reuse
consumers that must reconstruct it. Each term materializes a required
`AiTaxonomyConcept`, and the catalog must carry at least one term per concept — these
are exactly the controlled wordings the lane is required to implement:

- `suggested` — model-proposed guidance not yet accepted, applied, or validated.
- `proposed` — a concrete change put forward for review but not yet accepted.
- `draft` — generated content or a patch held outside canonical source truth.
- `context_used` — the context segments an answer actually used and the scope left out.
- `validation` — a validation state tied to a named validation record
  (`Validation not run`, `Validation passed`).
- `low_confidence` — confidence below the surface floor because evidence is limited.
- `review_required` — human, policy, ownership, or write-scope review remains required.
- `revert_undo_available` — a prior, known-good state can be reverted or undone.

Each concept belongs to one `AiCopyDomain` (`proposal_state`, `confidence`,
`validation`, `context_disclosure`, `review_posture`, `reversibility`), and a term's
declared `domain` must match its concept's domain.

### AI wording stays provisional and review-honest

Every term is `ai_provisional`: it is provisional AI wording, never a deterministic
guarantee. This is what keeps AI copy distinct from deterministic language-service or
formatter/refactor wording where that difference matters for trust. Low-confidence
and review-required terms carry `suppresses_direct_mutation`, so an AI surface can
never offer a direct apply from an unproven or unreviewed proposal.

`render_term_reference` reconstructs a term's exact in-product wording — label,
concept, domain, reserved meaning, required context, and the mutation-suppression
note — so a review, help, or support surface can reconstruct the proposal,
confidence, or validation language the user saw.

## Forbidden high-trust phrases

A `ForbiddenPhrase` is a lowercase `pattern` in a `ForbiddenPhraseClass`
(`perfection_guarantee`, `review_free_mutation`, `false_autonomy`,
`false_validation`, `confidence_overstatement`, `false_exhaustiveness`,
`false_freshness`), with a `rejection_reason` and the approved term ids that replace
it. The four phrases the lane is explicitly required to reject —
`guaranteed`, `perfect`, `done for you`, and `no review needed` — are always in the
register, and `forbidden_on` lists the protected surfaces each is forbidden on (the
seed forbids every high-trust overclaim on every protected surface).

### Lint

`AiCopyGuardrailCatalog::lint(candidate, surface)` lowercases the candidate copy and
returns an `AiCopyLintFinding` for every forbidden phrase whose `forbidden_on`
includes the surface and whose pattern appears as a substring. Each finding carries
the matched pattern, its class, and the approved replacement term ids, so a surface
or review tool both rejects the overclaim and is told what to say instead. The
headless emitter exposes this as `… lint "<candidate copy>"`.

The catalog eats its own dog food: `validate` runs every approved term's canonical
label and reserved meaning through the lint on every surface it governs, so the
approved vocabulary can never quietly smuggle in an overclaim.

## Locale neutrality

Machine-facing identity stays locale-neutral while human prose localizes around it.
Term ids, machine tokens, and phrase ids are lowercase ascii (`[a-z0-9_.]`); a
forbidden pattern is a stable lowercase phrase (`[a-z0-9 -]`). Only labels, reserved
meanings, required-context lines, forbidden-use lines, and rejection reasons carry
human prose. The localized overlay fixture rewrites every prose field into a
pseudo-localized form while keeping every id, token, concept, surface, consumer,
class, and pattern byte-for-byte identical — proving a translation can never fork the
meaning of a proposal, a confidence label, a validation state, or a forbidden
pattern.

## Cross-consumer reuse

The same term objects are reconstructed across the product UI, docs/help, support
exports, narrated announcements, and release/demo artifacts. The
`shared_reuse_term_ids` — `suggested`, `low_confidence`, `context_used` — must each
span at least `SHARED_TERM_MIN_REUSE_CONSUMERS` (3) distinct reuse consumers.
`cross_consumer_reuse` maps each term to the consumers that reconstruct it, and
validation fails if a shared term collapses to fewer consumers.

## Validation invariants

`AiCopyGuardrailCatalog::validate` enforces, among others:

- record kind, schema version, and identity are present;
- the five closed inventories match the canonical token lists;
- term ids, machine tokens, and labels are unique and locale-neutral, and each
  term's declared domain matches its concept's domain;
- every required taxonomy concept has a term;
- every term is `ai_provisional`, and low-confidence / review-required terms suppress
  direct mutation;
- no approved term's own copy overclaims (it passes the catalog's own lint);
- forbidden phrase ids are unique and locale-neutral, patterns are lowercase and
  unique, every replacement term id resolves, and the four required high-trust
  phrases are present;
- every domain, surface, consumer, and forbidden class is represented;
- each shared reuse term spans at least three consumers;
- the trust-review and consumer-projection invariants all hold;
- the export carries no raw boundary material.

## Acceptance mapping

| Acceptance clause | Resolved by |
|---|---|
| Claimed M5 AI and review surfaces no longer overstate certainty, autonomy, or validation state through copy alone. | The forbidden-phrase register, the per-surface `lint`, and the self-lint invariant that rejects overclaiming approved copy. |
| Review/help/support artifacts can reconstruct the exact proposal/validation/confidence language shown in-product. | `render_term_reference`, the support export, and the consumer-projection invariants. |
| AI wording remains distinct from deterministic language-service or formatter/refactor wording where that difference matters for trust. | The `ai_provisional` invariant on every term and the `ai_wording_distinct_from_deterministic_service_wording` trust-review flag. |

## Fixtures

The fixtures are valid, export-safe catalog packets minted from the same seed builder
as the canonical export by `aureline_shell_m5_ai_copy_guardrails`. See
[the fixtures README](../../../fixtures/content/m5-ai-copy-guardrails/README.md).
