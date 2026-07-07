# M5 escalation-packet-summary / handoff-timeline-row primitive

Status: implemented (B106, task M05-903)

This is the third `implement_` lane that narrows the frozen
[M5 support-intake / escalation component matrix](./m5_support_intake_escalation_component_matrix.md)
into reusable primitives — this time a **twin resolver** covering two of the
five governed families at once: the **escalation-packet summary** and the
**handoff-timeline row**. It closes the gap between the deeper Project Doctor
finding, crash-forensics, repair-transaction, escalation-manifest, and
supportability-handoff-packet systems and the reusable escalation / handoff
components a user and a human owner actually read, so a diagnosis-to-human
handoff preserves stable lineage instead of restarting from screenshots and
logs.

Truth source (checked in):

- Schema: `schemas/ui/m5-support-escalation-packet-summary.schema.json`
- Support export: `artifacts/release/m5-support-escalation-packet-summary-handoff-timeline-row-primitive-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-support-escalation-packet-summary-handoff-timeline-row-primitive-proof/matrix.csv`
- Design report: `artifacts/design/m5-support-escalation-packet-summary-handoff-timeline-row-primitive.md`
- Narrowed fixtures: `fixtures/ui/m5-support-escalation-packet-summary-handoff-timeline-row-primitive/`

The single mint-from-truth path is the headless emitter
`aureline_support_escalation_packet_summary_handoff_timeline_row_primitive`; the
in-code seed builders, the checked support export, and the fixtures never drift.

## What the primitive implements

The matrix names the escalation-packet summary and the handoff-timeline row as
two governed families and freezes their controlled vocabulary (scenario
families, Doctor finding families, escalation packet destinations, redaction
states, handoff stages, next human steps, approved repair classes, and case
dispositions, plus the shared surface families, deployment lines, consumer
surfaces, accessibility routes, qualification classes, and downgrade triggers).
This lane reuses every one of those verbatim so it never invents a parallel
lineage, destination, or next-step vocabulary, and mints new vocabulary only for
the two components themselves: their escalation / handoff consumers, their
anatomy parts, their derived postures, their bounded actions, and their export
fields.

### `resolve_escalation_packet_summary`

Takes one packet's id, scenario family, related finding families and opaque
evidence ids, repair attempts, redaction posture, build / profile identity,
destination, case disposition, and a share-requested signal. Derives the
**summary posture** in a fixed blocking-first order:

1. `escalation_blocked` — the destination is blocked or the export is blocked
   (redaction `export_blocked`); nothing leaves, only a local review remains.
2. `lineage_incomplete` — the scenario is uncategorized or no finding lineage is
   bound; the lineage cannot be continuous, so it must be completed first.
3. `redaction_review_required` — the destination leaves the device under a
   `full_metadata` posture; a redaction review is required before anything crosses.
4. `local_only_ready` — the packet stays on the device (a local-only bundle, or
   a share not yet requested).
5. `ready_to_escalate` — the lineage is continuous and the packet is ready to
   reach its destination.

The packet id, scenario code, finding lineage, evidence ids, repair attempts,
build / profile identity, destination, and case disposition are carried
explicitly and never collapsed into one blob. The summary always offers
`reveal_lineage`, `cancel_escalation` (so a user is never trapped
mid-escalation), and `export_packet`; offers `review_redaction` whenever a
review is required or the destination leaves the device; and offers
`confirm_escalation` only when the packet is genuinely ready to escalate.
`lineage_continuous` is the explicit AC-1 signal — a committed scenario with at
least one bound finding family.

### `resolve_handoff_timeline_row`

Takes one timeline event's identity, handoff stage, owner at the time, current
owner, related evidence, and next expected human step. Derives the **row
posture** in a fixed order: `awaiting_human` first, then `ownership_transferred`
(the event was handed off, or the current owner differs from the owner at the
time), then `repair_underway` (suggested or attempted), then `case_assembling`
(case built), and otherwise `local_diagnosis` (still locally owned). The event
identity, stage, owner, current owner, related evidence, and next step are
carried explicitly; `next_step_explicit` is always `true` — the next step is a
typed value and never a dead end (the AC-2 signal). The row always offers
`reveal_handoff_lineage`, `view_next_step`, and `export_row`, and offers
`contact_current_owner` whenever the row needs a human owner's attention.

## Acceptance criteria coverage

- **Scenario / finding / packet lineage stays continuous from local diagnosis
  through exported or shared escalation packets** — every escalation summary
  preserves its `packet_id`, `scenario_family`, `finding_families`,
  `related_evidence_ids`, `repair_attempts`, `build_profile_identity`,
  `destination`, and `case_disposition` verbatim, and exposes
  `lineage_continuous`. Proven by `validate_scenario_lineage_coverage` (every
  scenario and finding family exercised) and `validate_lineage_preservation`.
- **Human handoff consumers can reconstruct what was tried and what remains next
  without asking the user to restate the case** — every handoff row preserves
  its `event_identity`, `owner_role`, `current_owner_role`,
  `related_evidence_ids`, and `next_step`, with `ownership_transferred` making
  the current owner explicit and `next_step_explicit` guaranteeing the next step
  is never dropped. "What was tried" is proven by
  `validate_repair_attempt_coverage`; "what remains next" by
  `validate_next_step_coverage`; ownership continuity both ways by
  `validate_ownership_transfer_coverage`.

## Parity, coverage, and invariants

One row per claimed consumer — support-center escalation desk, recovery-center
handoff, Doctor handoff timeline, headless / CLI escalation, and support-packet
export — carries the shared anatomy, vocabulary, postures, bounded actions,
export fields, and non-visual accessibility routes, plus worked escalation and
handoff resolutions. Validators prove every summary posture, row posture,
scenario / finding family, destination, redaction state, repair class, case
disposition, handoff stage, and next step is exercised; that escalation gating
(confirm offered only when ready) and redaction review are proven; and that the
four hard invariants hold — no consumer masks the scenario / finding lineage,
hides the packet destination, drops the next human step, or collapses the case
into one opaque blob.

## Verify

```sh
cargo test -p aureline-support --lib implement_escalation_packet_summaries
cargo run -q -p aureline-support --bin aureline_support_escalation_packet_summary_handoff_timeline_row_primitive -- validate
```
