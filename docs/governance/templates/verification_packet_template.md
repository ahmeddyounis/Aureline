# Verification packet template (narrative form)

<!--
The machine-readable verification packet template lives at
artifacts/governance/governance_packet_template.yaml (the `verification`
stanza) and conforms to schemas/governance/governance_packet.schema.json.

This narrative form exists so that the written reasoning, per-lane
owner signoff, and waiver / risk narrative are first-class, not
compressed into a YAML value. When the two disagree, the YAML wins
for tooling and the narrative must be updated in the same change.
-->

- **Packet id:** `<verification-packet-id>` (matches the YAML instance)
- **Milestone slug:** `<milestone-slug>` (e.g. `pre-implementation-foundations`)
- **Opened on:** YYYY-MM-DD
- **Closed on:** YYYY-MM-DD or `pending`
- **Status:** draft | in_review | accepted | rejected | superseded
- **Owner:** `@handle`
- **Backup owner:** `@handle` or `null` with a cited waiver id
- **Evidence owner:** `@handle`
- **Covered lanes:** lane ids from `ownership_matrix.scorecard_lane_index`
- **Requirement ids:** canonical ids from `artifacts/governance/requirement_register_seed.yaml`

## Summary

Two or three sentences: what this milestone is claiming verified, and
what is explicitly out of scope.

## Per-lane signoff

One subsection per covered lane. Each subsection records the lane id,
its current status on the milestone scorecard, the evidence links, and
the named signoff.

### `<lane-id>`

- **Status:** green | yellow | red | waived | not_started
- **Signoff:** `@handle`, YYYY-MM-DD
- **Evidence:**
  - `<repo-relative path or stable uri>` — short description.
  - ...

## Waivers referenced

- `<waiver-id>` — one-sentence description of the effect on this packet.

## Decisions recorded

Decisions taken during this packet's lifecycle. Each entry mirrors a
row added to the relevant decision register entry (accept, reject,
defer, narrow, rebaseline).

- YYYY-MM-DD — `@handle` in `<forum>`: accept / reject / defer /
  narrow / rebaseline. Short rationale.

## Risks and follow-ups

Bulleted list of open risks and follow-ups that do not block
acceptance but must survive into the next milestone.

- ...

## Linked artifacts

- Machine-readable packet: `artifacts/governance/packets/<packet-id>.yaml`
- Scorecard: `artifacts/governance/scorecards/<milestone-slug>.yaml`
- Requirement register: `artifacts/governance/requirement_register_seed.yaml`
- ADRs / RFCs referenced: `docs/adr/NNNN-slug.md`, `docs/rfc/NNNN-slug.md`
