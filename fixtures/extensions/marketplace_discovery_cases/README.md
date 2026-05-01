# Marketplace discovery example fixtures

These fixtures anchor the vocabulary frozen in
[`/docs/extensions/marketplace_ranking_and_trust_contract.md`](../../../docs/extensions/marketplace_ranking_and_trust_contract.md)
and validated by the discovery boundary schemas at
[`/schemas/extensions/discovery_ranking_reason.schema.json`](../../../schemas/extensions/discovery_ranking_reason.schema.json)
and
[`/schemas/extensions/discovery_badge.schema.json`](../../../schemas/extensions/discovery_badge.schema.json),
together with the anti-abuse register at
[`/artifacts/extensions/anti_abuse_states.yaml`](../../../artifacts/extensions/anti_abuse_states.yaml).

The marketplace contract is at `Status: Proposed`. These fixtures
exercise the reserved field sets, enumerated vocabularies, and
schema `allOf` gates so the later marketplace UX, install-review
sheet, permission inspector, runtime-status pill, and support-export
lanes can be built against one contract rather than invent
marketplace-shaped fields ad hoc.

**Scope rules**

- Each fixture validates against
  `schemas/extensions/discovery_ranking_reason.schema.json` as a
  `discovery_result_row`.
- A fixture MUST exercise at least one frozen
  `registry_source_class`, `channel_class`,
  `approval_state_class`, `trust_claim_source`,
  `trust_badge_inheritance_rule`, `compatibility_claim_class`,
  `runtime_cost_class`, `runtime_cost_evidence_class`,
  `bridge_state_class`, `revocation_snapshot_age_class`,
  `ranking_reason_chip_class`, `ranking_floor_class`, or
  `discovery_position_class` and MUST name the contract section
  that motivates it.
- Raw artifact bytes, raw signing-key material, raw attestation-
  bundle bytes, raw URLs, raw repository paths, raw publisher-
  private data, and raw popularity counts MUST NOT appear; refs
  stand in for every field that would otherwise carry raw material.
- Ids, refs, aliases, and monotonic timestamps are opaque; they
  are chosen to read well rather than to reflect any real
  deployment.

**Index**

| Fixture                                                                                                          | Discovery position                                       | Key classes exercised                                                                                                                                          | Contract section                                              |
|------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------|
| [`public_package_promoted_first_party.yaml`](./public_package_promoted_first_party.yaml)                         | `promoted_safe_maintained_compatible_low_cost`           | `public_registry` / `inherits_origin_tier` / `compatible_on_all_declared_targets` / `runtime_cost_low_nominal` / `activation_evidence_packet_present`           | Default ranking-floor order; Discovery-position class         |
| [`private_registry_org_managed_approved.yaml`](./private_registry_org_managed_approved.yaml)                     | `elevated_compatibility_match`                           | `private_registry` / `capped_at_organisational_on_private_registry` / `managed_approved` / `policy_pinned` / `managed_only_install`                             | Default ranking-floor order; Ranking-reason chips             |
| [`typosquat_candidate_warning_demoted.yaml`](./typosquat_candidate_warning_demoted.yaml)                         | `demoted_warning_visible`                                | `typosquatting_candidate` / `typosquat_candidate_advisory` anti-abuse state / `anti_abuse_warning_raised` / `suppressed_in_ranking` / `publisher_unverified`     | Anti-abuse register; Disclosure invariants                    |
| [`revoked_artifact_removed_from_ranking.yaml`](./revoked_artifact_removed_from_ranking.yaml)                     | `not_in_ranking_review_only`                             | `revoked` / `artifact_revoked_blocking` anti-abuse state / `denial_no_install_path` / `block_install_no_recovery` / `removed_from_ranking`                       | Discovery-position class; Warnings                            |
| [`bridge_state_compatibility_bridge_required.yaml`](./bridge_state_compatibility_bridge_required.yaml)           | `standard_compatible_match`                              | `compatibility_bridge_required` / `bridge_required_compatibility_bridge_profile` / `bridge_state_required` chip                                                  | Ranking-reason chips; Badges; Bridge-state class              |
