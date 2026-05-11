# M1 contribution-governance seed validation lane

Unattended proof lane that validates the M1 contribution-governance
seed at
[`artifacts/governance/contribution_governance_seed.yaml`](../../../artifacts/governance/contribution_governance_seed.yaml)
against:

- [`schemas/governance/contribution_governance_seed.schema.json`](../../../schemas/governance/contribution_governance_seed.schema.json)
  — envelope schema (vocabularies, required coverage, row list);
- [`schemas/governance/contribution_governance_seed_row.schema.json`](../../../schemas/governance/contribution_governance_seed_row.schema.json)
  — row vocabulary;
- each row's canonical artifact (resolved on disk and scanned for the
  declared `canonical_artifact_marker` literal substring);
- each row's supporting artifacts (all resolved on disk); and
- each row's canonical envelope example under
  [`fixtures/governance/m1_contribution_governance_examples/`](../../../fixtures/governance/m1_contribution_governance_examples/).

The lane is deliberately runnable on CI / nightly without a graphical
display: it only consumes the pure data joined out of the canonical
sources.

## What the lane proves

For every row in the seed the runner asserts:

- **Envelope is canonical** — `record_kind` is
  `m1_contribution_governance_seed_row` and
  `contribution_governance_seed_row_schema_version` is `1`.
- **Closed vocabularies match the row schema verbatim** —
  `control_class`, `artifact_kind_class`, `enforcement_class`,
  `lifecycle_state_class`, and
  `contribution_governance_consumer_class` all match the row
  schema's `$defs`.
- **`control_id` matches the row's `control_class`** — signoff_dco
  rows start with `signoff.`, license_metadata with `license.`,
  third_party_import_record with `import_record.`,
  public_interface_versioning with `versioning.`,
  deprecation_packet_template with `deprecation.`, and
  repo_hygiene_scaffold with `repo_hygiene.`
  (`contribution_governance.row.control_id_prefix_mismatch`).
- **`artifact_kind_class` agrees with `control_class`** through the
  envelope's `control_class_to_artifact_kind_class` closed map
  (`contribution_governance.artifact_kind_class_disagrees_with_control_class`).
- **`enforcement_class` agrees with `control_class`** through the
  envelope's `control_class_to_enforcement_class` closed map
  (`contribution_governance.enforcement_class_disagrees_with_control_class`).
- **Canonical artifact is honest** — the row's `canonical_artifact_ref`
  resolves on disk and its text contains the row's
  `canonical_artifact_marker`
  (`contribution_governance.canonical_artifact_marker_missing`).
- **Supporting artifacts resolve** — every entry in
  `supporting_artifact_refs` exists on disk
  (`contribution_governance.supporting_artifact_ref_missing`).
- **Example payload agreement** — the canonical example carries
  `registry_example_kind: contribution_governance_seed_envelope_example`,
  the row's `control_id`, the row's `canonical_artifact_ref`, the
  row's `control_class`, the row's `artifact_kind_class`, the row's
  `enforcement_class`, and the row's `lifecycle_state_class`.
- **Named runtime consumer exists** —
  `named_runtime_consumer.consumer_ref` resolves on disk, its
  `consumer_class` is in the closed vocabulary, and
  `consumed_fields` is non-empty
  (`contribution_governance.named_runtime_consumer_consumed_fields_empty`).
- **Required control-class coverage is met** — at least one row
  exists for each of `signoff_dco`, `license_metadata`,
  `third_party_import_record`, `public_interface_versioning`,
  `deprecation_packet_template`, and `repo_hygiene_scaffold`.
- **Failure drills are listed and reproducible** — each row's
  `failure_drill.drill_id` is in `failure_drill_id_vocabulary` and
  the runner reproduces the exact `expected_check_id` when the drill
  is forced.

## Run

```bash
python3 tests/governance/m1_contribution_governance_seed_lane/run_m1_contribution_governance_seed_lane.py --repo-root .
```

The runner emits a deterministic JSON capture to
`artifacts/milestones/m1/captures/contribution_governance_seed_validation_capture.json`
and exits non-zero on any check failure.

### Force a named failure drill

```bash
python3 tests/governance/m1_contribution_governance_seed_lane/run_m1_contribution_governance_seed_lane.py \
    --repo-root . \
    --force-drill <control_id>:<drill_id>
```

In `--force-drill` mode the runner exits `0` only when the row's
declared `expected_check_id` is reproduced by the forced input.

| Row (`control_id`) | Drill | Expected check id |
|---|---|---|
| `signoff.contribution_certificate_baseline` | `signoff_drill.enforcement_class_relaxed_to_contributor_documentation` | `contribution_governance.enforcement_class_disagrees_with_control_class` |
| `license.spdx_per_file_baseline` | `license_drill.artifact_kind_class_drifted_to_third_party_import_register` | `contribution_governance.artifact_kind_class_disagrees_with_control_class` |
| `import_record.third_party_import_register_baseline` | `import_record_drill.canonical_artifact_marker_cleared` | `contribution_governance.canonical_artifact_marker_missing` |
| `versioning.public_interface_policy_baseline` | `versioning_drill.lifecycle_state_class_drifted_to_unknown` | `contribution_governance.row.lifecycle_state_class_unknown` |
| `deprecation.packet_template_baseline` | `deprecation_drill.example_payload_pinned_canonical_artifact_ref_drifted` | `contribution_governance.example_payload_pinned_canonical_artifact_ref_mismatch` |
| `repo_hygiene.scaffolding_baseline` | `repo_hygiene_drill.named_runtime_consumer_consumed_fields_cleared` | `contribution_governance.named_runtime_consumer_consumed_fields_empty` |

Optional flags:

- `--matrix <path>` — point at an alternate seed file.
- `--envelope-schema <path>` — alternate envelope schema.
- `--row-schema <path>` — alternate row schema.
- `--import-record-register <path>` — alternate import-record register.
- `--build-identity <path>` — alternate build-identity record.
- `--report <path>` — change the capture output path.

## Where the evidence lives

| Artifact | Path |
| --- | --- |
| Reviewer landing page | `docs/governance/contribution_and_signoff.md` |
| Seed (canonical) | `artifacts/governance/contribution_governance_seed.yaml` |
| Envelope schema | `schemas/governance/contribution_governance_seed.schema.json` |
| Row schema | `schemas/governance/contribution_governance_seed_row.schema.json` |
| Example payloads | `fixtures/governance/m1_contribution_governance_examples/*.json` |
| Import-record register | `artifacts/governance/import_record_seed.yaml` |
| Versioning policy | `docs/governance/public_interface_versioning_policy.md` |
| Repo-hygiene scaffold | `docs/governance/repo_hygiene_scaffolding.md` |
| Deprecation packet template | `docs/governance/deprecation_packet_template.md` |
| Latest capture | `artifacts/milestones/m1/captures/contribution_governance_seed_validation_capture.json` |
| Owning proof packet | `artifacts/milestones/m1/proof_packets/contribution_governance_seed.md` |

The lane is registered in
[`artifacts/milestones/m1/artifact_index.yaml`](../../../artifacts/milestones/m1/artifact_index.yaml)
under `proof_lanes.contribution_governance_seed` so reviewers can find
the latest capture, owner, and validation-lane reference without
searching ad hoc folders.

## Refresh policy

Re-run the lane (and refresh the capture) when any of the following
change:

- `artifacts/governance/contribution_governance_seed.yaml`
- `schemas/governance/contribution_governance_seed.schema.json`
- `schemas/governance/contribution_governance_seed_row.schema.json`
- any canonical artifact a row points at via `canonical_artifact_ref`
- any canonical example payload under
  `fixtures/governance/m1_contribution_governance_examples/`
- `docs/governance/contribution_and_signoff.md`
