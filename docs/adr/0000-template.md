# ADR 0000 — <decision title>

<!--
Copy this file to docs/adr/NNNN-slug.md using the next unused four-digit
number. Keep the slug short and decision-focused. Do not put milestone
IDs or task IDs in the filename.

This template mirrors the fields in artifacts/governance/decision_index.yaml.
When a field differs between the ADR and the register, the register wins
for tooling and the ADR must be updated in the same change.
-->

- **Decision id:** D-XXXX (must exist in `artifacts/governance/decision_index.yaml`)
- **Status:** Proposed | Accepted | Deferred | Narrowed-by-default | Superseded by ADR-NNNN
- **Decision date:** YYYY-MM-DD (or `pending`)
- **Freeze deadline:** YYYY-MM-DD (copied from the register)
- **Owner:** `@handle`
- **Backup owner:** `@handle` or `null` with a cited waiver id
- **Forum:** architecture_council | performance_council | security_trust_review | accessibility_review | compatibility_ecosystem_review | product_scope_review | release_council | shiproom_executive_scope_review
- **Related requirement ids:** e.g. `PRD-FOO-001` (or `none`)

## Context

One or two paragraphs describing the problem, the forces at play, and the
protected lanes or public-truth surfaces the decision touches. Reference
source anchors rather than restating the spec.

## Decision

The decision in a single paragraph. State the chosen option plainly.

## Consequences

Bulleted list of expected consequences. Call out anything that becomes
frozen, anything that becomes permitted, and anything that now needs a
follow-up change.

## Alternatives considered

- **Option A.** Short summary. Reason rejected.
- **Option B.** Short summary. Reason rejected.

If the decision has a default-if-unresolved narrowing posture (see the
register row), describe what that narrowing would look like in practice
so the reader can tell whether it actually landed or whether the forum
accepted a non-default outcome.

## Source anchors

Point to the doc lines in `.t2/docs/` or the protected-lane artifacts
that motivated this decision.

- `.t2/docs/<doc>.md:<line>` — short quoted phrase.

## Linked artifacts

- Decision register row: `artifacts/governance/decision_index.yaml#D-XXXX`
- RFC (if any): `docs/rfc/NNNN-slug.md`
- Affected packages / lanes: `crates/<crate>`, `artifacts/<family>/`, ...

## Supersession history

Leave empty on first acceptance. If this ADR supersedes an earlier one,
record `Supersedes ADR-NNNN (<slug>) — <reason>`. If this ADR is later
superseded, set `Status` to `Superseded by ADR-NNNN` and add a line here
— do not rewrite the body.
