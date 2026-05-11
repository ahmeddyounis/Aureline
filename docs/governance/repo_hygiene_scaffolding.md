# Repo-hygiene scaffolding for contributor-facing governance

This page is the canonical artifact for the `repo_hygiene_scaffold` row
of the contribution-governance seed at
[`/artifacts/governance/contribution_governance_seed.yaml`](../../artifacts/governance/contribution_governance_seed.yaml).
The reviewer-facing landing page for the seed is
[`./contribution_and_signoff.md`](./contribution_and_signoff.md).

Repo hygiene is the rule that contributor-facing governance files,
templates, and the canonical locations protected M1 artifacts use stay
reachable from one place and stay machine-readable. Without that rule,
review cost climbs every time a reviewer has to guess where a packet,
template, or seed lives.

## Canonical contributor-facing locations

The locations below are the canonical homes for the contributor-facing
governance files this scaffolding promises:

| Concern | Canonical location |
|---|---|
| Top-level contributor guide | [`/CONTRIBUTING.md`](../../CONTRIBUTING.md) |
| Agent / coding-assistant guide | [`/AGENTS.md`](../../AGENTS.md), [`/CLAUDE.md`](../../CLAUDE.md) |
| Security disclosure rules | [`/SECURITY.md`](../../SECURITY.md) |
| Pull-request review routing | [`/CODEOWNERS`](../../CODEOWNERS) |
| DRI map | [`./dri_map.md`](./dri_map.md) |
| Maintainer coverage policy | [`./maintainer_coverage_policy.md`](./maintainer_coverage_policy.md) |
| Decision workflow | [`./decision_workflow.md`](./decision_workflow.md) |
| Decision index (machine-readable) | [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml) |
| Decision register (human-readable) | [`./decision_register.md`](./decision_register.md) |
| Dependency review policy | [`./dependency_review_policy.md`](./dependency_review_policy.md) |
| Dependency register (machine-readable) | [`/artifacts/governance/dependency_register.yaml`](../../artifacts/governance/dependency_register.yaml) |
| Third-party import register (machine-readable) | [`/artifacts/governance/third_party_import_register.yaml`](../../artifacts/governance/third_party_import_register.yaml) |
| Import-record seed (M1) | [`/artifacts/governance/import_record_seed.yaml`](../../artifacts/governance/import_record_seed.yaml) |
| Contribution-governance seed (M1) | [`/artifacts/governance/contribution_governance_seed.yaml`](../../artifacts/governance/contribution_governance_seed.yaml) |
| Contribution-governance reviewer landing | [`./contribution_and_signoff.md`](./contribution_and_signoff.md) |
| Public-interface versioning policy | [`./public_interface_versioning_policy.md`](./public_interface_versioning_policy.md) |
| Deprecation packet template | [`./deprecation_packet_template.md`](./deprecation_packet_template.md) |
| Governance packet template (machine-readable) | [`/artifacts/governance/governance_packet_template.yaml`](../../artifacts/governance/governance_packet_template.yaml) |
| Provenance and compliance baseline | [`./provenance_and_compliance_baseline.md`](./provenance_and_compliance_baseline.md) |

The list is deliberately short. Adding a new contributor-facing
canonical location is allowed as long as the addition lands here and in
the relevant row of the contribution-governance seed in the same change.

## Templates

Templates live under
[`./templates/`](./templates/). Each template is a frozen shape future
governed work reuses instead of inventing a new packet format:

- [`./templates/waiver_template.md`](./templates/waiver_template.md)
  — for time-boxed waivers and freeze exceptions.
- [`./templates/freeze_exception_template.md`](./templates/freeze_exception_template.md)
  — for freeze-exception requests against a protected lane.
- [`./templates/exception_packet_template.md`](./templates/exception_packet_template.md)
  — for review-time exceptions on a governed contract.
- [`./templates/verification_packet_template.md`](./templates/verification_packet_template.md)
  — for verification packets attached to a release-evidence pack.
- [`./deprecation_packet_template.md`](./deprecation_packet_template.md)
  — the canonical deprecation packet shape for any schema, command,
  lifecycle, or interface deprecation.

## How repo hygiene is enforced

Repo hygiene is enforced through the contribution-governance seed at
[`/artifacts/governance/contribution_governance_seed.yaml`](../../artifacts/governance/contribution_governance_seed.yaml).
The seed's validation lane at
[`/tests/governance/m1_contribution_governance_seed_lane/`](../../tests/governance/m1_contribution_governance_seed_lane/)
re-parses the seed and asserts:

- the `repo_hygiene_scaffold` row's `canonical_artifact_ref` points at
  this file;
- this file contains the canonical-artifact-marker the row declares,
  so the seed cannot quietly point at the wrong document;
- every `supporting_artifact_ref` the row declares (templates,
  companion docs) exists on disk.

If this page and the contribution-governance seed disagree, the seed
wins and this page MUST be updated in the same change.
