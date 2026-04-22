# Protected Change-Budget Workflow

This workflow turns the phase-budget policy into one operational path
for freeze-era decisions. It exists so protected-path changes do not
become an invisible planning system.

Companion artifacts:

- [`/artifacts/governance/protected_change_budget.yaml`](../../artifacts/governance/protected_change_budget.yaml)
  — machine-readable matrix for protected paths, phase rows, review
  thresholds, and dashboard fields.
- [`/schemas/governance/exception_packet.schema.json`](../../schemas/governance/exception_packet.schema.json)
  — canonical packet contract for protected-path exceptions.
- [`./templates/exception_packet_template.md`](./templates/exception_packet_template.md)
  — narrative template for new packets.
- [`/schemas/governance/freeze_exception_packet.schema.json`](../../schemas/governance/freeze_exception_packet.schema.json)
  and [`./templates/freeze_exception_template.md`](./templates/freeze_exception_template.md)
  — legacy compatibility aliases for older references.
- [`./commitment_and_rebaseline_policy.md`](./commitment_and_rebaseline_policy.md)
  — commitment classes, phase budgets, and re-baseline policy.

## Protected paths covered now

The protected change budget currently freezes these decision paths:

- `renderer_direction`
- `text_model`
- `buffer_strategy`
- `ipc_event_model`
- `benchmark_hardware_and_corpus`
- `repo_topology`
- `canonical_work_package_ownership`

These are the architecture-freeze rows called out by the milestone
spec: renderer direction, text model, buffer strategy, IPC/event
model, benchmark hardware/corpus, repo topology, and canonical
work-package ownership.

## How to decide whether a change is allowed

1. Find the protected path in
   `artifacts/governance/protected_change_budget.yaml`.
2. Find the current phase row in that path's `phase_matrix`.
3. If the proposed change matches `allowed_changes`, it may proceed
   without an exception packet.
4. If it matches one of the phase's `exception_class_refs`, open an exception packet
   before the change lands.
5. If it matches `no_go_changes`, the default answer is no. The only
   override path is an approved exception packet at the named review
   threshold, and the burden of proof gets stronger each phase.

The artifact is intentionally shaped to answer both of these questions
directly:

- `is this change allowed in this phase?`
- `how much exception debt already exists on this subsystem or train?`

## Review thresholds and forced responses

The matrix names the review threshold that applies to each phase and the
extra thresholds that apply once exception debt starts accumulating.

The rule set is:

- First hit on a protected path: use the phase-default forum for the
  current phase.
- Second same-path hit or repeated waiver: the packet must choose
  `claim_narrowing` or `rebaseline`.
- Third same-path hit, repeated subsystem burn, or an aging open
  exception: the packet must choose `explicit_correction_work`.

The current seeded debt trigger for `explicit_correction_work` is any of:

- `same_path_prior_exception_count >= 2`
- `repeated_subsystem_exception_count >= 2`
- `oldest_open_exception_age_days >= 14`

These thresholds are machine-readable in the budget artifact and are
also enforced by the canonical packet schema.

## Exception-packet workflow

Open a packet whenever either of these is true:

- a freeze has already fired on the affected protected path; or
- the requested change exceeds the current phase budget even before the
  calendar freeze date arrives.

Every packet must capture:

- the protected budget row and exception class
- affected requirement ids, lanes, and protected path keys
- the current phase and default decision forum
- whether the change fits the phase budget or exceeds it
- blast radius across artifacts, claims, and user journeys
- compensating evidence and named evidence owner
- debt snapshot fields for dashboard and scorecard summaries
- rollback, expiry, escalation, and repeated-exception handling

The canonical packet shape is
`schemas/governance/exception_packet.schema.json`. New packets should
use `packet_kind: exception_packet`. Older FE packets may keep
`packet_kind: freeze_exception_packet`; both shapes are accepted, but
the field set is now shared.

## Dashboard feed

The budget matrix freezes these dashboard-feed fields:

- `budget_burn_count`
- `repeated_subsystem_exception_count`
- `oldest_open_exception_age_days`
- `claim_narrowing_triggered`
- `rebaseline_triggered`
- `explicit_correction_work_triggered`

These fields are carried in the packet's `budget_debt_snapshot` object
so scorecards, shiproom dashboards, and governance checks can summarize
exception debt without scraping prose.

The seeded dashboard scopes are split into:

- one train scope: `train.pre_implementation_foundations`
- one protected-subsystem scope per currently frozen area

Until concrete packet instances land, the dashboard seeds stay at zero
and act as the canonical field registry rather than as a live count.
