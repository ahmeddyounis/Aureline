# Proof packet: M1 schema-registry seed for telemetry, diagnostics, support-export, and usage-export payloads

Purpose: anchor proof captures for the unattended M1 lane that
validates the focused M1 schema-registry seed against:

- the envelope schema and row schema under
  [`schemas/registry/`](../../../schemas/registry/);
- the canonical consent-ledger registry the rows inherit broader
  posture from
  ([`artifacts/governance/consent_ledger_seed.yaml`](../../../artifacts/governance/consent_ledger_seed.yaml));
- each row's pinned family schema (telemetry, diagnostics,
  support-bundle manifest, usage-export packet); and
- each row's canonical envelope example under
  [`fixtures/schemas/m1_registry_examples/`](../../../fixtures/schemas/m1_registry_examples/).

Reviewer entry point:
[`/docs/governance/schema_registry_seed.md`](../../../docs/governance/schema_registry_seed.md).

## Canonical sources

- [`/schemas/registry/schema_registry.yaml`](../../../schemas/registry/schema_registry.yaml)
  — the seeded M1 registry rows.
- [`/schemas/registry/schema_registry.schema.json`](../../../schemas/registry/schema_registry.schema.json)
  — boundary schema for the seed envelope.
- [`/schemas/registry/schema_registry_row.schema.json`](../../../schemas/registry/schema_registry_row.schema.json)
  — boundary schema for one M1 row. Adding a new family_class,
  lifecycle_state_class, consent_class, default_posture_class,
  endpoint_class, redaction_class, or registry_consumer_class is
  additive-minor and bumps `schema_registry_row_schema_version`;
  repurposing an existing value is breaking.
- [`/fixtures/schemas/m1_registry_examples/`](../../../fixtures/schemas/m1_registry_examples/)
  — one canonical envelope example per family
  (`telemetry_payload.json`, `diagnostic_payload.json`,
  `support_export_payload.json`, `usage_export_payload.json`).
- [`/artifacts/governance/consent_ledger_seed.yaml`](../../../artifacts/governance/consent_ledger_seed.yaml)
  — canonical consent-ledger seed the M1 rows inherit from via
  `consent_ledger_entry_id_ref`. The validation lane re-parses this
  file and asserts that every M1 row's `consent_class` agrees with the
  parent row's `consent_class` in the consent ledger.
- [`/tests/governance/m1_schema_registry_seed_lane/run_m1_schema_registry_seed_lane.py`](../../../tests/governance/m1_schema_registry_seed_lane/run_m1_schema_registry_seed_lane.py)
  — unattended runner that replays every row, asserts schema
  membership, family-prefix invariants, redaction posture, consent
  agreement, schema pin agreement, example-payload agreement,
  compatibility horizon, deprecated-field honesty, and named-consumer
  presence, then emits the durable JSON capture.

## Named runtime consumer

- [`/docs/governance/schema_registry_seed.md`](../../../docs/governance/schema_registry_seed.md)
  — reviewer-facing landing page. Wired as the M1 named consumer
  through every row's `named_runtime_consumer.consumer_ref`. Consumed
  fields include `entry_id`, `schema_ref`, `schema_version_pin`,
  `consent_class`, `default_posture_class`, `endpoint_class`,
  `redaction_class`, `retention_note`, and `deprecated_fields`.

## Live runtime consumers (read-only)

- [`/artifacts/build/build_identity.json`](../../../artifacts/build/build_identity.json)
  — exact-build identity that the capture embeds for cross-artifact
  traceability.

## Validation captures

- [`/artifacts/milestones/m1/captures/schema_registry_seed_validation_capture.json`](../captures/schema_registry_seed_validation_capture.json)

## Refresh policy

Re-run the validation lane after a change to:

- the M1 schema-registry seed (`schemas/registry/schema_registry.yaml`);
- either the envelope or row schemas under `schemas/registry/`;
- any pinned family schema's `$id`, `record_kind`, or integer schema
  version;
- the parent rows in `artifacts/governance/consent_ledger_seed.yaml`
  the M1 rows inherit from;
- the reviewer-facing landing page at
  `docs/governance/schema_registry_seed.md`.

## Closure rule

The lane stays open until the latest capture lands under the governed
proof root and every row reports PASS for:

- closed-vocabulary membership (`family_class`,
  `lifecycle_state_class`, `consent_class`, `default_posture_class`,
  `endpoint_class`, `redaction_class`, `registry_consumer_class`);
- envelope discriminator + version pin
  (`schema_registry.row.record_kind_wrong`,
  `schema_registry.row.schema_version_wrong`);
- entry_id / family_class prefix agreement
  (`schema_registry.row.entry_id_family_prefix_mismatch`);
- redaction posture sanity
  (`schema_registry.redaction_class_relaxed_without_review`);
- consent-ledger agreement
  (`schema_registry.consent_class_disagrees_with_consent_ledger`);
- compatibility horizon
  (`schema_registry.compatibility_horizon_version_invalid`,
  `schema_registry.compatibility_horizon_note_required`);
- deprecated-field honesty
  (`schema_registry.deprecated_field_path_required`,
  `schema_registry.deprecated_field_version_invalid`,
  `schema_registry.deprecated_field_removal_window_note_required`,
  `schema_registry.deprecated_field_downgrade_action_unknown`);
- schema pin agreement
  (`schema_registry.schema_ref_missing`,
  `schema_registry.schema_version_uri_mismatch`,
  `schema_registry.schema_version_pin_invalid`);
- example payload agreement
  (`schema_registry.example_payload_kind_wrong`,
  `schema_registry.example_payload_entry_id_mismatch`,
  `schema_registry.example_payload_schema_ref_mismatch`,
  `schema_registry.example_payload_record_kind_mismatch`,
  `schema_registry.example_payload_schema_version_mismatch`,
  `schema_registry.example_payload_schema_version_pin_mismatch`);
- named-consumer presence
  (`schema_registry.named_runtime_consumer_missing`,
  `schema_registry.named_runtime_consumer_consumer_class_unknown`,
  `schema_registry.named_runtime_consumer_consumed_fields_empty`);
- and the row's named failure drill —

and the four required family classes (`telemetry_payload`,
`diagnostic_payload`, `support_export_payload`,
`usage_export_payload`) are all observed.
