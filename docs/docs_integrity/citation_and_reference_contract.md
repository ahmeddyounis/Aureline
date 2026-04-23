# Docs citation-anchor and symbol-linked-reference contract

This document freezes the citation object model and the symbol-linked
reference packet every docs-search result row, guided-tour step,
glossary card, onboarding step, docs-pane footer, AI-explanation
overlay citation, hosted-review evidence row, repair-packet citation,
and portability / offboarding export citation projects before
flattening a row into a user-visible chip.

The machine-readable boundaries are:

- [`/schemas/docs/citation_anchor.schema.json`](../../schemas/docs/citation_anchor.schema.json)
  — the `docs_citation_anchor_record` shape.
- [`/schemas/docs/symbol_linked_reference.schema.json`](../../schemas/docs/symbol_linked_reference.schema.json)
  — the `symbol_linked_reference_record` shape.

Worked cases (project docs outrank vendor docs, mirrored official
docs, extension curated pack, live external docs with browser handoff,
derived AI explanation with retained upstream anchors, localized
guided-tour step, hosted-review evidence, and repair-packet evidence)
live under
[`/fixtures/docs/citation_cases/`](../../fixtures/docs/citation_cases/).

The eventual docs-integrity crate's Rust types are the schema of
record. This document and the JSON Schema exports are the cross-tool
boundary every non-owning surface reads; if this document and ADR
0013 ([`docs/adr/0013-docs-help-service-health-truth.md`](../adr/0013-docs-help-service-health-truth.md))
or the docs-pack manifest contract
([`docs/docs/docs_pack_manifest_contract.md`](../docs/docs_pack_manifest_contract.md))
disagree, the ADR and the manifest contract win for vocabulary and
this document MUST be updated in the same change.

## Why freeze this now

ADR 0013 froze the `help_status_badge_record` and reserved a minimal
`citation_anchor_record` shape so the badge row could attach anchors
without speculating about the citation lane. The docs-pack manifest
contract then froze `citation_posture`,
`required_citation_anchor_kinds`, and `backlink_posture` so a pack
could declare its citation rules.

What those two contracts reserved but did not implement was the
citation object model every consuming surface (docs search, glossary
cards, guided tours, AI-explanation overlays, hosted-review evidence,
repair packets, portability exports) uses to reconstruct
"what was cited, from which source class, against which pack revision,
under which locale, at which freshness and version-match posture,
with which browser-handoff reason if any" without inventing per-surface
citation dialects.

Without this contract, each surface would invent its own citation
shape:

- Docs search would mint a flat `result_row` with a free-form
  `source` string, a copied snippet, and no link back to the pack
  manifest.
- Glossary cards would store a term-to-docs-page path and no pack
  revision ref, so a later reviewer could not tell which revision
  the card cited.
- The AI-explanation overlay would synthesize prose with no
  retained upstream anchors, violating the ADR-0013 rule that a
  derived explanation MUST cite at least one authoritative anchor.
- Guided tours would lose locale context when translated because
  their step anchors would carry only an English page path.
- Hosted-review evidence and repair packets would quote support
  runbook steps by free-form title, with no way to verify the step
  still resolves on the running build.
- Portability / offboarding exports would flatten every citation to
  a prose fragment, losing the pack-revision pin that lets a
  reviewer reconstruct the exact row the user saw.

The citation object model is the missing piece that lets one contract
back all of these surfaces without interpretation drift.

## Scope

Frozen at this revision:

- One `docs_citation_anchor_record` shape
  ([`schemas/docs/citation_anchor.schema.json`](../../schemas/docs/citation_anchor.schema.json))
  carrying source class, pack id, pack revision ref, typed target
  ref (docs page / symbol / code span / runbook step / release note
  / service-health event / glossary term / onboarding step),
  locale, target build identity ref, freshness class at mint,
  version-match state at mint, client scopes, derivation posture
  (`primary` / `derived` / `derived_with_upstream_anchors`),
  upstream anchor refs, reuse surfaces, browser-handoff reason and
  packet ref, superseded-by link, export posture, policy context,
  and redaction class.
- One `symbol_linked_reference_record` shape
  ([`schemas/docs/symbol_linked_reference.schema.json`](../../schemas/docs/symbol_linked_reference.schema.json))
  binding a product subject (symbol / file / setting / command /
  capability lifecycle / keybinding / runbook step / release note
  / glossary term / onboarding step / service-health event) to one
  or more citation anchors, with typed resolution class and
  fallback chain, project-vs-vendor truth cue, derived-explanation
  reuse state, browser-handoff fields, and repair-hook ref.
- Reuse rules that require onboarding tours, glossary cards, docs
  search, AI explanations, hosted-review evidence, repair packets,
  and portability / offboarding exports to retain upstream citations
  through every surface rather than flattening them into prose-only
  authority.
- Offline / export reconstructability rule: a reviewer holding an
  exported citation anchor can reconstruct the row the user saw by
  pairing `pack_id` + `pack_revision_ref` + `target_ref` +
  `target_build_identity_ref` with the anchor's `locale` and
  `freshness_class_at_mint`, even when the canonical owner is
  unreachable.

Out of scope until a superseding decision row opens:

- The full docs-browser UI (navigation, search ranking, hover card
  rendering). The contract reserves the citation shape; the UI
  lands later.
- The AI-explanation overlay renderer beyond the derived-explanation
  citation rule. Schema, citation shape, and denial posture are
  frozen here; the overlay's prompt / model wiring is a later lane.
- Citation-anchor signing / chain-of-custody beyond what the pack
  manifest's signing block already guarantees. The anchor pins the
  pack revision; the pack's signing block transitively signs the
  anchor.

## The one citation model

One `docs_citation_anchor_record` MUST back every user-visible
citation on every surface named below. Surfaces that "quote" a docs
row without attaching one of these anchors are non-conforming and
MUST be refused by the publisher.

| Surface                                                 | Anchor requirement                                                                 |
|---------------------------------------------------------|------------------------------------------------------------------------------------|
| Docs pane (row footer, breadcrumb, source chip)         | One anchor per cited row.                                                          |
| Docs browser (result list row, hover card)              | One anchor per result row.                                                         |
| Help / About (docs-pack summary, publisher row)         | One anchor per pack summary row.                                                   |
| Service-health view (event evidence)                    | One anchor per event row (`service_health_event_anchor`).                          |
| Support summary export                                  | Every cited row carries its anchor under the support-export redaction envelope.    |
| Onboarding / guided tour step                           | One anchor per step; `onboarding_step_anchor` with the step's locale.              |
| AI-explanation overlay                                  | One anchor per paragraph of evidence; `derivation = derived` with ≥1 upstream.     |
| Glossary card                                           | One anchor per term (`glossary_term_anchor` or a `symbol_reference_anchor`).       |
| Hosted-review evidence row                              | One anchor per evidence row (`runbook_step_anchor` / `release_note_anchor`).       |
| Repair-packet evidence citation                         | One anchor per evidence row (`runbook_step_anchor` / `release_note_anchor`).       |
| Portability / offboarding export                        | Anchors retained per `export_posture` (see below).                                 |

## Record fields

The full field set for `docs_citation_anchor_record` lives in
[`/schemas/docs/citation_anchor.schema.json`](../../schemas/docs/citation_anchor.schema.json).
The notable fields:

- **Identity.** `anchor_id` is the stable id every consuming surface
  carries through. `pack_id` + `pack_revision_ref` pin the pack and
  the exact revision the anchor was minted against.
- **Target.** `target_ref` is a typed discriminated object. Exactly
  one of `docs_page_ref`, `symbol_ref`, `code_span_ref`,
  `runbook_step_ref`, `release_note_ref`, `service_health_event_ref`,
  `glossary_term_ref`, or `onboarding_step_ref` is populated; the
  `target_kind` discriminator MUST agree with `anchor_kind`.
- **Source posture at mint.** `source_class`, `freshness_class_at_mint`,
  `version_match_state_at_mint`, `target_build_identity_ref`, and
  `locale` capture the posture the user saw so exports and support
  handoffs can recompute the chip a reviewer would have seen.
- **Reuse posture.** `client_scopes` name the surfaces the anchor
  was minted for; `reuse_surfaces` name the protected surfaces
  that may project the anchor into a user-visible row. Surfaces
  not named in `reuse_surfaces` MUST NOT render the anchor as
  authoritative.
- **Derivation.** `derivation` is one of `primary`, `derived`, or
  `derived_with_upstream_anchors`. `derived` / `derived_with_upstream_anchors`
  MUST carry at least one `upstream_citation_anchor_refs` entry,
  and at least one upstream anchor MUST resolve to an authoritative
  `source_class` (project docs, generated reference, mirrored
  official docs, curated knowledge pack, support runbook) or a
  release-note anchor; vendor-provider and external-status-feed
  anchors are not sufficient backing.
- **Browser-handoff posture.** `browser_handoff_reason` is a
  re-export of the ADR-0013 subset. Required (non-null) when the
  anchor's canonical owner resolves via an out-of-product browser
  handoff; `browser_handoff_packet_ref` is required in that case.
  Raw URLs never live on the anchor.
- **Export posture.** `export_posture` is one of `retain_full`,
  `retain_opaque_only`, or `strip_for_redaction_class`. A redaction
  class of `signing_evidence_only` or `internal_support_restricted`
  raises the posture to `strip_for_redaction_class` on
  `metadata_safe_default` exports; operators / admins may widen
  the posture only via a broader-capture opt-in.
- **Supersession.** `superseded_by_anchor_id` pins a later anchor
  that replaces this one. Surfaces render the typed "superseded"
  disclosure and route to `migrate_to_replacement` when set.

## Symbol-linked reference packet

The `symbol_linked_reference_record` in
[`/schemas/docs/symbol_linked_reference.schema.json`](../../schemas/docs/symbol_linked_reference.schema.json)
binds a product subject to one or more citation anchors. Every docs
search result row, glossary card, guided-tour step, hover card,
AI-explanation evidence row, hosted-review evidence row, and
repair-packet evidence row reuses this shape.

Notable fields:

- `subject_kind` + `subject_ref` pin the product subject the
  reference binds to (symbol, file, setting, command, capability
  lifecycle, keybinding, runbook step, release note, glossary term,
  onboarding step, service-health event).
- `binding_kind` names the kind of docs-side entry; an
  `ai_derived_explanation_entry` MUST carry at least one
  `citation_anchor_ref` whose anchor declares
  `derivation = derived` or `derived_with_upstream_anchors`. A
  `vendor_overlay_inspect_only` entry MUST carry
  `browser_handoff_reason` + `browser_handoff_packet_ref` and MUST
  NOT be projected into AI derived explanations as primary authority.
- `symbol_link_resolution_class` + `symbol_link_resolution_fallback_chain`
  are re-exports of the frozen vocabulary from
  [`/fixtures/docs/symbol_link_validation_manifest.yaml`](../../fixtures/docs/symbol_link_validation_manifest.yaml).
- `project_vs_vendor_truth_cue` names how project docs rank against
  vendor docs on this row.
- `derived_explanation_reuse_state` names whether a derived
  explanation may reuse the row (`reusable_with_citation_anchor`
  requires a non-empty `citation_anchor_refs` array; every
  `refused_*` state MUST carry a `repair_hook_ref`).
- `repair_hook_ref` is required when
  `symbol_link_resolution_class` is `unresolved_requires_refresh` or
  `no_claim_yet_support_routed`, or when the reference was refused
  for any derived-explanation reuse reason. Silent fallback is
  forbidden.

## Reuse rules across surfaces

### Docs search, docs browser, docs pane

Every result row / breadcrumb / hover card MUST attach one
`docs_citation_anchor_record` and (when a symbol / file / setting /
command is the subject) one `symbol_linked_reference_record`. The
help-badge row's `citation_anchor_refs` MUST carry the anchor's
`anchor_id`. Surfaces MAY collapse source / version / freshness into
a single chip for visual reasons, but the anchor record keeps each
axis addressable for the parity audit.

### Onboarding and guided tours

Every tour step MUST attach one `docs_citation_anchor_record` with
`anchor_kind = onboarding_step_anchor` (pointing at the tour step's
own anchor) and MAY attach additional `symbol_reference_anchor` /
`docs_page_anchor` anchors when the step cites an in-product symbol
or docs page.

Localized tours (Spanish, Portuguese-Brazilian, Simplified Chinese,
…) MUST NOT drop the upstream anchor. The citation anchor's `locale`
field names the locale the user saw; if the translation is partial
/ machine-assisted / stale against the pack's
`locale_coverage` row, the surface renders the typed locale
disclosure alongside the chip and keeps the upstream anchor in
place.

### Glossary cards

Every glossary card MUST attach one
`docs_citation_anchor_record` with
`anchor_kind = glossary_term_anchor` (or
`symbol_reference_anchor` when the term is a project symbol) and
SHOULD attach a `symbol_linked_reference_record` with
`binding_kind = glossary_card_entry`. Glossary cards that surface
without an anchor are non-conforming; the publisher MUST deny
render.

### AI-explanation overlays

AI-explanation paragraphs MUST attach one
`docs_citation_anchor_record` per paragraph of cited evidence, and
every anchor MUST declare `derivation = derived` or
`derived_with_upstream_anchors` with at least one upstream anchor
resolving to an authoritative source class (project docs, generated
reference, mirrored official docs, curated knowledge pack, support
runbook) or a release-note anchor. An empty
`upstream_citation_anchor_refs` array against a derived anchor is
refused with the ADR-0013 `derived_explanation_uncited` denial
reason; silent fallback to a generic answer is forbidden.

The symbol-linked reference record carries
`binding_kind = ai_derived_explanation_entry` and
`derived_explanation_reuse_state = reusable_with_citation_anchor`;
an overlay that fails to attach an anchor flips the state to
`refused_uncited` and MUST carry a `repair_hook_ref` (typically
`contact_support` or `refresh_freshness`).

### Hosted-review evidence and repair packets

Hosted-review evidence rows (the ADR-0015 hosted-review lane's
evidence rows pointing into support runbooks and release notes)
MUST attach one `docs_citation_anchor_record` per evidence row,
typically `runbook_step_anchor` or `release_note_anchor`. Repair
packets (the ADR-0011 repair hook lane's evidence citations) reuse
the same shape. Both surfaces carry the citation anchor through
without flattening it into prose; a reviewer holding only the
exported packet can recompute the row the user saw by looking up
the `pack_revision_ref` + `target_ref`.

### Portability / offboarding exports

Portability and offboarding exports MUST retain citation anchors
per `export_posture`. The default is `retain_full`. An anchor
whose `redaction_class` is `signing_evidence_only` or
`internal_support_restricted` is raised to
`strip_for_redaction_class` on a `metadata_safe_default` export;
the export then renders a typed "anchor redacted under
<redaction_class>" disclosure in its place. An operator / admin
may opt into a broader-capture export that widens the posture to
`retain_opaque_only` (carrying only `anchor_id`,
`pack_revision_ref`, and `target_build_identity_ref`) or
`retain_full`; widening is always operator-initiated and
auditable.

## Derived explanations never appear as uncited primary authority

This rule is the spine of the contract:

- An `docs_citation_anchor_record` with
  `source_class = derived_explanation` MUST declare
  `derivation = derived` or `derived_with_upstream_anchors` and
  MUST point `upstream_citation_anchor_refs` at least one anchor
  whose `source_class` resolves to an authoritative class
  (`project_docs`, `generated_reference`, `mirrored_official_docs`,
  `curated_knowledge_pack`, `support_runbook`) or whose
  `anchor_kind` is `release_note_anchor`.
- A `symbol_linked_reference_record` with
  `binding_kind = ai_derived_explanation_entry` MUST carry at
  least one `citation_anchor_ref`, and each referenced anchor MUST
  satisfy the rule above.
- Citing only a `vendor_provider_anchor` or
  `external_status_feed_anchor` is not sufficient. The publisher
  MUST refuse the surface with the ADR-0013
  `derived_explanation_uncited` denial reason.
- Silent fallback to a generic / prose-only answer is forbidden.
  The refused surface renders a typed disclosure plus a repair
  hook.

## Offline / exported reconstructability

A reviewer reconstructing what a user saw in an offline or exported
context has:

- the `anchor_id` (stable across exports),
- the `pack_id` + `pack_revision_ref` (resolves against the pack
  manifest at
  [`/schemas/docs/docs_pack_manifest.schema.json`](../../schemas/docs/docs_pack_manifest.schema.json)),
- the `target_ref` (typed ref into the pack's target),
- the `target_build_identity_ref` (the D-0011 build identity the
  anchor was minted under),
- the `locale` (lets the reviewer pick the translation row the user
  saw),
- the `freshness_class_at_mint` and
  `version_match_state_at_mint` (let the reviewer recompute the
  chip).

These five fields are sufficient to reconstruct the row a user saw,
even when the canonical owner is unreachable at review time. That
is the core acceptance-criterion of this contract: reviewers can
reconstruct what was cited, from which source class and version,
in offline or exported contexts.

## Linkage to neighbouring contracts

- **ADR 0013 truth-source vocabulary.**
  [`schemas/docs/help_status_badge.schema.json`](../../schemas/docs/help_status_badge.schema.json)
  reserves the minimal `citation_anchor_record` shape for badge-row
  inlining. `docs_citation_anchor_record` in this contract is a
  strict superset; `help_status_badge_record.citation_anchor_refs`
  resolve against the richer record.
- **Docs-pack manifest contract.**
  [`docs/docs/docs_pack_manifest_contract.md`](../docs/docs_pack_manifest_contract.md)
  froze `citation_posture`,
  `required_citation_anchor_kinds`, and `backlink_posture` on the
  pack manifest. Anchors in this contract pin into
  `pack_revision_ref` on a manifest whose `citation_posture` is
  `citation_required` / `citation_recommended` / `citation_not_required`.
- **Docs-browser packet (symbol-link validation corpus).**
  [`fixtures/docs/symbol_link_validation_manifest.yaml`](../../fixtures/docs/symbol_link_validation_manifest.yaml)
  froze the `symbol_link_resolution_class` /
  `project_vs_vendor_truth_cue` /
  `derived_explanation_reuse_state` vocabularies. The
  symbol-linked reference record re-exports those vocabularies
  without modification.
- **ADR 0011 capability lifecycle.** `freshness_class`,
  `client_scope`, `repair_hook_ref`, and `redaction_class` are
  re-exported from
  [`schemas/governance/capability_lifecycle.schema.json`](../../schemas/governance/capability_lifecycle.schema.json)
  without modification.
- **ADR 0010 browser handoff.** The contract does not mint browser
  handoff packets; surfaces that render a citation row and need to
  hand off to a browser quote the ADR-0010 envelope from the subset
  frozen in ADR 0013.
- **ADR 0008 settings resolver.** Admin policy may narrow which
  reuse surfaces may project an anchor, raise the freshness floor
  on cited content, force a step-up authenticator on a browser
  handoff from a citation, or widen the export posture. Policy MAY
  NOT silently widen beyond the frozen rules.
- **D-0011 exact-build identity.** `target_build_identity_ref`
  names the build identity frozen by `D-0011`; the
  `version_match_state_at_mint` axis is computed against that
  identity when the anchor is minted.

## Schema of record

Rust types in the eventual docs-integrity crate are the schema of
record. The JSON Schema exports at
[`/schemas/docs/citation_anchor.schema.json`](../../schemas/docs/citation_anchor.schema.json)
and
[`/schemas/docs/symbol_linked_reference.schema.json`](../../schemas/docs/symbol_linked_reference.schema.json)
are the cross-tool boundary every non-owning surface reads. Adding
a new anchor kind, target-ref kind, reuse-surface class, derivation
class, binding kind, subject kind, or export-posture value is
additive-minor and bumps
`docs_citation_contract_schema_version`; repurposing an existing
value is breaking and requires a new decision row. Both schemas
share the version integer and MUST evolve lock-step.

There is no external IDL or code-generator toolchain at this
milestone; this mirrors ADR 0004 through ADR 0014.

## Source anchors

- [`docs/adr/0013-docs-help-service-health-truth.md`](../adr/0013-docs-help-service-health-truth.md)
  — source-class vocabulary, `help_status_badge_record` shape,
  reserved `citation_anchor_record`, and the
  "follow-up: citation / symbol-reference packet lane" line this
  contract closes against.
- [`docs/docs/docs_pack_manifest_contract.md`](../docs/docs_pack_manifest_contract.md)
  — `pack_revision_ref` target, `citation_posture` /
  `required_citation_anchor_kinds` / `backlink_posture` fields,
  and the "what a publishable pack looks like" rules anchors pin
  into.
- [`docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md)
  — `freshness_class`, `client_scope`, `repair_hook_ref`,
  `redaction_class` vocabularies re-exported here.
- [`docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md)
  — `browser_handoff_packet` envelope the rendering surface quotes
  when a citation hands off.
- [`fixtures/docs/symbol_link_validation_manifest.yaml`](../../fixtures/docs/symbol_link_validation_manifest.yaml)
  — `symbol_link_resolution_class`,
  `project_vs_vendor_truth_cue`, and
  `derived_explanation_reuse_state` vocabularies re-exported on
  the symbol-linked reference record.
- [`.t2/docs/Aureline_Technical_Architecture_Document.md`](../../.t2/docs/Aureline_Technical_Architecture_Document.md),
  [`.t2/docs/Aureline_Technical_Design_Document.md`](../../.t2/docs/Aureline_Technical_Design_Document.md),
  [`.t2/docs/Aureline_PRD.md`](../../.t2/docs/Aureline_PRD.md),
  [`.t2/docs/Aureline_UI_UX_Spec_Document.md`](../../.t2/docs/Aureline_UI_UX_Spec_Document.md)
  — docs-pack governance, AI explanation citation rules,
  onboarding / guided-tour authority, hosted-review evidence
  envelopes, repair-packet evidence envelopes, and portability /
  offboarding export expectations.
