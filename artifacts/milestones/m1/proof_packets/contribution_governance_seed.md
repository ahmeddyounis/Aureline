# Proof packet: M1 contribution-governance seed for signoff/DCO, license metadata, third-party import records, public-interface versioning, deprecation-packet template, and repo-hygiene scaffolding

Purpose: anchor proof captures for the unattended M1 lane that
validates the focused M1 contribution-governance seed against:

- the envelope schema and row schema under
  [`schemas/governance/`](../../../schemas/governance/);
- each row's canonical artifact (resolved on disk and scanned for the
  declared `canonical_artifact_marker` literal substring);
- each row's supporting artifacts;
- the canonical envelope examples under
  [`fixtures/governance/m1_contribution_governance_examples/`](../../../fixtures/governance/m1_contribution_governance_examples/); and
- the named runtime consumer.

Reviewer entry point:
[`/docs/governance/contribution_and_signoff.md`](../../../docs/governance/contribution_and_signoff.md).

## Canonical sources

- [`/artifacts/governance/contribution_governance_seed.yaml`](../../../artifacts/governance/contribution_governance_seed.yaml)
  — the seeded contribution-governance rows.
- [`/schemas/governance/contribution_governance_seed.schema.json`](../../../schemas/governance/contribution_governance_seed.schema.json)
  — boundary schema for the seed envelope.
- [`/schemas/governance/contribution_governance_seed_row.schema.json`](../../../schemas/governance/contribution_governance_seed_row.schema.json)
  — boundary schema for one M1 contribution-governance row. Adding a
  new value to `control_class`, `artifact_kind_class`,
  `enforcement_class`, `lifecycle_state_class`, or
  `contribution_governance_consumer_class` is additive-minor and bumps
  `contribution_governance_seed_row_schema_version`; repurposing an
  existing value is breaking.
- [`/fixtures/governance/m1_contribution_governance_examples/`](../../../fixtures/governance/m1_contribution_governance_examples/)
  — one canonical envelope example per control family
  (`signoff_dco.json`, `license_metadata.json`,
  `third_party_import_record.json`,
  `public_interface_versioning.json`,
  `deprecation_packet_template.json`,
  `repo_hygiene_scaffold.json`).
- [`/artifacts/governance/import_record_seed.yaml`](../../../artifacts/governance/import_record_seed.yaml)
  — M1 import-record seed format that release-evidence packs,
  attribution notices, and SBOM tooling can reuse. Cited by the
  `third_party_import_record` row.
- [`/docs/governance/public_interface_versioning_policy.md`](../../../docs/governance/public_interface_versioning_policy.md)
  — first public-interface versioning and deprecation policy. Cited
  by the `public_interface_versioning` row.
- [`/docs/governance/deprecation_packet_template.md`](../../../docs/governance/deprecation_packet_template.md)
  — canonical deprecation-packet template. Cited by the
  `deprecation_packet_template` row.
- [`/docs/governance/repo_hygiene_scaffolding.md`](../../../docs/governance/repo_hygiene_scaffolding.md)
  — repo-hygiene scaffolding contract. Cited by the
  `repo_hygiene_scaffold` row.
- [`/tests/governance/m1_contribution_governance_seed_lane/run_m1_contribution_governance_seed_lane.py`](../../../tests/governance/m1_contribution_governance_seed_lane/run_m1_contribution_governance_seed_lane.py)
  — unattended runner that replays every row, asserts schema
  membership, control-id-prefix invariants, closed-map agreement,
  canonical-artifact marker presence, supporting-artifact resolution,
  named-consumer presence, and example-payload agreement, then emits
  the durable JSON capture.

## Named runtime consumer

- [`/docs/governance/contribution_and_signoff.md`](../../../docs/governance/contribution_and_signoff.md)
  — reviewer-facing landing page. Wired as the M1 named consumer
  through every row's `named_runtime_consumer.consumer_ref` except
  the `third_party_import_record` row, which names the M1
  import-record seed as its machine-readable consumer. Consumed
  fields include `control_id`, `canonical_artifact_ref`,
  `control_class`, `artifact_kind_class`, `enforcement_class`, and
  `lifecycle_state_class`.

## Live runtime consumers (read-only)

- [`/artifacts/build/build_identity.json`](../../../artifacts/build/build_identity.json)
  — exact-build identity that the capture embeds for cross-artifact
  traceability.

## Validation captures

- [`/artifacts/milestones/m1/captures/contribution_governance_seed_validation_capture.json`](../captures/contribution_governance_seed_validation_capture.json)

## Refresh policy

Re-run the validation lane after a change to:

- the M1 contribution-governance seed
  (`artifacts/governance/contribution_governance_seed.yaml`);
- either the envelope or row schemas under `schemas/governance/`;
- any canonical artifact a row points at via `canonical_artifact_ref`;
- any canonical example payload under
  `fixtures/governance/m1_contribution_governance_examples/`;
- the reviewer-facing landing page at
  `docs/governance/contribution_and_signoff.md`.

## Closure rule

The lane stays open until the latest capture lands under the governed
proof root and every row reports PASS for:

- closed-vocabulary membership (`control_class`,
  `artifact_kind_class`, `enforcement_class`,
  `lifecycle_state_class`,
  `contribution_governance_consumer_class`);
- envelope discriminator + version pin
  (`contribution_governance.row.record_kind_wrong`,
  `contribution_governance.row.schema_version_wrong`);
- control_id / control_class prefix agreement
  (`contribution_governance.row.control_id_prefix_mismatch`);
- closed-map agreement
  (`contribution_governance.artifact_kind_class_disagrees_with_control_class`,
  `contribution_governance.enforcement_class_disagrees_with_control_class`);
- canonical-artifact marker presence
  (`contribution_governance.canonical_artifact_ref_missing`,
  `contribution_governance.canonical_artifact_marker_missing`);
- supporting-artifact resolution
  (`contribution_governance.supporting_artifact_ref_missing`);
- example-payload agreement
  (`contribution_governance.example_payload_kind_wrong`,
  `contribution_governance.example_payload_control_id_mismatch`,
  `contribution_governance.example_payload_pinned_canonical_artifact_ref_mismatch`,
  `contribution_governance.example_payload_control_class_mismatch`,
  `contribution_governance.example_payload_artifact_kind_class_mismatch`,
  `contribution_governance.example_payload_enforcement_class_mismatch`,
  `contribution_governance.example_payload_lifecycle_state_class_mismatch`);
- named-consumer presence
  (`contribution_governance.named_runtime_consumer_missing`,
  `contribution_governance.named_runtime_consumer_consumer_class_unknown`,
  `contribution_governance.named_runtime_consumer_consumed_fields_empty`);
- and the row's named failure drill —

and the six required control classes (`signoff_dco`,
`license_metadata`, `third_party_import_record`,
`public_interface_versioning`, `deprecation_packet_template`,
`repo_hygiene_scaffold`) are all observed.
