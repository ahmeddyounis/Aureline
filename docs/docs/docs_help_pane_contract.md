# Docs / help pane source-version-freshness, mirror/offline, and external-open contract

This document freezes the embedded docs / help pane contract. Every
in-product docs pane, docs-browser body, Help / About pane,
service-health pane, support-summary pane, onboarding help overlay,
AI-explanation pane, and release-notice pane resolves one shared
`help_pane_state_record` before the pane body renders. The
machine-readable boundary is
[`/schemas/docs/help_pane_state.schema.json`](../../schemas/docs/help_pane_state.schema.json);
worked YAML cases live in
[`/fixtures/docs/help_pane_cases/`](../../fixtures/docs/help_pane_cases/).

The eventual docs-help-service-health crate's Rust types are the schema
of record. This document and the JSON Schema export are the cross-tool
boundary every non-owning pane reads; if this document and ADR 0013 /
ADR 0010 / ADR 0015 disagree, the ADRs win and this document MUST be
updated in the same change.

## Why freeze this now

The neighbouring docs / help contracts already pin the upstream truth
each pane projects from:

- [`/docs/docs/docs_pack_manifest_contract.md`](./docs_pack_manifest_contract.md)
  freezes the docs-pack manifest source class, version, signing,
  mirror lineage, locale coverage, example summary, and publishable
  state every pack-bearing pane reads.
- [`/docs/docs/help_about_service_health_routes.md`](./help_about_service_health_routes.md)
  freezes the destination descriptor (trust, owner, boundary, route
  class, external-open policy, auth expectation, data-exit boundary)
  every route-bearing pane quotes.
- ADR 0013 freezes the `help_status_badge_record` parity-audit fields
  every emitted chip carries.

What stayed reserved was the body of the embedded pane itself: the
shape that says, for each rendered pane, what its current cache and
install state is, whether the pane is satisfying the requested locale
or falling back to the primary, whether the pane's content is live,
warm-cached, cached-only because the canonical owner is unreachable,
mirror-only, expired-cached, or not installed, whether the pane is
narrowed by admin policy or workspace-trust, and which copy rule the
pane MUST render so cached / mirrored / stale / missing content cannot
read as live truth.

Without that shared pane shape:

- An embedded docs pane would render a cached snapshot or a mirror
  copy with the same chrome as a live page, dropping the truth that
  the canonical owner has not been contacted.
- The docs browser would let "open in browser" copy drift into
  free-form strings instead of citing one of the screenshot-safe /
  export-safe handoff reasons frozen in ADR 0013 and re-exported on
  the destination descriptor.
- Help / About, service-health, support-summary, onboarding help
  overlays, and AI-explanation overlays would each invent their own
  vocabulary for "cached only", "mirror only", "expired cache",
  "not installed", "locale unavailable", and "policy limited" and
  the parity audit between them would degenerate into a
  hand-mapping exercise.
- A pane could imply live canonical truth while serving content
  whose source has been cached, mirrored, narrowed by policy,
  superseded, or quarantined.

The pane-state record closes that gap by giving every embedded
docs / help pane one stable family for source, version, freshness,
cache class, install state, offline / mirror posture, locale
availability, external-open path, support class, policy-limited
behaviour, stale-example disclosure, denial reasons, and the copy
rule each state MUST render.

## Scope

Frozen at this revision:

- One `help_pane_state_record` shape with a closed set of pane
  roles, source classes, exact-build applicability values,
  version-match states, freshness classes, cache classes, install
  states, offline postures, mirror-chain statuses, locale-
  availability states, locale-fallback disclosure classes,
  external-open paths, browser-handoff reasons, data-exit
  boundaries, disclosure-safety classes, disclosure modes, support
  classes, client scopes, canonical-owner classes, policy-limited
  behaviours, stale-example disclosure classes, copy-rule classes,
  and denial reasons.
- Field-for-field references to upstream truth (the docs-pack
  manifest, the destination descriptor, the help status badge,
  and — when the pane sits inside a guided / first-run flow — the
  onboarding pack state) so the pane never re-mints source,
  version, signing, mirror-lineage, locale, route, or boundary
  vocabulary.
- Rules that keep `cached-only`, `mirror-only`, `stale`,
  `not-installed`, `unavailable-locale`, `version-mismatched`,
  `policy-limited`, `stale-example-disclosed`, and
  `external-open-required` states distinct, with the typed copy
  rule each state MUST render on the primary surface.
- Rules that keep `external_open_path` explicit (forbidden,
  optional, required-primary, required-fallback) and that cap
  the handoff reason at the screenshot-safe / export-safe subset
  frozen in ADR 0013.

Out of scope until a superseding decision row opens:

- The docs-browser runtime: search, index, navigation, history.
  ADR 0013 reserves the docs-browser surface; this contract pins
  the pane state every browser body reads, not the browser
  runtime.
- The help / docs renderer body itself. The pane projects state;
  rendering details, layout, and interaction land in the
  consuming surface specs.
- The docs-pack publishing pipeline (build, sign, distribute,
  fetch, refresh). The pane reads the manifest the pipeline
  emits.
- The browser-handoff packet body (ADR 0010) and the embedded
  surface boundary card body (ADR 0015). The pane points at
  those records and quotes their typed reason / boundary classes
  rather than re-embedding their contents.

## Who reads this contract

- **Embedded docs panes, docs-browser bodies, Help / About panes,
  service-health panes, support-summary panes, onboarding help
  overlays, AI-explanation panes, and release-notice panes** —
  to read **one** pane-state record family instead of inventing
  per-surface cache / locale / open-in-browser vocabulary.
- **Parity audits across Help, About, docs panes, the docs
  browser, the service-health view, and the support summary** —
  to compare panes against the same source / version / freshness
  / cache / install / locale / external-open / policy axes
  mechanically.
- **Reviewers (release, security, accessibility, claim manifest)**
  — to verify that no rendered pane implies live canonical truth
  while serving cached, mirrored, stale, not-installed, locale-
  blocked, version-mismatched, or policy-narrowed content; that
  the screenshot-safe / export-safe handoff-reason vocabulary
  stays the only handoff reason the pane quotes; and that
  not-installed / cached-only / mirror-only / unavailable-locale
  / stale states remain first-class pane states, not tooltip
  caveats.

## Two questions the contract answers

Any embedded docs / help pane MUST answer both questions
mechanically, without per-surface copy:

1. **Where does this pane's content come from right now?** Which
   docs-pack revision (or destination descriptor, or external
   status feed) is the pane projecting? What is the version-
   match state against the running build? What is the freshness
   class, and is the cache class live, warm-cached, cold-cached,
   cached-only because the owner is unreachable, mirror-only,
   mirror-broken, expired, not-installed, or disabled by policy?
   Which canonical owner published the truth, and is the pane
   build-bound or build-independent?
2. **What does the pane MUST disclose before letting the user
   read or act?** Which copy rule MUST render on the primary
   surface for the current state (cached-only, mirror-only,
   stale, not-installed, unavailable-locale, version-mismatched,
   policy-limited, stale-example-disclosed, external-open-
   required)? Is `external_open_path` forbidden, optional,
   required-primary, or required-fallback, and which screenshot-
   safe / export-safe reason backs that path? What is the
   data-exit boundary if the pane hands off, and what is the
   disclosure mode (inline, embedded-boundary-card, pre-handoff
   review)?

## Record fields

The full field set lives in
[`/schemas/docs/help_pane_state.schema.json`](../../schemas/docs/help_pane_state.schema.json).
The notable fields are:

- **Identity and references.** `pane_id` is the stable opaque id
  of the pane state; `pane_role` names which embedded surface
  consumes the record. `docs_pack_manifest_ref`,
  `destination_descriptor_ref`, `help_status_badge_ref`, and
  `onboarding_pack_state_ref` pin the upstream records the pane
  projects without re-minting their fields. `canonical_owner_class`
  + `canonical_owner_id` name the registry / aggregator /
  publisher that resolved the pane's truth.
- **Source and version.** `source_class` is re-exported from ADR
  0013 (`project_docs`, `generated_reference`,
  `mirrored_official_docs`, `curated_knowledge_pack`,
  `derived_explanation`, `vendor_provider_docs`, `support_runbook`,
  `external_status_feed`). `display_source_version`,
  `exact_build_applicability`, `version_match_state`, and
  `running_build_identity_ref` make build applicability and
  version match mechanically comparable to the docs-pack manifest
  and to other panes. Build-bound panes carry both the build id
  and the version-match state; build-independent panes
  (`not_build_bound`) keep both null instead of inventing a fake
  version chip.
- **Freshness, cache, and install.** `freshness_class` is re-
  exported from ADR 0011 / ADR 0013 (`authoritative_live`,
  `warm_cached`, `degraded_cached`, `stale`, `unverified`).
  `cache_class` is the new pane-level axis: `live_authoritative_no_cache`,
  `warm_cached_within_window`, `cold_cached_outside_window`,
  `cached_snapshot_only_owner_unreachable`,
  `mirror_only_verified_offline`, `mirror_continuity_broken`,
  `expired_cached_requires_refresh`, `not_cached_not_installed`,
  `cache_disabled_by_policy`. `install_state` mirrors the
  onboarding-pack install vocabulary so a pane can name
  `live_installed_current`, `cached_snapshot_current`,
  `cached_snapshot_stale`, `mirror_only_verified`,
  `local_only_starter`, `not_installed`, `remote_unavailable`,
  `policy_blocked`, `quarantined`, or `withdrawn` without
  re-minting.
- **Offline and mirror posture.** `offline_posture` re-exports
  the destination-descriptor offline behaviour and the
  onboarding-pack offline-posture vocabulary. `mirror_chain_status`
  re-exports the docs-pack mirror chain state.
  `last_refreshed_at` and `offline_expiration_at` make the
  refresh window auditable.
- **Locale.** `primary_locale`, `requested_locale`, and
  `effective_locale` are BCP-47 tags. `locale_availability_state`
  composes the docs-pack locale-coverage class with the
  onboarding-pack locale-presence class so a pane can name
  `requested_locale_authoritative`, `requested_locale_partial`,
  `requested_locale_machine_assisted`, `requested_locale_stub`,
  `requested_locale_stale_copy`,
  `requested_locale_missing_fallback_to_primary`,
  `requested_locale_not_installed`, `requested_locale_policy_blocked`,
  or `locale_not_applicable_to_pane`.
  `locale_fallback_disclosure_class` keeps the typed disclosure
  rule explicit; tooltip-only disclosure is forbidden for any
  class other than `no_fallback_primary_locale_only`.
- **External-open.** `external_open_path` re-exports the
  destination-descriptor external-open policy.
  `external_destination_descriptor_ref` pins the descriptor the
  pane hands off to. `browser_handoff_reason` is the screenshot-
  safe / export-safe subset frozen in ADR 0013.
  `data_exit_boundary` discloses what leaves the product before
  navigation. `disclosure_safety_class` MUST be
  `screenshot_safe_export_safe` whenever the pane offers an
  external open. `disclosure_mode` (inline, embedded boundary
  card, pre-handoff review) follows the destination-descriptor
  rule for the chosen route.
- **Support and client scope.** `support_class` and
  `client_scopes` remain separate axes; a pane may be cached
  but supported, available but only on desktop, or community-
  supported but not installed locally.
- **Policy-limited behaviour.** `policy_limited_behavior` names
  the dominant narrowing applied by ADR 0008 admin policy or
  ADR 0001 workspace-trust: `not_policy_limited`,
  `source_class_narrowed_by_policy`,
  `surface_narrowed_by_policy`, `locale_narrowed_by_policy`,
  `external_open_blocked_by_policy`,
  `freshness_floor_raised_by_policy`, `pack_pinned_by_policy`,
  `cache_disabled_by_policy`, or `pane_suppressed_by_policy`.
- **Stale-example disclosure.** `stale_example_disclosure_class`
  names how the pane handles the docs-pack manifest's stale /
  needs-review / quarantined examples on the primary surface.
  `stale_example_refs` pins the example ids the pane is
  rendering with the typed disclosure.
- **Known limits and support exports.** `known_limits_summary_ref`
  and `support_export_packet_ref` let Help / About,
  service-health, and support-summary panes cite known-limit and
  support-export descriptors without re-stating support copy.
- **Copy rule.** `copy_rule_class` is the contract between the
  pane and its renderer. Each pane state resolves to exactly one
  class so cached, mirrored, stale, not-installed, unavailable-
  locale, version-mismatched, policy-limited, and external-open-
  required states cannot collapse into a vague "available" or
  "learn more" affordance.
- **Renderable gate.** `renderable` is the boolean a renderer
  reads. `denial_reasons` is the closed list of typed reasons
  that flipped the gate; a pane whose `renderable` is false MUST
  carry at least one reason and a non-null `repair_hook_ref`.
- **Policy context and redaction.** `policy_context`
  (`policy_epoch`, `trust_state`, `execution_context_id`) and
  `redaction_class` are re-exported from ADR 0001 / ADR 0007 /
  ADR 0008 / ADR 0009 / ADR 0011 without modification.

## State copy rules

The closed `copy_rule_class` set names which copy each state MUST
render on the primary surface. The pairings below are normative;
silent collapse to "available" or "learn more" is non-conforming.

- `live_authoritative_truth_copy` — pane is rendering the
  canonical owner's live truth at the running build's exact
  identity. Only admissible when `cache_class =
  live_authoritative_no_cache` and `freshness_class =
  authoritative_live`. No disclosure chip required beyond the
  source / version / freshness chips on the help status badge.
- `warm_cached_within_window_copy` — pane is rendering a recent
  cache copy still inside the pack's refresh window. The pane
  MUST name the cache class on the primary surface and MUST
  NOT imply live authority.
- `cold_cached_outside_window_copy` — pane is rendering a cache
  copy past its refresh window while the live owner is still
  reachable. The pane MUST surface the typed `freshness_floor_unmet`
  cue on the primary surface and offer a `refresh_freshness`
  repair hook.
- `cached_snapshot_only_owner_unreachable_copy` — pane is
  rendering a cached snapshot because the canonical owner is
  unreachable. The pane MUST disclose that the snapshot is the
  only available copy and MUST NOT imply that the canonical
  owner has been contacted.
- `mirror_only_verified_offline_copy` — pane is rendering a
  signed-mirror copy whose canonical owner is out of scope. The
  pane MUST disclose the mirror-only posture inline and MUST
  cite the mirror's air-gapped origin label or signed-bundle
  acquisition class when present.
- `mirror_continuity_broken_copy` — pane is suppressed because
  the mirror chain failed verification. Used only with
  `renderable = false` and the `mirror_continuity_broken`
  denial reason.
- `expired_cached_requires_refresh_copy` — pane is suppressed
  because an offline / cache deadline has passed. Used only with
  `renderable = false` and either `cached_snapshot_expired` or
  `air_gapped_bundle_expired`.
- `not_installed_pack_copy` — pane is suppressed because no copy
  of the pack is resident. Used only with `renderable = false`
  and the `pack_not_installed` denial reason.
- `stale_example_disclosed_copy` — pane is rendering a
  partially-stale pack with at least one stale_example or
  needs_review_example entry on the primary surface. The pane
  MUST render the typed disclosure inline; tooltip-only
  disclosure is forbidden.
- `unavailable_locale_copy` — pane is rendering or suppressing
  a locale gap. When the requested locale is missing-and-
  fallback-to-primary, the disclosure renders inline; when the
  locale is missing-and-not-installed or policy-blocked, the
  pane is suppressed and the typed denial fires.
- `version_mismatched_copy` — pane is suppressed because the
  pack targets a build outside the running client's compat
  window. Used only with `renderable = false` and the
  `incompatible_drift_detected` denial reason.
- `policy_limited_copy` — pane is rendering or suppressing
  under an explicit ADR 0008 / ADR 0001 narrowing. The pane
  MUST surface the narrowing on the primary surface; admin /
  trust narrowings rendered as tooltips are non-conforming.
- `external_open_required_fallback_copy` — pane offers a
  required browser fallback because the in-product render
  cannot serve the same logical object. The pane MUST cite a
  screenshot-safe / export-safe browser_handoff_reason and the
  destination descriptor it hands off to.
- `external_open_required_primary_copy` — pane is browser-
  primary; the in-product preview is a typed projection only,
  never a parity replacement. The pane MUST disclose the
  browser-primary posture on the primary surface.
- `support_export_known_limits_copy` — Help / About, service-
  health, or support-summary pane is rendering a known-limits
  row sourced from a known-limits descriptor. The pane MUST
  cite the descriptor by id rather than restating support copy.

## Cached, mirrored, stale, not-installed, unavailable-locale rules

The pane vocabulary intentionally separates:

- `freshness_class` for how current the route's source truth is,
- `cache_class` for the cache state of the pane's copy,
- `install_state` for whether the pack copy is resident,
- `offline_posture` for what survives offline,
- `mirror_chain_status` for mirror continuity,
- `version_match_state` for build applicability,
- `locale_availability_state` for the requested-locale state,
- `support_class` for support commitment, and
- `client_scopes` for which clients may claim the pane.

Rules:

- A pane whose `cache_class` is one of
  `cached_snapshot_only_owner_unreachable`,
  `mirror_only_verified_offline`, `mirror_continuity_broken`,
  `expired_cached_requires_refresh`, or `not_cached_not_installed`
  MUST NOT declare `freshness_class = authoritative_live` and
  MUST NOT pick `copy_rule_class = live_authoritative_truth_copy`.
- A pane that is `version_mismatched` MUST suppress with
  `incompatible_drift_detected` and route to a
  `repair_hook_ref` (typically `upgrade_release_channel` or
  `refresh_freshness`).
- A pane whose mirror chain is `predecessor_missing` or
  `signing_chain_broken` MUST suppress with
  `mirror_continuity_broken`.
- A pane whose offline expiration has passed MUST suppress with
  `cached_snapshot_expired` or `air_gapped_bundle_expired`,
  regardless of `freshness_class`.
- A pane whose pack is `not_installed`, `quarantined`, or
  `withdrawn` MUST suppress with `pack_not_installed`,
  `pack_quarantined`, or `pack_withdrawn`.
- A pane whose `install_state` is `remote_unavailable` and whose
  `cache_class` is not one of
  `cached_snapshot_only_owner_unreachable` or
  `mirror_only_verified_offline` MUST suppress with
  `remote_owner_unreachable`.
- A pane whose `locale_availability_state` is
  `requested_locale_not_installed` MUST suppress with
  `locale_missing_not_installed`. A pane whose state is
  `requested_locale_policy_blocked` MUST suppress with
  `locale_policy_blocked`. The
  `requested_locale_missing_fallback_to_primary` state remains
  renderable but MUST set
  `locale_fallback_disclosure_class` to one of
  `inline_fallback_disclosure_rendered` or
  `embedded_boundary_card_disclosure_rendered`.
- A pane whose stale-example ratio crosses the publisher
  threshold (`stale_example_disclosure_class =
  stale_examples_exceed_threshold_pane_suppressed`) MUST
  suppress with `stale_examples_exceed_threshold`.

## External-open and source-of-truth rules

The pane MUST keep the open-in-browser action explicit and
truthful:

- `external_open_path = not_permitted` is the default for any
  pane whose canonical truth is local or whose route never
  leaves the local product boundary. Browser handoff reason and
  destination descriptor stay null.
- `optional_same_object` is admissible when the canonical owner
  publishes a browser route for the same logical object the
  pane already renders locally. The pane MUST cite a
  screenshot-safe / export-safe reason and the destination
  descriptor.
- `required_fallback_when_in_product_unavailable` is the
  truthful escape hatch when an embedded surface narrows below
  its declared capability set, when a cached / mirrored copy
  cannot be refreshed, or when the in-product render stops
  being truthful. The pane MUST disclose the fallback inline
  and route to the typed reason.
- `required_primary_route` is browser-first. The in-product
  preview is a typed projection only and MUST NOT imply
  parity with the canonical browser route.
- `policy_limited_behavior = external_open_blocked_by_policy`
  forces `external_open_path = not_permitted` and clears the
  handoff reason; the pane MUST disclose the policy narrowing
  on the primary surface.

The pane MUST NOT mint a parallel handoff-reason vocabulary; it
quotes the screenshot-safe / export-safe subset frozen in ADR
0013 and re-exported on the destination descriptor.

## Reuse in About, help, service-health, and onboarding

The same `help_pane_state_record` family is admissible across
embedded surfaces without inventing new vocabulary:

- **In-product docs pane and embedded docs-browser body** —
  read the record before showing the page body; quote source /
  version / cache / locale / external-open fields directly on
  the chrome.
- **Help / About pane** — read the record to render the source-
  version-freshness footer, the open-in-browser action, the
  policy narrowing line, and the known-limits row.
- **Service-health pane** — read the record to render the
  service-health row's source / freshness / cache / external-
  open posture without remixing into per-feature vocabulary.
- **Support-summary pane** — read the record to enumerate the
  source / version / signing / mirror-lineage / locale fields
  the support export carries without restating them.
- **Onboarding help overlay** — read the record (with the
  `onboarding_pack_state_ref` populated) to keep cached / not-
  installed / locale-missing first-class states in tour copy.
- **AI-explanation pane** — read the record to deny render with
  `derived_explanation_uncited` when the pane has no cited
  anchor against a `citation_required` pack, and to surface
  cached / mirrored sources as cached and mirrored.
- **Release-notice pane** — read the record (with
  `exact_build_applicability = not_build_bound` and a
  `release_notice_descriptor_publisher` canonical owner) to
  render version-independent notices without inventing a
  parallel pane vocabulary.

## Linkage to neighbouring contracts

- **ADR 0013 truth-source vocabulary.** `source_class`,
  `version_match_state`, `freshness_class`, `client_scopes`,
  and `redaction_class` are re-exported from
  [`/schemas/docs/help_status_badge.schema.json`](../../schemas/docs/help_status_badge.schema.json)
  without modification.
- **Destination descriptor (ADR 0013 + ADR 0010 + ADR 0015).**
  `exact_build_applicability`, `external_open_path`,
  `data_exit_boundary`, `disclosure_safety_class`,
  `disclosure_mode`, and `support_class` are re-exported from
  [`/schemas/docs/destination_descriptor.schema.json`](../../schemas/docs/destination_descriptor.schema.json).
- **Docs-pack manifest.** `mirror_chain_status`,
  `offline_posture`, locale coverage classes, and stale-example
  posture are re-exported from
  [`/schemas/docs/docs_pack_manifest.schema.json`](../../schemas/docs/docs_pack_manifest.schema.json).
- **Onboarding pack state.** `install_state`,
  `offline_posture`, `locale_availability_state`,
  `locale_fallback_disclosure_class`, and the onboarding-scoped
  `repair_hook_ref` extensions are re-exported from
  [`/schemas/docs/onboarding_pack_state.schema.json`](../../schemas/docs/onboarding_pack_state.schema.json)
  when the pane sits inside an onboarding flow.
- **ADR 0011 capability lifecycle.** `freshness_class`,
  `client_scope`, `repair_hook_ref` (base set), and
  `redaction_class` are re-exported from
  [`/schemas/governance/capability_lifecycle.schema.json`](../../schemas/governance/capability_lifecycle.schema.json)
  without modification.
- **D-0011 exact-build identity.** `running_build_identity_ref`
  pins the running build identity that drives `version_match_state`.

## Schema of record

Rust types in the eventual docs-help-service-health crate are the
schema of record. The JSON Schema export at
[`/schemas/docs/help_pane_state.schema.json`](../../schemas/docs/help_pane_state.schema.json)
is the cross-tool boundary every non-owning pane reads. Adding a
new pane role, source class, cache class, install state, offline
posture, mirror-chain status, locale-availability state, locale-
fallback disclosure class, external-open path, browser-handoff
reason, data-exit boundary, disclosure-safety class, disclosure
mode, support class, client scope, canonical-owner class,
policy-limited behaviour, stale-example disclosure class, copy-
rule class, denial reason, or repair-hook kind is additive-minor
and bumps `help_pane_state_schema_version`; repurposing an
existing value is breaking and requires a new decision row.

There is no external IDL or code-generator toolchain at this
milestone; this mirrors ADR 0004 through ADR 0015.

## Source anchors

- [`docs/adr/0013-docs-help-service-health-truth.md`](../adr/0013-docs-help-service-health-truth.md)
  — source-of-truth ownership, parity-audit fields, screenshot-
  safe / export-safe browser-handoff reason subset, and the
  reserved help-pane state slot this contract closes.
- [`docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md)
  — browser-handoff packet and destination class vocabulary.
- [`docs/adr/0015-embedded-surface-boundary-and-auth-handoff.md`](../adr/0015-embedded-surface-boundary-and-auth-handoff.md)
  — host-owned embedded boundary card and system-browser-first
  rules quoted by panes that disclose with the embedded
  boundary card.
- [`docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md)
  — `freshness_class`, `client_scope`, `repair_hook_ref`,
  `redaction_class` vocabularies re-exported here.
- [`docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`](../adr/0008-settings-definition-and-effective-configuration-resolver.md)
  — admin-policy narrowing rules quoted by
  `policy_limited_behavior`.
- [`docs/docs/docs_pack_manifest_contract.md`](./docs_pack_manifest_contract.md)
  — docs-pack source / version / signing / mirror lineage /
  locale / example-summary truth the pane projects from.
- [`docs/docs/help_about_service_health_routes.md`](./help_about_service_health_routes.md)
  — destination descriptor vocabulary the pane quotes for
  trust, owner, boundary, route class, external-open policy,
  auth expectation, and data-exit boundary.
- [`.t2/docs/Aureline_PRD.md`](../../.t2/docs/Aureline_PRD.md)
  — normative language for honest disclosure, exact-build
  truth, and browser-handoff honesty.
- [`.t2/docs/Aureline_Technical_Design_Document.md`](../../.t2/docs/Aureline_Technical_Design_Document.md)
  — About packet, service-health event, help destination
  descriptor, and embedded docs / help pane requirements.
- [`.t2/docs/Aureline_UI_UX_Spec_Document.md`](../../.t2/docs/Aureline_UI_UX_Spec_Document.md)
  — rule that support class, freshness, client scope, and
  cache state stay separate cues and that browser handoff
  preserves object identity and return path.
