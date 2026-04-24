# Issue-and-RFC routing matrix, disclosure states, and escalation rules

This document is the normative narrative companion to the
machine-readable routing matrix in
[`/artifacts/governance/issue_routing.yaml`](../../artifacts/governance/issue_routing.yaml).
It binds every class of community bug, performance regression,
docs-truth defect, benchmark dispute, compatibility regression,
design-review issue, public RFC, security issue, private partner /
design-partner case, and supportability escalation to **one** default
route, **one** privacy / disclosure posture, **one** redaction posture,
**one** public-summary expectation, and **one** owning forum chain
before the project widens its public contribution or benchmark /
public-proof lanes.

If this document and the YAML disagree, the YAML wins for tooling and
the narrative MUST be updated in the same change.

Companion artifacts:

- [`/artifacts/governance/issue_routing.yaml`](../../artifacts/governance/issue_routing.yaml)
  — machine-readable matrix, route-class, privacy-class,
  disclosure-class, redaction-class, and escalation-path vocabularies.
- [`/fixtures/governance/issue_routes/`](../../fixtures/governance/issue_routes/)
  — one worked example per seeded issue class, showing end-to-end
  routing.
- [`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml)
  — source of truth for forum ids that the matrix cites.
- [`/artifacts/governance/forum_matrix.yaml`](../../artifacts/governance/forum_matrix.yaml)
  — forum cadences, packet profiles, and escalation chains.
- [`/docs/security/intake_and_triage.md`](../security/intake_and_triage.md)
  — concrete private-security intake path, PGP fingerprint, and
  acknowledge-by-local-clock offset.
- [`/docs/governance/benchmark_council_charter.md`](./benchmark_council_charter.md)
  — benchmark council voting rules and dispute handling.
- [`/docs/governance/dogfood_issue_taxonomy.md`](./dogfood_issue_taxonomy.md)
  — dogfood sub-taxonomy referenced inside the class rows below.
- [`/CONTRIBUTING.md`](../../CONTRIBUTING.md) — contributor-facing
  "where do I file this?" entry point.
- [`/SECURITY.md`](../../SECURITY.md) — public security-contact page.

## Why this matrix exists

Before Aureline opens wider public contribution or starts publishing
benchmark and public-proof material, every incoming report has to
resolve mechanically to a lane without tribal knowledge. Two failure
modes have to be impossible:

1. A sensitive report (security, partner, live support bundle) silently
   leaking into a public tracker because the contributor did not know
   the right lane.
2. A community report (bug, regression, docs defect, RFC) disappearing
   into a private channel because no public lane looked "official."

The matrix resolves both by forcing every class to name its default
route, privacy posture, redaction posture, summary expectation, and
owning forum up front, and by making every change of disclosure
posture cite a named transition rule.

## The five things every class names

Every row in `issue_classes` names five required fields plus an
owning forum:

1. **`default_route_class`** — the lane a newly-filed report lands in
   by default. Seven values: `public_issue_tracker`,
   `public_rfc_forum`, `private_security_channel`,
   `private_partner_channel`, `private_support_channel`,
   `benchmark_council_queue`, `governance_packet_queue`.
2. **`privacy_class`** — whether the raw report may circulate
   publicly. Five values: `public`, `private_with_public_advisory`,
   `private_with_public_summary`, `private_partner_only`,
   `private_support_only`.
3. **`disclosure_class`** — when (if ever) a public surface is
   expected. Five values: `public_immediate`, `public_on_fix`,
   `public_on_advisory`, `public_sanitised_summary_on_fix`,
   `private_indefinite`.
4. **`public_summary_expectation`** — whether a public-facing summary
   is `required`, `recommended`, `none`, or `forbidden`.
5. **`redaction_class`** — how attached artefacts (logs, bundles,
   screenshots, repro cases) are redacted on the route. Six values:
   `field_safe_default`, `field_safe_with_route_metadata`,
   `security_redaction_raw_allowed_under_pgp`,
   `partner_contractual_redaction`,
   `support_bundle_redaction_profile`, `no_raw_attachments`.

Plus `owning_forum`, which cites a forum id from
`ownership_matrix.decision_forums[]` so that the report has a named
home without placeholder "tbd_*" sentinels.

A row that leaves any of these blank is a validation failure.

## Route-class semantics

- **`public_issue_tracker`** — the project's public issue tracker on
  the repository host. Default lane for OSS bugs, performance
  regressions, docs-truth defects, design-review issues,
  accessibility defects, compatibility regressions, and
  non-sensitive supportability issues.
- **`public_rfc_forum`** — the `/docs/rfc/` pull-request process.
  Default lane for public RFCs; the RFC itself is the public summary.
- **`private_security_channel`** — the private intake path in
  `SECURITY.md`. The only lane where raw secret material, raw
  exploit payloads, and raw trust-root bytes are acceptable (under
  the published PGP key).
- **`private_partner_channel`** — the partner's contractual channel
  under the signed partner or design-partner agreement. Default lane
  for `private_partner_case` and `design_partner_case`.
- **`private_support_channel`** — the private support intake path
  for dogfood, design-partner, and paying-customer cases that carry
  live device, account, or workspace content. Default lane for
  `supportability_escalation`.
- **`benchmark_council_queue`** — the intake reviewed by the
  benchmark council. Default for benchmark disputes and
  protected-fitness waiver requests.
- **`governance_packet_queue`** — the governance-packet pull-request
  lane for waivers, freeze exceptions, and governance-truth defects
  that need a packet trail rather than a live bug thread.

Every private route sets
`requires_explicit_disclosure_transition_before_public: true`. Tooling
or reviewers MUST NOT move a report out of a private route without
citing a row from `disclosure_transitions`.

## Privacy-class semantics

| privacy_class                        | raw report public? | public surface type                |
|--------------------------------------|--------------------|------------------------------------|
| `public`                             | yes                | raw report                         |
| `private_with_public_advisory`       | no                 | published advisory                 |
| `private_with_public_summary`        | no                 | sanitised public summary           |
| `private_partner_only`               | no                 | none unless transition fires       |
| `private_support_only`               | no                 | none unless transition fires       |

## Disclosure-class semantics

| disclosure_class                      | when does a public surface appear?                         |
|---------------------------------------|------------------------------------------------------------|
| `public_immediate`                    | at filing                                                  |
| `public_on_fix`                       | when the fix ships, via release notes or docs-truth family |
| `public_on_advisory`                  | when the coordinated advisory publishes                    |
| `public_sanitised_summary_on_fix`     | at fix time, via a sanitised summary with consent on file  |
| `private_indefinite`                  | never by default; only via a recorded disclosure transition |

## Redaction-class semantics

- **`field_safe_default`** — standard public-tracker redaction. Strip
  tokens, keys, hostnames, and PII per the ADR-0007 defaults before
  attaching.
- **`field_safe_with_route_metadata`** — public-tracker redaction
  plus route / origin / target / build metadata preserved for dogfood
  and compatibility diagnosis.
- **`security_redaction_raw_allowed_under_pgp`** — raw secrets and
  exploit payloads may be attached only under the published PGP key
  and the intake runbook. Public mirrors MUST strip raw bytes before
  any cross-post.
- **`partner_contractual_redaction`** — partner-provided artefacts
  obey the partner NDA; aureline-side summaries redact partner
  identity unless explicit written consent is on record.
- **`support_bundle_redaction_profile`** — the support-bundle
  `metadata_safe_default` redaction profile applied to any attached
  support bundle per the support-bundle contract.
- **`no_raw_attachments`** — raw artefacts MUST NOT be attached;
  references (hashes, tickets, bundle ids) are allowed.

## Disclosure-state transitions (who can change privacy)

Every change of privacy_class from a private posture to a public
posture (or the reverse) MUST cite one of the transitions below.
Tooling rejects close, label, or cross-post actions that would
effectively publicise a private-class report without such a row.

### `private_security_to_public_advisory`
- **From → to:** `private_with_public_advisory` → `public`.
- **Authorised forums:** `security_trust_review`.
- **Authorised roles:** security DRI, release-council chair for an
  embargoed release.
- **Required records:** published advisory in `/docs/security/` and a
  release-note or release-packet entry.
- **Required preconditions:** fix or mitigation shipped (or embargo
  agreed) and reporter notified of planned disclosure.
- **Forbidden shortcuts:** announcing content via a public issue
  comment or social-media post before the advisory is live.

### `private_partner_to_public_sanitised_summary`
- **From → to:** `private_partner_only` →
  `private_with_public_summary`.
- **Authorised forums:** `compatibility_ecosystem_review`,
  `product_scope_review`.
- **Authorised roles:** partner DRI, docs-public-truth DRI.
- **Required records:** partner's written consent on file; docs
  public-truth update or release note.
- **Required preconditions:** sanitised summary reviewed by partner.
- **Forbidden shortcuts:** docs updates without partner consent; a
  "sanitised" summary that still reveals partner identity.

### `private_support_to_public_docs_truth`
- **From → to:** `private_support_only` →
  `private_with_public_summary`.
- **Authorised forums:** `open_community_sync`,
  `product_scope_review`.
- **Authorised roles:** docs-public-truth DRI, supportability DRI.
- **Required records:** reporter consent recorded; docs-public-truth
  update.
- **Required preconditions:** personally identifying content removed
  (per the support-bundle redaction profile).
- **Forbidden shortcuts:** copying raw support-bundle content into
  the public tracker.

### `public_to_private_reclassification`
- **From → to:** `public` → `private_with_public_advisory`.
- **Authorised forums:** `security_trust_review`.
- **Authorised roles:** security DRI.
- **Required records:** reclassification note in the decision index;
  public issue locked or redacted with a visible pointer to the
  advisory.
- **Required preconditions:** reviewer believes the raw report
  enables exploitation.
- **Forbidden shortcuts:** silently deleting the public issue.

### `private_support_to_private_security`
- **From → to:** `private_support_only` →
  `private_with_public_advisory`.
- **Authorised forums:** `security_trust_review`.
- **Authorised roles:** security DRI, supportability DRI.
- **Required records:** handoff note in the private triage workspace.
- **Required preconditions:** supportability reviewer suspects a
  security impact.
- **Forbidden shortcuts:** handing a raw support bundle to an
  engineer outside the private security channel.

## Escalation paths from each route

Each route_class names three chains that reports can climb:

- **Primary forums** — the first forum that owns triage on this
  lane.
- **Topical forums** — domain owners (architecture, security,
  release, support, accessibility, performance, community) the
  primary forum cites when the report touches their turf.
- **Release-gate forums** — the release-facing forums that must
  record an outcome before a release ships a fix.

The full chains live in `escalation_paths` in the YAML; the table
below summarises the primary plus the most common topical route.

| route_class                    | primary forum                      | typical topical forums                                                                 | release-gate forums                             |
|--------------------------------|------------------------------------|----------------------------------------------------------------------------------------|-------------------------------------------------|
| `public_issue_tracker`         | `architecture_council`             | `performance_council`, `accessibility_review`, `compatibility_ecosystem_review`, `product_scope_review`, `open_community_sync` | `release_council`, `shiproom_executive_scope_review` |
| `public_rfc_forum`             | `architecture_council`             | `security_trust_review`, `compatibility_ecosystem_review`, `accessibility_review`, `performance_council`, `product_scope_review` | `release_council`                               |
| `private_security_channel`     | `security_trust_review`            | `architecture_council`, `compatibility_ecosystem_review`                                | `release_council`, `shiproom_executive_scope_review` |
| `private_partner_channel`      | `compatibility_ecosystem_review`   | `security_trust_review`, `accessibility_review`, `performance_council`, `product_scope_review` | `release_council`, `shiproom_executive_scope_review` |
| `private_support_channel`      | `product_scope_review`             | `security_trust_review`, `accessibility_review`, `compatibility_ecosystem_review`, `open_community_sync` | `release_council`, `shiproom_executive_scope_review` |
| `benchmark_council_queue`      | `performance_council`              | `architecture_council`, `compatibility_ecosystem_review`, `open_community_sync`         | `release_council`, `shiproom_executive_scope_review` |
| `governance_packet_queue`      | `architecture_council`             | `security_trust_review`, `performance_council`, `accessibility_review`, `compatibility_ecosystem_review`, `product_scope_review`, `open_community_sync` | `release_council`, `shiproom_executive_scope_review` |

## Seeded issue classes (at a glance)

The matrix seeds fifteen classes. Contributors pick a class, the
matrix resolves the rest.

| id                          | default_route             | privacy                           | disclosure                    | summary      | owning_forum                      |
|-----------------------------|---------------------------|-----------------------------------|-------------------------------|--------------|-----------------------------------|
| `oss_bug`                   | `public_issue_tracker`    | `public`                          | `public_immediate`            | recommended  | `architecture_council`            |
| `perf_regression`           | `public_issue_tracker`    | `public`                          | `public_immediate`            | required     | `performance_council`             |
| `rfc`                       | `public_rfc_forum`        | `public`                          | `public_immediate`            | required     | `architecture_council`            |
| `security_issue`            | `private_security_channel`| `private_with_public_advisory`    | `public_on_advisory`          | required     | `security_trust_review`           |
| `supportability_issue`      | `public_issue_tracker`    | `public`                          | `public_on_fix`               | recommended  | `product_scope_review`            |
| `supportability_escalation` | `private_support_channel` | `private_support_only`            | `private_indefinite`          | none         | `product_scope_review`            |
| `docs_truth_defect`         | `public_issue_tracker`    | `public`                          | `public_immediate`            | required     | `product_scope_review`            |
| `design_review_issue`       | `public_issue_tracker`    | `public`                          | `public_on_fix`               | recommended  | `accessibility_review`            |
| `accessibility_defect`      | `public_issue_tracker`    | `public`                          | `public_on_fix`               | required     | `accessibility_review`            |
| `compatibility_regression`  | `public_issue_tracker`    | `public`                          | `public_on_fix`               | required     | `compatibility_ecosystem_review`  |
| `waiver_request`            | `governance_packet_queue` | `public`                          | `public_immediate`            | required     | `architecture_council`            |
| `benchmark_dispute`         | `benchmark_council_queue` | `public`                          | `public_on_fix`               | required     | `performance_council`             |
| `private_partner_case`      | `private_partner_channel` | `private_partner_only`            | `private_indefinite`          | none         | `compatibility_ecosystem_review`  |
| `design_partner_case`       | `private_partner_channel` | `private_partner_only`            | `private_indefinite`          | none         | `compatibility_ecosystem_review`  |

## Consistency rules drift scans must enforce

The matrix names five invariants. Tooling that mutates issue state
MUST check them; CI MUST fail when a surface violates any of them.

1. **`rule:private_class_never_silently_public`** — any issue whose
   current `privacy_class` resolves to `private_*` MUST cite a
   `disclosure_transitions.id` when its `privacy_class` moves to
   `public`.
2. **`rule:public_summary_forbidden_requires_transition`** — issues
   whose class has `public_summary_expectation` = `none` or
   `forbidden` cannot produce a public summary without a recorded
   `disclosure_transitions` row authorising the summary.
3. **`rule:security_route_owns_raw_secret_payloads`** — raw secret
   material, raw exploit payloads, and raw trust-root bytes only
   belong on `private_security_channel` with the
   `security_redaction_raw_allowed_under_pgp` posture. Any other
   route that receives such content MUST reclassify via
   `public_to_private_reclassification` or
   `private_support_to_private_security` before the content is
   circulated further.
4. **`rule:partner_identity_requires_consent`** — partner identity
   MUST NOT appear in a public summary unless
   `partner_written_consent_on_file` is recorded.
5. **`rule:release_gate_co_sign`** — when an issue class sits in a
   release-blocking forum chain
   (`escalation_paths.release_gate_forums`), a release cannot go
   without that forum's recorded outcome on the issue.

## How to read the matrix as a contributor

1. Pick the row in the **contributor chooser** (YAML:
   `contributor_chooser.options[]`) that matches what you are trying
   to report. Example labels: "I think this is a security issue",
   "Previously-working importer / bridge / SDK / migration broke",
   "I need to share a support bundle / device log / account content".
2. The matching `issue_class` row names the route, privacy /
   disclosure posture, redaction posture, summary expectation, and
   owning forum. That is where you file.
3. If you are unsure whether a report is security-sensitive, default
   to `security_issue` (private, public-on-advisory). The security
   DRI can reclassify downward via
   `public_to_private_reclassification` or leave the report where it
   is. It is never the contributor's job to decide that a report was
   not serious enough for the private channel.

## How to read the matrix as a reviewer

1. Resolve the issue class first. If the class is wrong, fix the
   class (never the disclosure state) unless a transition row
   authorises it.
2. Read the `disclosure_class` and `public_summary_expectation`. Any
   change to disclosure posture without a recorded transition row is
   a validation failure; reject the action and cite the rule that
   fails.
3. If the report needs more than one forum, follow
   `escalation_paths` for the route_class. The primary forum owns
   coordination; topical forums co-sign on their domain; release-gate
   forums record the release outcome.
4. If a sensitive route (security, partner, support) receives content
   that should have come through a different sensitive route (for
   example a support case with security impact), cite
   `private_support_to_private_security` or the matching transition
   before the content is circulated further.

## How to read the matrix as a release reviewer

The release council and shiproom cannot ship a release if an issue
sitting in a release-blocking forum chain does not yet have that
forum's recorded outcome. The matrix records the chain; the
forum-matrix records the required output artefact (release packet,
advisory, accessibility packet, compatibility report, etc.).

## Out of scope at this revision

- Live integration with GitHub, Linear, Jira, or any other tracker.
  The matrix describes routes and transitions; it does not run them.
- Staffing moderation queues for the public issue tracker.
- Naming SLOs on triage response time; that belongs in the support
  blocker-aging SLA document and the private security intake
  runbook.
- Specific partner or design-partner identities.
