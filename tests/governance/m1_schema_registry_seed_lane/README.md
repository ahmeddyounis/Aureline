# M1 schema-registry seed validation lane

Unattended proof lane that validates the M1 schema-registry seed at
[`schemas/registry/schema_registry.yaml`](../../../schemas/registry/schema_registry.yaml)
against:

- [`schemas/registry/schema_registry.schema.json`](../../../schemas/registry/schema_registry.schema.json) — envelope schema (vocabularies, required coverage, row list);
- [`schemas/registry/schema_registry_row.schema.json`](../../../schemas/registry/schema_registry_row.schema.json) — row vocabulary;
- [`artifacts/governance/consent_ledger_seed.yaml`](../../../artifacts/governance/consent_ledger_seed.yaml) — parent consent-ledger registry the M1 rows inherit broader posture from;
- each row's pinned family schema (under `schemas/telemetry/`,
  `schemas/diagnostics/`, `schemas/support/`, or
  `schemas/governance/usage_export_record.schema.json`); and
- each row's canonical envelope example under
  [`fixtures/schemas/m1_registry_examples/`](../../../fixtures/schemas/m1_registry_examples/).

The lane is deliberately runnable on CI / nightly without a graphical
display: it only consumes the pure data joined out of the canonical
sources.

## What the lane proves

For every row in the seed the runner asserts:

- **Envelope is canonical** — `record_kind` is `m1_schema_registry_row`
  and `schema_registry_row_schema_version` is `1`.
- **Closed vocabularies match the row schema verbatim** —
  `family_class`, `lifecycle_state_class`, `consent_class`,
  `default_posture_class`, `endpoint_class`, `redaction_class`, and
  `registry_consumer_class` all match the row schema's `$defs`.
- **`entry_id` matches the row's `family_class`** — telemetry rows
  start with `telemetry.`, diagnostics with `diagnostics.`,
  support-export with `support_export.`, usage-export with
  `usage_export.` (`schema_registry.row.entry_id_family_prefix_mismatch`).
- **Consent agreement with the parent consent-ledger row** — the row's
  `consent_class` agrees with the parent row in
  `consent_ledger_seed.yaml`
  (`schema_registry.consent_class_disagrees_with_consent_ledger`).
- **Redaction posture sanity** — `diagnostic_payload` rows MUST default
  to `code_adjacent_redacted_default` or
  `high_risk_explicit_review_required`; relaxing to
  `metadata_safe_default` or `metadata_only_no_payload_bytes` fails
  loudly as `schema_registry.redaction_class_relaxed_without_review`.
- **Schema pin agreement** — the row's `schema_ref` exists on disk,
  the pinned schema's `$id` agrees with the row's
  `schema_version_uri`, and the pinned `record_kind` and integer
  schema-version field (`schema_version`,
  `collection_schema_version`, or
  `usage_export_record_schema_version`) match the row's
  `schema_version_pin`.
- **Example payload agreement** — the canonical example carries
  `registry_example_kind: schema_registry_envelope_example`, the
  row's `entry_id` and `schema_ref`, the family schema's
  `record_kind` (`schema_registry.example_payload_record_kind_mismatch`),
  and the pinned integer schema version
  (`schema_registry.example_payload_schema_version_mismatch`).
- **Compatibility horizon is reviewable** — `min_readable_version`
  and `min_writable_version` are positive integers and both
  `deprecation_window_note` and `sunset_window_note` are non-empty.
- **Deprecated fields are tracked honestly** — every
  `deprecated_field_record` carries a non-empty `field_path`, a
  positive `deprecated_since_schema_version`, a non-empty
  `removal_window_note`
  (`schema_registry.deprecated_field_removal_window_note_required`),
  and a typed `downgrade_action` in `{drop_field_on_read,
  preserve_as_unknown, refuse_read, refuse_export}`.
- **Named runtime consumer exists** —
  `named_runtime_consumer.consumer_ref` resolves on disk, its
  `consumer_class` is in the closed vocabulary, and
  `consumed_fields` is non-empty.
- **Required family-class coverage is met** — at least one row exists
  for each of `telemetry_payload`, `diagnostic_payload`,
  `support_export_payload`, and `usage_export_payload`.
- **Failure drills are listed and reproducible** — each row's
  `failure_drill.drill_id` is in `failure_drill_id_vocabulary` and
  the runner reproduces the exact `expected_check_id` when the drill
  is forced.

## Run

```bash
python3 tests/governance/m1_schema_registry_seed_lane/run_m1_schema_registry_seed_lane.py --repo-root .
```

The runner emits a deterministic JSON capture to
`artifacts/milestones/m1/captures/schema_registry_seed_validation_capture.json`
and exits non-zero on any check failure.

### Force a named failure drill

```bash
python3 tests/governance/m1_schema_registry_seed_lane/run_m1_schema_registry_seed_lane.py \
    --repo-root . \
    --force-drill <entry_id>:<drill_id>
```

In `--force-drill` mode the runner exits `0` only when the row's
declared `expected_check_id` is reproduced by the forced input.

| Row (`entry_id`) | Drill | Expected check id |
|---|---|---|
| `telemetry.m1_payload_baseline` | `telemetry_drill.consent_class_widened_to_implied` | `schema_registry.consent_class_disagrees_with_consent_ledger` |
| `diagnostics.m1_payload_baseline` | `diagnostics_drill.redaction_class_relaxed_to_metadata_only` | `schema_registry.redaction_class_relaxed_without_review` |
| `support_export.m1_payload_baseline` | `support_export_drill.example_payload_record_kind_drifted` | `schema_registry.example_payload_record_kind_mismatch` |
| `usage_export.m1_payload_baseline` | `usage_export_drill.deprecated_field_loses_sunset_note` | `schema_registry.deprecated_field_removal_window_note_required` |

Optional flags:

- `--matrix <path>` — point at an alternate seed file.
- `--envelope-schema <path>` — alternate envelope schema.
- `--row-schema <path>` — alternate row schema.
- `--consent-ledger <path>` — alternate consent-ledger registry.
- `--build-identity <path>` — alternate build-identity record.
- `--report <path>` — change the capture output path.

## Where the evidence lives

| Artifact | Path |
| --- | --- |
| Reviewer landing page | `docs/governance/schema_registry_seed.md` |
| Seed (canonical) | `schemas/registry/schema_registry.yaml` |
| Envelope schema | `schemas/registry/schema_registry.schema.json` |
| Row schema | `schemas/registry/schema_registry_row.schema.json` |
| Example payloads | `fixtures/schemas/m1_registry_examples/*.json` |
| Parent consent ledger | `artifacts/governance/consent_ledger_seed.yaml` |
| Latest capture | `artifacts/milestones/m1/captures/schema_registry_seed_validation_capture.json` |
| Owning proof packet | `artifacts/milestones/m1/proof_packets/schema_registry_seed.md` |

The lane is registered in
[`artifacts/milestones/m1/artifact_index.yaml`](../../../artifacts/milestones/m1/artifact_index.yaml)
under `proof_lanes.schema_registry_seed` so reviewers can find the
latest capture, owner, and validation-lane reference without searching
ad hoc folders.

## Refresh policy

Re-run the lane (and refresh the capture) when any of the following
change:

- `schemas/registry/schema_registry.yaml`
- `schemas/registry/schema_registry.schema.json`
- `schemas/registry/schema_registry_row.schema.json`
- any pinned family schema's `$id`, `record_kind`, or integer schema
  version
- `artifacts/governance/consent_ledger_seed.yaml` parent rows the M1
  rows inherit from
- `docs/governance/schema_registry_seed.md`
