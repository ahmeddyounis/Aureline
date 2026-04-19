# Decision backlog

This document is the human-readable form of the architecture decision
register. It exists so that architecture cannot be decided implicitly in
code: every protected-lane-visible behaviour change must link back to a
decision row in this backlog.

Companion artifacts:

- [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  — machine-readable register. Tooling reads this file; the narrative
  below describes the same rows.
- [`/schemas/governance/decision_index.schema.json`](../../schemas/governance/decision_index.schema.json)
  — schema the register conforms to.
- [`/docs/adr/`](../adr/) — decision records.
- [`/docs/rfc/`](../rfc/) — request-for-comment proposals.
- [`/docs/governance/decision_workflow.md`](./decision_workflow.md) —
  linkage, narrowing, and supersession rules.

**No silent decisions.** Every row must carry an owner, a forum, a
freeze date, a default-if-unresolved posture, and either a named backup
owner or a cited waiver. When this document and the register disagree,
the register wins for tooling and this document must be updated in the
same change.

## How rows move

- **`open`** — the decision is recognised but no forum has picked it up.
- **`deciding`** — the forum has opened an RFC or is actively running
  down the option space.
- **`decided`** — an ADR has landed; `linked_adr` points at it.
- **`deferred`** — the forum has accepted that the decision cannot land
  at the target milestone and has restated the freeze date.
- **`narrowed_by_default`** — the freeze date passed while the row was
  still open or deciding; the row's `default_if_unresolved` posture
  applied automatically.
- **`superseded`** — a later row replaces this one. The original row is
  not deleted; its `decision_history` preserves the transition.

Freeze deadlines are enforced by the `default_if_unresolved` field on
every row. On `applies_on`, if the row has not closed, tooling applies
the narrowing posture and records a `narrow` / `defer` / `freeze_lane`
/ `rebaseline` entry in `decision_history`. Worked example: decision
`D-0012` in the register demonstrates this transition.

## M0 decisions that must close before the first beta

Every row below is seeded in the register. Dates and owners live in the
register; this table is a navigational index only.

| Decision id | Title                                                               | Forum                       | Default if unresolved |
|-------------|---------------------------------------------------------------------|-----------------------------|-----------------------|
| D-0001      | Renderer stack and rendering primitive                              | Architecture council        | Narrow to spike surface |
| D-0002      | Buffer and editor-core persistence model                            | Architecture council        | Narrow to piece tree  |
| D-0003      | Workspace VFS path identity and watcher model                       | Architecture council        | Narrow to single-root |
| D-0004      | RPC transport and cross-process contract                            | Architecture council        | Freeze dependent lanes |
| D-0005      | Shared subscription envelope for reactive truth                     | Architecture council        | Freeze dependent lanes |
| D-0006      | Shell / command-system contract and non-throwaway home              | Architecture council        | Narrow to spike-hosted |
| D-0007      | Keyboard-complete command graph and input model                     | Architecture council        | Narrow to default scheme |
| D-0008      | Accessibility bridge and semantic surface contract                  | Architecture council        | Freeze accessibility claim |
| D-0009      | Identity modes and workspace-trust posture                          | Security / trust review     | Narrow to single trusted mode |
| D-0010      | Release posture — cadence, channels, and rollback                   | Release council             | Defer promotion |
| D-0011      | Exact-build identity — one identity across artifact families        | Release council             | Narrow to foundations identity |
| D-0013      | Secret broker, credential handles, trust-store classes, and redaction defaults | Security / trust review     | Narrow to alias-only / keychain-only; block broader projection lanes |

## Worked example of automatic narrowing

Row `D-0012` is a pedagogical example of the
`narrowed_by_default` transition. The row's freeze date has already
passed, no ADR closed it, and the `default_if_unresolved` posture is
`narrow`; tooling applied the posture automatically and recorded a
`narrow` entry in the row's `decision_history` dated on the
`applies_on` field. The row survives in the register so the narrowing
has an audit trail, and reopening the decision requires minting a new
`decision_id` rather than reusing D-0012.

## Assumptions and dependencies

The register also carries assumption (`A-*`) and dependency (`DEP-*`)
rows that decisions reference via `linked_assumptions` and
`linked_dependencies`. Invalidating an assumption reopens every
decision row that references it. Seeded entries cover the
solo-maintainer posture, the dev-host support matrix, the single
build-identity rule, the reproducible-build baseline dependency, and
the accessibility / input review packet seeding.

## Adding a new decision

1. Append a row to
   [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
   using the next unused `D-NNNN` id.
2. Fill every required field, including `default_if_unresolved`.
3. Add a navigational entry to the table above.
4. If an ADR or RFC is opened at the same time, set `linked_adr` or
   `linked_rfc` to point at it.

See [`decision_workflow.md`](./decision_workflow.md) for the full
workflow, including the linkage rule for broad implementation of
protected-lane behaviour changes.
