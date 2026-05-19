# Beta: About, source, community-handoff, and open-vs-managed boundary

This document is the M3 beta contract for the public/open-boundary lane. It
governs how About, Help, marketplace, issue/reporting, governance,
contributing, community-discussion, release-notes, upgrade-or-hosted,
sponsorship, troubleshooting, and source-repository surfaces describe
**where** the user can find official source, issue, discussion, governance,
funding/upgrade, and local-only fallback routes — and **what** each surface
costs in terms of identity, network, and data exit.

The contract has three artifacts:

- `schemas/public/about_destination.schema.json` — boundary schema for
  destination rows.
- `schemas/public/capability_boundary_card.schema.json` — boundary schema
  for the capability-boundary cards drawn on each surface.
- `crates/aureline-shell/src/public_truth/` — the Rust projection and
  validator that About, Help, marketplace, and support surfaces consume.

Positive and negative fixtures live in
`fixtures/public/m3/about_and_boundary_truth/`; the Rust test
`crates/aureline-shell/tests/public_truth_about_and_boundary_fixtures.rs`
loads every fixture and asserts the contract still rejects the documented
drift.

## Why one vocabulary

About and Help, the source-repository tile, the marketplace shelf, the
issue/reporting flow, governance panel, contributing guide,
community-discussion link, release-notes card, upgrade-or-hosted CTA,
sponsorship section, troubleshooting panel, and support-export packet all
need the same answer when a reviewer asks "is this destination official,
private, community, or third-party?" Forking a private label per surface
lets one row claim `Official` while another claims `Community` for the same
destination, and lets an upgrade CTA quietly demote a valid local/open path
without ever crossing a typed boundary.

Every surface in scope MUST render destinations using the closed four-class
vocabulary `Official public` / `Official private` / `Community` /
`Third-party / vendor`, and MUST render its capability-boundary card using
the closed posture vocabulary `local_open`, `local_open_account_optional`,
`managed_first_party`, `self_hosted_customer_operated`, `mirrored_offline`,
`premium_hosted`, `third_party_vendor`, `community_operated`. A surface
whose destination trust class or posture cannot be typed denies the row
rather than collapsing to `official_public` / `local_open`.

## Destinations

Each destination row carries seven typed axes:

| Axis | Vocabulary |
|---|---|
| `destination_class` | `official_public`, `official_private`, `community`, `third_party_vendor` |
| `destination_role_class` | source repo, issue tracker, discussion forum, RFC forum, governance charter, contributing guide, security/support intake, status page, docs/help, release notes / release packet, marketplace index, upgrade-or-hosted, sponsorship/funding, community-handoff router, local-only fallback, mirror/archive |
| `route_state_class` | `current`, `redirected`, `archived`, `replaced`, `decommissioned`, `unreachable_probed` |
| `account_requirement_class` | `none`, `optional_for_account_features`, `required_for_view`, `required_for_write`, `required_for_subscribe`, `required_for_premium_hosted` |
| `data_exit_boundary_class` | `no_payload_leaves_product`, `metadata_safe_object_refs`, `proposal_refs_only`, `redacted_support_packet`, `security_payloads_only`, `external_public_browse`, `vendor_or_third_party_outbound` |
| `support_prominence_class` | `troubleshooting_first`, `support_first`, `source_first`, `parity_with_upgrade`, `below_upgrade` |
| `local_only_parity_class` | `account_optional_local_parity`, `local_only_only`, `hosted_only_no_local_fallback`, `mixed_local_optional_account` |

Honesty rules enforced by the schema and the Rust validator:

- **Dead destinations are labeled, not silent.** Routes whose state is
  `redirected`, `archived`, or `replaced` MUST cite an explicit
  `replacement_destination_ref`. `decommissioned` routes MUST cite a
  `local_only_fallback_ref` so a dead link degrades to a discoverable
  successor path instead of silently failing.
- **Third-party / vendor destinations must say so.** `third_party_vendor`
  destinations only resolve to `external_public_browse` or
  `vendor_or_third_party_outbound` data exits.
- **Security intake is private and security-only.** `security_intake`
  destinations carry `security_payloads_only` data exit and an
  `official_public` or `official_private` trust class.
- **Support intake is private and redacted.** `support_intake` destinations
  are `official_private` with `redacted_support_packet` data exit.
- **Support routes never sink below upgrade.** Issue trackers, security
  intake, support intake, source repositories, contributing guides, and
  status pages MUST NOT carry `support_prominence_class = below_upgrade`.
  Upgrade and sponsorship routes are pinned to `parity_with_upgrade` or
  `below_upgrade`.
- **Account-required write or subscribe rows keep a local fallback.** When
  the row participates in a flow whose local use is still account-optional
  (`account_optional_local_parity` or `mixed_local_optional_account`), the
  row MUST cite a `local_only_fallback_ref`.
- **Local-only fallback rows never coerce account.** Their account
  requirement is `none`, parity is `account_optional_local_parity` or
  `local_only_only`, and prominence ranks support / source / trouble-
  shooting above upgrade.
- **Handoff lanes attach a versioned build-context export.** Issue
  trackers, support intake, security intake, community-handoff routers,
  and discussion forums MUST cite at least one `build_context_exports[]`
  entry. The block is versioned and audience-typed, so public issue
  templates, private support intake, private security intake, and
  community discussions all quote the same redaction-safe payload instead
  of a screenshot.

## Capability boundary cards

Each surface in scope renders one boundary card. Closed axes:

| Axis | Vocabulary |
|---|---|
| `surface_class` | About, Help, source-repository panel, issue-reporting panel, discussion-forum panel, governance panel, contributing-guide panel, marketplace panel, release-notes panel, upgrade-or-hosted CTA, sponsorship CTA, troubleshooting panel |
| `posture_class` | local-open, account-optional local-open, managed first-party, self-hosted customer-operated, mirrored offline, premium hosted, third-party vendor, community-operated |
| `identity_requirement_class` | none, optional local account, required account for write/subscribe, required security/support/vendor identity |
| `network_requirement_class` | offline local only, account-free metadata only, account-free browse, authenticated managed/premium plane, vendor or third-party call, community public call |
| `data_boundary_class` | stays on device, metadata-only outbound, redacted outbound, authenticated managed/premium outbound, vendor or third-party outbound, community public |
| `rollback_path_class` | continue local-only, downgrade to local-open, switch to mirrored offline, switch to self-hosted, none disclosed, not applicable |
| `upgrade_honesty_rule_class` | `local_path_visible`, `local_path_hidden_violation` (rejected), `no_local_path_applicable` |

Honesty rules enforced by the schema and the Rust validator:

- **Premium and managed surfaces never hide a valid local path.** Any
  boundary card whose posture is `premium_hosted` or `managed_first_party`
  MUST carry `upgrade_honesty_rule_class = local_path_visible` or
  `no_local_path_applicable`. `local_path_hidden_violation` is rejected.
- **Local-path-visible cards cite the path.** Cards declaring
  `local_path_visible` MUST cite a `continue_local_only_path_ref` and a
  rollback class that resolves to a local path
  (`continue_local_only`, `downgrade_to_local_open`,
  `switch_to_mirrored_offline`, or
  `switch_to_self_hosted_customer_operated`).
- **Local-open surfaces stay local-open.** Cards with `local_open` or
  `local_open_account_optional` posture MUST keep identity, network, and
  data boundary axes inside the account-optional local lane and MUST NOT
  declare parity `hosted_only_no_local_fallback`.
- **Support surfaces keep support routes ahead of upgrade.** Issue
  reporting, troubleshooting, source-repository, and contributing-guide
  panels MUST NOT carry `support_prominence_class = parity_with_upgrade` or
  `below_upgrade`.
- **Upgrade and sponsorship CTAs never outrank support on the same
  surface.** Upgrade-or-hosted and sponsorship CTAs are pinned to
  `parity_with_upgrade` or `below_upgrade`.
- **Third-party vendor surfaces disclose their network call honestly.**
  `third_party_vendor` posture requires `vendor_or_third_party_call` or
  `community_public_call` network requirement and
  `vendor_or_third_party_outbound` or `community_public` data boundary.

## Page projection

`AboutAndBoundaryTruthPage` bundles the destinations and boundary cards
rendered together on About, Help, or any other lane that mixes
destinations and cards. The page cross-validator enforces:

- destination ids and card ids are unique within the page;
- every `linked_destination_refs[]` entry on a card resolves to a
  destination on the same page;
- every `continue_local_only_path_ref` resolves to a destination on the
  same page;
- every `replacement_destination_ref` and `local_only_fallback_ref`
  resolves to a destination on the same page.

The page also renders a deterministic plaintext block that support exports,
release-evidence packets, governance reviews, and claim-manifest reviews
all quote without inventing their own ordering or vocabulary.

## What is out of scope in M3

This contract does not build a pricing system, a funding backend, or a
hosted subscription control plane. The upgrade-or-hosted destination row
and CTA card are descriptive only — they declare posture and honesty rule,
they do not provision premium services. M3 owns the boundary, not the
billing.
