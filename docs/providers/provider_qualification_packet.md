# Provider qualification packet

This document is the narrative companion to:

- [`/artifacts/providers/provider_support_matrix.yaml`](../../artifacts/providers/provider_support_matrix.yaml)
- [`/artifacts/qa/provider_handoff_parity_suite.yaml`](../../artifacts/qa/provider_handoff_parity_suite.yaml)
- [`/fixtures/providers/provider_qualification_cases/`](../../fixtures/providers/provider_qualification_cases/)

It seeds one canonical packet that turns provider capability claims
into typed, exportable evidence. A provider class is no longer
"integrated" by virtue of one happy-path screenshot. Every claim is
pinned to a typed matrix row, a typed parity drill, a typed worked
fixture, and a typed support-export channel — so a partial path can
ship honestly without collapsing the whole provider class into one
support claim.

If this document and either YAML disagree, the YAML wins for tooling
and this document updates in the same change.

## What this packet freezes

- one **provider support matrix** row set per
  `(provider_class, object_class)` pair, each carrying typed
  inspect / mutate / browser-only / local-draft / publish-later /
  callback / mirror-or-self-host postures and a `claim_strength`
  level;
- one **acting-identity qualification drill** per frozen actor class
  (`human_account`, `installation_or_app_grant`,
  `delegated_user_token`, `project_scoped_grant`,
  `policy_injected_service_identity`) so badge label, decision row,
  audit-event payload, and support-export row never collapse five
  identities into one "Connected" label;
- one **browser-handoff parity drill** set proving source context,
  return anchor, privacy note, and degraded alternative survive
  desktop → browser → desktop on every row that claims
  browser-only or browser-some posture;
- one **packet-field drill** set covering revoked grant, delayed
  delivery, host mismatch, partial import, and mirror-or-self-host
  routing so support-class wording can stay narrower than the
  evidence when the broader provider-class claim is not justified;
- one **fixture corpus** under
  `/fixtures/providers/provider_qualification_cases/` that every
  matrix row and every drill cites, so reviewers and support
  exporters read the same typed scenarios.

This packet does **not** certify a live provider, run a live
interoperability suite, or stand up a live qualification harness.
It freezes the matrix, the drills, and the fixture corpus that
future qualifications, RC packets, support exports, and admin
reconciliations will cite.

## Sources reused (do not redefine)

- [`/docs/providers/provider_mode_contract.md`](./provider_mode_contract.md)
  — frozen mutation modes, callback envelope, publish-later queue,
  account-mapping, and provider-object-relation vocabulary.
- [`/docs/providers/provider_link_header_and_handoff_contract.md`](./provider_link_header_and_handoff_contract.md)
  — frozen provider-linked header, browser-handoff sheet,
  return-anchor classes, and degraded-alternative records.
- [`/docs/providers/connected_account_registry_contract.md`](./connected_account_registry_contract.md)
  — frozen actor classes, acting-identity badge / label classes,
  and effective-scope resolution decisions.
- [`/docs/providers/provider_sync_health_contract.md`](./provider_sync_health_contract.md)
  — frozen sync-health current-mode classes, failure classes,
  cursor-state classes, degraded-import classes, and
  support-export-channel classes.
- [`/docs/providers/deferred_publish_queue_contract.md`](./deferred_publish_queue_contract.md)
  — typed deferred-publish queue items, stale-target review, and
  reauth / re-scope continuity rules.
- [`/docs/providers/provider_conflict_review_contract.md`](./provider_conflict_review_contract.md)
  — typed conflict-review record set used when partial import or
  host mismatch produces drift.
- [`/schemas/integration/browser_handoff_packet.schema.json`](../../schemas/integration/browser_handoff_packet.schema.json)
  and
  [`/schemas/integration/approval_ticket.schema.json`](../../schemas/integration/approval_ticket.schema.json)
  — frozen packet, ticket, audit-event, and reason-class
  vocabularies.

## Shared rules

| Rule | Required truth |
|---|---|
| Claims point at typed evidence | A provider-capability claim cites a `row_id` from the support matrix and at least one fixture under `/fixtures/providers/provider_qualification_cases/` once `claim_strength` is `fixture_seeded` or higher. |
| Per-object honesty | Inspect-only, local-draft-only, publish-capable, and browser-only behavior are named per `(provider_class, object_class)` row. A whole provider class is never collapsed into one support claim when its rows differ. |
| Acting identity is typed | Acting-identity badge label, the effective-scope decision row, and the audit-event payload all carry the same `actor_class`. A generic "Connected" label is forbidden. |
| Browser handoff is round-trip parity | Source context, return anchor, privacy note, and degraded alternative are visible on both the desktop → browser leg and the browser → desktop leg. |
| Packet fields are narrower than the slogan | Revoked grant, delayed delivery, host mismatch, partial import, and mirror-or-self-host routing each export typed fields so support copy can narrow the claim when the evidence does not justify the broader one. |
| No hidden fallback | Silent retry, silent downgrade, silent host-mismatch acceptance, silent partial-as-full rendering, and silent mirror-as-authoritative rendering are forbidden. |

## Provider support matrix

Authoritative rows live in
[`/artifacts/providers/provider_support_matrix.yaml`](../../artifacts/providers/provider_support_matrix.yaml).
The summary below names each seeded row and the postures it
declares; the YAML is the source of truth.

| Row id | Inspect | Mutate | Browser-only | Local draft | Publish-later | Callback / webhook | Mirror / self-host |
|---|---|---|---|---|---|---|---|
| `review_or_code_host__pull_request` | `full_typed_snapshot` | `publish_now_supported` | `browser_only_for_some_actions` | `supported` | `supported` | `required_for_authoritative_state` | `self_host_supported_with_host_match` |
| `review_or_code_host__check_run` | `full_typed_snapshot` | `not_supported` | `browser_only_for_some_actions` | `not_applicable` | `not_applicable` | `required_for_authoritative_state` | `self_host_supported_with_host_match` |
| `review_or_code_host__release_artifact` | `bounded_partial_snapshot` | `rides_release_lane` | `browser_only_for_some_actions` | `not_applicable` | `parked_publish_now_only` | `optional_freshness_only` | `self_host_supported_with_host_match` |
| `issue_or_planning_tracker__issue_or_work_item` | `full_typed_snapshot` | `publish_now_supported` | `browser_only_for_some_actions` | `supported` | `supported` | `required_for_authoritative_state` | `self_host_supported_with_host_match` |
| `ci_or_check_provider__check_run` | `full_typed_snapshot` | `deferred_publish_only` | `browser_only_for_some_actions` | `not_applicable` | `supported` | `required_for_authoritative_state` | `self_host_supported_with_host_match` |
| `docs_or_portal_provider__docs_page` | `full_typed_snapshot` | `deferred_publish_only` | `browser_only_for_some_actions` | `supported` | `supported` | `optional_freshness_only` | `self_host_supported_with_host_match` |
| `artifact_registry_or_package_registry__package_version` | `full_typed_snapshot` | `rides_release_lane` | `browser_only_for_some_actions` | `not_applicable` | `parked_publish_now_only` | `required_for_authoritative_state` | `self_host_supported_with_host_match` |
| `artifact_registry_or_package_registry__registry_entry` | `bounded_partial_snapshot` | `not_supported` | `browser_only_full_object` | `not_applicable` | `not_applicable` | `optional_freshness_only` | `self_host_supported_with_host_match` |
| `release_publisher_provider__release_artifact` | `full_typed_snapshot` | `deferred_publish_only` | `browser_only_for_some_actions` | `not_applicable` | `supported` | `required_for_authoritative_state` | `self_host_supported_with_host_match` |
| `identity_or_enterprise_provider__principal_subject` | `bounded_partial_snapshot` | `not_supported` | `browser_only_full_object` | `not_applicable` | `not_applicable` | `optional_freshness_only` | `host_pinned_authoritative_only` |
| `identity_or_enterprise_provider__consent_flow` | `bounded_partial_snapshot` | `not_supported` | `browser_only_full_object` | `not_applicable` | `not_applicable` | `not_supported` | `host_pinned_authoritative_only` |
| `identity_or_enterprise_provider__install_target` | `bounded_partial_snapshot` | `not_supported` | `browser_only_full_object` | `not_applicable` | `not_applicable` | `optional_freshness_only` | `host_pinned_authoritative_only` |
| `ai_provider__other` | `mirror_derived_only` | `local_draft_only` | `browser_only_for_some_actions` | `supported` | `not_supported` | `polling_only` | `mirror_supported_with_provenance` |
| `managed_admin_provider__admin_surface` | `bounded_partial_snapshot` | `not_supported` | `browser_only_full_object` | `not_applicable` | `not_applicable` | `optional_freshness_only` | `host_pinned_authoritative_only` |
| `managed_admin_provider__audit_entry` | `full_typed_snapshot` | `not_supported` | `full_in_product_alternative` | `not_applicable` | `not_applicable` | `optional_freshness_only` | `self_host_supported_with_host_match` |
| `callback_or_event_provider__other` | `bounded_partial_snapshot` | `not_supported` | `not_applicable` | `not_applicable` | `not_applicable` | `required_for_authoritative_state` | `self_host_supported_with_host_match` |

## Acting-identity qualification drills

Every actor class the registry supports has its own drill. The
drill exists so the badge label, the effective-scope decision row,
the audit-event payload, the desktop notification, and the
support-export row all carry the same typed `actor_class` — five
identities never collapse into one generic "Connected" label.

| Drill id | Actor class | Label class | Parity expectation |
|---|---|---|---|
| `acting_identity_human_account_qualification` | `human_account` | `you_label` | `badge_label_matches_actor_on_every_surface` |
| `acting_identity_installation_or_app_grant_qualification` | `installation_or_app_grant` | `install_label` | `badge_label_matches_actor_on_every_surface` |
| `acting_identity_delegated_user_token_qualification` | `delegated_user_token` | `delegated_label` | `badge_label_matches_actor_on_every_surface` |
| `acting_identity_project_scoped_grant_qualification` | `project_scoped_grant` | `project_scoped_grant_label` | `badge_label_matches_actor_on_every_surface` |
| `acting_identity_policy_injected_service_qualification` | `policy_injected_service_identity` | `policy_injected_service_label` | `badge_label_matches_actor_on_every_surface` |

`unknown_actor_class` is bound by the same schema rules and is
exercised in
[`/fixtures/providers/connected_account_cases/unknown_actor_class_denied_repair_required.yaml`](../../fixtures/providers/connected_account_cases/unknown_actor_class_denied_repair_required.yaml);
it is denied with a typed repair label and is intentionally not a
marketed actor class.

## Browser-handoff parity drills

Browser handoff parity is a round-trip claim: source context,
return anchor, privacy note, and degraded alternative MUST survive
both legs.

| Drill id | Return anchor | Parity expectation |
|---|---|---|
| `browser_handoff_parity_review_anchor_round_trip` | `review_anchor` | `browser_to_desktop_returns_to_anchor_with_privacy_note` |
| `browser_handoff_parity_object_link_anchor_round_trip` | `object_link_anchor` | `browser_to_desktop_returns_to_anchor_with_privacy_note` |
| `browser_handoff_parity_browser_unavailable_degraded_alternative` | `object_link_anchor` | `degraded_alternative_visible_when_browser_unavailable` |

Both round-trip drills exercise the desktop → browser leg and the
browser → desktop leg. The unavailable-browser drill exercises the
desktop → browser leg with a typed `local_or_cached_alternative`
record so a managed-workstation user is never silently retried
into the system browser.

## Packet-field drills

These five drills define the typed fields a support-export packet
MUST carry so a support class can stay narrower than the broader
provider-class claim when the evidence does not justify it.

| Drill id | Required typed fields | Channels |
|---|---|---|
| `revoked_grant_packet_field_qualification` | `revoked_grant_subject`, `revoked_grant_cause_class`, `revoked_grant_repair_action_class` | `support_bundle`, `object_handoff_packet` |
| `delayed_delivery_packet_field_qualification` | `delayed_delivery_event_ref`, `delayed_delivery_failure_class`, `delayed_delivery_retry_state_class` | `support_bundle`, `audit_packet` |
| `host_mismatch_packet_field_qualification` | `host_mismatch_observed_host`, `host_mismatch_expected_host`, `host_mismatch_repair_action_class` | `support_bundle`, `admin_assisted_handoff_packet` |
| `partial_import_packet_field_qualification` | `partial_import_object_class`, `partial_import_missing_count`, `partial_import_freshness_floor_ref` | `support_bundle`, `audit_packet`, `object_handoff_packet` |
| `mirror_or_self_host_routing_packet_field_qualification` | `mirror_or_self_host_routing_class`, `mirror_or_self_host_provenance_summary`, `support_class_narrowing_summary` | `support_bundle`, `migration_note`, `admin_assisted_handoff_packet` |

Each packet-field drill is exercised in a worked qualification
fixture; the support export and the audit packet read the same
typed fields.

## Disclosure rules for partial or unsupported paths

A provider class with sibling object classes that differ in
mutate posture MUST publish each row honestly. The packet rules:

1. A row whose `in_product_mutate_support_class` is `not_supported`
   on a class where another row is `publish_now_supported` MUST
   appear in the qualification packet's narrower-than-class section
   so support copy can narrow the claim.
2. A row whose `mirror_or_self_host_class` is
   `mirror_supported_with_provenance` MUST tag every imported
   overlay with `degraded_import_class = mirror_derived` and MUST
   NOT render the snapshot as authoritative.
3. A row whose `browser_only_posture_class` is
   `browser_only_full_object` MUST cite the typed
   `browser_handoff_sheet_record`, the
   `local_or_cached_alternative_record`, and a sample fixture in
   the qualification corpus. A raw URL launch is forbidden on
   protected surfaces.
4. A row whose `claim_strength` is `design_contract_only` MUST
   NOT appear in public capability marketing without an explicit
   capability narrowing in this packet.

## Cadence

| Cadence | Required truth |
|---|---|
| Targeted change validation | Any change touching provider-linked surfaces, callback envelopes, publish-later semantics, browser-handoff routing, or acting-identity badges runs the affected matrix rows and the affected parity-suite drills before claim wording widens. |
| Release candidate validation | Every matrix row at `qualification_drill_passed` or higher revalidates against its qualification fixture and its parity-suite drill; rows still at `design_contract_only` may not appear in public capability marketing without an explicit capability narrowing here. |

## Where this packet is read

- Provider capability claims (marketing, README, changelog) cite a
  `row_id` from the support matrix and a `drill_id` from the parity
  suite. Generic "integrated with provider X" wording is rejected.
- Support exports cite the typed packet-field drill rows and reuse
  the typed `support_export_channel_class` for the export channel.
- Admin reconciliations and provider-conflict reviews cite the
  same matrix rows and parity drills so the reviewer sees the same
  vocabulary the user saw.
- Future provider adapters land their qualification artifacts as
  fixtures under `/fixtures/providers/provider_qualification_cases/`
  and bump the row's `claim_strength` accordingly.
