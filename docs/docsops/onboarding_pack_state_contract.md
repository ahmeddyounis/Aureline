# Onboarding docs-pack locale, cached / not-installed state, and offline disclosure contract

This document freezes the onboarding-pack state contract every first-run
flow, Start Center tile, welcome surface, glossary card, guided-tour
step, and in-product help overlay reads before it embeds an onboarding
docs pack, glossary bundle, or guided-content pack. The machine-readable
boundary is
[`/schemas/docs/onboarding_pack_state.schema.json`](../../schemas/docs/onboarding_pack_state.schema.json);
worked fixtures live under
[`/fixtures/docs/onboarding_pack_examples/`](../../fixtures/docs/onboarding_pack_examples/).

The eventual docs-help-service-health crate's Rust types are the schema
of record. This document and the JSON Schema export are the cross-tool
boundary every non-owning onboarding / glossary / tour / help-overlay
surface reads; if this document disagrees with the docs-pack manifest
contract, the destination-descriptor contract, the onboarding-portability
contract, or the learnability contract it composes with, those upstream
contracts win and this document MUST be updated in the same change.

## Why freeze this now

The onboarding flow is where first impressions harden. Without a shared
state contract, every onboarding surface would invent its own answer to
"is this pack actually installed?", "is this locale reviewed, or am I
falling back?", "is this snapshot stale?", "can this open embedded, in a
system browser, or not at all?", and "does this row pretend to be live
truth when it is a cached local-only copy?". The onboarding measurement
plan, the no-account local-entry contract, the docs-pack manifest
contract, the destination-descriptor contract, and the learnability
contract already froze the upstream vocabularies; what was missing was
the governed state record each onboarding surface reads before it
decides whether to render a pack, render a fallback locale, open in a
system browser, or suppress the row with a typed cause.

Left implicit:

- First-run surfaces would render locale-fallback copies as though they
  were the requested locale, with no typed disclosure.
- Welcome-tour steps would treat a cached snapshot past its refresh
  window as live truth instead of typed `cached_snapshot_stale`.
- Glossary cards would silently collapse `not_installed`,
  `remote_unavailable`, and `policy_blocked` into "unavailable" without
  a typed cause the repair hook can route on.
- Air-gapped deployments would see the same onboarding prompts as
  managed-cloud without any disclosure that the pack is an offline
  signed-bundle copy with a monotonic expiration.
- Account-optional onboarding guidance would slide into forced-auth
  posture on accident when the running envelope (air-gapped,
  privacy-reduced, managed-fleet) disables the account surface.

Freezing the state record before first-run implementation lands makes
each of those failure modes a schema or policy violation rather than a
UX drift the product accumulates silently.

## Scope

Frozen at this revision:

- One `onboarding_pack_state_record` shape with a closed set of pack
  roles, install states, locale-presence classes, locale-fallback
  disclosure classes, offline postures, browser-handoff policies,
  account prerequisite classes, embedded-route policies, reset classes,
  and publishable-denial reasons.
- One `onboarding_pack_locale_row_record` shape so parity audits and
  support exports can enumerate locale rows across onboarding packs
  without holding a full state record.
- Rules for browser-handoff, embedded-surface, and local-only routes
  covering when onboarding / help content may open in the system
  browser, in an embedded surface with the ADR-0015 boundary card, or
  strictly in product chrome.
- Rules for locale-fallback, cached-but-stale, not-installed,
  remote-unavailable, withdrawn, and policy-blocked posture, with a
  typed denial reason in every suppressed case.
- Cross-contract reuse rules: this record composes the docs-pack
  manifest, destination-descriptor, guided-surface-state, and
  onboarding-portability vocabularies rather than re-minting them.

Out of scope until a superseding decision row opens:

- Authoring all localized onboarding content. The contract pins the
  state record; the authoring pipeline (translation review, string
  extraction, localization CMS) is a later lane.
- The first-run flow implementation (Start Center tiles, welcome
  banners, glossary-card UI). The record is the boundary; the surfaces
  land later.
- The docs-pack publishing pipeline and the onboarding locale-pack
  distribution pipeline.

## Upstream contracts this record composes

This record is a projection layer over four upstream contracts. It
cites them by reference and MUST NOT re-mint their vocabularies.

- **Docs-pack manifest contract**
  ([docs/docs/docs_pack_manifest_contract.md](../docs/docs_pack_manifest_contract.md),
  [schemas/docs/docs_pack_manifest.schema.json](../../schemas/docs/docs_pack_manifest.schema.json)) —
  identity, source class, publisher class, signing, mirror lineage,
  locale coverage, example summary, publishable state, and repair-hook
  vocabulary. The onboarding record's `docs_pack_manifest_ref` pins
  one manifest; the onboarding record's `install_state`,
  `freshness_class`, `version_match_state`, `locale_presence_class`,
  and `publishable` are derived from the manifest's signing / mirror /
  locale / publishable-state fields.
- **Destination-descriptor contract**
  ([docs/docs/help_about_service_health_routes.md](../docs/help_about_service_health_routes.md),
  [schemas/docs/destination_descriptor.schema.json](../../schemas/docs/destination_descriptor.schema.json)) —
  route class, preferred / fallback routes, external-open policy,
  embedded disclosure mode, auth expectation, data-exit boundary, and
  the screenshot-safe / export-safe `browser_handoff_reason` subset.
  The onboarding record's `destination_descriptor_ref` pins one
  descriptor; `browser_handoff_policy`, `browser_handoff_reason`, and
  `embedded_route_policy` quote the descriptor's route posture and
  disclosure mode.
- **Guided-surface state contract**
  ([docs/ux/learnability_contract.md](../ux/learnability_contract.md),
  [schemas/ux/guided_surface_state.schema.json](../../schemas/ux/guided_surface_state.schema.json)) —
  surface kind, guidance authority, suppression cause, dismissal /
  reset / progress-export class. The onboarding record's optional
  `guided_surface_state_ref` pins the guided surface the pack is
  consumed inside; glossary cards, guided-tour steps, exercise steps,
  and architecture explainers set this ref so the guided-surface
  contract's suppression rules and the onboarding record's install /
  locale rules compose deterministically.
- **Onboarding-portability contract**
  ([docs/ux/no_account_local_entry_contract.md](../ux/no_account_local_entry_contract.md),
  [schemas/ux/onboarding_portability_state.schema.json](../../schemas/ux/onboarding_portability_state.schema.json)) —
  entry-surface family, account-prompt class, state-portability class,
  reset / export / profile-scope class. The onboarding record's
  optional `onboarding_portability_state_ref` pins the portability
  state the pack carries (tour progress, dismissal, imported-profile
  history) so the no-account / account-optional / managed-forced lanes
  stay coherent across both contracts.

## Record fields

The full field set lives in
[`/schemas/docs/onboarding_pack_state.schema.json`](../../schemas/docs/onboarding_pack_state.schema.json).
Notable fields:

- **Identity.** `pack_id` and `pack_revision_ref` pin the pack
  revision. `docs_pack_manifest_ref` pins the manifest the record
  projects from; `destination_descriptor_ref` pins the destination
  descriptor the pack opens through. `guided_surface_state_ref` and
  `onboarding_portability_state_ref` are optional pins to composing
  contracts.
- **Pack role.** `pack_role` names one of `first_run_starter_pack`,
  `welcome_tour_pack`, `glossary_bundle`, `guided_content_pack`,
  `in_product_help_overlay_pack`, `account_optional_onboarding_pack`,
  `migration_welcome_pack`, or `air_gapped_offline_onboarding_pack`.
- **Install state.** `install_state` names one of `local_only_starter`,
  `live_installed_current`, `cached_snapshot_current`,
  `cached_snapshot_stale`, `mirror_only_verified`, `not_installed`,
  `remote_unavailable`, `policy_blocked`, `quarantined`, `withdrawn`.
  The surface NEVER renders a `not_installed`, `remote_unavailable`,
  `cached_snapshot_stale`, `policy_blocked`, `quarantined`, or
  `withdrawn` row as current live truth; each state MUST suppress the
  row with a typed denial reason.
- **Freshness and version.** `freshness_class` and
  `version_match_state` are re-exported from the docs-pack manifest /
  help-status-badge vocabulary without modification. A `cached_snapshot_stale`
  pack MUST declare `freshness_class` in `{stale, unverified}`; an
  `incompatible_drift_detected` row MUST suppress with the
  `incompatible_build_drift` denial reason.
- **Locale.** `primary_locale`, `requested_locale`, and
  `effective_locale` are BCP-47 tags. `locale_presence_class` names
  one of `locale_available_reviewed`,
  `locale_available_machine_assisted`, `locale_available_stub`,
  `locale_available_stale_copy`, `locale_missing_fallback_to_primary`,
  `locale_missing_not_installed`, or `locale_policy_blocked`.
  `locale_fallback_disclosure_class` names how the surface discloses
  the locale gap: `no_fallback_primary_locale_only` (baseline),
  `inline_fallback_disclosure_rendered` (chip on the primary surface),
  `embedded_boundary_card_disclosure_rendered` (ADR-0015 boundary card
  on an embedded surface), or `surface_suppressed_no_fallback_rendered`
  (row suppressed with a typed denial).
- **Locale rows.** `locale_rows` enumerates one
  `onboarding_pack_locale_row_record` per locale the pack advertises
  (including the primary locale and any missing / policy-blocked
  locale the surface may need to disclose). Each row carries a
  `locale_authoring_state` that pins how the locale was authored
  (authored_and_reviewed, machine_assisted_not_reviewed,
  authored_against_prior_source_revision, stub_titles_only,
  not_authored).
- **Offline posture.** `offline_posture` names one of
  `not_available_offline`, `fully_available_offline_local_build`,
  `cached_snapshot_offline`, `mirror_verified_offline`,
  `air_gapped_signed_bundle`, `offline_expired_requires_refresh`.
  `air_gapped_offline_onboarding_pack` rows MUST declare one of the
  last three.
- **Browser handoff.** `browser_handoff_policy` names one of
  `no_handoff_local_only`, `optional_same_object_system_browser`,
  `required_fallback_when_in_product_unavailable`,
  `required_primary_route_system_browser`,
  `required_primary_route_device_code`, or
  `handoff_blocked_by_policy`. Any policy other than
  `no_handoff_local_only` / `handoff_blocked_by_policy` MUST name a
  screenshot-safe / export-safe `browser_handoff_reason` from the
  destination-descriptor subset.
- **Account prerequisite.** `account_prerequisite_class` names one of
  `no_account_required`, `optional_account_context`,
  `deferrable_account_upgrade`, `required_authenticated_session`,
  `required_step_up`, `managed_admin_session`, or
  `account_unavailable_in_envelope`. First-run starter packs,
  glossary bundles, account-optional onboarding packs, and
  air-gapped offline onboarding packs MUST remain
  `no_account_required` so the local-first promise holds under
  every envelope.
- **Embedded route.** `embedded_route_policy` names one of
  `local_only_product_chrome`,
  `embedded_boundary_card_required`,
  `embedded_boundary_card_required_with_system_browser_fallback`,
  `system_browser_primary_with_product_chrome_preview`, or
  `embedded_route_blocked_by_policy`.
- **Publishable gate.** `publishable` is a boolean; when false, the
  record MUST carry at least one typed `publishable_denial_reason`
  (`pack_signature_unverified`, `mirror_continuity_broken`,
  `pack_quarantined`, `pack_withdrawn`, `pack_not_installed`,
  `remote_owner_unreachable`, `cached_snapshot_expired`,
  `air_gapped_bundle_expired`, `locale_missing_not_installed`,
  `locale_policy_blocked`, `incompatible_build_drift`,
  `account_required_but_envelope_blocks`, `policy_blocked`,
  `embedded_route_blocked`, `required_citation_anchors_missing`) and a
  `repair_hook_ref`.
- **Reset class.** `reset_class` names one of
  `not_resettable_packaged_with_binary`,
  `resettable_with_pack_refresh`,
  `resettable_with_reinstall_or_import`,
  `resettable_with_policy_change`, or
  `resettable_with_account_reset`.
- **Applicability and policy context.**
  `applicable_deployment_profiles` pins which profiles the pack
  applies to (individual_local, self_hosted, managed_cloud,
  managed_fleet, air_gapped); `policy_context` and `redaction_class`
  are re-exported from ADR-0001 / ADR-0007 / ADR-0008 / ADR-0009 /
  ADR-0011 without modification.

## Browser-handoff and embedded-route rules

Surfaces decide where onboarding / help content opens by reading
`preferred_route_class` and `fallback_route_classes` on the
destination descriptor and cross-checking them against the onboarding
record's `browser_handoff_policy` and `embedded_route_policy`:

- **Local-only product chrome.** `embedded_route_policy =
  local_only_product_chrome` is the default for first-run starter
  packs, glossary bundles, and in-product help overlays whose content
  renders inside native product chrome. `browser_handoff_policy` is
  typically `no_handoff_local_only` in this row; inline / primary
  locale rendering is the baseline. No browser handoff, no embedded
  surface, no account prompt.
- **Embedded surface with boundary card.** `embedded_route_policy =
  embedded_boundary_card_required` applies when the pack renders
  inside an embedded surface (webview, inline docs pane) and the host
  MUST render the ADR-0015 boundary card (owner / origin / data-exit).
  The onboarding record's `locale_fallback_disclosure_class` uses
  `embedded_boundary_card_disclosure_rendered` when a locale gap is
  disclosed through the boundary card rather than inline.
- **Embedded with system-browser fallback.** `embedded_route_policy =
  embedded_boundary_card_required_with_system_browser_fallback`
  applies when the embedded surface may refuse to render (certificate
  failure, cross-origin block, offline) and the host MUST offer an
  ADR-0013 screenshot-safe system-browser fallback. `browser_handoff_policy`
  is typically `required_fallback_when_in_product_unavailable` in this
  row.
- **System browser primary.** `embedded_route_policy =
  system_browser_primary_with_product_chrome_preview` applies when
  the canonical owner is the browser and the product renders a typed
  preview only. `browser_handoff_policy` is
  `required_primary_route_system_browser` (or
  `required_primary_route_device_code` for device-code flows). Rare
  for onboarding; admitted only for managed-admin or account-bound
  guidance.
- **Policy-blocked handoff.** `embedded_route_policy =
  embedded_route_blocked_by_policy` and `browser_handoff_policy =
  handoff_blocked_by_policy` both suppress the row with a typed
  denial. Silent suppression without the typed reason is forbidden.

Onboarding rows NEVER render a system-browser or device-code handoff
without quoting the destination-descriptor's screenshot-safe /
export-safe `browser_handoff_reason` subset
(`external_docs_or_runbook`, `provider_consent_flow`,
`provider_admin_delegation`, `license_or_portal_acceptance`,
`admin_only_surface`, `step_up_required`,
`mutation_not_supported_in_product`). Inventing a local handoff
reason is forbidden.

## Locale fallback rules

The surface NEVER falls back silently. Every fallback carries a typed
`locale_fallback_disclosure_class` and a `locale_rows` entry with the
corresponding `locale_presence_class` and `locale_authoring_state`:

- Requested locale is available and reviewed →
  `locale_presence_class = locale_available_reviewed`,
  `locale_authoring_state = authored_and_reviewed`,
  `locale_fallback_disclosure_class = no_fallback_primary_locale_only`.
- Requested locale is available but machine-assisted / stub / stale
  copy → `locale_presence_class` names the matching
  `locale_available_*` class and the surface renders the typed
  disclosure inline
  (`inline_fallback_disclosure_rendered`).
- Requested locale is not authored → `locale_presence_class =
  locale_missing_fallback_to_primary`, `fallback_locale` pins the
  locale the surface falls back to, and the surface renders the typed
  locale-gap disclosure (`inline_fallback_disclosure_rendered` on
  native chrome, `embedded_boundary_card_disclosure_rendered` inside
  an embedded surface). The repair hook is `switch_to_primary_locale`
  (admit the fallback) or `install_locale_pack` (when a locale pack
  can be installed).
- Requested locale has no content installed → `locale_presence_class =
  locale_missing_not_installed`, `publishable = false`,
  `publishable_denial_reasons` contains `locale_missing_not_installed`,
  `repair_hook_ref.hook_kind = install_locale_pack`.
- Requested locale is suppressed by policy → `locale_presence_class =
  locale_policy_blocked`, `publishable = false`,
  `publishable_denial_reasons` contains `locale_policy_blocked`,
  `repair_hook_ref.hook_kind = request_admin_policy_change`.

Silent locale fallback — rendering primary-locale content while the
surface label still shows the requested locale — is a contract
violation.

## Cached-but-stale and not-installed rules

Onboarding / help surfaces NEVER treat cached, stale, or
not-installed content as current live truth:

- `install_state = cached_snapshot_current` and `freshness_class in
  {authoritative_live, warm_cached}` is the only combination a row
  may render without a typed chip.
- `install_state = cached_snapshot_stale` → the surface renders the
  typed `cached_snapshot_expired` denial with the
  `refresh_freshness` repair hook. `freshness_class` MUST be `stale`
  or `unverified`.
- `install_state = not_installed` → the surface renders the typed
  `pack_not_installed` denial with the
  `import_offline_onboarding_bundle` repair hook (for air-gapped /
  mirror-only packs) or the `refresh_freshness` repair hook (for
  live-fetch packs).
- `install_state = remote_unavailable` → the surface renders the
  typed `remote_owner_unreachable` denial. When a cached snapshot
  exists inside its refresh window, the surface MAY render the
  cached copy with `warm_cached` / `degraded_cached` freshness
  instead; when no cached snapshot satisfies the freshness floor,
  the row suppresses with the `refresh_freshness` repair hook.
- `install_state = mirror_only_verified` and `offline_posture =
  offline_expired_requires_refresh` → the surface renders the typed
  `air_gapped_bundle_expired` denial with the
  `import_offline_onboarding_bundle` repair hook.
- `install_state = policy_blocked` → the surface renders the typed
  `policy_blocked` denial with the `request_admin_policy_change`
  repair hook. Inventing a local "admin-policy restricted" string is
  forbidden.
- `install_state = quarantined` → the surface renders the typed
  `pack_quarantined` denial. Repair hook routes to the publisher /
  support escalation.
- `install_state = withdrawn` → the surface renders the typed
  `pack_withdrawn` denial and routes to the superseding revision if
  one exists.

## Account-optional onboarding guidance

The first-run no-account local-entry contract freezes the account-
optional and account-forced lanes. The onboarding record keeps them
coherent by pinning the admissible `account_prerequisite_class`
values per `pack_role`:

- `first_run_starter_pack` / `glossary_bundle` /
  `account_optional_onboarding_pack` / `air_gapped_offline_onboarding_pack` →
  `account_prerequisite_class = no_account_required` is the only
  admissible value under the local-first guarantee.
- `welcome_tour_pack` / `guided_content_pack` /
  `in_product_help_overlay_pack` / `migration_welcome_pack` → MAY use
  `no_account_required`, `optional_account_context`, or
  `deferrable_account_upgrade` so the surface can invite an opt-in
  account posture without blocking the local lane. MUST NOT use
  `required_authenticated_session` / `required_step_up` /
  `managed_admin_session` unless the pack's canonical owner is
  explicitly account-bound (in which case the pack does not appear in
  the first-run or glossary rotation).

`account_unavailable_in_envelope` suppresses the row with the
`account_required_but_envelope_blocks` denial regardless of
`pack_role`; air-gapped and privacy-reduced envelopes use this to
remove account-bound onboarding entirely.

## Acceptance criteria

The contract is accepted when:

- Onboarding / help surfaces can signal `not_installed`,
  `cached_snapshot_stale`, `remote_unavailable`, `policy_blocked`,
  `quarantined`, and `withdrawn` content without masquerading as
  current live truth. The schema's `allOf` gates force a typed
  `publishable_denial_reasons` entry for every suppressed state.
- Locale gaps (`locale_missing_fallback_to_primary`,
  `locale_missing_not_installed`, `locale_policy_blocked`) are
  explicit rather than implied. The schema's gates force
  `locale_fallback_disclosure_class` off
  `no_fallback_primary_locale_only` whenever a locale-missing class
  is declared, and force a `locale_missing_not_installed` /
  `locale_policy_blocked` denial reason when the matching class is
  declared.
- The contract reuses docs-pack manifest freshness / signing / mirror
  / example vocabulary and destination-descriptor route / trust /
  boundary / handoff vocabulary by reference. No vocabulary is
  re-minted; adding a new enum value is additive-minor and bumps
  `onboarding_pack_state_schema_version`.
- At least one worked fixture exercises each of: local-only starter
  docs, cached-but-stale help, a missing locale pack, and
  account-optional onboarding guidance.

## Change discipline

- Adding a new `pack_role`, `install_state`,
  `locale_presence_class`, `locale_fallback_disclosure_class`,
  `offline_posture`, `browser_handoff_policy`,
  `account_prerequisite_class`, `embedded_route_policy`,
  `locale_authoring_state`, `reset_class`, or
  `publishable_denial_reason` is additive-minor and requires a
  schema-version bump.
- Adding a new repair-hook kind under `repair_hook_ref.hook_kind` is
  additive-minor and requires a schema-version bump; onboarding hooks
  compose with the ADR-0011 repair-hook vocabulary re-exported from
  the docs-pack manifest.
- Repurposing an existing value is breaking and requires a new
  decision row in
  [`artifacts/governance/decision_register.yaml`](../../artifacts/governance/decision_register.yaml)
  plus an ADR if the repurpose changes cross-surface semantics.
- Changes to this document, the schema, and any affected fixtures
  travel in the same change set under the
  [docs / help / migration same-change-set policy](same_changeset_policy.md).
