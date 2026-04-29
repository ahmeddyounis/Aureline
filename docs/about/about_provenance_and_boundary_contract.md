# About card, provenance, openness boundary, and reproducibility-packet contract

This document is the contract layer that turns the product's
self-description surface into product-bound state. It defines the
single record family every reviewer reads when they ask the same
question: *what build am I running, what proves it, what is open and
what is not, and where do issues actually go?*

The About surface does not invent a second vocabulary for build
identity, channel, signature, attestation, SBOM, mirror, advisory,
license, or community handoff. It projects the records already frozen
elsewhere into one place so a reader does not have to chase scattered
docs lookups.

Companion artifacts:

- [`/schemas/about/about_card.schema.json`](../../schemas/about/about_card.schema.json)
  - boundary schema for `about_card_record`.
- [`/schemas/about/reproducibility_packet.schema.json`](../../schemas/about/reproducibility_packet.schema.json)
  - boundary schema for `reproducibility_packet_record`.
- [`/fixtures/about/about_cases/`](../../fixtures/about/about_cases/)
  - worked cases for an open-source local-independent build, a managed
    build with optional services, a mirrored air-gapped build, and a
    reproducibility-packet export.

Upstream sources this contract projects from rather than restating:

- [`/schemas/build/exact_build_identity.schema.json`](../../schemas/build/exact_build_identity.schema.json)
  and
  [`/docs/build/exact_build_identity_model.md`](../build/exact_build_identity_model.md)
  - exact-build identity vocabulary the card and packet quote when they
    name what build is running and what artifact families resolve to it.
- [`/schemas/governance/provenance_badge.schema.json`](../../schemas/governance/provenance_badge.schema.json)
  and
  [`/docs/governance/provenance_badge_contract.md`](../governance/provenance_badge_contract.md)
  - signature, attestation, license, notice, mirror, freshness, and
    advisory vocabulary the card's `provenance_summary` block reuses.
- [`/schemas/governance/post_install_disclosure.schema.json`](../../schemas/governance/post_install_disclosure.schema.json)
  and
  [`/docs/governance/post_install_notice_and_provenance_contract.md`](../governance/post_install_notice_and_provenance_contract.md)
  - the post-install disclosure the About card links to from its
    `provenance_summary` and access-point rows.
- [`/schemas/security/advisory_record.schema.json`](../../schemas/security/advisory_record.schema.json)
  - advisory-history vocabulary the card and packet quote.
- [`/artifacts/governance/issue_routing.yaml`](../../artifacts/governance/issue_routing.yaml)
  and
  [`/docs/governance/issue_routing_matrix.md`](../governance/issue_routing_matrix.md)
  - route-class, privacy-class, disclosure-class, and redaction-class
    vocabulary the card's `community_handoff` and the packet's
    `public_private_routing` blocks reuse verbatim.
- [`/artifacts/governance/deployment_profiles.yaml`](../../artifacts/governance/deployment_profiles.yaml)
  and
  [`/docs/governance/deployment_profile_truth.md`](../governance/deployment_profile_truth.md)
  - deployment-profile, residency, and key-mode vocabulary the card's
    `openness_boundary` rows quote when they describe open/local,
    self-hosted, managed, mirrored, and optional-service postures.
- [`/schemas/docs/destination_descriptor.schema.json`](../../schemas/docs/destination_descriptor.schema.json)
  and
  [`/docs/docs/help_about_service_health_routes.md`](../docs/help_about_service_health_routes.md)
  - destination-descriptor refs the card's community-handoff and
    support-action rows resolve to.

Normative sources this contract projects from:

- `.t2/docs/Aureline_PRD.md` sections on open-source baseline,
  build-identity disclosure, signed updates, SBOM and notice
  inventory, advisory disclosure, deployment profiles, optional shared
  services, sovereign / air-gapped posture, and community contribution
  paths.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` sections on
  managed-service separation, local-first defaults, mirror or
  offline posture, exact-build publication, and supply-chain evidence.
- `.t2/docs/Aureline_Technical_Design_Document.md` sections on
  build-identity, signature stack, SBOM and reproducibility packs,
  support handoff, and deployment-profile differentiation.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` sections on About,
  provenance, and community-handoff surfaces.

If this document disagrees with those sources, those sources win and
this document, its companion schemas, and fixtures update in the same
change.

## Scope

Frozen here:

- one `about_card_record` shape rendered as the canonical product
  self-description surface inside Help / About, the command palette,
  the settings About pane, diagnostics export, support bundles, and
  release packets;
- one `reproducibility_packet_record` shape for the redaction-safe
  export bundle that links exact-build identity, symbol or SBOM
  availability, provenance bundle refs, public-vs-private issue routing,
  and an export-safe summary;
- a closed `openness_class` vocabulary
  (`open_source_local_independent`, `open_source_self_hosted`,
  `managed_service_with_open_client`, `mirrored_official_offline`,
  `optional_managed_service`, `not_applicable`) bound per row to the
  deployment posture so the About surface cannot silently overstate
  openness or local independence;
- the `community_handoff` block that binds every public/private route
  the card surfaces to the issue-routing matrix vocabulary;
- the `provenance_summary` block that resolves signature, attestation,
  SBOM, advisory, mirror-freshness, and post-install-disclosure refs
  through the same vocabulary the rest of the supply-chain surfaces use;
- the redaction profile every reproducibility packet MUST apply before
  crossing the customer, auditor, regulator, partner, community
  reviewer, or release-evidence boundary; and
- structural invariants that make over-claimed openness wording
  (`fully open source`, `runs entirely locally`, `no managed
  dependency`, `air-gapped`) impossible to render unless the active
  path satisfies the claim now.

Out of scope:

- maintaining community portals, public issue trackers, RFC forums,
  or partner support channels. The About card and reproducibility
  packet expose typed refs into those routes; operating them is owned
  outside this milestone.
- the About surface UI implementation (layout, copy polish, motion).
  This contract freezes the record shape and rendering invariants;
  surface owners design the UI on top.
- the runtime build-identity producer and the signature-verification
  engine. This contract freezes the record those producers publish
  into; how they compute the underlying state is owned upstream.
- raw signatures, raw attestations, raw key material, raw SBOM bodies,
  raw notice text, raw private mirror endpoints, raw advisory payloads,
  raw issue-template bodies, raw URLs, raw policy bodies, raw tenant
  identifiers, raw user identifiers, and raw paths. The card and the
  packet boundary forbid them.

## Surfaces

The About surface is one product surface composed of three record
families:

1. **About card.** The canonical per-build detail view. One
   `about_card_record` per running build identity. Carries the
   build-identity block, the provenance summary, the openness-boundary
   rows, the community-handoff rows, the support actions, the access
   points, the reproducibility-packet refs, and the rendering contract.
2. **Openness-boundary row.** A row inside the card that names one
   posture row (`open_local`, `self_hosted`, `managed_cloud`,
   `mirrored_offline`, `optional_service`), its `openness_class`, its
   `service_dependency_class`, and whether it is currently active for
   the workflow. Every claim-bearing About copy variant is gated on
   the row's resolved values.
3. **Reproducibility packet.** The redaction-safe bundle of one or
   more about cards plus the linked supply-chain evidence, frozen for
   regulator, auditor, customer, partner, community-reviewer,
   support, or release-evidence handoff. The packet is the only
   export form that crosses the Aureline boundary; raw evidence does
   not.

## Build identity

The card's `build_identity` block names what build is running through
the same vocabulary frozen in
[`/docs/build/exact_build_identity_model.md`](../build/exact_build_identity_model.md):

- `product_name_class` — frozen at `aureline` today.
- `channel_class` — `dev_local`, `nightly`, `preview`, `beta`,
  `stable`, `lts`, `hotfix`. Aligned with the exact-build channel
  vocabulary.
- `build_id` — the human-facing build id.
- `running_build_identity_ref` — opaque ref into the runtime build
  identity surface (`/schemas/build/build_identity.schema.json`).
- `exact_build_identity_ref` — opaque ref into the exact-build
  identity record. The card MUST resolve this ref so a reviewer can
  verify the build the user is running matches the build the rest of
  the release evidence was qualified against.
- `tree_state_class`, `signing_class`, and `producer_lane_class`
  refs reuse the exact-build vocabulary verbatim.
- `acquired_via_class` — names the install path the build came in
  through (`official_update_feed`, `enterprise_mirror`,
  `offline_bundle`, `air_gapped_media`, `local_build`,
  `side_loaded`).

A card MUST NOT render an exact-build claim it cannot resolve. When the
running build is a `dev_local` checkout or a `side_loaded` artifact,
the card narrows its claim-bearing wording per the rules in
[Active claim narrowing](#active-claim-narrowing) below.

## Provenance summary

The card's `provenance_summary` block reuses the provenance-badge
vocabulary so the same words mean the same thing on every supply-chain
surface:

- `signature_state`, `attestation_state`, `checksum_state`,
  `revocation_state`, and `revocation_freshness_class` mirror the
  values frozen in
  [`/schemas/governance/provenance_badge.schema.json`](../../schemas/governance/provenance_badge.schema.json).
- `sbom_state` and `sbom_formats` mirror the values frozen in
  [`/schemas/governance/post_install_disclosure.schema.json`](../../schemas/governance/post_install_disclosure.schema.json).
- `provenance_badge_refs` resolves to one or more
  `provenance_badge_record` ids covering the running build.
- `post_install_disclosure_ref` resolves to the post-install
  disclosure for the running build.
- `advisory_history_refs` resolves to the advisory records that
  describe the running build's known-issues posture.
- `mirror_provenance_class` and `mirror_freshness_class` mirror the
  provenance-badge mirror vocabulary; air-gapped and mirrored-offline
  cards MUST set these to `customer_mirror` /
  `offline_bundle_import` and a non-`expired` freshness class.

A card MAY NOT render `signature verified`, `attestation verified`,
`SBOM published`, or `no known advisories` wording when the underlying
ref or state is missing or stale; the rendering contract gates this
structurally (see [Rendering contract](#rendering-contract)).

## Openness boundary

`openness_boundary` is the field that prevents About copy from silently
overstating openness or local independence. Every card MUST populate
one row per row class below, and each row MUST resolve to one
`openness_class` plus a `service_dependency_class`:

| Row class            | What the row describes                                                                  |
|----------------------|-----------------------------------------------------------------------------------------|
| `open_local`         | The local product binary itself: open-source baseline, locally executed, no remote dep. |
| `self_hosted`        | Customer-operated services (registry mirror, identity provider, model gateway) the customer runs themselves under the open-source baseline. |
| `managed_cloud`      | First-party managed services (managed control plane, managed sync, managed AI) the customer opts into. |
| `mirrored_offline`   | Mirrored or offline transport of official artifacts (enterprise mirror, offline bundle, air-gapped media). |
| `optional_service`   | Optional add-on services (telemetry, hosted AI, hosted symbol service, hosted browser handoff) the customer can enable individually. |

Each row carries:

- `openness_class` — one of `open_source_local_independent`,
  `open_source_self_hosted`, `managed_service_with_open_client`,
  `mirrored_official_offline`, `optional_managed_service`,
  `not_applicable`.
- `service_dependency_class` — `none`, `optional`, `required`.
- `active_for_workflow` — `true` when the row's posture is currently
  enabled for the running workflow.
- `boundary_disclosure` — short reviewable sentence describing what is
  open and what is not for the row.
- `evidence_refs` — opaque refs into the deployment-profile artifacts,
  governance disclosures, or release evidence that prove the row's
  resolved values.

The `openness_class` values mean:

| `openness_class`                       | Meaning |
|----------------------------------------|---------|
| `open_source_local_independent`        | The row is fully open-source code that runs locally with no required service dependency. |
| `open_source_self_hosted`              | The row is open-source but requires the customer to operate one or more services themselves. |
| `managed_service_with_open_client`     | The client is open-source; the named service is operated by the vendor and is not part of the open-source baseline. |
| `mirrored_official_offline`            | The row is the official build delivered through a customer-operated mirror or offline bundle. |
| `optional_managed_service`             | Optional managed add-on; not required for product use; explicitly togglable. |
| `not_applicable`                       | The row class does not apply to the active deployment profile. |

A card MAY only render `fully open source` or `runs entirely locally`
copy when the `open_local` row resolves to
`open_source_local_independent` AND every other row either resolves to
`not_applicable` or has `active_for_workflow=false` AND
`service_dependency_class != required`. The schema rejects mismatches
structurally.

## Community handoff

`community_handoff` is the field that prevents About help from sending
sensitive reports to a public lane or burying community reports in a
private one. Every row in `community_handoff` reuses the issue-routing
vocabulary verbatim:

- `route_class` — `public_issue_tracker`, `public_rfc_forum`,
  `private_security_channel`, `private_partner_channel`,
  `private_support_channel`, `benchmark_council_queue`,
  `governance_packet_queue`.
- `privacy_class` — `public`, `private_with_public_advisory`,
  `private_with_public_summary`, `private_partner_only`,
  `private_support_only`.
- `disclosure_class` — `public_immediate`, `public_on_fix`,
  `public_on_advisory`, `public_sanitised_summary_on_fix`,
  `private_indefinite`.
- `public_summary_expectation` — `required`, `recommended`, `none`,
  `forbidden`.
- `redaction_class` — `field_safe_default`,
  `field_safe_with_route_metadata`,
  `security_redaction_raw_allowed_under_pgp`,
  `partner_contractual_redaction`, `support_bundle_redaction_profile`,
  `no_raw_attachments`.
- `destination_descriptor_ref` — opaque ref into a
  `destination_descriptor_record` that names the destination's trust
  class, owner class, boundary class, and disclosure mode.
- `intended_for_classes` — closed list of issue-class tokens (for
  example `oss_bug`, `perf_regression`, `security_issue`,
  `supportability_escalation`, `benchmark_dispute`, `docs_truth_defect`,
  `governance_truth_defect`, `partner_case`).
- `disclosure` — short reviewable sentence describing what may be
  shared on this route and what belongs elsewhere.

A card MUST contain at least one row in each of these route-class
subsets so a reader cannot leave the surface without a public lane and
a private lane:

1. one `public_issue_tracker` or `public_rfc_forum` row;
2. one `private_security_channel` row; and
3. one of `private_support_channel` or `private_partner_channel`.

The schema's `allOf` gates encode these structurally.

## Active claim narrowing

A card's claim-bearing copy MUST narrow when any of the following
fires; the schema rejects a card that declares
`active_openness_claim` of `fully_open_local`, `fully_open_self_hosted`,
or `mirrored_official_only` while these triggers are active:

- `dev_local_or_unsigned_build_running` — `channel_class=dev_local` or
  `signature_state` is not `signed_verified`.
- `optional_service_required_active` — at least one `optional_service`
  row has `service_dependency_class=required` and
  `active_for_workflow=true`.
- `managed_service_active` — at least one `managed_cloud` row has
  `active_for_workflow=true` and `service_dependency_class != none`.
- `mirror_revocation_stale` — `mirror_freshness_class` resolves to
  `stale_requires_review` or `expired`.
- `advisory_open_for_active_build` — a referenced advisory record is
  open for the running build and not yet mitigated.
- `provenance_evidence_missing_or_stale` — the resolved
  `post_install_disclosure_ref` reports
  `signature_missing`/`signature_revoked`/`signature_mismatch` or
  `attestation_missing`/`attestation_stale`.

The schema's `allOf` gates encode the narrowing:

1. A card whose `active_openness_claim` is `fully_open_local` MUST
   resolve `open_local` to `open_source_local_independent`, MUST set
   every other row to `not_applicable` or `active_for_workflow=false`,
   MUST resolve `signature_state` to `signed_verified` (when not a
   `dev_local` build), and MUST have an empty
   `active_narrowing_reasons` list.
2. A card whose `active_openness_claim` is `mirrored_official_only`
   MUST resolve the `mirrored_offline` row to
   `mirrored_official_offline` with `active_for_workflow=true` and
   MUST resolve `mirror_freshness_class` to a non-`expired` value.
3. A card whose `active_openness_claim` is `managed_with_optional_services`
   MUST contain at least one `managed_cloud` or `optional_service` row
   with `active_for_workflow=true`.
4. When `active_openness_claim` is narrower than `declared_openness_claim`,
   the card MUST set `auto_narrowed=true` and populate
   `active_narrowing_reasons`.

These gates are why over-claimed openness wording is structurally
impossible to render unless the active path satisfies the claim now.

## Reproducibility packet

The reproducibility packet is the redaction-safe bundle every external
reviewer reads. It carries:

- `packet_class` — `open_local_export`, `managed_export`,
  `mirrored_offline_export`, `regulated_review_export`,
  `community_review_export`, `release_evidence_export`. The class
  drives default redaction and default routing.
- `packet_state` — `draft`, `in_review`, `frozen`, `superseded`, or
  `withdrawn`. Frozen, superseded, and withdrawn packets MUST set
  `frozen_at`.
- `subject` — `about_card_ref`, `exact_build_identity_ref`,
  `product_name_class`, `channel_class`, and the running build id.
- `symbol_availability` — `symbol_class`, `symbol_refs`, and an
  `omission_reason` when symbols are not bundled.
- `sbom_availability` — `sbom_state`, `sbom_formats`, and `sbom_refs`.
- `provenance_bundle` — `provenance_badge_refs`,
  `signed_attestation_refs`, `signature_evidence_refs`,
  `mirror_or_offline_receipt_refs`, and `advisory_history_refs`.
- `public_private_routing` — `public_routes` and `private_routes`
  arrays drawn from the issue-routing vocabulary, the
  `default_route_for_packet_class`, and the
  `public_summary_expectation_class`.
- `export_safe_summary` — short export-safe sentences plus
  `included_field_classes` and `excluded_field_classes` enumerations.
- `redaction` — `redaction_class`, `redaction_rules_ref`, and a
  closed `redacted_field_classes` list.
- `handoff` — `recipient_class`, `recipient_label`,
  `delivery_channel_class`, `signed_at`, `signing_evidence_ref`.
- `freshness` — `captured_at`, `stale_after`, `freshness_state`,
  `next_review_target`.
- `limitations` — explicit limitations of the packet (for example
  "this packet asserts no FedRAMP authorization").

External-class packets (`community_review_export`,
`regulated_review_export`, `release_evidence_export`,
`managed_export`) MUST apply a redaction class narrower than
`internal_only`. Mirrored-offline packets MUST include at least one
`mirror_or_offline_receipt_refs` entry. Open-local packets MUST resolve
the subject's About card to a card whose `active_openness_claim` is
`fully_open_local` or `fully_open_self_hosted`.

`redacted_field_classes` is the only allowed enumeration of redacted
content. A packet may not invent ad-hoc redaction labels; readers can
reason about what is and is not in the packet from the closed list.

## Linkage rules

- A card's `build_identity.exact_build_identity_ref` MUST resolve to a
  record conforming to
  [`schemas/build/exact_build_identity.schema.json`](../../schemas/build/exact_build_identity.schema.json).
- A card's `provenance_summary.post_install_disclosure_ref` MUST
  resolve to a record conforming to
  [`schemas/governance/post_install_disclosure.schema.json`](../../schemas/governance/post_install_disclosure.schema.json).
- A card's `provenance_summary.provenance_badge_refs` MUST resolve to
  records conforming to
  [`schemas/governance/provenance_badge.schema.json`](../../schemas/governance/provenance_badge.schema.json).
- A card's `provenance_summary.advisory_history_refs` MUST resolve to
  records conforming to
  [`schemas/security/advisory_record.schema.json`](../../schemas/security/advisory_record.schema.json).
- A card's `community_handoff[].destination_descriptor_ref` MUST
  resolve to a record conforming to
  [`schemas/docs/destination_descriptor.schema.json`](../../schemas/docs/destination_descriptor.schema.json).
- A card's `community_handoff[].route_class`, `privacy_class`,
  `disclosure_class`, `public_summary_expectation`, and
  `redaction_class` MUST resolve to values frozen in
  [`/artifacts/governance/issue_routing.yaml`](../../artifacts/governance/issue_routing.yaml).
- A card's `openness_boundary[].evidence_refs` SHOULD include at least
  one ref into
  [`/artifacts/governance/deployment_profiles.yaml`](../../artifacts/governance/deployment_profiles.yaml)
  or the corresponding governance disclosure.
- A reproducibility packet's `subject.about_card_ref` MUST resolve to
  a card emitted under this contract.
- A reproducibility packet's `subject.exact_build_identity_ref` MUST
  match the resolved card's
  `build_identity.exact_build_identity_ref`.

## Rendering contract

Every card carries a `rendering_contract` block stating which fields
MUST be visible on every About surface that renders the card:

- `build_identity_visible` — build id, channel, exact-build ref, and
  acquired-via cue.
- `provenance_summary_visible` — signature, attestation, SBOM,
  advisory, and revocation cues.
- `openness_boundary_visible` — every populated row, with its
  `openness_class`, `service_dependency_class`, and
  `active_for_workflow` cue.
- `community_handoff_visible` — every populated route row, with the
  public/private cue and the destination-descriptor disclosure.
- `support_actions_visible` — every populated support action.
- `reproducibility_packet_visible` — at least one
  reproducibility-packet ref when the active build qualifies for
  export.
- `auto_narrowed_visible` — when `auto_narrowed=true`, the narrowed
  copy and the `active_narrowing_reasons` list MUST be visible.

Surfaces MAY add layout, ordering, copy polish, and motion on top, but
they MUST NOT hide a field that the rendering contract marks visible.

## Out of contract

- The eventual About-surface UI implementation. This contract freezes
  the record shape; surface owners design the UI on top.
- The signature-verification stack itself. The card carries the state;
  the signing stack is owned in the build / release contracts.
- The community-portal operator processes (issue tracker rules,
  RFC review cadence, partner-channel SLAs, security-disclosure
  cadence). The card and packet expose typed refs into those routes;
  operating them is owned in the governance and security domains.
- Pricing, commercial availability, or partner naming. The card freezes
  the openness and managed-service shape; concrete commercial language
  is owned outside this milestone.
- Final regulated launch wording. Concrete copy remains anchored to the
  upstream assurance-claim row's `canonical_copy` and the post-install
  disclosure's `visible_cues`; this contract freezes the shape that
  carries them.
