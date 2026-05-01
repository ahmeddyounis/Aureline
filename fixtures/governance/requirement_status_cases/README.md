# Requirement-status snapshot fixtures

Worked fixtures for the requirement-status snapshot, blocker-state
grammar, and evidence-link summary contract frozen in
[`/docs/governance/requirement_status_snapshot_contract.md`](../../../docs/governance/requirement_status_snapshot_contract.md)
and the boundary schema
[`/schemas/governance/requirement_status_snapshot.schema.json`](../../../schemas/governance/requirement_status_snapshot.schema.json).

Each case ships as a complete `requirement_status_snapshot_record`
covering one of the typed milestone-close postures. The fixtures
exercise the typed requirement-state vocabulary, the typed blocker-
state grammar (implementation state x verification state x release-
readiness meaning x integrated-versus-complete-for-close), the
narrowing-link grammar, the evidence-link summary, the per-axis
rollups, and the export-parity floor.

## Index

| Case | Fixture | Posture |
| --- | --- | --- |
| Foundations close with named narrowing | `foundations_milestone_close_with_named_narrowing.yaml` | `milestone_close_with_named_narrowing` (one `pass`, one `partial`, one integrated-only `partial`, one `narrowed`, one `waived`) |
| Failing protected metric | `milestone_close_blocked_failing_protected_metric.yaml` | `milestone_close_blocked` (one `fail` on save-pipeline floor against air-gapped lab profile) |
| Decision route unresolved | `decision_route_unresolved.yaml` | `milestone_close_blocked_decision_route_unresolved` (one `fail` with no forum named, no accountable owner, refusal sentinel rendered) |
| Deferred and out-of-scope rows | `deferred_and_out_of_scope_rows.yaml` | `milestone_close_with_named_narrowing` (one `deferred` to first-beta, one `out_of_scope` by decision register) |

## Intended usage

- **Requirement-state grammar conformance.** Every fixture renders one
  of the closed seven-class requirement-state tokens (`pass`, `fail`,
  `partial`, `waived`, `deferred`, `narrowed`, `out_of_scope`) per
  row. A surface that renders a free-text 'mostly done' chip is
  non-conforming.
- **Blocker-state grammar conformance.** Every row renders the four
  typed blocker-state axes verbatim. A surface that collapses
  `integrated_only_not_complete_for_close` into `pass` is non-
  conforming and is rejected by the schema's `allOf` block.
- **Narrowing visibility conformance.** Every narrowed, waived,
  deferred, or out-of-scope row cites the narrowing object and the
  affected claim surfaces. A surface that drops a narrowed row to
  keep the milestone-close posture clean is non-conforming.
- **Evidence-link conformance.** Every row's evidence-link summaries
  cite stable evidence packet ids resolved through
  `schemas/governance/evidence_packet_header.schema.json`. A surface
  that renders a narrative status without citing the evidence
  packet id is non-conforming.
- **Rollup conformance.** Every fixture renders rollups by
  requirement family, launch wedge, protected workflow, and claim
  family. The dashboard, the architecture pack, the release packet,
  the support packet, and the governance review flow MUST consume
  the same rollup array. Reconstructing a different per-family or
  per-wedge count from raw notes is non-conforming.
- **Decision-id linkage conformance.** Every row whose state is
  `fail` or `waived` cites a non-empty
  `linked_decision_right_card_refs` array; every row whose state is
  `narrowed` cites a non-empty `linked_known_limit_refs` or
  `linked_claim_row_refs` array; every waived row cites a non-empty
  `linked_waiver_register_refs` array. A surface that drops the
  linkage is non-conforming.
- **Export-parity conformance.** Every consuming surface MUST
  render the same `snapshot_id`, `snapshot_target.target_milestone`,
  `milestone_close_posture_class`, per-row `requirement_id`,
  `requirement_state_class`, `blocker_state` axes, blockers,
  narrowing links, evidence-link summaries, rollups, and degraded-
  state vocabulary.

## Acceptance coverage

The acceptance criteria from
[`/.plans/M00-517.md`](../../../.plans/M00-517.md) are covered as
follows:

- **"Milestone close can be reasoned per requirement row rather than
  by narrative status updates alone."** — every fixture lays out the
  per-row state, the gating decision-right card, the linked evidence
  packets, and the typed blocker grammar. Reviewers can read close
  posture directly from the schema-validated rows without consulting
  narrative meeting notes.
- **"Narrowed or waived requirements remain visible next to the
  evidence and affected claim surfaces they influence."** — the
  `foundations_milestone_close_with_named_narrowing.yaml` and
  `deferred_and_out_of_scope_rows.yaml` fixtures pin narrowed,
  waived, deferred, and out-of-scope rows to known-limit notes,
  claim-row ids, waiver register entries, and decision-register
  rows. The `linked_claim_surface_refs` array routes the affected
  claim surfaces (docs rows, About / Help destinations, public
  claim manifest rows) so the narrowing rides next to the surfaces
  it influences.
- **"The snapshot is exportable into architecture pack, release
  packet, support packet, and governance review flows without
  screenshots."** — every fixture sets every `export_fields` boolean
  to `true` and cites the consuming packet refs explicitly
  (`linked_milestone_scorecard_ref`,
  `linked_architecture_pack_ref`,
  `linked_release_truth_summary_refs`,
  `linked_support_export_packet_refs`,
  `linked_governance_packet_refs`).

## Out of scope

External project-management systems (GitHub Projects, Jira, Linear,
Asana, Monday) are explicitly out of scope per
[`.plans/M00-517.md`](../../../.plans/M00-517.md). The contract
defines the source object; integrations consume it.
