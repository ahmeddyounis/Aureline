# Assist-to-help bridge provenance contract

This document freezes the assist-to-help bridge that connects hover,
peek, completion details, diagnostic help, inline assist, glossary
cards, onboarding steps, AI explanations, docs panes, docs browser
rows, provider-native docs, and support exports.

The machine-readable boundaries are:

- [`/schemas/docs/assist_reference.schema.json`](../../schemas/docs/assist_reference.schema.json)
  — the `assist_reference_record` shape minted before a surface opens
  help.
- [`/schemas/docs/assist_help_handoff.schema.json`](../../schemas/docs/assist_help_handoff.schema.json)
  — the `assist_help_handoff_record` shape emitted when the user or
  system moves from an assist surface into docs, help, browser, or
  support/export context.

Worked cases live under
[`/fixtures/docs/assist_reference_cases/`](../../fixtures/docs/assist_reference_cases/).

The eventual docs-integrity crate's Rust types are the schema of
record. This contract composes with, and does not replace,
[`/docs/docs_integrity/citation_and_reference_contract.md`](./citation_and_reference_contract.md),
[`/docs/docs/docs_pack_manifest_contract.md`](../docs/docs_pack_manifest_contract.md),
[`/docs/docs/help_about_service_health_routes.md`](../docs/help_about_service_health_routes.md),
[`/docs/navigation/navigation_and_saved_query_contract.md`](../navigation/navigation_and_saved_query_contract.md),
and
[`/docs/language/provider_graph_and_arbitration_contract.md`](../language/provider_graph_and_arbitration_contract.md).
If those contracts or the source product docs disagree with this file,
the upstream contract wins and this file plus the schemas update in the
same change.

## Why Freeze This Now

Hover, peek, docs, and AI assist are adjacent enough that it is easy to
lose provenance during a transition:

- A symbol hover can open a docs pane that remembers only a page title,
  losing the exact symbol, file, and mapping quality that caused the
  lookup.
- A peek panel can promote itself to a full docs tab and accidentally
  create a navigation entry without a return target.
- An AI explanation can paraphrase local docs, mirrored docs, and a
  provider page, then appear as primary authority because the citation
  chain was flattened into prose.
- A support export can capture "opened help" without enough identity to
  reconstruct whether the answer came from a local docs pack, a stale
  mirror, provider-native docs, or a live browser handoff.
- Onboarding and glossary cards can quote a docs row but drop locale,
  freshness, or the original reason the lookup happened.

The bridge closes that gap by preserving the initiating object,
selected source, citation chain, version/freshness/locale posture, and
return path before any receiving surface renders a help row.

## Scope

Frozen at this revision:

- one `assist_reference_record` shape that captures the initiating
  surface, action, lookup intent, subject identity, mapping quality,
  source-selection class, source class, authority posture, stale or
  fallback label, citation-retention posture, pack/revision refs,
  citation refs, symbol-linked-reference refs, locale, build identity,
  freshness, version match, browser/provider handoff refs, policy
  context, redaction posture, repair hook, denial reason, and audit
  events;
- one `assist_help_handoff_record` shape that captures the transition
  source/destination, handoff reason, subject snapshot, source snapshot,
  return path, explanation-source posture, retained citations, consumer
  classes, support-export visibility, admission/denial state, browser
  handoff refs, policy context, redaction posture, and audit events;
- closed vocabularies for hover / peek / docs / assist surfaces,
  transition kinds, lookup intents, mapping quality, source reuse,
  stale/fallback labels, return targets, and admission states;
- reuse rules for project docs, generated reference, offline docs packs,
  mirrored docs, live external docs, provider-native docs, support
  runbooks, and derived explanations.

Out of scope:

- a full docs viewer, hover renderer, or browser integration;
- authoring docs/help content;
- AI prompt construction beyond retaining citations and provenance;
- provider authentication flows beyond quoting the ADR-0010 handoff
  packet.

## Core Invariant

Every help transition from an assist surface MUST preserve three things:

1. **What caused the lookup.** The originating surface, action, lookup
   intent, symbol/file/docs/diagnostic identity, and mapping quality.
2. **What backed the answer.** The source class, source-selection class,
   pack/revision refs, citation anchors, symbol-linked-reference refs,
   locale, freshness, version match, and whether the answer is primary,
   derived, inspect-only, stale, or fallback-labeled.
3. **How to return or review it.** The return target, focus-restore
   policy, handoff reason, browser/provider handoff packet if any,
   support-export visibility, redaction posture, and denial/admission
   state.

Visual chips may collapse these fields, but the packet shape may not.

## `assist_reference_record`

An `assist_reference_record` is minted before a surface opens help. It
answers: "what object and source caused this docs/help lookup?"

Required identity fields:

- `assist_reference_id`
- `initiating_surface_class`
- `assist_action_class`
- `lookup_intent_class`
- `subject.subject_kind`
- `subject.subject_ref`
- `subject.display_label`

Required source fields:

- `mapping_quality_class`
- `source_selection_class`
- `source_class`
- `authority_posture_class`
- `stale_or_fallback_label_class`
- `citation_retention_class`
- `pack_id`
- `pack_revision_ref`
- `citation_anchor_refs`
- `symbol_linked_reference_refs`
- `upstream_citation_anchor_refs`
- `locale`
- `target_build_identity_ref`
- `freshness_class_at_mint`
- `version_match_state_at_mint`

The record carries only opaque ids and short privacy-safe labels. Raw
source bytes, docs bodies, provider payloads, prompt text, completion
text, and URLs are forbidden.

## `assist_help_handoff_record`

An `assist_help_handoff_record` is emitted when help opens or when the
transition is refused. It answers: "where did the user go, why, and how
can the origin be restored or reconstructed?"

The handoff packet MUST:

- point at exactly one `assist_reference_id_ref`;
- carry `source_surface_class`, `destination_surface_class`,
  `transition_class`, and `handoff_reason_class`;
- copy a `subject_snapshot` and `source_snapshot` so exports remain
  reconstructable even if the originating reference is unavailable;
- carry a `return_path` for editor, hover, peek, docs-scroll,
  AI-thread, onboarding, glossary, or export-only origins;
- retain `retained_citation_anchor_refs` and, when derived,
  `retained_upstream_citation_anchor_refs`;
- record whether the handoff was `admitted`, admitted with a stale or
  fallback label, or denied with a typed reason.

Opening a system browser or provider-native view requires
`browser_handoff_reason` and `browser_handoff_packet_ref`. Raw URLs stay
inside the ADR-0010 browser-handoff packet owner and never enter this
schema.

## Source Reuse Rules

### Project Docs and Generated Reference

Project docs and generated reference are primary authoritative sources
when their pack/revision refs match the running build and the citation
anchor resolves. For repo-specific symbols, project docs outrank vendor
or provider docs unless policy explicitly narrows the claim; the bridge
records that decision through `source_selection_class`,
`source_class`, `authority_posture_class`, and citation refs.

### Offline Docs Packs

Offline docs packs may satisfy help lookups only when the pack is
signed, within its declared freshness or offline window, and carries
citation anchors for the requested subject. The record MUST use
`source_selection_class = offline_docs_pack` and disclose cached or
stale posture with `stale_or_fallback_label_class` when the pack is not
live-current.

Offline content is still reconstructable because the bridge keeps
`pack_id`, `pack_revision_ref`, `target_build_identity_ref`, `locale`,
`freshness_class_at_mint`, and `version_match_state_at_mint`.

### Mirrored Docs

Mirrored docs are admissible when the mirror chain and version window are
valid. A stale mirror may still be shown as a fallback if the user would
otherwise lose help, but it MUST render with `stale_mirror_label` and
MUST NOT present itself as live upstream truth.

### Live External Docs

Live external docs are outside durable local truth. They require a
browser handoff packet, a visible boundary label, and
`authority_posture_class` that does not imply local primary authority.
They may supplement AI explanations only when at least one authoritative
project/generated/mirrored/curated/runbook anchor is also retained.

### Provider-Native Docs

Provider-native docs are inspect-only unless a later provider contract
explicitly grants stronger authority. The bridge records
`provider_native_docs_ref`, `browser_handoff_reason`, and
`browser_handoff_packet_ref`; provider pages cannot become primary AI
evidence by themselves.

### Support Runbooks

Support runbooks are authoritative only for support, doctor, recovery,
and hosted-review contexts. When support runbook help is exported, the
handoff packet carries `support_export_visibility_class` and redaction
posture so reviewers know whether they received the full packet,
metadata only, opaque ids, or a redacted placeholder.

### Derived Explanations

Derived explanations are never primary authority on their own. Any
assist reference or handoff whose `source_class = derived_explanation`
or `explanation_source_posture_class = derived_cited_summary` MUST
retain upstream citation anchors. If it cannot, the handoff is denied
with a missing-citation / uncited-derived reason rather than rendering
uncited prose.

## Return Context Rules

Hover and peek are transient, but their help transitions are not
allowed to lose context.

- Hover-to-docs and hover-to-peek transitions preserve an editor
  selection or hover origin in `return_path`.
- Peek-to-docs transitions preserve the peek origin and do not mint an
  ordinary history entry until the user promotes the peek.
- AI-to-docs transitions preserve the AI thread turn and the cited
  context.
- Browser handoffs preserve only an in-product return note and require
  `manual_return_required_external_browser`; they cannot promise a
  browser tab will return focus.
- Support-export captures may set `no_return_available_export_only`,
  but they still carry the subject and source snapshots.

## Rejection Rules

A bridge publisher MUST deny the transition instead of rendering a
misleading help row when:

- subject identity is missing;
- symbol/file/docs mapping is unresolved and no repair hook exists;
- a citation-required source has no citation anchor;
- a derived explanation has no upstream authoritative anchor;
- provider-native or live external docs are being reused as primary
  authority;
- a browser/provider transition lacks a handoff packet;
- policy or redaction posture forbids the surface.

Denials use the closed denial vocabularies in the schemas. Generic
"help unavailable" copy without one of those reasons is
non-conforming.

## Acceptance Mapping

This contract satisfies the docs-integrity acceptance bar as follows:

- A reviewer can reconstruct the symbol, file, span, docs anchor,
  diagnostic, glossary term, onboarding step, or runbook step that
  caused a lookup from `subject` / `subject_snapshot`.
- A reviewer can distinguish local packs, mirrored docs, live external
  docs, provider-native docs, support runbooks, generated reference, and
  derived explanations from `source_selection_class` and `source_class`.
- A reviewer can verify source authority from citation refs,
  symbol-linked-reference refs, pack/revision refs, freshness, version
  match, locale, and build identity.
- Assist-to-help transitions preserve return context through
  `return_path`.
- Derived explanations retain upstream anchors and cannot be flattened
  into uncited primary authority.
- Docs browser, onboarding, glossary, provider-integrated help, AI
  explanations, and support exports can extend one provenance model.

## Schema of Record

Rust types in the eventual docs-integrity crate are the schema of
record. The JSON Schema exports at
[`/schemas/docs/assist_reference.schema.json`](../../schemas/docs/assist_reference.schema.json)
and
[`/schemas/docs/assist_help_handoff.schema.json`](../../schemas/docs/assist_help_handoff.schema.json)
are the cross-tool boundary every non-owning surface reads. Both
schemas share `assist_reference_schema_version` and MUST evolve
lock-step.

## Source Anchors

- [`.t2/docs/Aureline_Technical_Design_Document.md`](../../.t2/docs/Aureline_Technical_Design_Document.md)
  §7.1.13 — symbol-linked docs / hover bridge must preserve symbol
  identity, version-match state, source class, mapping quality, why the
  lookup happened, and what anchor caused it.
- [`.t2/docs/Aureline_Technical_Design_Document.md`](../../.t2/docs/Aureline_Technical_Design_Document.md)
  §7.3.6 — docs results and symbol-linked reference cards show source
  class, version match, freshness/cache state, locale, browser handoff,
  and citation retention.
- [`.t2/docs/Aureline_Technical_Architecture_Document.md`](../../.t2/docs/Aureline_Technical_Architecture_Document.md)
  §14.7 — documentation, codebase explainers, glossary extraction, and
  learning surfaces consume shared graph identity, provenance,
  freshness, mirrorability, and citation rules.
- [`.t2/docs/Aureline_UX_Design_System_Style_Guide.md`](../../.t2/docs/Aureline_UX_Design_System_Style_Guide.md)
  §16.66 and §18.11 — docs result rows, symbol-linked reference cards,
  hovercards, and peek panels expose source, version, cache/freshness,
  symbol identity, return behavior, and keyboard-equivalent flows.
- [`/docs/docs_integrity/citation_and_reference_contract.md`](./citation_and_reference_contract.md)
  — citation-anchor and symbol-linked-reference record models reused by
  this bridge.
- [`/docs/docs/docs_pack_manifest_contract.md`](../docs/docs_pack_manifest_contract.md)
  — docs pack source, freshness, version, mirror, citation, and
  publishability rules.
- [`/docs/docs/help_about_service_health_routes.md`](../docs/help_about_service_health_routes.md)
  — destination descriptor and browser-handoff reason subset.
- [`/docs/navigation/navigation_and_saved_query_contract.md`](../navigation/navigation_and_saved_query_contract.md)
  — return-path, peek, history, and docs-anchor target semantics.
