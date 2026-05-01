# Publisher-lifecycle and registry-parity example fixtures

These fixtures anchor the vocabulary frozen in
[`/docs/extensions/publisher_lifecycle_and_registry_parity_contract.md`](../../../docs/extensions/publisher_lifecycle_and_registry_parity_contract.md)
and validated by the boundary schemas at
[`/schemas/extensions/publisher_lifecycle_event.schema.json`](../../../schemas/extensions/publisher_lifecycle_event.schema.json)
and
[`/schemas/extensions/registry_parity.schema.json`](../../../schemas/extensions/registry_parity.schema.json).

The publisher-lifecycle and registry-parity contract is at
`Status: Proposed`. These fixtures exercise the reserved field sets,
enumerated vocabularies, and schema `allOf` gates so the later
discovery, install-review sheet, installed-row notification, support-
export, claim-manifest, and moderation review surfaces can be built
against one contract rather than invent lifecycle- or parity-shaped
fields ad hoc.

**Scope rules**

- Each fixture is a multi-document YAML file. Document one is fixture
  metadata; documents two and three validate against
  `schemas/extensions/publisher_lifecycle_event.schema.json` (a
  `publisher_lifecycle_event_record` and an
  `installed_package_lifecycle_notification_record`); document four
  validates against
  `schemas/extensions/registry_parity.schema.json` (a
  `registry_parity_assertion_record`).
- A fixture MUST exercise at least one frozen
  `lifecycle_event_class`, `lifecycle_actor_class`,
  `lifecycle_reason_class`, `lifecycle_effect_on_trust_badge_class`,
  `lifecycle_effect_on_compatibility_badge_class`,
  `lifecycle_effect_on_runtime_cost_badge_class`,
  `lifecycle_effect_on_anti_abuse_state_class`,
  `installed_package_notification_disposition_class`,
  `installed_package_repair_affordance_class`,
  `lifecycle_denial_reason_class`, `parity_axis_class`,
  `parity_state_class`, or `parity_violation_consequence_class` and
  MUST name the contract section that motivates it.
- Raw artifact bytes, raw signing-key material, raw attestation-
  bundle bytes, raw URLs, raw repository paths, raw publisher-private
  data, raw email addresses, and raw popularity counts MUST NOT
  appear; refs stand in for every field that would otherwise carry
  raw material.
- Ids, refs, aliases, and monotonic timestamps are opaque; they are
  chosen to read well rather than to reflect any real deployment.

**Index**

| Fixture                                                                                              | Lifecycle event                              | Installed-row disposition                              | Parity overall state                                | Contract section                                                                                                |
|------------------------------------------------------------------------------------------------------|----------------------------------------------|--------------------------------------------------------|-----------------------------------------------------|-----------------------------------------------------------------------------------------------------------------|
| [`verified_publisher_transfer.yaml`](./verified_publisher_transfer.yaml)                             | `publisher_transfer_completed`               | `notify_with_required_review_pending`                  | `parity_held_full_vocabulary_match`                 | Publisher transfer or revocation changes trust state visibly across discovery, installed rows, and support/export. |
| [`abandoned_publisher.yaml`](./abandoned_publisher.yaml)                                             | `publisher_abandonment_observed`             | `notify_with_optional_update_offered`                  | `parity_held_with_capped_inheritance`               | Private-registry packages do not bypass trust vocabulary or lifecycle disclosure merely because they are internal. |
| [`mirror_promoted_package.yaml`](./mirror_promoted_package.yaml)                                     | `mirror_promotion_admitted`                  | `notify_only_no_action_required`                       | `parity_held_with_capped_inheritance`               | Fixtures cover verified publisher transfer, abandoned publisher, mirror-promoted package, quarantine, and restored publisher. |
| [`quarantine_engaged.yaml`](./quarantine_engaged.yaml)                                               | `publisher_quarantine_engaged`               | `notify_with_install_disabled_pending_review`          | `parity_held_full_vocabulary_match`                 | Publisher transfer or revocation changes trust state visibly across discovery, installed rows, and support/export. |
| [`restored_publisher_after_review.yaml`](./restored_publisher_after_review.yaml)                     | `publisher_restoration_after_review`         | `notify_with_required_update_pending`                  | `parity_held_full_vocabulary_match`                 | Fixtures cover verified publisher transfer, abandoned publisher, mirror-promoted package, quarantine, and restored publisher. |
