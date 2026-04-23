# Docs-browser source/version/cache honesty, symbol-linked reference, offline mirror drill, and privacy verification seed

This packet freezes one shared verification story for the docs
browser: which fields a docs-result row MUST project, how source
class / version-match / freshness / cache state are rendered without
collapsing into a single ambiguous chip, how symbol-linked references
resolve (exact symbol match, nearby-version fallback, package-level
guide fallback), when project docs outrank vendor docs, how derived
explanations reuse or refuse a pack's citation surface, how browser
handoff is explained on every out-of-product link, and what the
docs-pack offline / mirror drill looks like when the canonical owner
is unreachable. It exists so later docs-browser, AI-explanation
overlay, onboarding, support-export, and citation surfaces reuse one
inspectable object model instead of inventing per-surface freshness
labels, silent cache-vs-live confusion, empty-state copy, or
uncontrolled upload of surrounding code to a remote docs provider.

If this packet, the
[`symbol_link_validation_manifest.yaml`](../../fixtures/docs/symbol_link_validation_manifest.yaml)
corpus, the
[`offline_mirror_drill_manifest.yaml`](../../fixtures/docs/offline_mirror_drill_manifest.yaml)
drill roster, the
[`privacy_context_sharing_review.md`](../../artifacts/docs/privacy_context_sharing_review.md)
review, and the frozen docs / badge / manifest / destination-descriptor
schemas disagree, the frozen ADR-0013 badge vocabulary, the frozen
docs-pack manifest contract, the ADR-0010 browser-handoff packet,
and the ADR-0015 embedded-surface boundary win for tooling and this
packet must update in the same change.

Companion artifacts:

- [`/fixtures/docs/symbol_link_validation_manifest.yaml`](../../fixtures/docs/symbol_link_validation_manifest.yaml)
  — machine-readable symbol-linked reference validation roster
  covering exact symbol match, nearby-version fallback, package-level
  guide fallback, project-doc outranking vendor-doc, cached / mirrored
  / live / stale states, citation-anchor retention, and offline
  docs-pack recovery.
- [`/fixtures/docs/offline_mirror_drill_manifest.yaml`](../../fixtures/docs/offline_mirror_drill_manifest.yaml)
  — machine-readable offline / mirror drill roster covering signed
  offline bundle import, cached degradation to project docs,
  mirror-stale disclosure, mirror-continuity-broken denial, and
  unreachable-owner empty-state avoidance.
- [`/artifacts/docs/privacy_context_sharing_review.md`](../../artifacts/docs/privacy_context_sharing_review.md)
  — reviewer-facing privacy review naming what identity, path, and
  symbol data baseline docs-search flows MAY send to remote docs
  providers; what remains strictly local; and what requires higher-
  trust context sharing elsewhere with an explicit approval surface.
- [`/docs/adr/0013-docs-help-service-health-truth.md`](../adr/0013-docs-help-service-health-truth.md)
  — canonical source-class / version-match / freshness /
  service-contract / degraded-state / citation-anchor / browser-
  handoff-reason / badge-record vocabulary this packet reuses.
- [`/docs/docs/docs_pack_manifest_contract.md`](../docs/docs_pack_manifest_contract.md)
  — canonical docs-pack manifest fields, publishable states,
  blocking reasons, and offline / mirror lineage shape this packet
  projects onto result rows.
- [`/docs/docs/help_about_service_health_routes.md`](../docs/help_about_service_health_routes.md)
  — canonical destination-descriptor route rules and external-open
  policies the docs browser quotes on every out-of-product link.
- [`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md)
  — canonical browser-handoff packet vocabulary the docs browser
  quotes (narrowed by ADR-0013) on every handoff row.
- [`/docs/adr/0015-embedded-surface-boundary-and-auth-handoff.md`](../adr/0015-embedded-surface-boundary-and-auth-handoff.md)
  — canonical embedded-surface boundary card requirements the docs
  browser honours when a row renders inside an embedded pane.
- [`/schemas/docs/help_status_badge.schema.json`](../../schemas/docs/help_status_badge.schema.json)
  — boundary schema carrying `help_status_badge_record`,
  `citation_anchor_record`, and
  `docs_help_service_health_audit_event_record` shapes every docs-
  browser result row projects.
- [`/schemas/docs/docs_pack_manifest.schema.json`](../../schemas/docs/docs_pack_manifest.schema.json)
  — boundary schema the docs browser reads to resolve the source
  pack a result row comes from.
- [`/schemas/docs/destination_descriptor.schema.json`](../../schemas/docs/destination_descriptor.schema.json)
  — boundary schema the docs browser quotes on every out-of-product
  route row without minting per-surface handoff copy.
- [`/artifacts/docs/help_badge_vocabulary.yaml`](help_badge_vocabulary.yaml)
  — worked badge-vocabulary examples the docs browser's result rows
  MUST resolve against.
- [`/fixtures/docs/docs_pack_examples/`](../../fixtures/docs/docs_pack_examples/)
  — existing schema-conforming docs-pack manifest fixtures this
  packet composes over (project-fresh, mirrored-offline, partially-
  stale, mixed-locale, newer-than-client, non-publishable) instead
  of re-minting.

Normative sources projected here:

- `.t2/docs/Aureline_PRD.md`
  — requirement register, evidence-governance posture, and explicit
  honesty rules for documentation freshness, source, and cache
  state; forbids baseline docs flows silently uploading surrounding
  code to remote providers.
- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  — docs browser subscribes to the canonical owners named in
  ADR-0013; docs panes never re-mint their own freshness; mirror
  and offline lineage preserved on every render.
- `.t2/docs/Aureline_Technical_Design_Document.md`
  — docs result row shape, cache-state disclosure, and the rule
  that project-authoritative docs outrank vendor docs on in-scope
  topics with a typed disclosure when policy inverts the default.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md`
  — docs-browser result-row chip discipline, stale-example
  disclosure rules, citation-anchor rendering, and browser-handoff
  disclosure obligations.
- `.t2/docs/Aureline_Milestones_Document.md`
  — docs-browser, symbol-linked reference, offline-drill, and
  privacy claims remain inspectable packets during the foundations
  phase rather than live product surfaces.

## Shared header

```yaml
schema_version: 1
header_kind: evidence_packet_header
packet_family: verification_packet
packet_id: verification.docs_browser.source_version_cache_honesty_seed
evidence_id: evidence.verification.docs_browser.packet
title: Docs-browser source/version/cache honesty, symbol-linked reference, offline mirror drill, and privacy verification seed
ownership:
  owner_dri: "@ahmeddyounis"
  evidence_owner: "@ahmeddyounis"
  backup_owner: null
  backup_waiver: single-maintainer-backup
coverage:
  requirement_ids:
    - GOV-EVID-901
    - GOV-TRUTH-901
    - GOV-CORPUS-901
    - GOV-DATA-002
    - ARCH-PACK-901
  claim_row_refs:
    - packet_row:docs_browser.result_row_contract
    - packet_row:docs_browser.source_version_cache_honesty
    - packet_row:docs_browser.symbol_linked_reference_resolution
    - packet_row:docs_browser.project_vs_vendor_truth
    - packet_row:docs_browser.derived_explanation_reuse
    - packet_row:docs_browser.browser_handoff_explanation
    - packet_row:docs_browser.offline_mirror_drill
    - packet_row:docs_browser.privacy_context_sharing
    - packet_row:docs_browser.seed_corpus
  covered_lanes:
    - release_evidence
    - support_export
    - governance_packets
    - docs_public_truth
result_status: seed_only
visibility_class: internal
freshness:
  captured_at: 2026-04-23T00:00:00Z
  stale_after: P30D
  freshness_class: warm_cached
  source_revision: docs_browser_seed@1
  trigger_revision: docs_browser_packet@1
environment:
  channel_context: not_applicable
  deployment_context:
    - not_applicable
  environment_summary: >
    Seed packet over the frozen ADR-0013 docs / help / service-health
    truth-source vocabulary, the frozen docs-pack manifest contract,
    the frozen ADR-0010 browser-handoff packet, the frozen ADR-0015
    embedded-surface boundary, and the already-seeded destination
    descriptor, help-badge vocabulary, and docs-pack manifest
    fixtures. No docs-browser webview, AI-explanation overlay,
    onboarding surface, or live remote docs-provider connection is
    wired to this packet yet. Claims are structural: every row in
    the symbol-link validation manifest, every drill in the offline
    / mirror manifest, and every rule in the privacy review reuses
    existing frozen tokens rather than minting new per-surface
    language.
artifact_links:
  supporting_evidence_ids:
    - evidence.verification.docs_browser.symbol_link_validation
    - evidence.verification.docs_browser.offline_mirror_drill
    - evidence.docs.privacy_context_sharing_review
    - evidence.docs.help_badge_vocabulary
    - evidence.docs.destination_descriptor_seed
    - evidence.docs.docs_pack_examples
  exact_build_identity_refs: []
  fixture_refs:
    - fixtures/docs/symbol_link_validation_manifest.yaml
    - fixtures/docs/offline_mirror_drill_manifest.yaml
    - fixtures/docs/docs_pack_examples/project_docs_fresh.json
    - fixtures/docs/docs_pack_examples/mirrored_official_docs_offline.json
    - fixtures/docs/docs_pack_examples/curated_knowledge_pack_partially_stale.json
    - fixtures/docs/docs_pack_examples/project_docs_mixed_locale_coverage.json
    - fixtures/docs/docs_pack_examples/support_runbook_newer_than_client.json
    - fixtures/docs/docs_pack_examples/generated_reference_non_publishable.json
  archetype_refs: []
  source_anchor_refs:
    - docs/adr/0013-docs-help-service-health-truth.md
    - docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md
    - docs/adr/0015-embedded-surface-boundary-and-auth-handoff.md
    - docs/docs/docs_pack_manifest_contract.md
    - docs/docs/help_about_service_health_routes.md
    - artifacts/docs/help_badge_vocabulary.yaml
    - artifacts/docs/destination_descriptor_seed.yaml
    - schemas/docs/help_status_badge.schema.json
    - schemas/docs/docs_pack_manifest.schema.json
    - schemas/docs/destination_descriptor.schema.json
  waiver_refs: []
  known_limit_refs: []
  migration_packet_refs: []
```

## Summary

This seed packet freezes:

- one reviewer-facing `docs_browser_result_record` object that names
  the result-row identity, the docs-pack manifest it resolves
  against, the `help_status_badge_record` it projects, the cache
  state (live / warm-cached / mirrored / offline-bundled / stale),
  the symbol-link resolution class, the project-vs-vendor truth
  cue, the derived-explanation reuse state, and the browser-handoff
  explanation when the row exposes an out-of-product action;
- one closed `docs_browser_cache_state_class` vocabulary so the
  result row renders mirror / cached / live / stale / offline
  separately from freshness class and from mirror-lineage state;
- one closed `symbol_link_resolution_class` vocabulary with a
  fallback ladder (exact symbol match → nearby-version fallback →
  package-level guide fallback → unresolved-requires-refresh) so a
  symbol-linked reference cannot silently point at a drifted
  definition or collapse into a generic "see docs" link;
- one closed `project_vs_vendor_truth_cue` vocabulary so the
  project-docs-outranks-vendor-docs default, the typed inversion
  under admin policy, and the vendor-provider-overlay case remain
  separately addressable;
- one closed `derived_explanation_reuse_state` vocabulary naming
  whether a result row is reusable by an AI overlay, refused for
  lack of citable anchors, refused for signature / continuity
  breach, or requires higher-trust context sharing;
- one closed `browser_handoff_explanation_class` vocabulary
  projected from the ADR-0013 subset of ADR-0010 reason codes so
  every out-of-product link carries a visible, screenshot-safe,
  export-safe reason;
- one closed `offline_mirror_drill_outcome_class` vocabulary so a
  drill can claim graceful cache degradation, graceful offline-
  bundle recovery, denied rendering on a broken mirror, deferred
  rendering pending re-verify, or fall-through to project docs
  without an empty-state lie; and
- one seed corpus (across the two companion manifests) covering
  every scenario the spec enumerates: exact symbol match, nearby
  version fallback, package-level guide fallback, project-doc
  outranking vendor-doc, cached / mirrored / live / stale states,
  citation-anchor retention, and offline docs-pack recovery.

It does not claim a docs-browser webview, an AI overlay
implementation, an onboarding surface, or a live remote docs
provider is wired up. It claims only that the packet, the symbol-
link validation manifest, the offline / mirror drill manifest, and
the privacy review now exist in one reviewable form and reuse the
frozen docs / badge / manifest / destination-descriptor vocabulary
already landed elsewhere.

## Claim coverage

| Packet row | Requirement id(s) | Status | Visibility | Supporting evidence ids | Notes |
|---|---|---|---|---|---|
| `packet_row:docs_browser.result_row_contract` | `GOV-EVID-901`, `GOV-TRUTH-901` | `seed_only` | `internal` | `evidence.verification.docs_browser.symbol_link_validation` | Freezes one machine-readable `docs_browser_result_record` shape every docs-browser row reuses. |
| `packet_row:docs_browser.source_version_cache_honesty` | `GOV-TRUTH-901`, `ARCH-PACK-901` | `seed_only` | `internal` | `evidence.verification.docs_browser.symbol_link_validation`, `evidence.docs.help_badge_vocabulary` | Source class, version-match state, freshness class, and cache state are separately addressable fields on every result row. |
| `packet_row:docs_browser.symbol_linked_reference_resolution` | `GOV-TRUTH-901`, `GOV-CORPUS-901` | `seed_only` | `internal` | `evidence.verification.docs_browser.symbol_link_validation` | Closed symbol-link resolution class vocabulary with a typed fallback ladder and a denied `unresolved_requires_refresh` case. |
| `packet_row:docs_browser.project_vs_vendor_truth` | `GOV-TRUTH-901`, `ARCH-PACK-901` | `seed_only` | `internal` | `evidence.verification.docs_browser.symbol_link_validation` | Project-authoritative docs outrank vendor / provider docs on in-scope topics; admin policy may narrow the default with an explicit typed disclosure. |
| `packet_row:docs_browser.derived_explanation_reuse` | `GOV-EVID-901`, `GOV-DATA-002` | `seed_only` | `internal` | `evidence.verification.docs_browser.symbol_link_validation`, `evidence.docs.privacy_context_sharing_review` | Derived-explanation reuse state names when an AI overlay MAY consume a docs row and when it MUST deny render for uncited / unverified sources. |
| `packet_row:docs_browser.browser_handoff_explanation` | `GOV-TRUTH-901` | `seed_only` | `internal` | `evidence.docs.destination_descriptor_seed`, `evidence.verification.docs_browser.symbol_link_validation` | Every out-of-product link carries a typed reason drawn from the ADR-0013 subset of ADR-0010 reason codes. |
| `packet_row:docs_browser.offline_mirror_drill` | `GOV-CORPUS-901`, `GOV-TRUTH-901` | `seed_only` | `internal` | `evidence.verification.docs_browser.offline_mirror_drill` | Offline / mirror drill outcomes freeze how the product degrades to cached / project docs rather than an empty-state lie. |
| `packet_row:docs_browser.privacy_context_sharing` | `GOV-DATA-002`, `GOV-TRUTH-901` | `seed_only` | `internal` | `evidence.docs.privacy_context_sharing_review` | Baseline docs search MAY NOT silently upload surrounding code, private docs, or workspace identity to a remote docs provider; higher-trust context sharing lives on a separately-approved surface. |
| `packet_row:docs_browser.seed_corpus` | `GOV-CORPUS-901`, `GOV-EVID-901` | `seed_only` | `internal` | `evidence.verification.docs_browser.symbol_link_validation`, `evidence.verification.docs_browser.offline_mirror_drill` | Stable case-id sets cover the required scenarios named in the spec. |

## What this seed freezes

- One `docs_browser_result_record` shape every docs-browser
  surface, AI-explanation overlay, onboarding step that quotes a
  docs row, and support-export row projects.
- One `docs_browser_cache_state_class` vocabulary so the row renders
  live / warm-cached / mirrored-current / mirrored-stale / offline-
  bundle-pinned / vendored-local-pinned / unknown-pending-reverify
  independently of freshness class.
- One `symbol_link_resolution_class` vocabulary with a typed
  fallback ladder so symbol-linked references cannot silently point
  at a drifted definition.
- One `project_vs_vendor_truth_cue` vocabulary so the project-
  outranks-vendor default and the typed admin-policy inversion
  stay separately addressable.
- One `derived_explanation_reuse_state` vocabulary so AI overlays
  cannot silently reuse docs rows whose citation posture or
  signature is unmet.
- One `browser_handoff_explanation_class` vocabulary (subset of
  ADR-0010 / ADR-0013) for every out-of-product link.
- One `offline_mirror_drill_outcome_class` vocabulary covering
  graceful cache degradation, offline-bundle recovery, denied
  rendering on broken mirrors, deferred rendering pending re-verify,
  and project-docs fall-through.
- One privacy posture that forbids silent upload of surrounding
  code, private docs, workspace identity, path, or symbol data
  from baseline docs-search flows.

## Docs-browser result record

Every row in the machine-readable manifests resolves to one
`docs_browser_result_record` with these required fields. The field
set projects ADR-0013 badge fields and the docs-pack manifest fields
already frozen; this packet does not redefine them.

- `case_id`
- `result_row_id` — opaque, stable id, safe to log.
- `result_kind` — one of `docs_page_result`,
  `generated_reference_result`, `symbol_reference_result`,
  `runbook_step_result`, `curated_pack_result`,
  `vendor_provider_result`, `external_status_feed_result`,
  `derived_explanation_result`.
- `display_title` — short human label; never a raw URL.
- `pack_id` — docs-pack manifest id the row resolves against. Null
  only for `external_status_feed_result` rows.
- `pack_revision_ref` — pack revision the row resolves against.
  Null only for `external_status_feed_result` rows.
- `help_status_badge_record_ref` — opaque id of the
  `help_status_badge_record` this row projects. The badge record
  carries source class, version-match state, freshness class,
  client scopes, degraded-state cause, browser-handoff reason,
  external-status-feed flag, vendor-overrides-project flag,
  citation anchor refs, policy context, and redaction class without
  re-minting them.
- `cache_state_class` — see §Source, version, and cache honesty.
- `symbol_link_resolution_class` — see §Symbol-linked reference
  resolution.
- `symbol_link_resolution_fallback_chain` — ordered list of
  `symbol_link_resolution_class` tokens naming the fallback steps
  taken to resolve the row, when any. Empty when the resolution
  was `exact_symbol_match`.
- `project_vs_vendor_truth_cue` — see §Project-vs-vendor truth.
- `derived_explanation_reuse_state` — see §Derived-explanation
  reuse.
- `browser_handoff_explanation_class` — see §Browser-handoff
  explanation. Null when the row does not expose an out-of-product
  action.
- `destination_descriptor_ref` — opaque id of the
  `destination_descriptor_record` the row quotes. Required (non-
  null) when `browser_handoff_explanation_class` is non-null.
- `citation_anchor_refs` — array of citation-anchor ids the row
  backs itself with. MUST be non-empty when `result_kind` is
  `derived_explanation_result` or when the source pack's
  `citation_posture = citation_required`.
- `locale` — BCP-47 locale the row renders in; MUST be one of the
  pack's `available_locales`.
- `embedded_boundary_card_ref` — opaque id of the host-owned
  embedded-surface boundary card when the row renders inside an
  embedded pane. Null otherwise (native / in-product pane).
- `policy_context` — `policy_epoch`, `trust_state`,
  `execution_context_id` (ADR-0001 / ADR-0008 / ADR-0009).
- `redaction_class` — ADR-0007 / ADR-0011 redaction class.
- `export_inclusion_posture` — `metadata_safe_default`,
  `operator_only_restricted`, or `broadened_capture_opt_in`.
- `minted_at` — monotonic timestamp.

Rule: a docs-browser row that cannot fill
`help_status_badge_record_ref`, `cache_state_class`,
`symbol_link_resolution_class`, `project_vs_vendor_truth_cue`, or
`derived_explanation_reuse_state` MUST deny render with the
ADR-0013 denial `parity_field_missing` or
`source_class_unresolved` rather than fall back to a generic "docs
unavailable" chip. Silent empty-state copy is non-conforming.

## Source, version, and cache honesty

The docs browser surfaces four separately addressable axes on every
result row. Chip collapsing in the UI is a freedom; record
addressability is mandatory.

1. **Source class.** One of the ADR-0013 `source_class` tokens
   (`project_docs`, `generated_reference`, `mirrored_official_docs`,
   `curated_knowledge_pack`, `derived_explanation`,
   `vendor_provider_docs`, `support_runbook`, `external_status_feed`).
2. **Version-match state.** One of the ADR-0013
   `version_match_state` tokens (`exact_build_match`,
   `compatible_minor_drift`, `incompatible_drift_detected`,
   `pre_release_unverified`, `unknown_target_build`), computed at
   render time against the running build's exact-build identity.
3. **Freshness class.** One of the ADR-0011 / ADR-0013
   `freshness_class` tokens (`authoritative_live`, `warm_cached`,
   `degraded_cached`, `stale`, `unverified`).
4. **Cache state.** One of the `docs_browser_cache_state_class`
   tokens frozen below. The cache state is a rendering concern:
   where did the row's bytes come from *right now*, independent of
   declared freshness and independent of mirror continuity.

### `docs_browser_cache_state_class` (frozen)

| Token | Meaning | Typical rendering |
|---|---|---|
| `live_owner_fetch` | Row resolved directly from its canonical owner inside the refresh window; freshness MAY be `authoritative_live`. | No cache chip; freshness chip only. |
| `warm_cache_within_window` | Row resolved from a local cache that was refreshed against the canonical owner inside the refresh window. | "cached" chip with last-refreshed-at. |
| `mirror_snapshot_current` | Row resolved from a mirror whose snapshot is current vs the declared mirror freshness floor. | Mirror chip with snapshot ref; chip MAY NOT read "live". |
| `mirror_snapshot_stale` | Row resolved from a mirror whose snapshot is older than the declared mirror freshness floor; row MUST render the stale-mirror disclosure on the primary surface. | Mirror chip with stale-since timestamp and `freshness_floor_unmet` degraded-state cause. |
| `offline_bundle_pinned` | Row resolved from a signed offline bundle pinned by digest; canonical owner is unreachable from this deployment. | Offline-bundle chip with bundle digest ref and offline-expiration timestamp. |
| `vendored_local_pinned` | Row resolved from a vendored-local copy inside the workspace; ownership is workspace-trusted. | Vendored-local chip with workspace-trust ref. |
| `unknown_pending_reverify` | Cache state cannot be resolved because signature re-verify or cache-index refresh is in flight; row MUST defer render. | Chip with `pending_reverify` disclosure; row renders with `deferred_pending_freshness` disposition. |

Rules (frozen):

1. `cache_state_class = live_owner_fetch` MAY pair only with
   `freshness_class` in `{authoritative_live, warm_cached}`. Any
   lower freshness class on a `live_owner_fetch` row is non-
   conforming.
2. `cache_state_class = mirror_snapshot_stale` MUST set
   `degraded_state_cause = freshness_floor_unmet` or
   `mirror_continuity_broken` on the projected badge record and
   MUST render the stale-since timestamp on the primary surface.
3. `cache_state_class = offline_bundle_pinned` MUST carry the
   bundle digest and the `offline_expiration_at` monotonic
   deadline on the manifest the row resolves against; past the
   deadline the row MUST be re-labelled `stale` and refuse to
   render as authoritative.
4. `cache_state_class = unknown_pending_reverify` MUST set the
   row's disposition to `deferred_pending_freshness` rather than
   fall through to an empty result. A docs-browser row that
   renders an empty result body while the cache state is
   `unknown_pending_reverify` is non-conforming.
5. The docs-browser MAY collapse multiple rows with the same
   `cache_state_class` under a single chip in the UI, but each
   row's record MUST retain its own `cache_state_class` value.

## Symbol-linked reference resolution

Symbol-linked references are the hardest row class in the docs
browser: the anchor comes from source code (a function, a type, a
setting id, a command id) and the docs the user wants may live at
an exact match, a nearby version, a package-level guide, or nowhere
yet. The row MUST name which rung of the fallback ladder it hit so
reviewers can see the drift.

### `symbol_link_resolution_class` (frozen)

| Token | Meaning | Required rendering |
|---|---|---|
| `exact_symbol_match` | The referenced symbol id resolves inside the pack against the running build's exact-build identity. | No fallback chip; symbol ref projects directly. |
| `nearby_version_fallback` | No exact match at the running build; a matching symbol exists in a pack within the declared compat window. Row renders with `version_match_state = compatible_minor_drift` on the badge and a typed fallback chip. | Fallback chip with the matched pack version. |
| `package_level_guide_fallback` | No symbol-level match in any compat-window pack; a package-level guide (module overview, command family overview, settings-group overview) is the best available surface. | Fallback chip labelled "package guide"; surface MUST NOT imply the row covers the exact symbol. |
| `project_docs_outranks_vendor_match` | Both project docs and vendor-provider docs have a candidate match; project docs take precedence by default (see §Project-vs-vendor truth). | Project-authoritative chip; vendor result may render as a secondary row with the `vendor_overrides_project = false` flag. |
| `vendor_overrides_project_disclosed` | Admin policy has inverted the default to render vendor docs in place of in-scope project docs. Row MUST render the typed `vendor_docs_overrides_project` disclosure on the primary surface. | Vendor-override disclosure chip with the policy-pack ref that inverted the default. |
| `unresolved_requires_refresh` | No match at any rung; pack index is stale / freshness floor unmet / cache unreachable. Row denies render and routes to a `refresh_freshness` repair hook. | Denial chip with `unresolved_requires_refresh` cause; row carries a `repair_hook_ref`. |
| `no_claim_yet_support_routed` | No match at any rung and the freshness floor is met; the symbol has no documented coverage yet. Row renders the honest no-claim state and offers a `contact_support` / `request_docs_coverage` hook; MAY NOT fabricate a fallback. | No-claim chip with a typed hook. |

Rules (frozen):

1. A row whose `symbol_link_resolution_class` is
   `nearby_version_fallback` or `package_level_guide_fallback` MUST
   set `symbol_link_resolution_fallback_chain` to the ordered list
   of rungs it traversed, so a reviewer can see exact → nearby →
   package fallbacks explicitly. Silent dropping of the chain is
   non-conforming.
2. A row whose `symbol_link_resolution_class` is
   `vendor_overrides_project_disclosed` MUST project
   `vendor_overrides_project = true` on the badge record and MUST
   cite the policy-pack ref that inverted the default. A vendor-
   override rendered without the typed disclosure is non-conforming
   and projects the ADR-0013
   `vendor_docs_overrode_project_without_disclosure` denial reason.
3. A row whose `symbol_link_resolution_class` is
   `unresolved_requires_refresh` MUST set
   `cache_state_class = unknown_pending_reverify` and dispose as
   `deferred_pending_freshness`. Silent fall-through to an empty
   body is non-conforming.
4. A row whose `symbol_link_resolution_class` is
   `no_claim_yet_support_routed` MUST NOT fabricate a body from a
   derived-explanation source. The honest no-claim chip is the
   correct posture; manufacturing an AI-generated stand-in violates
   the ADR-0013 `derived_explanation_uncited` rule.
5. The resolution class MUST be computed fresh on each row render
   (not cached past the row's freshness window) so a docs-pack
   refresh that adds a symbol-level page promotes future renders
   from `nearby_version_fallback` to `exact_symbol_match`
   automatically.

## Project-vs-vendor truth

Project-authoritative docs (the docs pack the running build was
published against, generated reference derived from the binary,
curated knowledge packs the project allow-lists) outrank vendor /
provider docs on in-scope topics by default. Admin policy MAY
invert the default for a named topic but MAY NOT silently widen it.

### `project_vs_vendor_truth_cue` (frozen)

| Token | Meaning | Required rendering |
|---|---|---|
| `project_authoritative_only` | Only project-authoritative candidates exist for the topic; vendor docs have no applicable row. | Project chip; no vendor disclosure. |
| `project_outranks_vendor_default` | Both project and vendor candidates exist; project takes precedence by default. Vendor row MAY render as a secondary row with `vendor_overrides_project = false`. | Project chip on the primary row; vendor row disclosed as secondary. |
| `vendor_overrides_project_by_policy` | Admin policy has allow-listed vendor docs to override project docs for this topic; the row MUST render the typed inversion disclosure. | Vendor chip with `vendor_overrides_project = true` and the policy-pack ref. |
| `vendor_provider_overlay_inspect_only` | The vendor / provider docs row is an ADR-0010 `inspect_only` overlay (e.g. live vendor portal row); it carries provider freshness and an external-docs-or-runbook browser-handoff explanation. | Provider-overlay chip; row renders a typed browser-handoff disclosure. |
| `no_project_claim_vendor_available` | No project-authoritative row exists for the topic and vendor docs have an applicable row; row renders the vendor candidate with a typed "no project claim yet" disclosure. | Vendor chip with a typed no-project-claim disclosure; row never implies project coverage. |

Rules (frozen):

1. `vendor_overrides_project_by_policy` MUST cite the policy-pack
   narrowing ref on the projected badge record. A row that renders
   vendor docs without the typed inversion disclosure and without
   the policy ref is non-conforming.
2. `vendor_provider_overlay_inspect_only` rows MUST quote the ADR-
   0010 browser-handoff packet with `reason_code =
   external_docs_or_runbook` and MUST NOT cache past the provider's
   declared freshness window.
3. `no_project_claim_vendor_available` rows MUST NOT imply the
   vendor candidate is project-authoritative. The "no project
   claim" disclosure renders on the primary surface; tooltip-only
   disclosure is forbidden.
4. Chip collapsing MAY merge `project_authoritative_only` and
   `project_outranks_vendor_default` into one chip, but the
   underlying record keeps the cue separately addressable so
   parity audits can count vendor-available-but-outranked rows.

## Derived-explanation reuse

A docs-browser row MAY be reused by an AI-explanation overlay only
when the pack's citation posture is met, the source signature is
verified, and the derived explanation cites the row with an
authoritative anchor. The `derived_explanation_reuse_state` names
the reuse posture so an AI overlay cannot silently consume a row
whose citation posture is unmet.

### `derived_explanation_reuse_state` (frozen)

| Token | Meaning | Required AI-overlay behaviour |
|---|---|---|
| `reusable_with_citation_anchor` | Pack citation posture is met; signature is verified; derived explanation MUST cite the anchor returned by the row. | AI overlay MAY render the derived explanation citing this row. |
| `refused_uncited` | Pack's `citation_posture = citation_required` but no citation anchor resolves on this row yet. | AI overlay MUST deny render with the ADR-0013 `derived_explanation_uncited` denial. |
| `refused_signature_unverified` | Pack signature is not `signed_and_verified`. | AI overlay MUST deny render and route to a `contact_support` hook. |
| `refused_mirror_continuity_broken` | Pack's mirror lineage is broken (`predecessor_missing` or `signing_chain_broken`). | AI overlay MUST deny render with the ADR-0013 `mirror_continuity_broken` denial. |
| `refused_vendor_overlay_requires_higher_trust` | Row is a `vendor_provider_docs` overlay; derived reuse requires the higher-trust context-sharing approval (see §Privacy context sharing). | AI overlay MUST deny render and route to the higher-trust approval surface. |
| `refused_external_status_feed` | Row is an `external_status_feed`; external feeds are never sufficient citations for a derived explanation. | AI overlay MUST deny render with `derived_explanation_uncited` even if the row carries an anchor. |

Rules (frozen):

1. `reusable_with_citation_anchor` MUST pair with a non-empty
   `citation_anchor_refs` list on the row and MUST pair with a
   non-null `pack_revision_ref`. A row that declares reusable
   without an anchor is non-conforming.
2. `refused_*` states MUST each pair with a typed repair hook on
   the projected badge record and MUST emit the corresponding
   ADR-0013 audit event (`derived_explanation_uncited_refused`,
   etc.).
3. An AI overlay MAY NOT paper over a `refused_*` state by citing
   an unrelated row. The derived-explanation reuse posture is a
   first-class field, not a tooltip.

## Browser-handoff explanation

Every out-of-product link from the docs browser carries one of the
ADR-0013 browser-handoff reason-subset tokens. Reasons outside the
subset (e.g. `publish_now`, `release_publish`) are non-conforming
when emitted from a docs-browser row.

### `browser_handoff_explanation_class` (frozen subset of ADR-0013)

- `external_docs_or_runbook` — content lives outside the local
  product (mirrored upstream docs portal, vendor portal, external
  runbook). Most common reason on docs-browser rows.
- `provider_consent_flow` — user must complete a provider consent
  flow before continuing.
- `provider_admin_delegation` — admin / operator must delegate a
  permission on the provider before continuing.
- `license_or_portal_acceptance` — license or portal acceptance
  must occur on the destination.
- `admin_only_surface` — destination is admin-only on the
  provider; surface routes to the admin path.
- `step_up_required` — destination requires a step-up
  authenticator.
- `mutation_not_supported_in_product` — the action exists only on
  the provider's web surface.

Rules (frozen):

1. Every docs-browser row that exposes an out-of-product action
   MUST quote one `browser_handoff_explanation_class` token on the
   record and MUST carry a `destination_descriptor_ref` to the
   canonical destination descriptor (see
   `artifacts/docs/destination_descriptor_seed.yaml`). A row
   without both is non-conforming.
2. The docs browser MUST NOT call the system browser directly; raw
   URL launches from a docs-browser row are forbidden. The
   ADR-0010 `browser_handoff_packet` is the single envelope.
3. The `disclosure_summary` (ADR-0010) MUST render on the primary
   surface before the handoff fires; tooltip-only disclosure is
   non-conforming.
4. The label "Open in browser" is reserved for rows that carry a
   `browser_handoff_explanation_class`; the docs browser MUST NOT
   reuse the label for in-product navigation.

## Offline / mirror drill outcomes

The docs browser's honesty contract is most visible when the
canonical owner is unreachable. The drill outcomes name how the
product degrades; silent fall-through to an empty result body is
non-conforming in every case.

### `offline_mirror_drill_outcome_class` (frozen)

| Token | Meaning | Required rendering |
|---|---|---|
| `graceful_cache_within_window` | Owner unreachable; warm cache is still within its refresh window; row renders with `cache_state_class = warm_cache_within_window` and a `warm_cached` freshness chip. | "cached" chip with last-refreshed-at; row remains readable. |
| `graceful_offline_bundle_recovery` | Owner unreachable; signed offline bundle resolves the row with a pinned digest; row renders with `cache_state_class = offline_bundle_pinned`. | Offline-bundle chip with digest ref and offline-expiration. |
| `graceful_project_docs_fallthrough` | Owner for a vendor / mirrored pack unreachable; row falls through to project-authoritative docs with a typed `project_outranks_vendor_default` cue and a `vendor_docs_override_withheld_offline` disclosure. | Project chip with typed offline-override disclosure. |
| `denied_mirror_continuity_broken` | Mirror lineage is broken (`predecessor_missing` / `signing_chain_broken`); row denies render with the ADR-0013 `mirror_continuity_broken` denial and routes to a `refresh_freshness` hook. | Denial chip; no silent downgrade to warm-cached. |
| `denied_signature_unverified` | Pack signature is missing / revoked / unverified; row denies render with the ADR-0013 `signature_unverified` denial. | Denial chip; AI overlay reuse also denied. |
| `deferred_pending_reverify` | Signature re-verify or cache refresh in flight; row defers render with `cache_state_class = unknown_pending_reverify` and disposes as `deferred_pending_freshness`. | Pending-reverify chip; row never shows an empty body. |
| `denied_offline_expiration_past` | Offline-bundle deadline past; row denies render as `stale` and routes to a `refresh_freshness` / `upgrade_release_channel` hook. | Denial chip with stale-since timestamp. |

Rules (frozen):

1. An offline / mirror drill that emits no outcome token is non-
   conforming; empty-state copy without a typed outcome is
   forbidden.
2. `graceful_project_docs_fallthrough` MUST emit the typed
   offline-override disclosure on the primary surface and MUST NOT
   claim the fallthrough is equivalent to the vendor pack.
3. `denied_*` outcomes MUST each pair with a typed repair hook on
   the projected badge record and MUST emit the corresponding
   ADR-0013 audit event.
4. `deferred_pending_reverify` MUST never silently auto-retry past
   the row's freshness window; the retry is triggered by an
   explicit refresh event, not by a docs-browser re-render.

## Privacy context sharing (baseline)

Baseline docs-search flows (exact symbol lookup, fuzzy docs search
by title, navigation inside a pack) MUST NOT silently upload
surrounding code, private docs, workspace identity, file paths, or
arbitrary context to a remote docs provider. The detailed review
lives in
[`/artifacts/docs/privacy_context_sharing_review.md`](../../artifacts/docs/privacy_context_sharing_review.md);
the packet freezes the following posture tokens the row record
projects.

### `privacy_context_sharing_posture` (frozen)

| Token | Meaning | Required rendering |
|---|---|---|
| `local_only_no_remote_transmission` | Row resolution stays inside the local product (project docs, generated reference, vendored-local, offline-bundle, warm cache). No identity, path, or symbol data leaves the local boundary. | No upload disclosure; row renders normally. |
| `mirror_fetch_metadata_minimum` | Row resolution refreshes a mirror over a network call; only the pack id, pack revision ref, and a coarse locale tag leave the boundary. Workspace identity, file paths, symbol graph context, and surrounding code stay local. | Mirror-refresh chip; typed minimum-metadata disclosure available on request. |
| `vendor_overlay_inspect_only_minimum` | Row is a vendor / provider `inspect_only` overlay under ADR-0010; only the pack id / provider record id / exact query string typed by the user crosses the boundary. Surrounding code and workspace identity do not. | Provider-overlay chip; typed minimum-metadata disclosure on the primary surface. |
| `higher_trust_context_sharing_required` | Row would require broader context (surrounding code, symbol graph neighbourhood, private docs) to resolve; baseline flows deny and route to the separately-approved higher-trust surface. | Denial chip with typed higher-trust approval route; row MAY NOT auto-approve broader upload. |
| `refused_policy_blocked_remote` | Admin policy blocks remote docs providers on this surface; row refuses any remote call. | Policy-blocked chip with the policy-pack ref. |

Rules (frozen):

1. `local_only_no_remote_transmission` is the default posture for
   every `project_docs`, `generated_reference`, `curated_knowledge_pack`,
   `support_runbook`, `offline_bundle_pinned`, and
   `vendored_local_pinned` row.
2. `higher_trust_context_sharing_required` is a denial, not a
   silent escalation. The docs browser MUST NOT quietly upload
   surrounding code or private docs to resolve a row; the higher-
   trust approval surface is a separate, explicitly-approved path
   whose design lives outside this packet.
3. `refused_policy_blocked_remote` MUST cite the policy-pack ref on
   the projected badge record and MUST route to a
   `request_admin_policy_change` hook; silent retry against the
   remote provider is non-conforming.
4. Citation anchors emitted from any posture MUST carry ids only
   — never raw page bodies, raw symbol definitions, or raw
   workspace paths — as required by ADR-0013.

## Seed corpus

The two companion manifests seed the following case ids. Every
case carries one `docs_browser_result_record` (for symbol-link
rows) or one offline-mirror-drill record and at least one
conformance-test ref.

### Symbol-link validation cases (see `symbol_link_validation_manifest.yaml`)

| Case id | Symbol-link resolution class | Cache state | Source class | Vendor cue | Notes |
|---|---|---|---|---|---|
| `docs_browser.exact_symbol_match.project_pack` | `exact_symbol_match` | `live_owner_fetch` | `project_docs` | `project_authoritative_only` | Baseline exact symbol match inside the project pack. |
| `docs_browser.nearby_version_fallback.compat_window` | `nearby_version_fallback` | `warm_cache_within_window` | `project_docs` | `project_authoritative_only` | Running build is a patch ahead of the project-docs pin; symbol resolves in a compat-window pack. |
| `docs_browser.package_level_guide_fallback.no_symbol_page` | `package_level_guide_fallback` | `live_owner_fetch` | `project_docs` | `project_authoritative_only` | Exact symbol has no dedicated page yet; module overview is the best available surface. |
| `docs_browser.project_outranks_vendor.both_match` | `project_docs_outranks_vendor_match` | `live_owner_fetch` | `project_docs` | `project_outranks_vendor_default` | Both project and vendor candidates exist; project takes precedence by default, vendor renders as secondary row. |
| `docs_browser.vendor_overrides_project_disclosed.by_policy` | `vendor_overrides_project_disclosed` | `live_owner_fetch` | `vendor_provider_docs` | `vendor_overrides_project_by_policy` | Admin policy has inverted the default; typed inversion disclosure renders on the primary surface. |
| `docs_browser.unresolved_requires_refresh.stale_index` | `unresolved_requires_refresh` | `unknown_pending_reverify` | `project_docs` | `project_authoritative_only` | Pack index is stale / freshness floor unmet; row defers render and routes to `refresh_freshness`. |
| `docs_browser.no_claim_yet_support_routed.uncovered_symbol` | `no_claim_yet_support_routed` | `live_owner_fetch` | `project_docs` | `project_authoritative_only` | No coverage exists yet; honest no-claim chip with a `contact_support` / `request_docs_coverage` hook. |
| `docs_browser.citation_anchor_retained.exported_row` | `exact_symbol_match` | `live_owner_fetch` | `project_docs` | `project_authoritative_only` | Export preserves the `citation_anchor_record` for later reconstruction of the exact row the user saw. |
| `docs_browser.cached_state_disclosure.warm_cache` | `exact_symbol_match` | `warm_cache_within_window` | `project_docs` | `project_authoritative_only` | Warm cache within window; row renders with "cached" chip alongside `warm_cached` freshness. |
| `docs_browser.mirrored_state_disclosure.current_snapshot` | `exact_symbol_match` | `mirror_snapshot_current` | `mirrored_official_docs` | `project_outranks_vendor_default` | Mirror snapshot is current; row renders with mirror chip and a pinned snapshot ref. |
| `docs_browser.stale_state_disclosure.mirror_past_floor` | `exact_symbol_match` | `mirror_snapshot_stale` | `mirrored_official_docs` | `project_outranks_vendor_default` | Mirror snapshot past declared freshness floor; typed stale-mirror disclosure renders on the primary surface. |

### Offline / mirror drill cases (see `offline_mirror_drill_manifest.yaml`)

| Case id | Outcome class | Notes |
|---|---|---|
| `docs_browser.offline_drill.graceful_cache_within_window` | `graceful_cache_within_window` | Owner unreachable; warm cache still within refresh window; row remains readable with a "cached" chip. |
| `docs_browser.offline_drill.offline_bundle_recovery` | `graceful_offline_bundle_recovery` | Signed offline bundle pinned by digest resolves a previously-live row; typed offline-bundle chip renders. |
| `docs_browser.offline_drill.project_docs_fallthrough` | `graceful_project_docs_fallthrough` | Vendor pack unreachable; typed offline-override disclosure falls through to project docs. |
| `docs_browser.offline_drill.mirror_continuity_broken_denied` | `denied_mirror_continuity_broken` | Mirror lineage broken; denial chip with `mirror_continuity_broken` cause and a `refresh_freshness` hook. |
| `docs_browser.offline_drill.signature_unverified_denied` | `denied_signature_unverified` | Pack signature unverified; denial chip; AI overlay reuse also denied. |
| `docs_browser.offline_drill.deferred_pending_reverify` | `deferred_pending_reverify` | Signature re-verify in flight; row defers render with `unknown_pending_reverify` cache state. |
| `docs_browser.offline_drill.offline_expiration_past_denied` | `denied_offline_expiration_past` | Offline-bundle deadline past; row denies render as stale and routes to an upgrade hook. |

## Surface admissibility

| Surface | May mint `docs_browser_result_record` | May claim cache state | May claim symbol-link resolution | Projection rule |
|---|---|---|---|---|
| `docs_browser_result_row` | yes | yes | yes | MUST emit one record per row; MUST project the `help_status_badge_record`, cache state, symbol-link resolution, project-vs-vendor cue, and derived-explanation reuse inline. |
| `docs_pane` | yes (in-product) | yes | yes (docs-page rows only; symbol-jump inside a pane reuses the docs-browser record) | Preserves the same record shape; chip collapsing allowed but record fields stay separately addressable. |
| `ai_explanation_overlay` | no | no | no | Quotes docs-browser records; reuse state MUST be `reusable_with_citation_anchor`; otherwise MUST deny render. |
| `onboarding_step` | no | no | no | Quotes a docs-browser record at `freshness_class` in `{authoritative_live, warm_cached}`; denies step when cache state is `unknown_pending_reverify`. |
| `support_export_row` | no | yes (quoted) | yes (quoted) | Preserves the record and citation anchors under the support-export redaction envelope; MUST NOT re-mint cache state. |
| `help_about_footer` | no | yes (quoted) | no | Quotes the docs-pack pin and freshness; MUST render `exact_build_match` for the running build. |

Rule: any surface not named here MAY NOT claim a docs-browser
result record; it quotes one minted by the docs browser.

## Evidence joins

| `evidence_id` | Family / source kind | Why it is linked here | Freshness note | Artifact ref |
|---|---|---|---|---|
| `evidence.verification.docs_browser.symbol_link_validation` | `verification_corpus` | Defines the case roster every docs-browser symbol-link row cites. | current | `fixtures/docs/symbol_link_validation_manifest.yaml` |
| `evidence.verification.docs_browser.offline_mirror_drill` | `verification_corpus` | Defines the drill roster every docs-browser offline / mirror outcome cites. | current | `fixtures/docs/offline_mirror_drill_manifest.yaml` |
| `evidence.docs.privacy_context_sharing_review` | `design_review` | Freezes the baseline privacy posture and the higher-trust escalation boundary the docs-browser honours. | current | `artifacts/docs/privacy_context_sharing_review.md` |
| `evidence.docs.help_badge_vocabulary` | `source_anchor` | Canonical badge vocabulary worked-examples the docs-browser projects onto every row. | current | `artifacts/docs/help_badge_vocabulary.yaml` |
| `evidence.docs.destination_descriptor_seed` | `source_anchor` | Canonical destination-descriptor rows the docs-browser quotes on every out-of-product link. | current | `artifacts/docs/destination_descriptor_seed.yaml` |
| `evidence.docs.docs_pack_examples` | `source_anchor` | Canonical docs-pack manifest fixtures (project-fresh, mirrored-offline, partially-stale, mixed-locale, newer-than-client, non-publishable) the docs-browser resolves result rows against. | current | `fixtures/docs/docs_pack_examples/` |

## Verification method

- **Verification classes used:** design review, vocabulary-reuse
  review, fixture review, schema-alignment review.
- **Procedure summary:** verified that the packet and its companion
  manifests and review reuse the ADR-0013 source-class / version-
  match / freshness / degraded-state / browser-handoff-reason /
  badge-record vocabularies, the ADR-0011 freshness / client-scope /
  redaction vocabularies, the ADR-0010 browser-handoff envelope,
  the ADR-0015 embedded-surface boundary, and the frozen docs-pack
  manifest / destination-descriptor contracts without minting
  parallel tokens. Verified that the cache-state, symbol-link
  resolution, project-vs-vendor truth-cue, derived-explanation
  reuse, offline-mirror drill outcome, and privacy context-sharing
  posture vocabularies are closed vocabularies and that seed cases
  exercise every scenario named in the spec (exact symbol match,
  nearby-version fallback, package-level guide fallback, project-
  doc outranking vendor-doc, cached / mirrored / live / stale,
  citation-anchor retention, offline docs-pack recovery).
- **Automation refs:** `not_yet_seeded` for a dedicated docs-browser
  corpus validator; structural parsing is currently the available
  automation. The docs-pack manifest fixtures are separately
  validated against
  `schemas/docs/docs_pack_manifest.schema.json`.

## Known gaps and waivers

- **Waiver refs:** `none`.
- **Known-limit refs:** `none`.
- **Migration-packet refs:** `none`.
- **Explicit gaps:** no docs-browser webview, AI-explanation overlay,
  onboarding surface, or live remote docs provider connection is
  wired to this packet yet.
- **Explicit gaps:** no dedicated JSON Schema exists yet for the
  `docs_browser_result_record` family, the symbol-link resolution
  row shape, the offline / mirror drill row shape, or the privacy
  context-sharing posture shape. Reserved shapes are documented
  here for later schema landing.
- **Explicit gaps:** the higher-trust context-sharing surface
  referenced by the `higher_trust_context_sharing_required` posture
  is out of scope here; its approval packet, audit-event lane, and
  redaction envelope will land with a later milestone.

## Reviewer signoff

- **Reviewer / forum:** `@ahmeddyounis`
- **Decision:** `needs_follow_up`
- **Date:** `2026-04-23`
- **Reviewed claim rows:**
  `packet_row:docs_browser.result_row_contract`,
  `packet_row:docs_browser.source_version_cache_honesty`,
  `packet_row:docs_browser.symbol_linked_reference_resolution`,
  `packet_row:docs_browser.project_vs_vendor_truth`,
  `packet_row:docs_browser.derived_explanation_reuse`,
  `packet_row:docs_browser.browser_handoff_explanation`,
  `packet_row:docs_browser.offline_mirror_drill`,
  `packet_row:docs_browser.privacy_context_sharing`,
  `packet_row:docs_browser.seed_corpus`
- **Blocking refs:** `none`

## Refresh trigger

- **Named rerun trigger:** `corpus_or_privacy_review_revision_changed`.
- **Expected freshness window:** `P30D`.
- **Next packet family to update with the same evidence ids:**
  AI-explanation overlay packet, onboarding-step packet, or
  support-export packet that starts quoting docs-browser result
  records.
