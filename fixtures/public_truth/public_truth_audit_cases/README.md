# Public-truth audit-case fixtures

Worked fixtures for the public-truth audit packet, the public-drift
ledger, and the closure-SLA contract frozen in
[`/docs/public_truth/help_about_service_health_audit_packet.md`](../../../docs/public_truth/help_about_service_health_audit_packet.md)
and the boundary schema
[`/schemas/public_truth/public_drift_item.schema.json`](../../../schemas/public_truth/public_drift_item.schema.json).

Each case ships as a complete `public_drift_item_record` covering
one of the typed audited surfaces, mismatch categories, severity
classes, closure SLA classes, escalation states, ledger states, and
closure actions. The fixtures exercise the typed window-kind,
audience, redaction-profile, audited-surface-class,
mismatch-category, severity, narrowing-path, closure-SLA,
escalation-state, escalation-owner-role, ledger-state, closure-action,
linked-artifact-family, and consuming-surface-parity vocabularies plus
the schema-enforced pairings.

## Index

| Case | Fixture | Posture |
| --- | --- | --- |
| About-pane overclaim, release-blocking | `open_help_about_overclaim_release_blocking.yaml` | `release_train_window`, `about_pane`, `projection_broader_than_owner` × `release_blocking_overclaim`, `time_boxed_next_channel_move`, `within_sla`, `open_blocking_release` |
| Service-health stale proof, time-boxed 24h | `open_service_health_stale_time_boxed.yaml` | `weekly_governance_review_window`, `service_health_pane`, `proof_packet_out_of_sync` × `time_boxed_truth_defect`, `time_boxed_24h`, `sla_warning`, `open_blocking_widening` |
| Release-notes drops `version_skew_alias` caveat | `open_release_notes_known_limit_dropped_same_change_blocker.yaml` | `release_train_window`, `release_notes_card`, `known_limit_missing` × `same_change_blocker`, `same_change_set_required`, `within_sla`, `open_blocking_release`, support-handoff audience |
| Public-proof packet aged past SLA, widening frozen | `breached_public_proof_widening_frozen_under_waiver.yaml` | `release_train_window`, `public_proof_packet`, `proof_packet_out_of_sync` × `release_blocking_overclaim`, `time_boxed_next_channel_move`, `sla_breached_widening_frozen`, `open_blocking_widening`, public-proof-safe audience, held under active waiver |
| About-pane drift closed via claim-row narrowing | `closed_via_claim_narrowing_about_pane.yaml` | `ad_hoc_review_window`, `about_pane`, `projection_broader_than_owner` × `release_blocking_overclaim`, `claim_narrowed_no_sla`, `closed_via_claim_narrowing`, enterprise-audit audience |
| Known-limits owner correction in same change set | `closed_via_owner_correction_known_limits.yaml` | `weekly_governance_review_window`, `known_limits_section`, `owner_row_missing` × `same_change_blocker`, `same_change_set_required`, `closed_via_owner_correction`, engineering-internal audience |
| Help-pane drift closed via late-copy narrowing | `closed_via_late_copy_help_pane_overclaim.yaml` | `release_train_window`, `help_pane`, `projection_broader_than_owner` × `release_blocking_overclaim`, `time_boxed_next_channel_move`, `closed_via_public_copy_narrowing` via `public_copy_narrowed_via_late_copy_packet`, release-readiness audience |

## Intended usage

- **First-class governed object conformance.** Every fixture carries
  a stable `drift_item_id`, a stable `audit_packet_ref`, a typed
  `audit_packet_window`, a typed `audited_surface_class`, the typed
  mismatch-category × severity pair, the typed `narrowing_path_class`,
  the typed closure-SLA / escalation-state / escalation-owner-role
  trio, the typed ledger-state / closure-action pair, the typed
  audience and redaction-profile pair, the typed
  `linked_artifact_families` block, and the typed
  `consuming_surface_parity` floor. A surface that renders the drift
  item as a free-text status note is non-conforming.
- **Audience and redaction conformance.** The schema enforces that
  `audience_class` matches `redaction_profile_class` per the
  contract's pairing table. The fixtures cover engineering-internal,
  support-handoff, release-readiness, enterprise-audit, and
  public-proof-safe audiences with the matching redaction profiles.
- **Category × severity pair conformance.** The schema enforces the
  `mismatch_category_class` × `severity_class` pair table from
  [`drift_blocking_rules.md`](../../../docs/governance/drift_blocking_rules.md).
  The fixtures exercise `owner_row_missing` × `same_change_blocker`,
  `projection_broader_than_owner` × `release_blocking_overclaim`,
  `known_limit_missing` × `same_change_blocker`, and
  `proof_packet_out_of_sync` × both admissible severities
  (`time_boxed_truth_defect` and `release_blocking_overclaim`).
- **Closure SLA conformance.** The schema enforces that
  `same_change_set_required`, `time_boxed_24h`, and
  `time_boxed_next_channel_move` require non-null `sla_due_at`, and
  that `internal_only_no_sla` and `claim_narrowed_no_sla` pair with
  `sla_due_at = null`. The closed-via-claim-narrowing fixture
  demonstrates the `claim_narrowed_no_sla` path.
- **Escalation-state conformance.** The schema enforces that
  `sla_breached_release_blocking` requires a
  release-blocking severity and that `sla_breached_widening_frozen`
  requires `channel_widening_blocked = true`. The
  breached-widening-frozen fixture exercises the latter under an
  active waiver register entry.
- **Closure-action conformance.** The schema enforces:
  - `owner_artifact_updated` pairs with
    `closed_via_owner_correction`;
  - `public_copy_narrowed_in_release` pairs with
    `closed_via_public_copy_narrowing`;
  - `public_copy_narrowed_via_late_copy_packet` requires non-empty
    `linked_late_copy_packet_refs[]` and pairs with
    `closed_via_public_copy_narrowing`;
  - `claim_row_effective_posture_narrowed` requires
    `narrowing_path_class = claim_row_narrowing` and pairs with
    `closed_via_claim_narrowing`;
  - `claim_row_retired_from_public_lane` requires
    `narrowing_path_class = public_claim_retirement` and pairs with
    `closed_via_claim_narrowing`;
  - `held_under_active_waiver` requires non-empty
    `linked_waiver_register_refs[]`.
- **Surface-pairing conformance.** The schema enforces that
  `help_pane`, `about_pane`, and `service_health_pane` cite at
  least one `destination_descriptor_row_refs` entry; that
  `release_notes_card` cites at least one
  `whats_new_card_row_refs` entry; that `help_pane` additionally
  cites at least one `help_pane_state_refs` entry; that
  `public_proof_packet` cites at least one
  `public_proof_row_refs` entry; and that `support_export_card`
  cites at least one `support_export_packet_refs` entry.
- **Export-parity conformance.** Every fixture sets the
  `consuming_surface_parity` booleans honestly. The
  release-blocking-overclaim and late-copy fixtures render on every
  surface; the support-handoff release-notes fixture suppresses
  `render_on_public_proof_index`; the public-proof-safe widening-
  frozen fixture suppresses `render_on_support_export`.

## Acceptance coverage

The acceptance criteria from
[`/.plans/M00-523.md`](../../../.plans/M00-523.md) are covered as
follows:

- **"Public-truth drift older than the defined SLA is visible and can
  block stronger claims or wider support posture."** — the
  breached-widening-frozen fixture pins `escalation_state_class =
  sla_breached_widening_frozen` after the SLA pinned to the
  2026-04-23 channel-move timestamp expired; channel widening is
  frozen and the fixture is held open under an active waiver register
  entry.
- **"Help/About/service-health alignment can be audited mechanically
  against current evidence instead of by manual memory."** — every
  fixture pins the canonical owner artifact ref, the affected claim
  row refs, the affected known-limit refs, and the affected evidence
  refs against the same `claim_manifest_baseline_ref` so a reviewer
  can resolve the alignment without re-screenshotting.
- **"The drift ledger preserves exact affected claim rows and closure
  actions for later audits."** — every fixture sets
  `affected_claim_row_refs[]` non-empty (schema-enforced),
  `closure_action_class` typed, and the closed fixtures additionally
  set `closed_at`, `closure_notes`, and the linked change-set / late-
  copy / waiver refs the closure consumed.

## Out of scope

Documentation publishing, service-status tooling, live drift-detection
backends, and automated surface scraping are explicitly out of scope
per [`.plans/M00-523.md`](../../../.plans/M00-523.md). The contract
defines the source object; integrations consume it.
