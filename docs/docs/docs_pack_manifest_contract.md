# Docs-pack manifest contract

This document freezes the docs-pack manifest contract every canonical
owner named in [ADR 0013](../adr/0013-docs-help-service-health-truth.md)
publishes and every docs pane, docs browser, Help / About footer,
service-health row, support summary, onboarding step, and AI-explanation
overlay reads before resolving a `help_status_badge_record`. The
machine-readable boundary is
[`/schemas/docs/docs_pack_manifest.schema.json`](../../schemas/docs/docs_pack_manifest.schema.json);
worked examples (fresh, partially stale, offline / mirrored, mixed-locale,
newer-than-client, and non-publishable) live in
[`/fixtures/docs/docs_pack_examples/`](../../fixtures/docs/docs_pack_examples/).

The eventual docs-help-service-health crate's Rust types are the schema
of record. This document and the JSON Schema export are the cross-tool
boundary every non-owning surface reads; if this document and the ADR
disagree, [ADR 0013](../adr/0013-docs-help-service-health-truth.md)
wins and this document MUST be updated in the same change.

## Why freeze this now

ADR 0013 froze the `source_revision_ref` slot on every
`help_status_badge_record` and the `signature_unverified` /
`mirror_continuity_broken` / `pack_quarantined` degraded-state causes.
What the ADR reserved but did not implement was the governed artifact
those refs resolve into: the docs-pack manifest. Left implicit, each
pack kind (project docs, generated reference, mirrored upstream docs,
curated knowledge packs, support runbooks) would invent its own
version string, its own locale disclosure, its own signing metadata,
and its own rule for "can this pack render on a user surface", and the
parity audit between Help, About, docs panes, the service-health view,
and the support summary would degenerate into a hand-mapping exercise
between five manifest dialects.

The manifest is the missing piece that lets:

- the docs pane render a typed `source_class` / `version_match_state` /
  `freshness_class` chip without re-deriving the mapping per pack kind,
- the docs browser enumerate available locales honestly on mixed-locale
  packs instead of pretending every locale is complete,
- the support summary quote the pack's signing and lineage fields
  verbatim instead of re-minting them, and
- the AI explanation overlay reject a pack whose citation posture is
  unmet rather than silently falling back to a generic answer.

## Scope

Frozen at this revision:

- One `docs_pack_manifest_record` shape with a closed set of source
  classes, publisher classes, signature-status states, mirror-chain
  statuses, acquired-via values, locale-coverage classes,
  example-label classes, stale-example reasons, citation postures,
  backlink postures, publishable states, and publishable-blocking
  reasons.
- One `docs_pack_example_record` shape so parity audits and support
  exports can enumerate examples across packs without holding a full
  manifest.
- Rules for stale-example downgrade behaviour, newer-than-client
  rendering, missing-citation denial, and the closed list of reasons
  that make a pack non-publishable.
- Source, version, locale, freshness, client-scope, citation /
  backlink, stable / stale labeling, and offline / mirror lineage
  field sets that align with ADR 0013 so a surface can project a
  `help_status_badge_record` field-for-field without interpretation
  drift.

Out of scope until a superseding decision row opens:

- The docs-browser UI (search, navigation, footer). ADR 0013 and this
  document reserve the contract; the UI lands later.
- The docs-pack publishing pipeline (build, sign, distribute, mirror
  fetch). This document pins the manifest the pipeline emits; the
  pipeline itself is a later lane.
- The citation / symbol-reference packet body. ADR 0013 reserved
  `citation_anchor_record`; this document pins `citation_posture`,
  `backlink_posture`, and `required_citation_anchor_kinds` on the
  manifest so the packet lane has a stable target.
- AI explanation infrastructure beyond the citation posture the
  manifest declares.

## Canonical ownership (re-export from ADR 0013)

Each docs-pack source class has exactly one canonical owner; surfaces
that render a row owned by another family MUST quote the owner's
`help_status_badge_record` rather than mint a copy.

| Source class              | Canonical owner                     |
|---------------------------|-------------------------------------|
| `project_docs`            | `docs_pack_registry`                |
| `generated_reference`     | `generated_reference_index`         |
| `mirrored_official_docs`  | `mirrored_docs_index`               |
| `curated_knowledge_pack`  | `curated_knowledge_pack_registry`   |
| `support_runbook`         | `support_runbook_registry`          |

`vendor_provider_docs`, `derived_explanation`, and
`external_status_feed` from the ADR-0013 source-class vocabulary are
deliberately not admissible on this manifest: provider docs ride the
ADR-0010 connected-provider record as an `inspect_only` overlay,
derived explanations are session-scoped and cite into packs rather
than ship as packs, and external status feeds are live feeds that
never pass through a pack manifest.

## Record fields

The full field set lives in
[`/schemas/docs/docs_pack_manifest.schema.json`](../../schemas/docs/docs_pack_manifest.schema.json).
The notable fields are:

- **Identity.** `pack_id` is the stable id the registry uses across
  revisions; `pack_revision_ref` is the opaque pin a
  `help_status_badge_record.source_revision_ref` points to.
- **Source and publisher.** `source_class` (see above);
  `publisher_class` is one of `first_party_project`,
  `permitted_publisher`, `official_upstream_mirror`,
  `operator_curated`, `support_pipeline`.
- **Version and compat.** `display_version` renders to users;
  `semver_version` drives the `version_match_state` computation
  against the running build's exact-build identity;
  `compat_window_semver` declares the window inside which the
  pack is authoritative. A pack that targets a build above the
  running client's ceiling (that is, newer than the client)
  resolves to `version_match_state = incompatible_drift_detected`
  and surfaces render the typed chip plus an
  `upgrade_release_channel` or `refresh_freshness` repair hook;
  silent render as authoritative is forbidden.
- **Signing.** `signing.signature_status` is the gate. Only
  `signed_and_verified` clears the publishable gate; every other
  state is a publishable blocker and surfaces render the
  `signature_unverified` degraded-state cause.
- **Mirror lineage.** `mirror_lineage` is required on every manifest.
  Packs that do not mirror upstream set
  `mirror_chain_status = not_applicable` and leave the other
  fields null. Mirrored packs MUST carry
  `mirror_of_pack_id`, `upstream_revision_ref`, and a
  `predecessor_pack_revision_ref` for every continuous revision
  after the first. `predecessor_missing` and
  `signing_chain_broken` are publishable blockers and surface as
  the ADR-0013 `mirror_continuity_broken` degraded-state cause.
  `acquired_via` covers `online_fetch`, `signed_bundle_import`,
  `air_gapped_media`, `local_build_output`, and
  `host_process_internal`; `offline_expiration_at` pins the
  monotonic deadline after which an offline / air-gapped
  acquisition MUST be treated as stale regardless of
  `declared_freshness_class`.
- **Locales.** `primary_locale` is always a BCP-47 tag;
  `available_locales` is the full set (including the primary);
  `locale_coverage` is a per-locale row naming one of `complete`,
  `partial`, `machine_assisted`, `stub`, `stale_copy`. Mixed-locale
  packs MUST render per-locale disclosures on the primary surface
  (tooltip-only disclosure is forbidden for `partial`,
  `machine_assisted`, and `stale_copy`).
- **Freshness and client scope.** `declared_freshness_class` and
  `client_scopes` are re-exported from ADR 0011 / ADR 0013 without
  modification. `refresh_window_seconds` pins the window the
  registry uses to decide whether the pack may continue to
  declare `authoritative_live` / `warm_cached`;
  `last_refreshed_at` is the monotonic timestamp of the last
  refresh against the canonical owner.
- **Citation posture.** `citation_posture` is one of
  `citation_required`, `citation_recommended`,
  `citation_not_required`. When `citation_required` is set,
  `required_citation_anchor_kinds` MUST be non-empty; surfaces
  that cannot attach at least one anchor of a declared kind deny
  render with `derived_explanation_uncited` (for AI overlays) or
  `required_citation_anchors_missing` (for publishable packs).
- **Backlink posture.** `backlink_posture` is one of
  `backlink_resolvable`, `backlink_deferred`,
  `backlink_not_required`. `backlink_deferred` is admissible for
  offline / mirrored packs whose canonical owner is currently
  unreachable but whose anchors will round-trip when the next
  refresh succeeds; parity audits treat deferred backlinks as
  valid but lower freshness. `backlink_unresolvable` is a
  publishable blocker.
- **Example summary.** `example_summary` splits the pack's
  examples into `stable`, `stale`, `needs_review`, and
  `quarantined` with a `stale_examples_exceed_threshold`
  boolean. See "Stale-example downgrade behaviour" below.
- **Publishable gate.** `publishable_state` is one of
  `publishable`, `draft`, `blocked`, `withdrawn`, `quarantined`.
  `publishable_blocking_reasons` is a closed enum. A pack in
  `blocked` state MUST carry at least one reason and a
  `repair_hook_ref`; a pack in `publishable` state MUST carry
  zero reasons.
- **Supersession.** `supersedes_pack_revision_ref` pins the
  prior revision this manifest replaces. Withdrawn manifests
  still resolve for support bundles and evidence packets.
- **Policy context and redaction.** `policy_context`
  (`policy_epoch`, `trust_state`, `execution_context_id`) and
  `redaction_class` are re-exported from ADR 0001 / ADR 0007 /
  ADR 0008 / ADR 0009 / ADR 0011 without modification.

## Source / version / freshness visibility

ADR 0013 froze the parity-audit contract: `surface_class`,
`source_class`, `version_match_state`, `running_build_identity_ref`,
`freshness_class`, `client_scopes`, `degraded_state_cause`,
`browser_handoff_reason`, `external_status_feed`,
`vendor_overrides_project`, `policy_context`, and `redaction_class`
are separately addressable fields on every emitted
`help_status_badge_record`. The manifest is the record those fields
project from:

- The chip on a docs-pane row renders `source_class`,
  `version_match_state`, and `freshness_class` from the manifest
  (and, when a pack is stale or drifted, the typed
  `degraded_state_cause` + `repair_hook_ref` from the manifest's
  blocking reason or from a surface-side computation).
- The Help / About footer and the docs-browser footer quote the
  manifest's `display_version`, `publisher_class`, `source_class`,
  and `declared_freshness_class` rather than re-deriving them.
- The support summary enumerates `pack_id`, `pack_revision_ref`,
  `source_class`, `publisher_class`, `signature_status`,
  `mirror_chain_status`, `primary_locale`, `available_locales`,
  and the full `publishable_blocking_reasons` list under the
  support-export redaction envelope.

Chip collapsing is a UI freedom, not a record-shape freedom. Surfaces
that collapse source / version / freshness into one visual chip MUST
retain each axis as a separately addressable field on the record they
emit so a later parity audit can compare peers mechanically.

## Pack newer than the local client

When a pack targets a build above the running client's compat ceiling
(that is, the pack was published against a newer binary than the one
the user is running):

- `version_match_state` resolves to `incompatible_drift_detected`.
- The pack's `publishable_state` may still be `publishable` at the
  publisher, but the rendering surface MUST NOT render it as
  authoritative; it renders the typed `version_mismatch`
  degraded-state cause from ADR 0013 with a
  `repair_hook_ref.hook_kind` of either `upgrade_release_channel`
  (when the user can move to a newer channel) or
  `refresh_freshness` (when the registry can fetch a back-compat
  pack revision).
- About / Help denies render and routes to a repair hook if it
  cannot resolve a pack whose compat window includes the running
  build; silent placeholder is forbidden.

This mirrors the ADR-0013 rule that `incompatible_drift_detected`
and `unknown_target_build` rows MUST render with a typed
`degraded_state_cause` and a `repair_hook_ref`; silent fallback to
"row available" is forbidden.

## Missing required citations

When `citation_posture = citation_required` and the running build's
canonical owner cannot resolve at least one anchor from a
`required_citation_anchor_kinds` entry for a given claim:

- The pack is non-publishable with the
  `required_citation_anchors_missing` blocking reason. The
  publisher fails the publish with a typed denial the pipeline
  logs on the `docs_help_service_health` audit stream.
- If a surface has already fetched the manifest and the anchor
  resolution fails at render time (the pack shipped with the
  anchor-kind declaration but the body is unreachable), the
  surface denies render with
  `derived_explanation_uncited` for AI overlays and with
  `required_citation_anchors_missing` for protected docs panes.
  Silent fallback to a generic answer is forbidden.

AI explanation overlays that consume a `citation_required` pack
MUST carry a non-empty `citation_anchor_refs` array on their
`help_status_badge_record`; an overlay with an empty array against
a `citation_required` pack is denied with
`derived_explanation_uncited` per ADR 0013.

## Stale-example downgrade behaviour

Every example a pack ships resolves to exactly one `example_label_class`
in the manifest's `example_summary`:

- `stable_example` is the publishable default. No disclosure is
  required; surfaces render the example without a chip.
- `stale_example` carries a typed `stale_reason` (one of
  `deprecated_target_symbol`, `removed_target_symbol`,
  `renamed_target_symbol`, `lifecycle_state_moved`,
  `setting_default_changed`, `behavior_changed`,
  `version_mismatch`, `client_scope_no_longer_supported`,
  `policy_blocked_in_context`). Surfaces MUST render the typed
  disclosure on the primary surface alongside the example; a
  surface that hides the disclosure in a tooltip while the
  example reads as stable is non-conforming. When
  `superseding_example_id` is set, the disclosure MUST offer a
  `migrate_to_replacement` repair hook routing the user to the
  replacement.
- `needs_review_example` is the same posture as `stale_example`
  with a typed "reviewer has not re-verified" disclosure; the
  example is rendered with the disclosure until a reviewer
  relabels it `stable_example` or `stale_example`.
- `quarantined_example` is hidden from docs panes entirely but
  still enumerated in the manifest so parity audits can count it.

`example_summary.stale_examples_exceed_threshold = true` is a
publishable blocker. Packs whose
(`stale` + `needs_review` + `quarantined`) to total-examples ratio
exceeds the publisher's configured threshold MUST move to
`publishable_state = blocked` with the
`stale_examples_exceed_threshold` reason (or to `draft` /
`withdrawn`). The `declared_freshness_class` on such a pack MUST
NOT declare `authoritative_live`; it declares `degraded_cached`
or `stale` and surfaces render the typed
`degraded_state_cause` + `repair_hook_ref`.

## Offline and mirrored packs

Offline and mirrored packs (including air-gapped enterprise
distributions) are first-class and MUST NOT be rendered as a
fallback for an unreachable online pack:

- `acquired_via` names how the revision reached the running
  instance. `signed_bundle_import` and `air_gapped_media`
  identify offline acquisitions; `online_fetch` identifies a
  live fetch.
- `offline_expiration_at` is the monotonic deadline after which
  the acquisition MUST be treated as stale regardless of
  `declared_freshness_class`; when the deadline passes, the
  pack surfaces the `freshness_floor_unmet` degraded-state cause
  with a `refresh_freshness` repair hook.
- `air_gapped_origin_label` is an operator-supplied short label
  that support exports can surface verbatim (e.g. "enterprise
  distribution 2026 Q2"). It MUST NOT contain raw URLs or raw
  file paths.
- `mirror_chain_status` pins mirror continuity. Every mirror
  revision after the first carries a
  `predecessor_pack_revision_ref`; `predecessor_missing` or
  `signing_chain_broken` is a publishable blocker and surfaces
  the ADR-0013 `mirror_continuity_broken` degraded-state cause.

## When a pack is not publishable

A pack is not publishable — `publishable_state` MUST NOT be
`publishable` — when any of the closed blocking reasons apply.
The reasons below are the full set the registry may emit; adding
a reason is additive-minor and bumps
`docs_pack_manifest_schema_version`.

- `signature_unverified` — the signing gate did not clear
  (`signature_missing`, `signed_but_unverified`, or
  `signature_revoked`).
- `mirror_continuity_broken` — mirror chain is missing a
  predecessor or the signing chain broke.
- `pack_quarantined` — admin policy or the publishing pipeline
  quarantined the pack.
- `source_class_unresolved` — the source class could not be
  resolved (mirror manifest unavailable, generated-reference
  build identity unknown).
- `client_scope_empty` — `client_scopes` would be empty.
- `locale_set_empty` — `available_locales` would be empty.
- `missing_target_build_identity` — the pack could not pin a
  `target_running_build_identity_ref` (About / Help denies
  render on packs missing this field).
- `required_citation_anchors_missing` — `citation_posture =
  citation_required` but at least one required anchor kind could
  not be resolved.
- `stale_examples_exceed_threshold` — the stale-example ratio
  exceeds the publisher-configured threshold.
- `contract_version_unknown` — the registry does not understand
  the pack's declared `docs_pack_manifest_schema_version`.
- `backlink_unresolvable` — the pack declares
  `backlink_resolvable` but the canonical owner cannot resolve at
  least one anchor round-trip at publish time.
- `publisher_not_permitted` — admin policy narrowed the set of
  permitted publishers and this pack's `publisher_class` /
  `publisher_id` falls outside it.
- `policy_blocked` — admin policy is denying this source class on
  this surface or client scope.

Non-publishable packs MUST carry a `repair_hook_ref` pointing into
the ADR-0011 repair-hook vocabulary (typically
`request_admin_policy_change`, `refresh_freshness`,
`upgrade_release_channel`, `contact_support`). Silent rendering as
available is forbidden; denial is typed, visible, auditable, and
repairable.

## Linkage to neighbouring contracts

- **ADR 0013 truth-source vocabulary.** The manifest's
  `source_class`, `version_match_state`, `freshness_class`,
  `client_scopes`, and `redaction_class` are re-exported from
  [`schemas/docs/help_status_badge.schema.json`](../../schemas/docs/help_status_badge.schema.json)
  without modification. The manifest's
  `pack_revision_ref` is exactly the
  `help_status_badge_record.source_revision_ref` value surfaces
  carry.
- **ADR 0011 capability lifecycle.** `freshness_class`,
  `client_scope`, `repair_hook_ref`, and `redaction_class` are
  re-exported from
  [`schemas/governance/capability_lifecycle.schema.json`](../../schemas/governance/capability_lifecycle.schema.json)
  without modification.
- **ADR 0010 browser handoff.** The manifest does not mint
  browser-handoff packets; surfaces that render a pack row and
  need to hand off to a browser quote the ADR-0010
  `browser_handoff_packet` envelope from the subset frozen in
  ADR 0013.
- **ADR 0008 settings resolver.** Admin policy may narrow which
  source classes render on a surface, pin a docs source class
  per surface, raise the freshness floor, force a step-up
  authenticator on a browser handoff from docs / help, or
  quarantine a pack. Policy MAY NOT silently widen beyond the
  frozen rules.
- **D-0011 exact-build identity.** The manifest's
  `target_running_build_identity_ref` names the build identity
  frozen by `D-0011`; the `version_match_state` axis is
  computed against that identity on render.

## Schema of record

Rust types in the eventual docs-help-service-health crate are the
schema of record. The JSON Schema export at
[`/schemas/docs/docs_pack_manifest.schema.json`](../../schemas/docs/docs_pack_manifest.schema.json)
is the cross-tool boundary every non-owning surface reads. Adding
a new source class, publisher class, signature-status state,
mirror-chain-status value, acquired-via value, locale-coverage
class, example-label class, stale-example reason, citation posture,
backlink posture, publishable state, publishable-blocking reason,
or required-anchor-kind entry is additive-minor and bumps
`docs_pack_manifest_schema_version`; repurposing an existing value
is breaking and requires a new decision row.

There is no external IDL or code-generator toolchain at this
milestone; this mirrors ADR 0004 through ADR 0014.

## Source anchors

- [`docs/adr/0013-docs-help-service-health-truth.md`](../adr/0013-docs-help-service-health-truth.md)
  — source-class vocabulary, version-match-state vocabulary,
  degraded-state-cause vocabulary, `help_status_badge_record`
  shape, and the "follow-up: docs-pack manifest lane fills in
  `source_revision_ref`" line this document closes against.
- [`docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md)
  — `freshness_class`, `client_scope`, `repair_hook_ref`,
  `redaction_class` vocabularies re-exported here.
- [`docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md)
  — `browser_handoff_packet` envelope the rendering surface
  quotes when it hands off.
- [`docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`](../adr/0008-settings-definition-and-effective-configuration-resolver.md)
  — "one registry, no shadows" rule mirrored for docs packs.
- [`artifacts/docs/help_badge_vocabulary.yaml`](../../artifacts/docs/help_badge_vocabulary.yaml)
  — worked examples of `help_status_badge_record` that point
  into pack revisions this manifest shape resolves.
- [`.t2/docs/Aureline_Technical_Architecture_Document.md`](../../.t2/docs/Aureline_Technical_Architecture_Document.md),
  [`.t2/docs/Aureline_Technical_Design_Document.md`](../../.t2/docs/Aureline_Technical_Design_Document.md),
  [`.t2/docs/Aureline_PRD.md`](../../.t2/docs/Aureline_PRD.md),
  [`.t2/docs/Aureline_UI_UX_Spec_Document.md`](../../.t2/docs/Aureline_UI_UX_Spec_Document.md)
  — docs-pack governance, source classes, About / Help
  disclosure, docs source visibility, and the no-shadow-docs
  rule.
