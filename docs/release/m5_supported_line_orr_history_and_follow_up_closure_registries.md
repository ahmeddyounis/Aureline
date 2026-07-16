# M5 ORR-history-event and follow-up-closure registries

This lane preserves supported-line launch and servicing memory so later promotion, support, and postmortem work
never depends on shiproom folklore, over the frozen
[M5 supported-line-transparency matrix](./m5-supported-line-transparency-ops.md). It archives one
*ORR-history event* per recorded operational-readiness decision on each active stable or LTS-candidate line — an
archived ORR packet, a freeze exception, a rehearsal outcome, a cohort transition, a go/no-go decision, or a
post-review action-item closure — so partner reviews, procurement checks, support escalations, and governance
consumers inherit durable change context rather than oral history, and emits a typed *follow-up closure* event
whenever a line's follow-up state drifts — instead of letting an unclosed action item or stale rehearsal evidence
sit only in an archived meeting packet. It records the *ORR-history-event* grammar (one typed archive entry per
recorded operational-readiness decision, tracked against exact build / release-line identity, each bound to one
supported-line identity with its decision dates, cohort transitions, freeze exceptions, and follow-up closure state,
and public-safe cohort-transition and go/no-go decision history separated from internal-only freeze / rehearsal /
action-item minutiae) and the *follow-up-closure* grammar (the closure-drift scope a line's follow-up state sits in
versus its archived ORR history — an unclosed action item, stale rehearsal evidence, or a line history that can no
longer be reconstructed from the archive) into registry resolvers that produce export-safe, honest projections, so
release / help, docs, support, and governance surfaces resolve one canonical, freshness-checked truth instead of
re-synthesizing product truth by hand. The archive and the closure event are separated in runtime and serialized
state: the recorded decision, its go/no-go outcome, the cohort and freeze context, the linked supported-line-matrix
/ active-claim / correction-train / line-history refs, and the recorded decision history live on the ORR-history
event, while the resolved line identity, affected history-entry reference, archived-versus-active-line reference,
closure-scope state, and active closure reason live on the follow-up-closure event, and a line's recorded decision
history stays preserved so a go/no-go or cohort claim never runs ahead of it.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_supported_line_orr_history_and_follow_up_closure_registries` (the authoritative
  validator).
- **Combined schema:**
  `schemas/program/m5-supported-line-orr-history-and-follow-up-closure-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/program/m5-supported-line-orr-history.schema.json`](../../schemas/program/m5-supported-line-orr-history.schema.json)
  (reused from the frozen matrix — the ORR-history record each supported-line readiness decision is archived against)
  and
  [`schemas/program/m5-follow-up-closure.schema.json`](../../schemas/program/m5-follow-up-closure.schema.json)
  (minted by this lane) as its canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-supported-line-orr-history-and-follow-up-closure-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`). This checked-in proof is the first supported-line ORR-history
  archive — it demonstrates one durable ORR-history retention loop end to end for at least one active supported line.
- **Narrowed fixtures:**
  `fixtures/release/m5-supported-line-orr-history-and-follow-up-closure-registries/`
  (`orr_history_event_beta_narrowed.json`, `follow_up_closure_preview_narrowed.json`).

## Two registries

1. **ORR-history event** (`resolve_orr_history_event_entry`) — archives one typed entry per recorded
   operational-readiness decision, per active supported line: the event class and its canonical mode, the recorded
   decision rows, the linked supported-line-matrix / active-claim / correction-train / line-history refs, the decision
   outcome, the rollback / rehearsal target, and the owning roster, with public-safe cohort-transition and go/no-go
   decision history separated from internal-only freeze / rehearsal / action-item minutiae. A clean entry names a
   canonical registry token, a classified event class, and a transparency role, covers the canonical / accessible /
   audit resolution forms, publishes a complete object, preserves its recorded decision history before a claim widens,
   and keeps a public-facing event class's published claim matched to recorded ORR history. Otherwise it degrades
   honestly — a line widening its claim on stale rehearsal evidence, or a public-facing event class running its
   published claim ahead of recorded history, degrades to
   `orr_history_event_lets_line_widen_without_rollback_or_runs_support_ahead_of_proof`, the structured blocker reason a
   widen-on-stale-history attempt must surface.
2. **Follow-up closure** (`resolve_follow_up_closure_entry`) — turns a change in a line's follow-up state into a typed
   closure event against its archived ORR history rather than a forgotten shiproom note. A clean entry names a
   classified closure scope (unclosed-action-item, stale-rehearsal-evidence, or unreconstructable-line-history) and
   provides the complete line-identity / affected-history-entry / archived-versus-active-line / closure-scope /
   active-reason closure object; a closure event that would keep a claim ahead of recorded ORR history, hide the
   closure, or let an unclosed follow-up masquerade as closed degrades to
   `follow_up_closure_runs_support_ahead_of_proof_or_drops_follow_up_closure`.

## Per-entry ORR-history reference

The archived event class carries its canonical mode, and the resolver publishes the full archive object, so the
registry — never an archive merely assumed to still be current — is the single source of truth.
`orr_history_event_object_is_complete` rejects an object missing any archive field,
`line_preserves_rollback_and_diagnostics_before_widening` rejects a claim widening on stale rehearsal evidence or a
published claim running ahead of recorded history, and `follow_up_closure_stays_honest` rejects a closure event that
has kept a claim ahead of recorded ORR history.

## Acceptance criteria (proven by resolved examples)

- **At least one supported line exposes an ORR-history archive with line identity, decision dates, cohort
  transitions, freeze exceptions, and closure state for follow-up actions.** Clean ORR-history-event entries cover the
  canonical orr-packet-archive / freeze-exception / rehearsal-outcome / cohort-transition / go-no-go-decision /
  action-item-closure event classes and the first release-center / shiproom / executive-steering /
  program-governance / support surfaces, an object-incomplete example degrades, and no clean archive entry published
  an incomplete object.
- **A current supported line can be reconstructed from ORR history without relying on separate shiproom notes or oral
  memory.** A widen-on-stale-history example and an unbound example degrade, a clean archive entry is present, and no
  clean entry is unbound or missing its recorded decision history.
- **Unclosed follow-up work or stale rehearsal evidence is visible on the active line rather than only in archived
  meeting packets.** Clean follow-up-closure entries cover the unclosed-action-item / stale-rehearsal-evidence /
  unreconstructable-line-history closure scopes with full resolution-form coverage while providing the complete
  closure object — the resolved line identity and the active closure reason — and a closure event that would keep a
  claim ahead of recorded ORR history or drop the closure degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_supported_line_orr_history_and_follow_up_closure_registries -- support-export
cargo run -p aureline-ui --example dump_m5_supported_line_orr_history_and_follow_up_closure_registries -- csv
cargo run -p aureline-ui --example dump_m5_supported_line_orr_history_and_follow_up_closure_registries -- report
cargo run -p aureline-ui --example dump_m5_supported_line_orr_history_and_follow_up_closure_registries -- orr-history-event-table
cargo run -p aureline-ui --example dump_m5_supported_line_orr_history_and_follow_up_closure_registries -- fixture-orr-history-event-beta-narrowed
cargo run -p aureline-ui --example dump_m5_supported_line_orr_history_and_follow_up_closure_registries -- fixture-follow-up-closure-preview-narrowed
```
