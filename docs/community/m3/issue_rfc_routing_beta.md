# Beta issue and RFC routing

This is the beta entrypoint for public project issues, public RFCs,
private security reports, partner/design-partner cases, support
escalations, benchmark disputes, and governance packet requests.

It does not create beta-only lanes. The canonical routing source is
[`artifacts/governance/issue_routing.yaml`](../../../artifacts/governance/issue_routing.yaml)
and the narrative cross-milestone companion is
[`docs/governance/issue_routing_matrix.md`](../../governance/issue_routing_matrix.md).
Contributor-facing entry points remain
[`CONTRIBUTING.md`](../../../CONTRIBUTING.md) and
[`SECURITY.md`](../../../SECURITY.md).
The beta open-project packet validates this page against those sources:
[`artifacts/milestones/m3/open_project_beta_packet.md`](../../../artifacts/milestones/m3/open_project_beta_packet.md).

## Filing rule

Choose the issue class first. The class determines route, privacy,
disclosure, redaction, public-summary expectation, and owning forum.
Do not move a private report to a public lane without a named
disclosure transition from the matrix.

## Public project lanes

| Issue class | Default route | Privacy | Public-summary posture | Use for |
|---|---|---|---|---|
| `oss_bug` | `public_issue_tracker` | `public` | `recommended` | Non-security product defects safe to discuss in public. |
| `perf_regression` | `public_issue_tracker` | `public` | `required` | Protected-metric or benchmark regressions with field-safe route metadata. |
| `rfc` | `public_rfc_forum` | `public` | `required` | Public design proposals and broad contract/API discussions. |
| `supportability_issue` | `public_issue_tracker` | `public` | `recommended` | Doctor, support-bundle, recovery, or export defects that do not need private workspace data. |
| `docs_truth_defect` | `public_issue_tracker` | `public` | `required` | Public docs, Help/About, migration, or release-copy truth mismatch. |
| `design_review_issue` | `public_issue_tracker` | `public` | `recommended` | Design-system, component, or UX review defect safe for public discussion. |
| `accessibility_defect` | `public_issue_tracker` | `public` | `required` | Keyboard, screen-reader, IME, focus, or accessibility regression. |
| `compatibility_regression` | `public_issue_tracker` | `public` | `required` | Compatibility, migration, archetype, SDK, or version-skew regression safe for public reporting. |
| `benchmark_dispute` | `benchmark_council_queue` | `public` | `required` | Benchmark methodology, corpus, or protected-fitness challenge. |
| `waiver_request` | `governance_packet_queue` | `public` | `required` | Waiver, freeze exception, or governance packet request. |

## Private lanes

| Issue class | Default route | Privacy | Public-summary posture | Use for |
|---|---|---|---|---|
| `security_issue` | `private_security_channel` | `private_with_public_advisory` | `required` at advisory publication | Vulnerabilities, exploit details, credential exposure, signing/trust-root issues, and security-sensitive payloads. |
| `supportability_escalation` | `private_support_channel` | `private_support_only` | `none` by default | Live workspace, account, device, support-bundle, or tenant context that is not safe for the public tracker. |
| `private_partner_case` | `private_partner_channel` | `private_partner_only` | `none` by default | Partner-contractual reports, NDA-bound evidence, or partner-identity-sensitive issues. |
| `design_partner_case` | `private_partner_channel` | `private_partner_only` | `none` by default | Design-partner or managed-pilot evidence that names private workspaces, users, or partner identity. |

## RFC baseline

Public RFCs use `issue_class = rfc` and `default_route_class =
public_rfc_forum`. The RFC itself is the required public summary.

An RFC belongs in the public lane when it can be reviewed without raw
partner evidence, raw security payloads, customer workspace contents,
private benchmark inputs, or private support bundles. If the motivating
evidence is private, file the private report in its owning lane first
and publish only a sanitized RFC once the owning forum approves the
transition.

## Disclosure transitions

The beta baseline admits only these private/public transitions:

- `private_security_to_public_advisory` - private security report to
  public advisory.
- `private_partner_to_public_sanitised_summary` - partner case to a
  sanitized public summary after partner consent.
- `private_support_to_public_docs_truth` - private support report to a
  public docs-truth or release-note update after reporter consent.
- `public_to_private_reclassification` - public report reclassified
  into the security lane when raw details enable exploitation.
- `private_support_to_private_security` - support escalation moved to
  the security lane when a reviewer suspects security impact.

Cross-posting without one of these transitions is a routing failure.

## Product-local handoff mapping

Help/About, docs browser, migration center, and service-health surfaces
use product-local handoff classes, then map them back to canonical
issue classes before opening a destination:

| Product-local class | Canonical issue class | Route |
|---|---|---|
| `docs_truth_mismatch` | `docs_truth_defect` | `public_issue_tracker` |
| `migration_compatibility_regression` | `compatibility_regression` | `public_issue_tracker` |
| `design_proposal` | `rfc` | `public_rfc_forum` |
| `security_sensitive` | `security_issue` | `private_security_channel` |
| `private_workspace_support` | `supportability_escalation` | `private_support_channel` |

The product-local class is an interaction hint, not a new governance
row. The destination, redaction posture, and disclosure state come from
`issue_routing.yaml`.

## How to verify

Run the open-project beta packet validator:

```sh
python3 ci/check_m3_open_project_beta_packet.py --repo-root .
```

The beta enablement starter pack also checks that this page is the
community entrypoint:

```sh
python3 ci/check_m3_beta_enablement_starter_pack.py --repo-root .
```
