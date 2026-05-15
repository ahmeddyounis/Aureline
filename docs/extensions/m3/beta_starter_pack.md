# M3 extension-author beta starter pack

This pack is the reviewer-facing entrypoint for extension authors who
want to bring an add-on through the M3 beta admission lane. It binds
the SDK overview, sample/compatibility expectations, revocation rules,
and issue-routing lanes into one starter pack that reads from the same
canonical truth as the rest of the M3 beta program.

The pack is governed. The canonical machine source is
`artifacts/milestones/m3/beta_enablement_starter_pack.yaml`; the
validator is `ci/check_m3_beta_enablement_starter_pack.py`. When the
pack and the canonical source disagree, the canonical source wins and
this doc MUST be updated in the same change set.

- Lane id: `starter_pack_lane:extension_author`
- Primary cohort: `cohort:extension_author`
- Primary beta surfaces: `beta_surface:extension_runtime`,
  `beta_surface:packaging_update_rollback`
- Secondary cohorts: `cohort:design_partner_managed_pilot`,
  `cohort:internal_dogfood`
- Secondary surface: `beta_surface:policy_proxy_transport`

## How to use this pack

1. Read the [beta admission matrix](../../milestones/m3/beta_admission_matrix.md)
   for the canonical claim surface and cohort vocabulary.
2. Read the SDK publication contract and the host/runtime baselines
   linked below before publishing or upgrading an extension into the
   beta channel.
3. Run a reproducible sample against the current beta build using one
   of the sample packs.
4. Cite the named issue-routing classes in
   `artifacts/governance/issue_routing.yaml` when filing reports.
5. Escalate through the named owner-handoff path when the validator,
   compatibility report, or rollback drill fails.

## SDK overview

These are the canonical, in-tree SDK and runtime contracts. They are
the same docs the validator at
`ci/check_m3_beta_enablement_starter_pack.py` audits, so every
extension-author surface points back to one source of truth.

- SDK release-bundle and conformance contract:
  [`docs/extensions/sdk_publication_contract.md`](../sdk_publication_contract.md)
- Permission and publisher baseline:
  [`docs/extensions/m1_permission_and_publisher_baseline.md`](../m1_permission_and_publisher_baseline.md)
- Publisher lifecycle and registry parity:
  [`docs/extensions/publisher_lifecycle_and_registry_parity_contract.md`](../publisher_lifecycle_and_registry_parity_contract.md)
- Local dev and sideload:
  [`docs/extensions/local_dev_and_sideload_contract.md`](../local_dev_and_sideload_contract.md)
- Dev-loop rebinding and repro:
  [`docs/extensions/dev_loop_rebinding_and_repro_contract.md`](../dev_loop_rebinding_and_repro_contract.md)
- WIT host contract seed:
  [`docs/extensions/wit_host_contract_seed.md`](../wit_host_contract_seed.md)

## Compatibility and skew expectations

Every extension-author beta row resolves through the M3 compatibility
report and skew matrix. Do not invent a parallel chip set for
"verified" or "stable"; the support classes below are the only
authorized ones.

- Generated compatibility report (markdown):
  [`artifacts/compat/m3/compatibility_report.md`](../../../artifacts/compat/m3/compatibility_report.md)
- Generated compatibility report (JSON):
  [`artifacts/compat/m3/compatibility_report.json`](../../../artifacts/compat/m3/compatibility_report.json)
- M3 skew-window matrix:
  [`artifacts/compat/m3/skew_window_matrix.yaml`](../../../artifacts/compat/m3/skew_window_matrix.yaml)

Authorized support classes (read verbatim from the claimed-surface
register): `certified`, `supported`, `limited`, `experimental`,
`community`, `retest_pending`, `evidence_stale`, `unsupported`.

## Sample packs

Reproducible sample extensions are the precondition for a beta-class
SDK publication. The validator confirms each path resolves under the
repo so an author cannot land a starter row that points at vapor.

- SDK publication cases:
  [`fixtures/extensions/sdk_publication_cases`](../../../fixtures/extensions/sdk_publication_cases)
- Manifest examples:
  [`fixtures/extensions/manifest_examples`](../../../fixtures/extensions/manifest_examples)
- Host negotiation examples:
  [`fixtures/extensions/host_negotiation_examples`](../../../fixtures/extensions/host_negotiation_examples)
- Activation cases:
  [`fixtures/extensions/activation_cases`](../../../fixtures/extensions/activation_cases)

## Revocation, quarantine, and rollback

A beta extension MUST be revocable and rolledbackable without user
data loss. These are the contracts that govern the revocation surface
the cohort scorecard checks against:

- Effective-permission review:
  [`docs/extensions/effective_permission_review_contract.md`](../effective_permission_review_contract.md)
- Extension lifecycle and quarantine sequence:
  [`docs/extensions/extension_lifecycle_and_quarantine_sequence.md`](../extension_lifecycle_and_quarantine_sequence.md)
- Marketplace ranking and trust:
  [`docs/extensions/marketplace_ranking_and_trust_contract.md`](../marketplace_ranking_and_trust_contract.md)
- Update and rollback contract:
  [`docs/release/update_and_rollback_contract.md`](../../release/update_and_rollback_contract.md)

## Known-limits vocabulary

Read these in full before publishing any beta-class extension surface
or filing a "missing capability" report. The lane validator confirms
the same docs are linked here and from every other beta-enablement
surface, so authors and reviewers read one vocabulary.

- Cross-milestone known-limits contract:
  [`docs/product/known_limits_contract.md`](../../product/known_limits_contract.md)
- External alpha known limits packet (still in scope for beta intake):
  [`artifacts/feedback/external_alpha_known_limits.md`](../../../artifacts/feedback/external_alpha_known_limits.md)

## Cohort guardrails and scorecard

The extension-author cohort has a checked-in scorecard whose front
matter is consumed by the cohort/archetype scorecard validator. Read
it to know the current effective support class for this lane before
publishing.

- Cohort guardrails (canonical):
  [`artifacts/milestones/m3/cohort_guardrails.yaml`](../../../artifacts/milestones/m3/cohort_guardrails.yaml)
- Extension-author cohort scorecard:
  [`artifacts/milestones/m3/cohorts/extension_author_scorecard.md`](../../../artifacts/milestones/m3/cohorts/extension_author_scorecard.md)
- Scorecard index:
  [`artifacts/milestones/m3/cohorts/scorecard_index.yaml`](../../../artifacts/milestones/m3/cohorts/scorecard_index.yaml)
- Public-proof review packet template:
  [`artifacts/milestones/m3/review_packet_template.md`](../../../artifacts/milestones/m3/review_packet_template.md)

### Automatic downgrade triggers

The cohort scorecard fires these triggers automatically; tooling
applies them without reviewer interpretation.

| Trigger | Auto-state | Effect on the lane |
|---|---|---|
| SDK breaking change without migration | `retest_pending` | Beta publication is paused; an SDK-line migration note is required before retest. |
| Publication mirror unavailable | `limited` | Sample packs and offline-bundle rows are narrowed in docs and Help/About until the mirror recovers. |
| Extension rollback drill failure | `retest_pending` | Rollback drill must pass before any new publication into the beta channel. |
| Evidence past freshness window | `evidence_stale` | Compatibility chips and starter pack rows render as `evidence_stale` until refreshed. |

## Issue routing and disclosure posture

File reports through the named lanes in
`artifacts/governance/issue_routing.yaml`. The validator confirms each
class below resolves to one row in that file, so authors and
maintainers route the same way.

| Issue class | Default route | Public-summary expectation |
|---|---|---|
| `oss_bug` | Public issue tracker | Recommended |
| `perf_regression` | Public issue tracker | Required |
| `compatibility_regression` | Public issue tracker | Required |
| `docs_truth_defect` | Public issue tracker | Recommended |
| `rfc` | Public RFC forum (`/docs/rfc/`) | Required (the RFC itself) |
| `supportability_issue` | Public issue tracker (no live field content) | Recommended |
| `waiver_request` | Governance packet queue | Required |

Sensitive cases (security, partner identity, support exports with
live field content) MUST route through their own lanes — read
[`docs/community/m3/public_private_issue_routing.md`](../../community/m3/public_private_issue_routing.md)
for the full matrix before filing.

## Escalation

When a validator or compatibility row blocks publication and the
trigger is not in the table above:

- Owner handoff (intake, triage, release-council escalation): see the
  cohort scorecard above.
- Decision rights:
  [`docs/governance/decision_rights_and_signoff_matrix.md`](../../governance/decision_rights_and_signoff_matrix.md)
- Routing matrix:
  [`docs/governance/issue_routing_matrix.md`](../../governance/issue_routing_matrix.md)

## How to verify

This pack is governed by `ci/check_m3_beta_enablement_starter_pack.py`.
Run the validator and refresh the capture in the same change set when
any field, cohort binding, surface binding, or issue-routing reference
changes:

```
python3 ci/check_m3_beta_enablement_starter_pack.py --repo-root .
```

Use `--check` in CI to fail when the capture on disk would drift:

```
python3 ci/check_m3_beta_enablement_starter_pack.py --repo-root . --check
```
