# Canonical assumption ledger

This document is the human-readable companion to the canonical
assumption ledger. The ledger exists so that every milestone-shaping
assumption has one owner, one validation deadline, one validation
checkpoint, one required proof, one default response if it is
falsified, and one named set of downstream consumers — and so that
no program-level premise survives the milestone as hidden team
folklore.

Companion artifacts:

- [`/artifacts/governance/assumption_rows.yaml`](../../artifacts/governance/assumption_rows.yaml)
  — machine-readable ledger. Tooling reads this file; the narrative
  below describes the same rows.
- [`/schemas/governance/assumption_row.schema.json`](../../schemas/governance/assumption_row.schema.json)
  — schema the ledger conforms to. Rows are strict
  `additionalProperties: false` and validate the linkage, lifecycle,
  and waiver rules below.
- [`/artifacts/governance/decision_register.yaml`](../../artifacts/governance/decision_register.yaml)
  — launch decision register. Assumption rows cite launch-register
  ids (`LR-NNNN`) via `linked_launch_register_rows`; launch-register
  rows cite rolling-index and launch-register-local assumption rows
  via their own `linked_*` fields. Program-level assumptions live
  here under `AL-NNNN` ids and are cited by scorecard lanes, risk
  rows, and dependency rows.
- [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  — rolling architecture-ADR decision index. Rows cite rolling-index
  ids (`D-NNNN`) via `linked_decision_index_rows`.
- [`/artifacts/governance/dependency_ledger.yaml`](../../artifacts/governance/dependency_ledger.yaml)
  — program dependency ledger. Rows cite dependency ids directly
  via `linked_dependency_ledger_rows`.
- [`/artifacts/milestones/M0_scorecard.yaml`](../../artifacts/milestones/M0_scorecard.yaml)
  — milestone scorecard. Lanes cite assumption ids directly rather
  than restating the assumption statement.
- [`/artifacts/milestones/M0_risk_register.yaml`](../../artifacts/milestones/M0_risk_register.yaml)
  — risk register. Risk rows cite assumption ids via
  `linked_risk_register_rows` for material framing.
- [`/docs/governance/commitment_and_rebaseline_policy.md`](./commitment_and_rebaseline_policy.md)
  — commitment classes, phase budgets, and rebaseline rules that a
  falsified assumption routes into.
- [`/docs/governance/maintainer_coverage_policy.md`](./maintainer_coverage_policy.md)
  — protected-path maintainer-coverage rules the coverage rows rest
  on.

**One ledger, one assumption id.** Scorecard lanes, risk rows,
dependency rows, and launch-register rows cite `AL-NNNN` directly.
They do not mint a parallel status field. When an assumption fires
its default response, every downstream artifact picks the response
up verbatim from `default_response_if_false.description`.

## Why this ledger exists

The rolling decision index (`decision_index.yaml`) and the launch
decision register (`decision_register.yaml`) hold decision-local
assumptions under `A-NNNN` and `LA-NNNN`. Those rows carry the
premises attached to a specific decision: "renderer direction
resolves into wgpu-class primitives," "launch accessibility scope is
WCAG AA on the keyboard-complete command graph."

The assumption ledger holds a different shape: program-level
premises that shape **the milestone envelope itself** rather than
the internals of any one decision. Whether the launch wedge is
narrow enough to defend. Whether the local core stands without
hosted services. Whether staffing depth holds through launch.
Whether one command-graph identity serves every surface. Whether
exact-build identity joins every artifact family. Whether docs and
help refresh on the build cadence. Whether clean-room and mirror /
offline rebuilds reproduce at parity. Whether protected-path
maintainer coverage holds through launch.

These premises do not close through one ADR or one scope-review
packet. They close through cadence proof — an evidence packet, a
signed-off staffing roster, a release-evidence cycle, a freshness
automation run. Treating them as first-class rows means they can
age, validate, invalidate, defer, or supersede on the same
append-only discipline as decisions; they never disappear into
meeting notes.

## How rows move

Every ledger row moves through a six-state lifecycle, mirroring the
discipline used by the decision registers:

- **`open`** — the assumption is recognised and carries a deadline,
  but has not been validated or invalidated yet. Every seeded row
  begins here.
- **`validated`** — the named proof has landed, `validated_on` is
  set, and the row is closed on the affirmative outcome. The row
  survives as audit; it does not delete.
- **`invalidated`** — the assumption has been falsified,
  `invalidated_on` is set, and the row's
  `default_response_if_false` has fired. Widening the affected
  claim set requires a new assumption row, not a re-open of the
  invalidated row.
- **`deferred`** — the forum has accepted that the assumption
  cannot be validated at its target milestone and has restated
  `validation_deadline` on the history entry. The row remains open
  but its clock is moved.
- **`superseded`** — a later `AL-NNNN` replaces this one. The
  original row is not deleted; its `history` preserves the
  transition and `superseded_by` points at the replacement.
- **`closed_not_applicable`** — the premise no longer shapes any
  active milestone row (for example: the feature the assumption
  underwrites was cut). The row is closed without validation
  because the question is no longer load-bearing.

Rows never leave the file. A row still `open` past its
`validation_deadline` is a validation failure and is surfaced
through the same reviewer surface used for stale evidence.

## Widening posture

Every row declares a widening posture:

- **`blocks_widening`** — while the row is open, no launch or
  milestone claim may widen past the assumption-bounded scope
  without an explicit re-baseline record. The launch wedge, local-
  core non-dependence, staffing depth, one-command-system, exact-
  build identity, clean-room / mirror / offline reproducibility,
  and protected-path maintainer-coverage rows are all blocks-
  widening: a widening that rides on any of them while the row is
  open is a validation failure.
- **`narrows_claims_only`** — the row bounds current claim wording
  but does not block unrelated lane work or unrelated widening. The
  docs / help cadence row is narrows-claims-only: if the freshness
  cadence slips, the row narrows the "help mirrors the running
  build" claim without freezing every lane the docs touch.

## Default-response shapes

Every row names one shape of response that fires if the assumption
is invalidated, and one concrete sentence describing what the
narrowing / deferral / freeze / rebaseline / descope / cut means for
this specific assumption:

- **`narrow_scope`** — reduce committed scope to the sentence in
  `default_response_if_false.description`, and retract the wider
  claim from the claim manifest.
- **`defer_milestone`** — move the closure target out by an
  explicit milestone, with a restated deadline captured on the
  history entry.
- **`freeze_lane`** — block dependent work on the affected lane
  until a replacement assumption lands or the falsification is
  rolled back. The default for protected-path maintainer coverage
  because coverage gaps cannot silently ride the lane.
- **`rebaseline_milestone`** — trigger a milestone rebaseline
  review covering scope, date, or acceptance thresholds. The
  default for launch-bundle staffing depth because a depth slip on
  a Committed lane is a milestone-shape event, not a lane-local
  event.
- **`descope_feature`** — cut a named feature below its current
  commitment class.
- **`open_exception_packet`** — route to a freeze-exception packet
  and record a dated plan.
- **`cut_public_claim`** — retract the affected claim from the
  claim manifest. The default for the docs / help cadence row
  because retracting the claim is the narrowest possible response.

Some default responses require approval from a named forum before
they fire; those forums appear under
`default_response_if_false.requires_approval_from`. When the list
is empty the response fires automatically on invalidation.

## Linkage rules

Every ledger row MUST link to at least one of:

- one or more launch-register rows (`LR-NNNN`),
- one or more rolling-index rows (`D-NNNN`),
- one or more dependency-ledger rows (`progdep.*`),
- one or more scorecard lanes, or
- one or more risk-register rows (`RISK-NNN`).

The schema enforces this. A row with no downstream consumer is a
validation failure — the ledger is a program-control artifact, not
a private notebook.

Downstream consumers cite assumption ids **directly**:

- Scorecards cite `AL-NNNN` against a lane rather than restating
  the assumption statement.
- Risk rows cite `AL-NNNN` to name the material premise their risk
  framing rests on.
- Dependency rows cite `AL-NNNN` to name the premise a dependency
  runs against.
- Launch-register rows cite `AL-NNNN` when the decision's narrower
  default depends on the assumption holding.

A later change that re-frames a row mints a new `AL-NNNN` and
points `superseded_by` at it. The original `AL-NNNN` never
disappears; every scorecard, risk row, or dependency row that once
cited it continues to resolve through the history trail.

## Validation-checkpoint cadence

Every row names one validation checkpoint. A row whose checkpoint
reads "TBD" is a validation failure — rows live in real cadences,
not in planning meetings. The seeded checkpoint classes are:

- **`shiproom_review`** and **`milestone_review`** — program-level
  cadences covering staffing and scope questions.
- **`architecture_council_review`** — cadence for architecture
  shape and command-graph questions.
- **`product_scope_review`** — cadence for launch-wedge and local-
  core questions.
- **`security_trust_review`**, **`accessibility_review`**,
  **`compatibility_ecosystem_review`** — cadences for trust,
  accessibility, and ecosystem questions.
- **`release_council_review`** — cadence for exact-build identity
  and clean-room / mirror / offline reproducibility.
- **`performance_council_review`** — cadence for benchmark and
  performance-shape questions.
- **`open_community_sync`** — cadence for public docs and help
  surfaces.
- **`evidence_refresh`** — non-forum cadence driven by the public-
  truth freshness automation.
- **`backlog_review`** — cadence for planning-only premises.

The list of valid classes is fixed by the schema. Adding a new
cadence is additive-minor and bumps `schema_version`.

## Shiproom and milestone-review projection

Because every row is machine-readable, shiproom and milestone
reviews can project open assumptions directly from
`assumption_rows.yaml` without meeting memory:

- One table of every row whose status is `open`, grouped by
  `owner_forum`, sorted by `validation_deadline`. Rows past their
  deadline appear first and are flagged.
- One table of every row whose status is `invalidated` since the
  last review, showing the fired default response and the lane /
  claim / packet it landed in.
- One table of every row whose `widening_posture` is
  `blocks_widening` and whose dependent `linked_*` set includes a
  claim that has widened since last review — those are treated as
  validation failures until the row validates or narrows.

The same projection feeds the sample freeze report used by the
launch-register narrower-default automation, so the two registers
produce consistent freeze-posture language.

## Seeded rows

The ledger seeds eight program-level assumptions covering the
milestone-shape envelope. Full statements and all fields live in
`assumption_rows.yaml`; the table below is navigational.

| Id      | Title                                                                 | Forum                           | Widening posture    | Default if false          |
|---------|-----------------------------------------------------------------------|---------------------------------|---------------------|---------------------------|
| AL-0001 | Launch wedge stays narrow enough for staffed lanes to defend          | product_scope_review            | blocks_widening     | narrow_scope              |
| AL-0002 | Local-core value does not depend on hosted services                   | product_scope_review            | blocks_widening     | narrow_scope              |
| AL-0003 | Launch-bundle staffing depth holds through launch                     | shiproom_executive_scope_review | blocks_widening     | rebaseline_milestone      |
| AL-0004 | One keyboard-complete command system serves every launch surface     | architecture_council            | blocks_widening     | narrow_scope              |
| AL-0005 | One build identity covers every release artifact family               | release_council                 | blocks_widening     | narrow_scope              |
| AL-0006 | Docs and in-product help refresh on the build-identity cadence        | open_community_sync             | narrows_claims_only | cut_public_claim          |
| AL-0007 | Clean-room and mirror / offline builds reproduce at launch parity    | release_council                 | blocks_widening     | narrow_scope              |
| AL-0008 | Protected-path maintainer coverage holds through launch               | architecture_council            | blocks_widening     | freeze_lane               |

Every row carries a concrete validation deadline, validation
checkpoint, required proof with a named evidence owner, default
response with a concrete description, and at least one downstream
link (launch register, rolling index, dependency ledger, scorecard
lane, or risk register). New assumption rows land as additive-minor
changes (new `AL-NNNN`, no schema bump); repurposing an existing
field is breaking and requires a new assumption row plus a schema
version bump.

## Out of scope at this revision

This artifact freezes the tracking and response model; it does not
prove every seeded assumption inside the foundations milestone.
Validation of the individual rows happens on the cadences named in
each `validation_checkpoint` and is recorded through the row's
`required_proof.evidence_refs` as evidence lands. Live integration
with the shiproom and milestone-review tools that render the
projection tables above is tracked separately under the governance
tooling lane.
