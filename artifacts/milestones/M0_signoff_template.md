# M0 exit signoff packet template

Machine-readable companion: `artifacts/milestones/M0_signoff_packet.json`

- Packet id: `<m0-exit-signoff-packet-id>`
- Packet state: `draft` | `in_review` | `accepted` | `blocked` | `superseded`
- Readiness: `releasable` | `narrow_claims` | `blocked`
- Decision requested: `close` | `conditional_close_with_explicit_blockers` | `hold`
- Opened on: `YYYY-MM-DD`
- Assembled on: `YYYY-MM-DDTHH:MM:SSZ`
- Owner: `@handle`
- Evidence owner: `@handle`

## Decision requested

One sentence on whether the milestone should close, close conditionally, or
hold, and what the default posture is if blockers remain.

## Milestone objective

State what the milestone was supposed to prove and which packet or scorecard
is the canonical evidence tree.

## Hero workflow result

- Result: `pass` | `mixed` | `fail`
- Primary evidence:
- Notes:

## Readiness scorecard

- Scorecard:
- Top-level calls:
  - `renderer_viability:`
  - `benchmark_governance:`
  - `ownership:`
  - `public_truth_seeds:`
  - `unresolved_narrowing_decisions:`

## Changed scope since last review

- Added:
- Cut:
- Narrowed:
- Why:

## Waivers

- Waiver id:
  - Owner:
  - Expiry:
  - Effect:

## Evidence index

- Architecture pack:
- Scorecard:
- Risk register:
- Control-artifact index:
- Decision register:
- Compatibility inventory:
- Continuity drill seed:
- Security intake baseline:

## Rollback / recovery posture

Explain what defaults back to narrow, deferred, or blocked if the next
milestone tries to build past an unresolved family.

## Next-milestone risk

- Risk 1:
- Risk 2:
- Risk 3:

## Named signoffs

| Reviewer | Owner | State | Primary blockers |
|---|---|---|---|
| Architecture | `@handle` | `pending` | |
| Product | `@handle` | `pending` | |
| Design | `@handle` | `pending` | |
| QE / Perf | `@handle` | `pending` | |
| Accessibility | `@handle` | `pending` | |
| Docs / Truth | `@handle` | `pending` | |
| Support | `@handle` | `pending` | |
| Security | `@handle` | `pending` | |
| Release | `@handle` | `pending` | |

## Mandatory signed-packet sections

### Deployment-profile truth

- Required refs:

### Canonical decision register

- Required refs:

### Notification and chronology primitives

- Required refs:

### Local-history contract

- Required refs:

### Security-intake baseline

- Required refs:

## Contract-family matrix

| Family | Reviewer-visible state | Architecture pack | Compatibility inventory | QE lane | Assurance claim | Public-proof coverage | Exception |
|---|---|---|---|---|---|---|---|
| `<family_id>` | `frozen` | | | | | | none |

Use only these reviewer-visible states:

- `frozen`
- `seeded`
- `deferred_signed_exception`

If the state is `deferred_signed_exception`, the packet must carry the signed
exception ref and the validator must fail if it is absent.

## Evidence freshness

| Evidence | Captured at | `stale_after` | Current posture | Notes |
|---|---|---|---|---|
| `<evidence_id>` | `YYYY-MM-DDTHH:MM:SSZ` | `<ISO-8601 duration or null>` | `current` | |

## Review notes

### Architecture

- Verdict:
- Evidence refs:
- Blockers:
- Follow-up:

### Product

- Verdict:
- Evidence refs:
- Blockers:
- Follow-up:

### Design

- Verdict:
- Evidence refs:
- Blockers:
- Follow-up:

### QE / Perf

- Verdict:
- Evidence refs:
- Blockers:
- Follow-up:

### Accessibility

- Verdict:
- Evidence refs:
- Blockers:
- Follow-up:

### Docs / Truth

- Verdict:
- Evidence refs:
- Blockers:
- Follow-up:

### Support

- Verdict:
- Evidence refs:
- Blockers:
- Follow-up:

### Security

- Verdict:
- Evidence refs:
- Blockers:
- Follow-up:

### Release

- Verdict:
- Evidence refs:
- Blockers:
- Follow-up:
