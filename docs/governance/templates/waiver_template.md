# Waiver template

<!--
Copy this template when opening a new waiver. The machine-readable form
lives under `waivers:` in artifacts/governance/ownership_matrix.yaml —
this narrative form explains the reasoning and supersedes nothing in
the matrix. If they disagree, the matrix wins for tooling and the
narrative must be updated in the same change.
-->

- **Waiver id:** `<waiver-id>` (must exist in
  `artifacts/governance/ownership_matrix.yaml#waivers`)
- **Status:** active | renewed | closed
- **Opened on:** YYYY-MM-DD
- **Expires on:** YYYY-MM-DD
- **Closed on:** YYYY-MM-DD or `n/a`
- **Requirement ids:** canonical ids from `artifacts/governance/requirement_register_seed.yaml`
- **Covered lanes / decisions:** list of lane ids and / or decision ids
- **Accepted by:** `@handle`
- **Forum:** architecture_council | performance_council | security_trust_review | accessibility_review | compatibility_ecosystem_review | product_scope_review | release_council | shiproom_executive_scope_review
- **Policy refs:** `docs/governance/normative_requirement_policy.md` and `artifacts/governance/requirement_lifecycle_states.yaml`

## Reason

Plain-language description of what is being waived and why. Do **not**
frame this as a general exception; scope it to a specific protected
behaviour or backup-owner absence.

## Escalation path

Ordered list of who must be informed if the waiver fires. Under the
current solo-maintainer posture, this must include the maintainer's
self-escalation entry and a public contributor-community thread; see
[`/docs/governance/dri_map.md`](../dri_map.md).

1. ...
2. ...
3. ...

## Exit criteria

Named, testable conditions that close the waiver. A waiver that lists
"indefinite" or "until fixed" is not acceptable — restate it with a
concrete signal (named backup lands, fitness function repaired,
correction program accepted).

- [ ] ...
- [ ] ...

## Renewal discipline

A waiver may be renewed once. A second renewal on the same protected
lane triggers the `Recurring waiver` row in the blocker-aging table in
`dri_map.md` and MUST be converted into a tracked correction program.
Record each renewal with a new `opened_on` / `expires_on` pair and the
`forum` that approved the renewal; do not overwrite earlier history.

Waivers for `SEC`, `REL`, `PERF`, `A11Y`, or architecture-invariant
rows that last longer than 90 days, cross more than one release train,
or repeat on the same protected path escalate under
[`/docs/governance/normative_requirement_policy.md`](../normative_requirement_policy.md).
Expired waivers fail closed for release-readiness until renewed or
closed by the authorized forum.

## Linked artifacts

- Ownership matrix entry: `artifacts/governance/ownership_matrix.yaml#waivers.<waiver-id>`
- Affected decision rows (if any): `artifacts/governance/decision_index.yaml#D-XXXX`
- Affected lanes: `crates/<crate>`, governance lane ids, ...
