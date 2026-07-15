# M5 Dogfood-Ring, Certification-Cohort, ORR, Rehearsal, Freeze-Exception, and Go/No-Go Control Contract

Status: frozen (B145 opening matrix)

This contract freezes Aureline's concrete launch-control model — its cohort taxonomy, readiness events,
rehearsal cadence, freeze-exception packets, and explicit go/no-go decisions — into one export-safe matrix. It
is the canonical source of M5 launch-control truth: later shiproom, release-center, executive-steering,
program-governance, docs/help, and support/export surfaces consume it directly rather than reconstructing
cohort, rehearsal, or go/no-go state from meeting folklore.

- Matrix schema: `schemas/program/m5-launch-control-matrix.schema.json`
- Cohort-descriptor domain schema (core-team canary / design-partner preview): `schemas/program/m5-cohort-descriptor.schema.json`
- Freeze-exception-packet domain schema (extension-author / public preview): `schemas/program/m5-freeze-exception-packet.schema.json`
- Go/no-go-decision domain schema (certified-archetype): `schemas/program/m5-go-no-go-decision.schema.json`
- Support export: `artifacts/release/m5-orr-rehearsal-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-orr-rehearsal-proof/matrix.csv`
- Design report: `artifacts/program/m5-launch-control-matrix.md`
- Launch-control dashboard: `dashboards/m5-launch-control-dashboard.json`
- Narrowed fixtures: `fixtures/release/m5-launch-control/`
- Authoritative validator: `crates/aureline-ui` (`m5_launch_control_matrix`)
- Emitter (single mint-from-truth path): `cargo run -p aureline-ui --example dump_m5_launch_control_matrix`

## Governed cohorts

The matrix freezes **five** launch-bearing cohorts, each qualified independently and each pointing at one
canonical domain schema:

| Cohort | Launch-control concern | Owner | Domain schema |
| --- | --- | --- | --- |
| `core_team_canary` | Internal dogfood ring entered with known limits published and an armed rollback-stop rule | Core-team canary owner | cohort-descriptor |
| `design_partner_preview` | Enrolled partners whose feedback triages to requirements; support language matches cohort proof | Design-partner preview owner | cohort-descriptor |
| `extension_author` | Compatibility rehearsals current; freeze exceptions documented, not implicit | Extension-author cohort owner | freeze-exception-packet |
| `public_preview` | Publish/rollback, advisory/revocation, and support-handoff drills current | Public preview owner | freeze-exception-packet |
| `certified_archetype` | ORR signed and an explicit go/no-go decision recorded with a preserved evidence snapshot | Certified-archetype owner | go-no-go-decision |

## Shared launch-control-role vocabulary

Every consumer binds to one controlled role vocabulary; no surface invents a parallel word:

`cohort_membership`, `readiness_event`, `rehearsal_currency`, `freeze_exception_authority`,
`go_no_go_authority`, `rollback_stop`, `regression_asset`.

The cohort-membership / readiness-event / go-no-go-authority / freeze-exception-authority roles
(`cohort_membership`, `readiness_event`, `go_no_go_authority`, `freeze_exception_authority`) must preserve the
evidence snapshot and signoff roster before widening — a stable claim may never widen without current cohort
and rehearsal evidence, a shiproom row may never imply green while go/no-go or ORR state is stale, a freeze
exception may never be left undocumented, and a go/no-go decision may never rest on a stale evidence snapshot.
The descriptive structure roles (`rehearsal_currency`, `rollback_stop`, `regression_asset`) are inspectable
descriptors.

## Widening stages

Each cohort gates the widening stages it must clear before claiming the next channel, answering which cohort or
readiness event is required before **alpha**, **beta**, **release_candidate**, **stable**, and
**long_term_support** widening once rather than leaving it to meeting folklore.

## Hard invariants (release blockers)

Every row carries five hard-invariant booleans that must be `false`, and the governance-review block asserts
the corresponding fleet-level guarantees:

1. A stable claim never widens without current cohort and rehearsal evidence.
2. A freeze exception never becomes undocumented scope widening.
3. A Sev-1/Sev-2 incident is never closed without a regression asset.
4. A shiproom surface never implies green when go/no-go records or ORR packets are stale.
5. Partner or public support language never outruns current cohort proof.

The frozen downgrade triggers also enumerate the remaining release blockers: a stale rehearsal cadence, an
undocumented freeze exception, a missing registry reference, a cohort leaving its membership / readiness / go
or no-go state unstated, and a stale proof packet.

## Automatic narrowing

Claim publication and support/export narrow cohort claims automatically when the B145 registry is missing,
stale, or not yet qualified. Two narrowed fixtures demonstrate honest narrowing while keeping every cohort
visible:

- `public_preview_beta_narrowed.json` — the public preview cohort held at **Beta** pending current rehearsal
  evidence across every drill.
- `certified_archetype_preview_narrowed.json` — the certified-archetype cohort narrowed to **Preview** pending
  a signed go/no-go decision with a preserved evidence snapshot.

## Bound source contracts

The matrix binds back to already-landed truth so launch-control truth is never split across scattered meeting
notes: the cohort-scoreboard schema (`schemas/release/cohort_scoreboards.schema.json`) and the freeze-exception
packet schema (`schemas/governance/freeze_exception_packet.schema.json`).
