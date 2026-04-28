# Normative Requirement Policy

This policy is the interpretation layer for Aureline requirements,
waivers, verification evidence, and mandatory review artifacts. It does
not rewrite the source specifications; it tells reviewers and automation
how to decide what is binding, what is guidance, and what is only an
example.

Machine-readable companions:

- [`/artifacts/governance/requirement_lifecycle_states.yaml`](../../artifacts/governance/requirement_lifecycle_states.yaml)
  defines requirement families, lifecycle states, owner and waiver
  authority resolution, and stale-after-change behavior.
- [`/artifacts/governance/verification_classes.yaml`](../../artifacts/governance/verification_classes.yaml)
  defines the canonical verification class ids consumed by requirement
  rows, verification packets, scorecards, and signoff checks.
- [`/artifacts/governance/mandatory_review_artifacts.yaml`](../../artifacts/governance/mandatory_review_artifacts.yaml)
  defines when ADRs, RFCs, design packets, verification packets,
  compatibility reports, benchmark reports, and waiver packets are
  mandatory and what each must contain.

Related control artifacts:

- [`/artifacts/governance/requirement_register_seed.yaml`](../../artifacts/governance/requirement_register_seed.yaml)
  is the canonical requirement-id register.
- [`/docs/governance/requirement_alias_crosswalk.md`](./requirement_alias_crosswalk.md)
  explains how local labels resolve back to canonical requirement ids.
- [`/docs/governance/verification_packet_template.md`](./verification_packet_template.md)
  is the reviewer-facing packet template.
- [`/docs/governance/templates/waiver_template.md`](./templates/waiver_template.md)
  is the waiver authoring template.
- [`/docs/governance/evidence_freshness_policy.md`](./evidence_freshness_policy.md)
  defines proof freshness and stale-propagation behavior.

## Scope

This policy applies to:

- canonical requirement rows;
- requirement aliases and local scorecard or packet labels;
- ADRs and RFCs that satisfy, refine, narrow, or put requirements at
  risk;
- waivers and exception packets;
- verification, benchmark, compatibility, release, and signoff packets;
- product, architecture, technical design, and UX source documents when
  a reviewer needs to classify a row as normative, advisory, or
  illustrative.

This policy does not create product scope on its own. A new product
obligation still needs a canonical requirement row, a source anchor, an
owner, an evidence owner, and a verification path.

## Precedence

When artifacts disagree, reviewers apply this order:

1. Canonical requirement rows derived from approved PRD requirement IDs,
   plus active approved waivers that cite those rows.
2. Approved ADRs and accepted RFCs that close as ADRs.
3. Technical architecture and technical design rules that are not
   contradicted by a higher artifact.
4. UX specification contracts and design-system contracts for the
   affected surface, interaction, accessibility, or token behavior.
5. Templates, examples, guidance prose, and non-normative
   recommendations.
6. Implementation convenience.

An active waiver does not delete or rewrite a requirement. It creates a
time-bounded, scoped exception to the cited requirement. If the waiver is
expired, out of scope, missing required fields, or approved by the wrong
forum, it has no release-readiness force.

An ADR or RFC may refine how a requirement is implemented, but it may
not silently widen, narrow, or replace a canonical requirement row. A
change that alters stable public behavior, a protected requirement, or a
public schema must update the requirement register, affected ADRs or
RFCs, and verification plan in the same change set.

## Classification

Every row, paragraph, packet field, or table cell that participates in
review is classified as one of the following.

Use this quick reference before reading source prose:

| Source content | Classification rule |
|---|---|
| Canonical requirement-register row | Normative when the row is `approved`, `in_progress`, `implemented`, `verified`, `waived`, or `deprecated`; non-binding when `proposed`; historical only when `removed`. |
| Requirement ID cited in approved source text | Normative for the cited requirement scope. |
| Uppercase BCP 14 keyword in an approved PRD, architecture, technical design, UX contract, ADR, RFC, schema, or governance policy | Normative, unless the artifact explicitly marks the section as illustrative. |
| Accepted ADR/RFC outcome | Normative for the affected requirement ids and interfaces named by the decision. |
| Active approved waiver | Normative only as a scoped, time-bounded exception to the cited requirement ids. |
| Expired, out-of-scope, incomplete, or wrongly approved waiver | Not valid review evidence; the underlying requirement remains blocking when release-bearing. |
| Templates, samples, mock packets, and example values | Illustrative unless a schema, catalog, or requirement row promotes the field or value. |
| Rationale, strategy, lower-case recommendation, or implementation note | Advisory unless tied to a requirement id or accepted decision. |
| Implementation convenience | Never overrides an approved requirement, decision, contract, or waiver rule. |

### Normative

Treat content as normative when any of these are true:

- it is a canonical requirement row or directly cites a canonical
  `Requirement ID`;
- it is an uppercase BCP 14 keyword statement (`MUST`, `MUST NOT`,
  `SHOULD`, `SHOULD NOT`, or `MAY`) inside an approved requirements,
  architecture, technical design, UX contract, ADR, RFC, schema, or
  governance policy artifact;
- it is a machine-readable control-artifact field that a schema or
  policy marks as required;
- it is an active approved waiver or exception packet within its stated
  scope and expiry;
- it is a mandatory review artifact outcome accepted by the authorized
  forum.

Normative content can block merge, milestone close, claim publication,
or release promotion.

### Advisory

Treat content as advisory when it guides implementation but lacks a
canonical requirement id, BCP 14 keyword, required schema field, or
accepted decision outcome. Advisory content includes strategy prose,
recommended approaches, non-blocking rationale, and lowercase "should"
phrasing outside a normative row.

Advisory content can justify a review question or follow-up, but it does
not block release on its own.

### Illustrative

Treat content as illustrative when it is explicitly an example,
placeholder, sample value, mock packet, diagram, or explanatory scenario.
Illustrative content does not create a requirement unless another
normative artifact promotes it.

Examples that show a required field name or enum value remain
illustrative unless the field or enum is also defined by a schema,
machine-readable vocabulary, or requirement row.

## Requirement Rows

Promotion-grade requirement rows must resolve:

- `requirement_id`, using the `<CLASS>-<DOMAIN>-<NNN>` pattern;
- requirement family from the lifecycle-state catalog (`FR`, `LANG`,
  `PERF`, `REL`, `SEC`, `A11Y`, `ECOS`, `AI`, `OPS`, `GOV`,
  `ARCH`, `TOOL`, `CERT`, `ENT`, `REPO`, or `COMP`);
- lifecycle state from the lifecycle-state catalog;
- primary owner;
- evidence owner;
- waiver authority, resolved by the family or the row's governing forum;
- one or more verification class ids from the verification-class
  catalog;
- source anchors;
- current evidence refs or an active waiver.

Aliases, scorecard calls, packet labels, local fitness rows, and legacy
ids are not binding identities. They must resolve back to canonical
requirement ids before they can appear in waivers, signoff automation, or
release packets.

Protected fitness ids, dashboard row ids, and packet-local ids are
evidence handles. They can substantiate a requirement, but a waiver or
release packet must still cite the canonical requirement id that the
handle maps to.

## Lifecycle

The canonical lifecycle states are:

| State | Review meaning | Release effect |
|---|---|---|
| `proposed` | Candidate obligation under discussion. | Not release-blocking until approved. |
| `approved` | Binding planning and design requirement. | Blocks claimed scope unless implemented, verified, or actively waived. |
| `in_progress` | Implementation or rollout is underway. | Not sufficient for milestone or release close. |
| `implemented` | Code, content, or process landed, but accepted proof is missing or stale. | Blocks release-bearing claims until verified or actively waived. |
| `verified` | Required proof exists, is accepted, and is fresh for the current scope. | May support release, milestone close, or public claims. |
| `waived` | Requirement is not met in the scoped area, with accepted risk and expiry. | May pass only inside the waiver scope and only before expiry. |
| `deprecated` | Still supported, but scheduled for retirement or replacement. | Must carry support-window and replacement truth. |
| `removed` | No longer shipped or no longer normative. | Historical record only. |

Older source text and schemas may still mention `draft` or `replaced`.
For review, `draft` normalizes to `proposed`. `replaced` is historical
supersession bookkeeping and must resolve to a current replacement row or
to `removed` for release-gating purposes.

### Stale Verified Requirements

`verified` means current proof, not past proof. A verified requirement is
treated as stale when:

- its evidence packet is past `captured_at + stale_after`;
- a named rerun trigger fires;
- the requirement text, source anchor, verification class, protected
  interface, corpus, environment, dependency, or release scope changes;
- an incident or regression contradicts the accepted proof.

If implementation still exists but proof is stale, signoff automation
treats the row as `implemented` until fresh proof is accepted. If the
scope or requirement meaning changed, the row returns to `approved` or
`in_progress` review and needs re-approval before it can be verified
again.

Stale proof on a release-blocking row is a release blocker unless an
active waiver cites the affected requirement id, exact scope, risk,
mitigation, owner, expiry, and planned exit.

## Verification Classes

Requirement rows and evidence packets use the class ids in
[`verification_classes.yaml`](../../artifacts/governance/verification_classes.yaml).
The current canonical ids are:

| Class id | Meaning |
|---|---|
| `BENCH` | Benchmark, trace, protected fitness, or performance-lab proof. |
| `CONF` | Conformance, interoperability, schema, contract, or functional-suite proof. |
| `RELTEST` | Failure, recovery, rollback, corruption, or fault-injection proof. |
| `SECREV` | Security, privacy, threat-model, fuzzing, policy, or red-team proof. |
| `A11YTEST` | Accessibility, assistive-tech, keyboard, IME, locale, contrast, or motion proof. |
| `UXVAL` | Design QA, usability, workflow validation, or design-partner signoff. |
| `OPSVAL` | Operational, outage, supportability, release, or field-recovery proof. |
| `DOCVAL` | Documentation, schema-publication, migration-note, sample, or SDK-doc proof. |

Packets may carry multiple classes. A reviewer may not substitute a
weaker class for a stronger one without a waiver. For example, a UX
review does not replace a security review, and a benchmark dashboard
does not replace a release evidence packet.

## Mandatory Review Artifacts

The mandatory review-artifact catalog defines the minimum artifact class
for each change type. The short rule is:

- ADRs are required for architecture invariants, storage or schema
  changes, runtime boundaries, trust-model changes, and durable
  protected-lane contracts.
- RFCs are required for broad subsystem changes spanning multiple teams,
  packages, or long-lived public surfaces before they close as ADRs.
- Design packets are required for launch-critical UX surfaces,
  state-model changes, degraded paths, accessibility-sensitive behavior,
  or workflow changes that need visual and interaction evidence.
- Verification packets are required for milestone exit, release
  candidates, major graduation, claim-bearing rows, and accepted proof
  for implemented requirements.
- Compatibility reports are required for public interface, SDK,
  file-format, importer, bridge, support-class, or reference-workspace
  claims.
- Benchmark reports are required for performance-sensitive features,
  protected metrics, regression budgets, startup, typing, indexing,
  memory, energy, or release gates.
- Waiver packets are required for any accepted bypass of a requirement,
  protected behavior, release blocker, or owner requirement.

Missing mandatory artifacts make the change not reviewable. A reviewer
should ask for the artifact rather than treating narrative prose or a PR
comment as an equivalent substitute.

## Waivers

Every waiver must carry:

- stable waiver id;
- cited canonical requirement ids;
- exact scope;
- reason and justification;
- user, enterprise, security, reliability, accessibility, performance,
  or architectural risk;
- mitigation;
- primary owner;
- waiver authority or approving forum;
- evidence owner when evidence refresh is part of closure;
- approving forum or approvers;
- expiry date;
- planned exit;
- escalation path;
- public disclosure posture when user-facing or enterprise-relevant.

Waivers for `SEC`, `REL`, `PERF`, `A11Y`, or architecture-invariant rows
must not exceed 90 days or one minor release train without escalation to
the release council or shiproom forum named by the forum matrix.

A second waiver or renewal on the same protected path, same requirement,
or same protected metric triggers correction review. Correction review
must choose one of: close the waiver with fresh proof, narrow the claim,
rebaseline the scope, or open explicit correction work with a dated exit.

Expired waivers fail closed:

- the waived requirement is no longer release-ready;
- milestone and release packets must mark the row blocked or narrowed;
- public and partner-facing claims must narrow or be held;
- renewal requires the same or higher authority as the original waiver;
- a renewal cannot silently inherit the previous risk acceptance.

## Review Automation Rules

Automation and reviewers read the machine-readable catalogs in this
order:

1. Resolve requirement ids and aliases through the requirement register
   and crosswalk.
2. Resolve lifecycle state and stale-after-change behavior through the
   lifecycle-state catalog.
3. Resolve verification class ids through the verification-class catalog.
4. Resolve mandatory artifact classes through the review-artifact
   catalog.
5. Resolve proof freshness through the evidence freshness policy and SLO
   catalog.
6. Resolve waiver scope, expiry, and escalation through this policy, the
   waiver template, ownership matrix, and forum matrix.

When metadata is missing, ambiguous, expired, or cannot be resolved,
automation fails closed for scorecard, signoff, claim-bearing, and
release uses.
