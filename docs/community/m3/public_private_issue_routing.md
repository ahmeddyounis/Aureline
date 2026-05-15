# M3 community and public/private issue & RFC routing

This document is the M3 reviewer-facing entrypoint for community
contributors, issue filers, RFC authors, benchmark-dispute filers,
docs-truth defect reporters, and private-case escalators during the
beta train. It does not replace the cross-milestone routing matrix
([`docs/governance/issue_routing_matrix.md`](../../governance/issue_routing_matrix.md))
and machine-readable matrix
([`artifacts/governance/issue_routing.yaml`](../../../artifacts/governance/issue_routing.yaml));
it points at them and ties their lanes to the M3 truth vocabulary so
that incoming reports route mechanically, without parallel "beta-only"
lanes or improvised forums.

The pack is governed. The canonical machine source is
`artifacts/milestones/m3/beta_enablement_starter_pack.yaml`; the
validator is `ci/check_m3_beta_enablement_starter_pack.py`. When this
doc and the canonical source disagree, the canonical source wins and
this doc MUST be updated in the same change set.

- Lane id: `starter_pack_lane:community`
- Audience: community contributors, RFC authors, bug filers, dispute
  filers, and any escalator who reads CONTRIBUTING.md or SECURITY.md
  first

## Two failure modes this lane prevents

Before Aureline widens its public contribution surface during the M3
beta, two failure modes have to be impossible:

1. A sensitive report (security, partner identity, live support
   bundle) leaking into a public lane because the contributor did not
   know the right route.
2. A community report (bug, regression, docs defect, RFC,
   accessibility defect) disappearing into a private channel because
   no public lane looked "official".

The routing matrix resolves both by forcing every issue class to name
its default route, privacy posture, redaction posture, summary
expectation, and owning forum up front. This lane simply ties those
classes to the M3 vocabulary downstream surfaces read.

## How to use this lane

1. Read the entry points:
   - Top-level contributor entry:
     [`CONTRIBUTING.md`](../../../CONTRIBUTING.md)
   - Public security-contact page:
     [`SECURITY.md`](../../../SECURITY.md)
2. Read the cross-milestone routing matrix:
   - Narrative:
     [`docs/governance/issue_routing_matrix.md`](../../governance/issue_routing_matrix.md)
   - Machine-readable matrix:
     [`artifacts/governance/issue_routing.yaml`](../../../artifacts/governance/issue_routing.yaml)
3. Read the M3 truth vocabulary so the chips, packets, and freshness
   words you cite in a report match what reviewers and tooling
   already use:
   - Beta admission matrix:
     [`docs/milestones/m3/beta_admission_matrix.md`](../../milestones/m3/beta_admission_matrix.md)
   - Public-proof index:
     [`artifacts/milestones/m3/public_proof_index.md`](../../../artifacts/milestones/m3/public_proof_index.md)
   - Review-packet template:
     [`artifacts/milestones/m3/review_packet_template.md`](../../../artifacts/milestones/m3/review_packet_template.md)
   - Publication shelf-life policy:
     [`docs/governance/m3/publication_shelf_life_policy.md`](../../governance/m3/publication_shelf_life_policy.md)
4. Pick one issue class below and follow its default route.

## Issue classes admitted by the M3 community lane

Every class below resolves to one `issue_classes[].id` in
`artifacts/governance/issue_routing.yaml`. The full row (privacy,
disclosure, redaction, summary expectation, owning forum) is in the
matrix; the table here is the short M3 entry point.

| Issue class | Default route | Privacy class | Public-summary expectation |
|---|---|---|---|
| `oss_bug` | Public issue tracker | Public | Recommended |
| `perf_regression` | Public issue tracker | Public | Required |
| `rfc` | Public RFC forum (`/docs/rfc/`) | Public | Required (the RFC is the summary) |
| `security_issue` | Private security channel (see SECURITY.md) | Private with public advisory | Required at disclosure |
| `supportability_issue` | Public issue tracker | Public | Recommended |
| `supportability_escalation` | Private support channel | Private support only | None |
| `docs_truth_defect` | Public issue tracker | Public | Recommended |
| `design_review_issue` | Public issue tracker / design forum | Public | Recommended |
| `accessibility_defect` | Public issue tracker | Public | Required |
| `compatibility_regression` | Public issue tracker | Public | Required |
| `waiver_request` | Governance packet queue | Public summary required | Required |
| `benchmark_dispute` | Benchmark council queue | Public sanitised summary on close | Required |
| `private_partner_case` | Private partner channel | Private partner only | Sanitised public summary on close, when applicable |
| `design_partner_case` | Private partner channel | Private partner only | Sanitised public summary on close, when applicable |

A class not in this list is still admitted via the cross-milestone
matrix — the table above lists only the classes the M3 community lane
makes explicit. New classes MUST land in
`artifacts/governance/issue_routing.yaml` and be referenced here in
the same change set.

## Public-private transitions

A sensitive route MUST NOT flip into a public lane without one of the
named disclosure transitions in
`artifacts/governance/issue_routing.yaml`. The validator confirms each
transition id below resolves to a `disclosure_transitions[]` row:

- `private_security_to_public_advisory` — security issue → public
  advisory at disclosure.
- `private_partner_to_public_sanitised_summary` — partner case → a
  sanitised public summary on close, when applicable.
- `private_support_to_public_docs_truth` — supportability escalation
  → public docs-truth or release-notes update when the fix changes
  shared truth.
- `public_to_private_reclassification` — a public report flips
  private when it turns out to contain partner or security content.
- `private_support_to_private_security` — a support escalation flips
  to security when the incident is in scope for the security route.

Cross-posting between public and private lanes without a transition
row is a routing-matrix failure, not a maintainer judgment call.

## M3 truth vocabulary contributors read from

Match these vocabularies whenever a report or RFC cites support
classes, lifecycle labels, or freshness chips. Inventing parallel
"verified", "GA", or "stable" copy fails the docs / public-truth
freshness gate and the public-proof shelf-life policy.

- Lifecycle labels: `preview`, `beta`
- Release channels: `nightly`, `preview`, `beta`, `stable`
- Support classes: `certified`, `supported`, `limited`,
  `experimental`, `community`, `retest_pending`, `evidence_stale`,
  `unsupported`
- Downgrade states: `retest_pending`, `limited`, `policy_blocked`,
  `unsupported`, `evidence_stale`

These come from
[`artifacts/milestones/m3/claimed_surface_register.json`](../../../artifacts/milestones/m3/claimed_surface_register.json)
and
[`artifacts/milestones/m3/cohort_guardrails.yaml`](../../../artifacts/milestones/m3/cohort_guardrails.yaml).

## Known-limits vocabulary

"Missing capability" reports route through the known-limits
vocabulary. Do not propose a parallel "beta-only scope" register.

- Cross-milestone known-limits contract:
  [`docs/product/known_limits_contract.md`](../../product/known_limits_contract.md)
- External-alpha known-limits packet (still in scope for beta intake):
  [`artifacts/feedback/external_alpha_known_limits.md`](../../../artifacts/feedback/external_alpha_known_limits.md)

## Escalation

When a routing decision is unclear, escalate through the named owning
forum named in `artifacts/governance/issue_routing.yaml`; for tie-
breakers, use the decision-rights matrix.

- Decision rights and signoff matrix:
  [`docs/governance/decision_rights_and_signoff_matrix.md`](../../governance/decision_rights_and_signoff_matrix.md)
- Cross-milestone routing matrix narrative:
  [`docs/governance/issue_routing_matrix.md`](../../governance/issue_routing_matrix.md)

## How to verify

This pack is governed by `ci/check_m3_beta_enablement_starter_pack.py`.
Run the validator and refresh the capture in the same change set when
any issue-class binding, disclosure transition, or truth vocabulary
reference changes:

```
python3 ci/check_m3_beta_enablement_starter_pack.py --repo-root .
```

Use `--check` in CI to fail when the capture on disk would drift:

```
python3 ci/check_m3_beta_enablement_starter_pack.py --repo-root . --check
```
