# M1 schema-registry seed for telemetry, diagnostics, support-export, and usage-export payloads

This page is the reviewer-facing entry point into the M1 schema-registry
seed at
[`/schemas/registry/schema_registry.yaml`](../../schemas/registry/schema_registry.yaml).
The seed is the focused M1-bearing subset of the canonical
schema-registry contract. It exists so later privacy, support, and
release work cannot rely on undocumented JSON blobs for the four
payload families that drive M1 surfaces:

- **telemetry** payloads (opt-in onboarding task-success and
  first-useful-work measurement);
- **diagnostics** payloads (locally-captured problem-evidence chains
  produced by project doctor, repair preview, and recovery surfaces);
- **support-export** payloads (the manifest the user assembles when they
  explicitly export a support bundle);
- **usage-export** payloads (the customer-visible usage summary
  produced under managed admin posture).

## Companion artifacts

- [`/schemas/registry/schema_registry.yaml`](../../schemas/registry/schema_registry.yaml)
  — the seeded registry rows.
- [`/schemas/registry/schema_registry.schema.json`](../../schemas/registry/schema_registry.schema.json)
  — boundary schema for the seed envelope (vocabularies + row list).
- [`/schemas/registry/schema_registry_row.schema.json`](../../schemas/registry/schema_registry_row.schema.json)
  — boundary schema for one M1 registry row.
- [`/fixtures/schemas/m1_registry_examples/`](../../fixtures/schemas/m1_registry_examples/)
  — one canonical envelope example per family
  (`telemetry_payload.json`, `diagnostic_payload.json`,
  `support_export_payload.json`, `usage_export_payload.json`).
- [`/tests/governance/m1_schema_registry_seed_lane/`](../../tests/governance/m1_schema_registry_seed_lane/)
  — unattended validation lane that re-parses the seed and the
  example payloads end-to-end.
- [`./telemetry_and_support_schema_registry.md`](./telemetry_and_support_schema_registry.md)
  — the broader, more comprehensive registry contract this M1 seed
  inherits from. The M1 seed never forks that contract's vocabularies
  and never relaxes its posture; rows here cite their parent
  consent-ledger row via `consent_ledger_entry_id_ref`.
- [`/artifacts/governance/consent_ledger_seed.yaml`](../../artifacts/governance/consent_ledger_seed.yaml)
  — canonical consent-ledger seed the M1 rows inherit from.

If this page and the boundary schemas disagree, the schemas win and
this page must be updated in the same change.

## Why an M1-bearing seed

The canonical registry at
[`./telemetry_and_support_schema_registry.md`](./telemetry_and_support_schema_registry.md)
covers fourteen payload families with the full closed vocabulary the
program expects to use over time. That registry is broad on purpose —
but if every M1 surface had to read all of it before claiming a
payload exists, the registry would become a side document rather than
a contract. The M1 seed is the smaller proof set that:

- names exactly the families M1 surfaces touch (telemetry,
  diagnostics, support-export, usage-export);
- pins each family's authoritative schema file and integer
  `schema_version_pin`;
- carries the lifecycle state, consent class summary, default-posture
  class, endpoint class, redaction class, retention note, and
  compatibility horizon (deprecation + sunset windows) for the family;
- tracks deprecated fields with an explicit `downgrade_action` so
  fields are never silently dropped;
- inherits the broader consent / endpoint / build-flavor / downgrade
  /exclusion vocabularies from the canonical registry through
  `consent_ledger_entry_id_ref`;
- and is consumable by exactly one M1 surface today (this page) without
  pretending more surfaces exist than have actually landed.

## Row shape (M1 subset)

Every `m1_schema_registry_row` keeps the M1-bearing axes separate:

- `family_class` names the M1 family the row is canonical for
  (`telemetry_payload`, `diagnostic_payload`, `support_export_payload`,
  `usage_export_payload`). Telemetry, diagnostics, support-export, and
  usage-export rows stay separate even when their transport, redaction,
  or upload plumbing is shared.
- `owner_dri` names the M1 DRI. A family without an owner is a
  governance bug.
- `schema_ref` and `schema_version_pin` pin the family's authoritative
  schema and version; `schema_version_uri` is the stable `$id` URI.
- `lifecycle_state_class` is one of `seeded`, `stabilising`, `stable`,
  `deprecated`, or `retired`.
- `consent_class` and `default_posture_class` summarise the consent
  and default posture; the validation lane asserts the
  `consent_class` agrees with the parent row in
  `consent_ledger_seed.yaml`.
- `endpoint_class` names where the payload may travel
  (`local_device_only`, `local_authoritative_with_optional_upload`,
  `export_only_user_initiated`, `managed_authoritative_when_enabled`).
- `redaction_class` names the redaction default
  (`metadata_safe_default`, `metadata_only_no_payload_bytes`,
  `code_adjacent_redacted_default`, or
  `high_risk_explicit_review_required`).
- `retention_note` is a reviewable sentence stating how long the
  payload lives and what closes the window. Authoritative posture is
  inherited from the record-class registry.
- `compatibility_horizon` carries `min_readable_version`,
  `min_writable_version`, a `deprecation_window_note`, and a
  `sunset_window_note`.
- `deprecated_fields` track each deprecated field with its
  `deprecated_since_schema_version`, `removal_window_note`,
  `downgrade_action`, and optional `replacement_field_path`.
- `consent_ledger_entry_id_ref` is the entry id of the parent row in
  `consent_ledger_seed.yaml`; the M1 row inherits the broader posture
  from there.
- `named_runtime_consumer` names exactly one real M1 surface, doc page,
  Rust validator, or CI gate that reads the row.
- `example_payload_ref` points at the family's canonical envelope
  example under `fixtures/schemas/m1_registry_examples/`.

## Seeded families

| Entry id | Family class | Schema (pinned v1) | Consent class | Default posture | Redaction class |
|---|---|---|---|---|---|
| `telemetry.m1_payload_baseline` | `telemetry_payload` | `schemas/telemetry/m1_onboarding_metrics.schema.json` | `explicit_opt_in_required` | `off_by_default_no_emission_until_consent` | `metadata_safe_default` |
| `diagnostics.m1_payload_baseline` | `diagnostic_payload` | `schemas/diagnostics/problem_evidence_chain.schema.json` | `explicit_opt_in_required` | `local_capture_no_upload_by_default` | `code_adjacent_redacted_default` |
| `support_export.m1_payload_baseline` | `support_export_payload` | `schemas/support/support_bundle_manifest.schema.json` | `export_only_on_user_request` | `user_initiated_export_only` | `high_risk_explicit_review_required` |
| `usage_export.m1_payload_baseline` | `usage_export_payload` | `schemas/governance/usage_export_record.schema.json` | `admin_policy_gated` | `admin_policy_gated_default` | `metadata_only_no_payload_bytes` |

## How a new payload enters the registry

1. Land the family's authoritative schema (or update an existing one)
   and bump its integer `schema_version` if the change is breaking.
2. Add a row to
   [`/schemas/registry/schema_registry.yaml`](../../schemas/registry/schema_registry.yaml)
   conforming to
   [`/schemas/registry/schema_registry_row.schema.json`](../../schemas/registry/schema_registry_row.schema.json):
   pick a `family_class`, name the owner, point `schema_ref` at the
   schema file, set `schema_version_pin` to the integer the row is
   committing to, summarise consent / posture / endpoint / redaction
   classes, write the retention note and compatibility horizon, and
   cite the parent consent-ledger row in
   `consent_ledger_entry_id_ref`.
3. Land one envelope example under
   [`/fixtures/schemas/m1_registry_examples/`](../../fixtures/schemas/m1_registry_examples/)
   that carries `registry_example_kind: schema_registry_envelope_example`,
   the `schema_registry_row_entry_id` of the new row, the
   `pinned_record_kind`, and the integer `pinned_schema_version`. The
   validation lane proves the row pin and the example agree without
   re-stating the full family contract.
4. Cite at least one real consumer in `named_runtime_consumer`. The
   consumer must exist on disk; pointing at a hypothetical surface is
   a review finding.
5. Run the validation lane and refresh the capture under
   `/artifacts/milestones/m1/captures/`.

## How deprecated fields are tracked

A deprecated field is a `deprecated_field_record` on the row, not a
side document. Every deprecated field carries:

- `field_path` — dotted JSON path of the deprecated field within the
  family's schema (for example `envelope.legacy_provider_account_id`);
- `deprecated_since_schema_version` — the integer schema version at
  which the field was marked deprecated;
- `removal_window_note` — a reviewable sentence stating when the
  field will actually be removed and what the downgrade window looks
  like;
- `downgrade_action` — a typed action a reader or writer must take
  when it encounters the field
  (`drop_field_on_read`, `preserve_as_unknown`, `refuse_read`, or
  `refuse_export`);
- `replacement_field_path` — optional pointer at the replacement field
  if one exists.

A deprecation without a `removal_window_note` is a review finding (the
validation lane reproduces it as
`schema_registry.deprecated_field_removal_window_note_required`); a
deprecation that does not name a `downgrade_action` is the same.

## What this seed is not

- It is **not** a telemetry backend design. Running a telemetry
  pipeline, storing events, billing usage, or auto-uploading crash
  dumps is out of scope. The seed fixes payload-family pin policy
  before any of that is implemented.
- It is **not** a substitute for the canonical
  [`./telemetry_and_support_schema_registry.md`](./telemetry_and_support_schema_registry.md)
  contract. The M1 seed inherits the full closed-vocabulary set
  through `consent_ledger_entry_id_ref`.
- It is **not** the support-bundle contract. That remains the
  support-bundle contract under `/docs/support/`. The seed quotes its
  manifest schema.
- It is **not** the record-class registry. Retention, delete, hold,
  and offboarding posture continue to live in the record-class
  registry. The seed pins the schema family, not the record-class
  semantics.

## Refresh policy

Re-run the validation lane (and refresh the capture) when any of the
following change:

- `/schemas/registry/schema_registry.yaml` (rows, vocabularies, or
  metadata);
- `/schemas/registry/schema_registry.schema.json` or
  `/schemas/registry/schema_registry_row.schema.json`;
- any pinned family schema's `$id`, `record_kind`, or integer schema
  version;
- the parent rows in
  `/artifacts/governance/consent_ledger_seed.yaml` that the M1 rows
  inherit from.
