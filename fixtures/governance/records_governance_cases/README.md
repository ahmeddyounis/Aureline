# Records-governance packet fixtures

Worked fixtures for the records-governance packet, retention-hold
delta summary, and offboarding-caveat contract frozen in
[`/docs/governance/records_governance_packet_contract.md`](../../../docs/governance/records_governance_packet_contract.md)
and the boundary schema
[`/schemas/governance/records_governance_packet.schema.json`](../../../schemas/governance/records_governance_packet.schema.json).

Each case ships as a complete `records_governance_packet_record`
covering one of the typed packet windows, audiences, and change-
significance postures. The fixtures exercise the typed window-kind,
audience, redaction-profile, record-class registry diff,
retention-and-legal-hold policy diff, export/delete contract status,
offboarding evidence links, open held-data caveat, linked artifact
families, change-significance, and consuming-surface-parity
vocabularies plus the schema-enforced pairings.

## Index

| Case | Fixture | Posture |
| --- | --- | --- |
| Informational milestone-close baseline | `informational_milestone_close_baseline.yaml` | `milestone_close_window`, `engineering_internal`, no diff entries, `informational` |
| Release-bearing managed copy added | `release_bearing_managed_copy_added.yaml` | `release_train_window`, `release_readiness`, crash-diagnostic managed-copy and 90-day floor, `release_bearing` |
| Claim narrowing under AI broker parity | `claim_narrowing_ai_evidence_managed_retention.yaml` | `ad_hoc_review_window`, `public_proof_safe`, AI-retained-evidence retention floor, `claim_narrowing` |
| Three open caveats — legal hold, backlog, unreachable import | `multiple_open_caveats_legal_hold_and_backlog.yaml` | `weekly_governance_review_window`, `support_handoff`, `release_bearing`, three typed caveats |

## Intended usage

- **First-class governed object conformance.** Every fixture carries a
  stable `packet_id`, a typed `window_kind_class`, a typed
  `audience_class` paired with a typed `redaction_profile_class`, the
  full five-section diff body (record-class registry, retention and
  legal-hold policy, export/delete contract status, offboarding
  evidence links, open held-data caveats), the typed
  `change_significance_summary`, and the typed
  `consuming_surface_parity` floor. A surface that renders the packet
  as a free-text status note is non-conforming.
- **Audience and redaction conformance.** The schema enforces that
  `audience_class` matches `redaction_profile_class` per the
  contract's pairing table. The four fixtures cover
  engineering-internal, release-readiness, public-proof-safe, and
  support-handoff audiences with the matching redaction profiles.
- **Linked-artifact-family conformance.** A packet whose record-class
  diff touches `crash_diagnostic_payload`, `support_bundle_archive`,
  `collaboration_evidence_packet`, `ai_retained_evidence_packet`,
  `offboarding_exit_packet`, or `destruction_receipt_record` cites at
  least one matching ref in the corresponding family bucket. The
  release-bearing fixture cites the telemetry schema-registry entry,
  the support export packet, and the offboarding exit packet; the
  claim-narrowing fixture cites the AI retained evidence packet and
  the offboarding exit packet; the open-caveats fixture cites the
  support export packet and the collaboration evidence packet.
- **Change-significance conformance.** The schema enforces that
  `release_bearing` requires a non-null
  `release_bearing_rationale` and a non-empty
  `linked_release_truth_summary_refs`; `claim_narrowing` requires a
  non-empty `claim_narrowing_links[]` array; and `claim_widening_blocked`
  requires a non-null `claim_widening_blocked_decision_ref`.
- **Open-held-data-caveat conformance.** The open-caveats fixture
  pins three typed caveat classes (`legal_hold_active`,
  `provider_backlog`, `import_source_unreachable`) with their
  `affected_matrix_row_refs`, `affected_record_class_refs`, and
  `expected_clear_at` chronology. A surface that filters out a
  caveat under a "review needed" affordance is non-conforming.
- **Export-parity conformance.** Every fixture sets the
  `consuming_surface_parity` booleans honestly. The informational,
  release-bearing, and claim-narrowing fixtures render on every
  surface; the support-handoff fixture suppresses
  `render_on_claim_manifest` and `render_on_public_proof_index`
  per the audience-redaction pairing.

## Acceptance coverage

The acceptance criteria from
[`/.plans/M00-519.md`](../../../.plans/M00-519.md) are covered as
follows:

- **"Release or milestone readiness can state exactly what changed in
  governed record classes and what caveats remain."** — every fixture
  carries the typed registry diff, the typed retention-and-hold
  diff, the typed contract status, and the typed open-caveat list as
  one schema-validated record.
- **"Legal hold, retention, delete/export, and offboarding status are
  visible in one packet instead of isolated subsystem notes."** —
  every fixture renders the five typed sections side by side; the
  open-caveats fixture demonstrates that a tenant legal hold, a
  provider backlog, and an unreachable import source coexist on the
  same packet without scattering across appendix paragraphs.
- **"The packet is reusable across public-proof, support, enterprise,
  and ship-room workflows without changing vocabulary."** — the four
  fixtures cover the engineering-internal, release-readiness,
  public-proof-safe, and support-handoff audiences with matching
  redaction profiles. A reviewer can read each packet against the
  same vocabulary regardless of the audience.

## Out of scope

Retention enforcement, deletion backends, legal-hold tooling, and
compliance automation are explicitly out of scope per
[`.plans/M00-519.md`](../../../.plans/M00-519.md). The contract
defines the source object; integrations consume it.
