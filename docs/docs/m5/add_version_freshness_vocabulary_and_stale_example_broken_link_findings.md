# Version/Freshness Vocabulary And Stale-Example/Broken-Link Findings

This document is the contract for one controlled version-match/freshness
vocabulary plus the stale-example and broken-link findings that ride on top of
it. Earlier rows each minted their own version and freshness chips; this lane
freezes a single vocabulary so docs/search result rows, symbol-linked reference
cards, docs pages, AI citation chips, onboarding/glossary surfaces, and support
exports project one truth rather than re-deriving it.

- Record kind: `docs_version_freshness_findings_packet`
- Schema: [`schemas/docs/add-version-freshness-vocabulary-and-stale-example-broken-link-findings.schema.json`](../../../schemas/docs/add-version-freshness-vocabulary-and-stale-example-broken-link-findings.schema.json)
- Canonical support export: [`artifacts/docs/m5/add_version_freshness_vocabulary_and_stale_example_broken_link_findings/support_export.json`](../../../artifacts/docs/m5/add_version_freshness_vocabulary_and_stale_example_broken_link_findings/support_export.json)
- Summary artifact: [`artifacts/docs/m5/add_version_freshness_vocabulary_and_stale_example_broken_link_findings.md`](../../../artifacts/docs/m5/add_version_freshness_vocabulary_and_stale_example_broken_link_findings.md)
- Fixtures: [`fixtures/docs/m5/add_version_freshness_vocabulary_and_stale_example_broken_link_findings/`](../../../fixtures/docs/m5/add_version_freshness_vocabulary_and_stale_example_broken_link_findings/)
- Producer: `aureline_docs::current_stable_docs_version_freshness_export`

## The controlled vocabulary

`DocsVersionFreshnessState` has exactly eight states, each its own badge:

| State | Meaning |
| --- | --- |
| `exact` | Source exactly matches the active code/package version; current authoritative guidance. |
| `nearby` | A near-version match (compatible drift); correctness could change, so it must not read as exact-current. |
| `project_specific` | Workspace/project documentation — not vendor docs; current to the project but scoped to it. |
| `mirrored` | Served from a pinned, signed mirror of upstream docs. |
| `cached` | Served from a local cache and not verified live; freshness is unverified. |
| `stale` | Known stale; must not claim current authority. |
| `policy_blocked` | Blocked by policy; not rendered inline and a reason is named. |
| `browser_handoff_required` | Requires handing off to a browser/provider console; not answered inline and a reason is named. |

The states stay **distinct**: `browser_handoff_required`, `cached`, `mirrored`,
and `project_specific` must never collapse into one generic info badge, because
the distinction is part of the product truth.

## Confidence treatment

Every state maps to a `DocsVersionFreshnessConfidence` treatment, kept as a
separate axis from the badge so a surface renders both the exact state token and
a confidence tier. Only `exact` maps to `current_exact`; `nearby`, `cached`, and
every other state map to lower, *distinct* tiers (`qualified_nearby`,
`cached_unverified`, …). A card whose declared confidence disagrees with its
state's confidence class blocks promotion (`card_confidence_collapsed`), so
cached or nearby-version documentation can never render with the same confidence
as exact current documentation.

## Version disclosure

The version-mismatch states (`nearby`, `mirrored`, `cached`, `stale`) require a
`DocsVersionFreshnessDisclosure` that names both the active code/package version
and the viewed docs version, with a flag for whether the difference changes API
or workflow truth. A version-mismatch card that hides either version blocks
promotion (`version_disclosure_missing`). The not-rendered-inline states
(`policy_blocked`, `browser_handoff_required`) instead require a `state_reason`.

## Stale-example and broken-link findings

A `DocsVersionFreshnessFinding` is an actionable review item with stable object
identity. It compares a doc subject — a code block, command, API reference,
config path, or link — against the current graph/pack metadata:

| Field | Meaning |
| --- | --- |
| `finding_id` | Stable id carried verbatim across surfaces and exports. |
| `finding_class` | `stale_example`, `broken_link`, `nearby_version_example`, `removed_api_reference`, `changed_config_path`, or `command_syntax_changed`. |
| `subject_kind` | `code_block`, `command`, `api_reference`, `config_path`, or `link`. A `broken_link` finding must be about a `link`; every other class about a non-link subject. |
| `card_id_ref` | The card the finding annotates; a finding referencing an unknown card is an orphan. |
| `observed_ref` / `compared_against_ref` / `current_ref` | The documented value, the current graph/pack metadata it was compared with, and the current correct value. |
| `actions` | The preserved suppress / compare / open-current-source actions. |

Findings carry their own severity (`advisory`, `narrowing`, `blocking`) and feed
promotion, so a blocking finding gates the stable claim while an advisory one
leaves the packet stable. A suppressed finding (with a disclosed reason) drops
back to advisory. Every finding must keep its compare and open-current-source
actions; dropping either blocks promotion (`finding_actions_missing`).

## Consumer projections

A `DocsVersionFreshnessConsumerProjection` records, per surface, that the version
state, confidence treatment, version disclosure, and findings are reused without
drift. The packet requires a projection for every surface:

- `result_row`
- `symbol_reference_card`
- `docs_page`
- `ai_citation_chip`
- `onboarding_glossary`
- `support_export`

Each projection asserts it preserves the state badge, keeps the states distinct,
preserves the confidence treatment, shows the version disclosure, preserves the
findings and their actions, and excludes raw private material.

## Invariants

`DocsVersionFreshnessPacket::materialize` derives validation findings and a
promotion state (`stable`, `narrowed_below_stable`, `blocks_stable`). The
validator blocks promotion when any invariant fails:

- A card's confidence collapses into a treatment its state does not allow.
- A version-mismatch card hides the active or viewed version.
- A not-rendered-inline state drops its reason.
- The cards do not exercise every state in the controlled vocabulary.
- A finding is incomplete, has a subject inconsistent with its class, is an
  orphan, or drops its compare/open-current-source actions.
- A consumer projection collapses the distinct state badges, drops a
  preservation flag, or references the wrong packet id.
- Raw document bodies, raw URLs, raw provider payloads, or credentials cross the
  boundary.

## Consumers

The docs/search result rows, symbol-linked reference cards, docs pages, AI
citation chips, onboarding/glossary surfaces, and support export consume the
checked-in packet directly. The support export preserves the exact packet
identity without exporting raw private material or ambient authority.
