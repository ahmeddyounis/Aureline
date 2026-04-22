# Forum Packet Templates

This guide defines the required input-packet profiles and output landing
rules for Aureline's standing decision forums. It does not create a new
packet schema family. Each profile below reuses the repo's existing
artifact homes.

Companion artifacts:

- [`/artifacts/governance/forum_matrix.yaml`](../../artifacts/governance/forum_matrix.yaml)
  — authoritative packet-profile ids and forum-to-profile bindings.
- [`./forum_charters.md`](./forum_charters.md) — narrative charter pack.
- [`./decision_workflow.md`](./decision_workflow.md) — ADR/RFC and
  narrowing workflow.
- [`./templates/waiver_template.md`](./templates/waiver_template.md) and
  [`./templates/exception_packet_template.md`](./templates/exception_packet_template.md)
  — waiver and exception packet skeletons.
- [`./verification_packet_template.md`](./verification_packet_template.md)
  — shared verification and evidence-link posture.
- [`../accessibility/a11y_ime_packet_template.md`](../accessibility/a11y_ime_packet_template.md)
  — accessibility packet skeleton reused by `accessibility_review`.

## Common minimum fields

Every forum packet or note must name:

- the `forum_id`
- the `packet_profile_id`
- the review window or meeting date
- the owner or chair
- the affected lane ids and requirement ids
- the input refs reviewed
- the output artifact refs updated
- whether the outcome was `accept`, `reject`, `defer`, `narrow`, or
  `rebaseline`
- the escalation path if the forum could not close the item

## Profile summary

| Packet profile id | Primary forums | Default output landing |
|---|---|---|
| `architecture_decision_bundle` | `architecture_council` | `decision_index.yaml` plus ADR/RFC |
| `performance_readout_bundle` | `performance_council` | scorecard, perf ledgers, benchmark packets |
| `security_trust_packet` | `security_trust_review` | decision row, waiver/exception packet, security artifacts |
| `accessibility_review_packet` | `accessibility_review` | accessibility packet family and scorecard |
| `compatibility_review_packet` | `compatibility_ecosystem_review` | compatibility or migration packet, docs/support truth |
| `milestone_scope_packet` | `product_scope_review` | milestone scorecard, decision row, exception packet |
| `community_sync_note` | `open_community_sync` | public docs or README update with linked evidence |
| `release_readiness_packet` | `release_council`, `shiproom_executive_scope_review` | release packet family and claim or scorecard posture |

## `architecture_decision_bundle`

Use when a protected-lane contract, stable-surface rule, schema,
storage boundary, or other ADR-bound decision is on the table.

The input packet must include:

- the `decision_id` row from
  [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
- an ADR draft or an RFC draft that will close as an ADR
- source anchors and affected requirement ids
- affected lane ids and any frozen interface rows
- any active waiver or exception that the decision is replacing or
  narrowing

The output must land in:

- the decision row's `decision_history`
- [`/docs/adr/`](../adr/) or [`/docs/rfc/`](../rfc/)
- the milestone scorecard or exception packet when scope changes

This profile is incomplete if the outcome is recorded only in a meeting
note, PR description, or shiproom packet.

## `performance_readout_bundle`

Use for weekly protected-metric review, threshold changes, corpus or
hardware-baseline moves, and performance dispute handling.

The input packet must include:

- the affected protected-metric rows and current threshold posture
- active waivers and expiry dates
- the benchmark corpus or hardware refs involved
- scorecard status for affected lanes
- any release-bearing claim rows or dashboard snapshots touched by the
  outcome

The output must land in at least one of:

- the current milestone scorecard
- a protected-metric or latency-budget ledger under
  [`/artifacts/perf/`](../../artifacts/perf/)
- a benchmark packet or publication artifact under
  [`/docs/benchmarks/`](../benchmarks/)
- a decision row if the forum changed durable policy rather than a
  single run disposition

When the forum approves a waiver, the waiver packet or equivalent record
must carry expiry, mitigation, and escalation. A dashboard screenshot
alone is never a valid output artifact.

## `security_trust_packet`

Use for permission, policy, trust-boundary, incident, and release-
blocking security decisions.

The input packet must include:

- the issue class or decision row in scope
- the affected trust or permission boundary
- release impact and any affected deployment profiles
- mitigation, rollback, and disclosure posture
- any required public-summary or advisory path

The output must land in one or more of:

- a decision row and ADR when the trust contract moved
- a waiver or exception packet when the forum accepted a time-bounded gap
- the canonical security or issue-routing artifact when disclosure rules
  changed
- the release packet when the decision narrowed or blocked a live claim

Security findings may stay private where policy requires it, but the
typed packet or decision record still has to exist.

## `accessibility_review_packet`

Use for accessibility sign-off, accessibility regressions, and any
release exception touching keyboard, screen reader, focus, IME, locale,
or reduced-motion behaviour.

The input packet must include:

- the affected surfaces and workflows
- keyboard, screen-reader, IME, and focus results
- reduced-motion, high-contrast, and locale-impact notes where relevant
- the current lane or scorecard status
- explicit statement of whether the forum is signing off, blocking, or
  accepting a time-bounded exception

The output must land in:

- the accessibility packet family under
  [`/docs/accessibility/`](../accessibility/) or
  [`/artifacts/accessibility/`](../../artifacts/accessibility/)
- the milestone scorecard when the outcome changes release readiness
- the release waiver family when a time-bounded exception is accepted

If the result changes a public claim, the same change set must also
update the release or docs-truth artifact that carries that claim.

## `compatibility_review_packet`

Use for importer cutline, extension bridge posture, SDK churn review,
launch-archetype support posture, or compatibility-report acceptance.

The input packet must include:

- the compatibility or migration rows being reviewed
- any SDK, schema, or bridge diff in scope
- known limits, rollback posture, and support-class language
- the docs or support surfaces that need to change if the outcome lands

The output must land in:

- a compatibility or migration artifact under
  [`/artifacts/compat/`](../../artifacts/compat/) or
  [`/docs/state/`](../state/)
- the decision row and ADR when the underlying contract changed
- docs or support truth when the supported row set or known limits moved

This profile is the default home for ecosystem and compatibility cutline
decisions. It is not valid to close those changes only in roadmap prose.

## `milestone_scope_packet`

Use for weekly cut/add decisions, dependency-health review, commitment-
class movement, explicit claim narrowing, and rebaseline requests.

The input packet must include:

- the affected scorecard lanes and commitment-class movement
- dependency-health, blocker, and waiver context
- the phase-budget posture and whether the requested change fits it
- the user-facing or public-claim impact

The output must land in:

- the milestone scorecard
- a decision row when the cutline becomes durable policy
- an exception packet when the change crosses the current phase budget
- the release packet when the scope change narrows a live claim

Any rebaseline request must name whether it is asking to cut scope, add
capacity, or move dates. "Carry forward quietly" is not an allowed
output.

## `community_sync_note`

Use for public roadmap truth, contributor asks, and governance
transparency updates. This is the only profile here that may close as a
public note rather than a packet family document, but it still has to be
structured.

The input note must include:

- the scorecard, release packet, or decision rows being summarized
- explicit current narrow or unclaimed scope
- contributor asks or public follow-ups that need routing
- next review date or next forum that can unblock the item

The output must land in:

- [`/docs/`](../) or [`/README.md`](../../README.md) as a public summary
- the canonical issue-routing artifact when disclosure or handoff
  expectations changed

This profile may not widen a public promise. If the roadmap narrows, the
note must say so directly and link the packet or scorecard that approved
it.

## `release_readiness_packet`

Use for release-council review and shiproom go or hold decisions.

The input packet must include:

- the release evidence packet or shiproom packet in review
- active waivers and their expiry posture
- compatibility, benchmark, accessibility, and docs-truth status for any
  claim-bearing row touched by the release
- named signoffs, rollback posture, and on-call coverage

The output must land in:

- the release packet family under
  [`/artifacts/release/`](../../artifacts/release/)
- the milestone scorecard when the release outcome changes claim posture
- docs or claim-truth surfaces when the release is held, narrowed, or
  promoted with updated public language

The release-ready output is never just "go". It must leave a typed
record of hold, narrow, or promote, plus the open risks and the named
owner for each unresolved item.
