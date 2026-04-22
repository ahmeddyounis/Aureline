# RFC 0000 — <proposal title>

<!--
Copy this file to docs/rfc/NNNN-slug.md using the next unused four-digit
number. Keep the slug short and option-focused.

An RFC explores an option space. A separate ADR records the chosen
outcome. Metadata lives on the decision register row referenced below.
-->

- **Decision id:** D-XXXX (must exist in `artifacts/governance/decision_index.yaml`)
- **Status:** Draft | In review | Accepted (see ADR-NNNN) | Rejected | Withdrawn | Superseded by RFC-NNNN
- **Opened on:** YYYY-MM-DD
- **Closed on:** YYYY-MM-DD or `pending`
- **Owner:** `@handle`
- **Backup owner:** `@handle` or `null` with a cited waiver id
- **Forum:** architecture_council | performance_council | security_trust_review | accessibility_review | compatibility_ecosystem_review | product_scope_review | release_council | shiproom_executive_scope_review
- **Freeze deadline:** copied from the register
- **Related requirement ids:** e.g. `PRD-FOO-001` (or `none`)

## Summary

Two or three sentences: what this RFC is about and why it is being
proposed now.

## Motivation

Longer problem statement. Reference source anchors; do not restate the
spec. Call out the protected lanes or public-truth surfaces in scope.

## Options

- **Option A.** Description, cost, benefit, risk.
- **Option B.** Description, cost, benefit, risk.
- **Option C.** Description, cost, benefit, risk.

## Unresolved questions

Questions the forum needs to resolve before this RFC can close.

## Default if this RFC is not resolved by the freeze deadline

Every RFC that targets a register row MUST describe the narrowing
posture that applies if the forum does not close the decision by the
row's freeze date. Copy the `default_if_unresolved` field from the
register and expand it here.

## Recommendation

Left empty while the RFC is in review. When ready to close, the RFC
records a concrete recommendation pointing at the ADR that will land
the decision.

## Source anchors

- `.t2/docs/<doc>.md:<line>` — short quoted phrase.

## Linked artifacts

- Decision register row: `artifacts/governance/decision_index.yaml#D-XXXX`
- ADR closing this decision (once accepted): `docs/adr/NNNN-slug.md`
- Affected packages / lanes: `crates/<crate>`, `artifacts/<family>/`, ...

## Supersession history

Leave empty on first publication. If this RFC supersedes an earlier
one, add `Supersedes RFC-NNNN (<slug>) — <reason>`. If this RFC is
later superseded, set the status field above and add a line here —
do not rewrite the body.
