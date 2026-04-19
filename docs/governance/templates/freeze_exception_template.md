# Freeze-exception packet template

<!--
A freeze-exception packet is the written record of a change that lands
AFTER a freeze deadline has passed — either a protected-lane freeze,
a decision register freeze, or a release-candidate freeze. Opening one
is the only permitted way to move through a freeze; silent exceptions
are not allowed.
-->

- **Packet id:** `<freeze-exception-packet-id>`
- **Opened on:** YYYY-MM-DD
- **Status:** proposed | approved | rejected | applied | closed
- **Freeze being crossed:**
  - `decision_register` — decision row freeze deadline passed.
  - `protected_lane` — lane-level feature freeze passed.
  - `release_candidate` — feature / schema freeze on the RC passed.
- **Affected decision id(s):** `D-XXXX` (one per freeze being crossed, if any)
- **Affected lane(s):** lane ids from `ownership_matrix.scorecard_lane_index`
- **Owner:** `@handle`
- **Backup owner:** `@handle` or `null` with a cited waiver id
- **Required approving forum(s):** forums per the authority table in
  [`/docs/governance/dri_map.md`](../dri_map.md) (release-candidate
  exceptions require `shiproom_executive_scope_review`).

## Rationale

Plain-language description of what changed after the freeze and why
holding to the freeze is not acceptable. Describe the user-visible
impact of **not** taking the exception, not just the engineering
convenience of taking it.

## Scope

Exactly what is and is not included in this exception. A freeze
exception must be as narrow as possible; "also while we're here" work
does not belong inside one.

- In scope: ...
- Out of scope: ...

## Risk and rollback

- **Risk if accepted:** ...
- **Rollback plan:** what to revert, which release-evidence packet to
  open, who approves the rollback.

## Decision

Filled in when the packet is approved or rejected.

- **Decided on:** YYYY-MM-DD
- **Decided by:** `@handle` in `<forum>` (plus co-required forums).
- **Outcome:** approved | rejected.
- **Conditions:** any constraints that apply if approved (e.g.
  "approved for the renderer lane only", "approved with a 7-day
  post-merge monitoring window").

## Linked artifacts

- Affected decision rows: `artifacts/governance/decision_index.yaml#D-XXXX`
- Affected ADRs / RFCs: `docs/adr/NNNN-slug.md`, `docs/rfc/NNNN-slug.md`
- Release evidence packet (if release-candidate freeze): `artifacts/release/...`
- Scorecard update: `artifacts/governance/scorecards/<milestone-slug>.yaml`

## History

If this exception is itself superseded by a later exception (because
scope widened or the approval conditions changed), record that here.
Do not rewrite the rationale or decision fields after approval.
