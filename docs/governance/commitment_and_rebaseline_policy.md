# Commitment-Class And Rebaseline Policy

This policy turns the milestone document's commitment-class, phase-budget,
and assumption-invalidation rules into repository-operable governance.
Its purpose is simple: scope growth, stale proof, and repeated exceptions
must become explicit program decisions rather than quiet carry-forward.

Companion artifacts:

- [`/artifacts/governance/commitment_classes.yaml`](../../artifacts/governance/commitment_classes.yaml)
- [`/artifacts/governance/protected_change_budget.yaml`](../../artifacts/governance/protected_change_budget.yaml)
- [`/schemas/governance/exception_packet.schema.json`](../../schemas/governance/exception_packet.schema.json)
- [`/docs/governance/templates/exception_packet_template.md`](./templates/exception_packet_template.md)
- [`/schemas/governance/freeze_exception_packet.schema.json`](../../schemas/governance/freeze_exception_packet.schema.json)

## Commitment classes

| Class | Meaning | May do | Must not do | Evidence rule |
|---|---|---|---|---|
| **Committed** | Required for milestone identity, protected-path integrity, or a live public claim | Appear in exit gates, consume critical-path capacity, justify a freeze exception when the exception protects launch truth or narrows risk | Slip silently, broaden late without forum review, or sit in public claims without current proof | MUST name target milestone, owner, and evidence owner |
| **Target** | Strongly desired if dependency and staffing reality hold | Use slack, appear in weekly scope review, and land pre-integration work | Redefine milestone close, force late freeze exceptions, or appear in public promises before promotion | SHOULD name an evidence owner before integration starts |
| **Stretch** | Useful upside when the team is ahead | Prototype, gather evidence, or preserve promotion-ready hooks | Take capacity from protected paths or create implied commitments | Evidence may stay exploratory until promoted |
| **Parked / hook-only** | Contract or extensibility accommodation kept visible for later milestones | Preserve seams, dependency markers, and narrow architecture hooks | Count as delivered scope or appear as alpha/stable value | Evidence records the hook, not the missing feature |
| **Explicitly cut** | Outside the current milestone or public truth | Stay visible in backlog history, scorecards, and change logs | Re-enter active scope without explicit re-baselining | Evidence records the cut or narrowed claim, not a future promise |

## Projection rules

- Critical-path epic rows MUST carry `commitment_class`, target milestone,
  owner, and evidence owner. The current planning backlog records the
  evidence owner as `evidence_owner_team` because staffing is still a
  single-maintainer posture.
- Decision rows are the machine-readable ADR/RFC rows for this repo.
  They MUST carry `commitment_class`, target milestone, owner, and
  `evidence_owner`.
- Milestone scorecard rows SHOULD carry `commitment_class` on every lane
  in scope and MUST carry `evidence_owner` on rows used in milestone
  signoff, claim publication, or freeze-exception justification.
- Public claims, release packets, and milestone-close arguments may only
  cite **Committed** rows or an explicit narrowed replacement recorded in
  the same train.

## Re-baseline triggers

Any trigger below starts a five-business-day clock. Within that window,
product scope review, architecture council, and release council MUST
record one of three actions: `cut scope`, `add capacity`, or `change
dates`. Silent carry-forward is not a valid outcome.

| Trigger | Why the current baseline no longer holds | Default response |
|---|---|---|
| Staffing remains below the assumed shape entering a hardening phase | Parallel beta/stable work no longer fits the planned lane model | Cut scope or move dates explicitly |
| Benchmark, compatibility, certification, or supportability evidence automation is stale, failing, or still manual at the planned checkpoint | Quality truth turns into ad hoc batch work instead of continuous proof | Redirect capacity to the evidence system before adding breadth |
| Launch rows widen beyond the currently staffed language, framework, or deployment wedge | The launch claim has expanded without the quality depth to defend it | Downgrade widened rows to Target/Stretch or re-baseline staff and dates |
| Optional service-plane or later-differentiation work lands on the same protected-path team as the desktop core | Core quality starts paying for later breadth | Ring-fence the work or defer it |
| The same protected path needs repeated waivers or freeze exceptions | The exception has become hidden product policy | Require claim narrowing, a recorded rebaseline, or explicit correction work |
| Certified-archetype, migration, docs, or compatibility publication remains manual and fragile past the planned checkpoint | Stable publication can no longer be treated as one current truth | Open a correction program before widening claims |

## Phase budgets and default forums

Freeze exceptions are judged against the current phase budget first and
the packet must say whether the requested change fits that budget or
exceeds it.

The canonical phase rows, protected-path matrix, review thresholds, and
dashboard-feed fields now live in
[`/artifacts/governance/protected_change_budget.yaml`](../../artifacts/governance/protected_change_budget.yaml).

| Current phase | Budget allows by default | Explicit exception for | Default decision forum |
|---|---|---|---|
| **M0-M1 truth-establishment** | architecture shaping, harness refinement, shell/editor contract closure, prototype-only experiments | new release-bearing promises, new critical-path subsystems without owners, broad service-plane commitments | architecture council |
| **M2 alpha wedge** | launch-wedge completion, trust labeling, search/index usefulness, supportability alpha, narrow archetype proof | new P0 personas, late language/framework additions, new deployment-profile promises, cross-cutting schema churn | milestone scope review |
| **M3 beta hardening** | packaging, migration, SDK/CLI stabilization, policy/transport hardening, support/export, compatibility publication | new stable-class surfaces, major public-interface additions, late workflow families on protected paths | release council |
| **M4 RC/stable** | blocker fixes, claim narrowing, docs/compatibility corrections, release-evidence refresh, rollback/runbook hardening | feature additions, support-class widening, late schema or SDK changes, new deployment claims | shiproom |

The packet's `default_decision_forum_ref` MUST resolve to the repo forum
id for that phase:

- `architecture_council`
- `product_scope_review` for milestone scope review
- `release_council`
- `shiproom_executive_scope_review`

## Exception-packet requirements

An exception packet is required whenever work lands after a freeze fires
or when a change exceeds the current phase budget even if the calendar
freeze has not passed yet. Legacy references may still call this a
freeze-exception packet.

Every packet MUST carry:

- the protected budget row, exception class, and review threshold being
  invoked;
- the exact change and the freeze being crossed;
- affected lane ids, protected-path keys, and requirement ids;
- the current milestone phase and default decision forum for that phase;
- a budget assessment saying whether the change fits the phase budget or
  exceeds it, plus why;
- blast radius across lanes, claims, artifacts, or user journeys;
- compensating evidence and the named evidence owner;
- a budget-debt snapshot for dashboard and scorecard rollups;
- rollback path, expiry, and escalation path;
- repeat-exception handling showing whether the same protected path has
  already been excepted or waived.

If the change exceeds the phase budget, the packet MUST name the
escalation forum set explicitly rather than relying on narrative urgency.

## Repeated exception rule

The protected change budget makes exception debt visible in three steps:

- First hit on a protected path: use the phase-default forum and record
  the debt snapshot.
- Second same-path exception or waiver, whether inside one milestone or
  across consecutive milestones: the packet MUST choose
  `claim_narrowing` or `rebaseline`.
- Third same-path exception, repeated subsystem burn, or an aging open
  exception: the packet MUST choose `explicit_correction_work`.

`silent carry-forward` is not an allowed response.

When claim narrowing is chosen, the affected scorecard, claim packet, or
decision row must be updated in the same change. When rebaseline is
chosen, the milestone scorecard moves to `rebaselined` until the new
cutline is approved. When explicit correction work is chosen, the packet
must name the correction-work artifact and due date in-line.
