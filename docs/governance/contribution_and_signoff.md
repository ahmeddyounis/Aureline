# M1 contribution and sign-off governance seed

This page is the reviewer-facing entry point into the M1 contribution-
and-sign-off governance seed at
[`/artifacts/governance/contribution_governance_seed.yaml`](../../artifacts/governance/contribution_governance_seed.yaml).
The seed is the focused M1-bearing subset of the contribution-governance
contract. It exists so later release, compliance, and ecosystem work
cannot rely on undocumented prose for the six control families that gate
governed M1 changes:

- **signoff_dco** — Developer Certificate of Origin sign-off
  expectations and CI gate;
- **license_metadata** — per-file SPDX/REUSE licensing posture;
- **third_party_import_record** — provenance recording format for
  vendored or mirrored third-party bytes;
- **public_interface_versioning** — public-interface versioning and
  deprecation policy for schemas, command IDs, CLI surfaces, RPC
  envelopes, manifests, and saved artifacts;
- **deprecation_packet_template** — one canonical packet shape future
  schema, command, lifecycle, or interface changes reuse instead of
  inventing ad hoc release notes;
- **repo_hygiene_scaffold** — repo-hygiene scaffolding for
  contributor-facing governance files, templates, and canonical
  locations used by protected M1 artifacts.

## Companion artifacts

- [`/artifacts/governance/contribution_governance_seed.yaml`](../../artifacts/governance/contribution_governance_seed.yaml)
  — the seeded contribution-governance rows.
- [`/schemas/governance/contribution_governance_seed.schema.json`](../../schemas/governance/contribution_governance_seed.schema.json)
  — boundary schema for the seed envelope (vocabularies + row list).
- [`/schemas/governance/contribution_governance_seed_row.schema.json`](../../schemas/governance/contribution_governance_seed_row.schema.json)
  — boundary schema for one M1 contribution-governance row.
- [`/fixtures/governance/m1_contribution_governance_examples/`](../../fixtures/governance/m1_contribution_governance_examples/)
  — one canonical envelope example per control family.
- [`/tests/governance/m1_contribution_governance_seed_lane/`](../../tests/governance/m1_contribution_governance_seed_lane/)
  — unattended validation lane that re-parses the seed and the
  example payloads end-to-end.
- [`./public_interface_versioning_policy.md`](./public_interface_versioning_policy.md)
  — first public-interface versioning and deprecation policy this seed
  cites as the canonical artifact for `public_interface_versioning`.
- [`./repo_hygiene_scaffolding.md`](./repo_hygiene_scaffolding.md)
  — repo-hygiene scaffolding contract this seed cites for
  `repo_hygiene_scaffold`.
- [`./deprecation_packet_template.md`](./deprecation_packet_template.md)
  — canonical deprecation-packet template this seed cites for
  `deprecation_packet_template`.
- [`/artifacts/governance/import_record_seed.yaml`](../../artifacts/governance/import_record_seed.yaml)
  — versioned import-record seed that later release evidence can reuse,
  cited by the `third_party_import_record` row.

If this page and the boundary schemas disagree, the schemas win and
this page must be updated in the same change.

## Why an M1-bearing seed

The repository already carries broader contribution and supply-chain
contracts:

- `CONTRIBUTING.md` carries the full contributor guide, including the
  Developer Certificate of Origin (DCO) text and the per-file SPDX/REUSE
  posture;
- `docs/governance/dependency_review_policy.md` carries the third-party
  admission policy and the link to
  `artifacts/governance/third_party_import_register.yaml`;
- `docs/governance/provenance_and_compliance_baseline.md` carries the
  provenance and SBOM expectations a release pack must honour.

Those documents are deliberately broad. If every M1 surface had to read
all of them before claiming a contribution-governance control exists, the
contract would become a side document rather than a registry. The M1
seed is the smaller proof set that:

- names exactly the six controls protected M1 changes gate against;
- pins each control's canonical artifact, artifact-kind class, and
  enforcement class so later work cannot drift these axes without
  updating the row;
- carries one canonical example payload per row that the validation
  lane re-parses end-to-end so the row pin is honoured.

## How a contribution-governance row stays honest

For every row, the validation lane at
[`/tests/governance/m1_contribution_governance_seed_lane/run_m1_contribution_governance_seed_lane.py`](../../tests/governance/m1_contribution_governance_seed_lane/run_m1_contribution_governance_seed_lane.py)
asserts:

- the row's `record_kind` is `m1_contribution_governance_seed_row` and
  its `contribution_governance_seed_row_schema_version` is `1`;
- the envelope's closed vocabularies (`control_class`,
  `artifact_kind_class`, `enforcement_class`, `lifecycle_state_class`,
  `contribution_governance_consumer_class`) agree verbatim with the
  row schema's `$defs`;
- the `control_id` prefix matches the `control_class` (signoff_dco
  rows start with `signoff.`, license_metadata with `license.`,
  third_party_import_record with `import_record.`,
  public_interface_versioning with `versioning.`,
  deprecation_packet_template with `deprecation.`,
  repo_hygiene_scaffold with `repo_hygiene.`);
- the row's `artifact_kind_class` agrees with `control_class` through
  the closed map
  `control_class_to_artifact_kind_class`;
- the row's `enforcement_class` agrees with `control_class` through
  the closed map
  `control_class_to_enforcement_class`;
- the `canonical_artifact_ref` exists on disk and its text contains the
  row's `canonical_artifact_marker` so the seed cannot quietly point
  at the wrong document;
- every `supporting_artifact_ref` exists on disk;
- `named_runtime_consumer.consumer_ref` exists, its `consumer_class` is
  in the closed vocabulary, and `consumed_fields` is non-empty;
- the canonical example payload at `example_payload_ref` exists,
  carries `registry_example_kind: contribution_governance_seed_envelope_example`,
  cites the row's `control_id`, and pins the row's
  `canonical_artifact_ref`;
- the row's failure drill is listed in `failure_drill_id_vocabulary`
  and reproducible under `--force-drill`.

The seed cannot close until every required control class
(`signoff_dco`, `license_metadata`, `third_party_import_record`,
`public_interface_versioning`, `deprecation_packet_template`,
`repo_hygiene_scaffold`) is observed.

## Failure drills

The seed declares one failure drill per row so the runner can prove
the lane surfaces drift loudly:

| Row (`control_id`) | Drill | Expected check id |
|---|---|---|
| `signoff.contribution_certificate_baseline` | `signoff_drill.enforcement_class_relaxed_to_contributor_documentation` | `contribution_governance.enforcement_class_disagrees_with_control_class` |
| `license.spdx_per_file_baseline` | `license_drill.artifact_kind_class_drifted_to_third_party_import_register` | `contribution_governance.artifact_kind_class_disagrees_with_control_class` |
| `import_record.third_party_import_register_baseline` | `import_record_drill.canonical_artifact_marker_cleared` | `contribution_governance.canonical_artifact_marker_missing` |
| `versioning.public_interface_policy_baseline` | `versioning_drill.lifecycle_state_class_drifted_to_unknown` | `contribution_governance.row.lifecycle_state_class_unknown` |
| `deprecation.packet_template_baseline` | `deprecation_drill.example_payload_pinned_canonical_artifact_ref_drifted` | `contribution_governance.example_payload_pinned_canonical_artifact_ref_mismatch` |
| `repo_hygiene.scaffolding_baseline` | `repo_hygiene_drill.named_runtime_consumer_consumed_fields_cleared` | `contribution_governance.named_runtime_consumer_consumed_fields_empty` |

Each drill is reproducible with:

```bash
python3 tests/governance/m1_contribution_governance_seed_lane/run_m1_contribution_governance_seed_lane.py \
    --repo-root . \
    --force-drill <control_id>:<drill_id>
```

In `--force-drill` mode the runner exits `0` only when the row's
declared `expected_check_id` is reproduced by the forced input. Drift
on other rows still fails the lane.

## Refresh policy

Re-run the validation lane after a change to:

- `artifacts/governance/contribution_governance_seed.yaml`;
- either the envelope or row schemas under `schemas/governance/`;
- any canonical artifact a row points at via `canonical_artifact_ref`;
- the canonical example payloads under
  `fixtures/governance/m1_contribution_governance_examples/`;
- this reviewer-facing landing page.

## Closure rule

The lane stays open until the latest capture lands under the governed
proof root at
`artifacts/milestones/m1/captures/contribution_governance_seed_validation_capture.json`
and every row reports PASS for the row contract above.
