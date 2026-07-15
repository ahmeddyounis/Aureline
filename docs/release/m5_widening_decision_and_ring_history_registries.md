# M5 stable go/no-go widening-decision and ring-history registries

This lane makes every claimed stable-widening event reconstructible after the fact — materializing an explicit
stable go/no-go decision record and a preserved ring-history snapshot for each widening event rather than relying
on tribal memory and ad hoc meeting notes — over the frozen
[M5 launch-control matrix](./m5_launch_control_contract.md). It mints two domain contracts —
[`schemas/program/m5-widening-decision-packet.schema.json`](../../schemas/program/m5-widening-decision-packet.schema.json)
and [`schemas/program/m5-ring-history.schema.json`](../../schemas/program/m5-ring-history.schema.json) —
and implements them as registry resolvers that produce export-safe, honest projections. It turns the *stable
go/no-go decision-record* grammar (how each widening event records its final go/no-go decision, its open risks, its
narrowed claims, its named on-call and signoff roster, its exact evidence snapshot, and its decision-freshness
expiry so a stable claim can never widen on a stale, dropped, or undocumented record) and the *ring-history
snapshot* grammar (how a launch-bearing lane preserves the ring history, the prior blockers, and the previous
packet freshness with the preserved evidence snapshot, signoff, and named on-call roster that justified widening)
into one canonical launch-control truth the shiproom, release-center, executive-steering, program-governance,
correction-line, docs, CLI, support, and public-proof surfaces resolve directly instead of restating widening
decisions and their evidence by hand.

- **Canonical Rust module:** `crates/aureline-ui/src/m5_widening_decision_and_ring_history_registries` (the
  authoritative validator).
- **Combined schema:** `schemas/program/m5-widening-decision-and-ring-history-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/program/m5-widening-decision-packet.schema.json`](../../schemas/program/m5-widening-decision-packet.schema.json)
  and
  [`schemas/program/m5-ring-history.schema.json`](../../schemas/program/m5-ring-history.schema.json)
  as its canonical domain contracts.
- **Checked proof:** `artifacts/release/m5-widening-decision-and-ring-history-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:** `fixtures/release/m5-widening-decision-and-ring-history-registries/`
  (`widening_decision_beta_narrowed.json`, `ring_history_preview_narrowed.json`).

## Two registries

1. **Stable go/no-go decision record** (`resolve_widening_decision_entry`) — publishes one durable go/no-go record
   per widening event: the decision kind and its canonical mode, the final-decision reference, the open-risks
   reference, the narrowed-claims reference, the on-call-roster reference, the signoff-roster reference, the
   evidence-snapshot reference, and the decision-freshness expiry. The six canonical decision kinds are the alpha,
   beta, release-candidate, stable, long-term-support, and correction-reissue widening (plus an unclassified
   sentinel). A clean entry names a canonical registry token, a classified decision kind, and a launch-control
   role, covers the canonical / accessible / audit resolution forms, publishes a complete record (final decision
   plus open risks, narrowed claims, on-call / signoff roster, evidence snapshot, and decision-freshness expiry),
   keeps the record documented before widening, and — for the stable and long-term-support widenings, whose
   partner/public exposure demands it — keeps the support language matched to proof. Otherwise it degrades
   honestly: a lane that would widen on a stale or dropped record, or that runs a claim ahead of proof, degrades
   to `widening_decision_widens_scope_undocumented_or_runs_claim_ahead_of_proof`, the structured blocker a
   widen-on-stale-record attempt must surface.
2. **Ring-history snapshot** (`resolve_ring_history_entry`) — preserves the ring history behind a widening event so
   later incident or support review can reconstruct why a lane widened, keeping it honest and queryable. A clean
   entry names a classified snapshot scope (ring-history scope, prior-blocker scope, or packet-freshness scope) and
   provides the complete resolved-coverage-identity / ring-history-ledger / signoff / on-call-roster /
   packet-freshness / widening-stage / last-ring-history-revision record; a snapshot that would imply green while
   its preserved signoff or packet-freshness state is stale, drop the ring-history evidence, or let a
   roster-coverage gap masquerade as covered degrades to
   `ring_history_drops_evidence_or_implies_green_while_stale`.

## Per-entry widening-decision reference

Each decision kind carries its canonical mode, and the resolver publishes the full decision record, so the
registry — never an implicit meeting note — is the single source of truth. `widening_decision_object_is_complete`
rejects a record missing any field, `widening_decision_stays_documented_before_widening` rejects a decision kind
that widens on a stale or dropped record, and `ring_history_stays_honest` rejects a snapshot that would imply green
while its preserved evidence is dropped or a gap is unflagged.

| widening event | decision-kind mode |
| --- | --- |
| alpha widening | `alpha_widening_decision_kind` |
| beta widening | `beta_widening_decision_kind` |
| release-candidate widening | `release_candidate_widening_decision_kind` |
| stable widening | `stable_widening_decision_kind` |
| long-term-support widening | `long_term_support_widening_decision_kind` |
| correction-reissue widening | `correction_reissue_decision_kind` |

A lane that widens on a stale or dropped record degrades to
`widening_decision_widens_scope_undocumented_or_runs_claim_ahead_of_proof`, an incomplete record degrades to
`widening_decision_object_incomplete`, and a snapshot that drops the evidence or implies green while stale
degrades to `ring_history_drops_evidence_or_implies_green_while_stale`, so a widen-on-stale-record attempt,
an incomplete record, or a stale-green ring-history snapshot can never turn release evidence green.

## Acceptance criteria (proven by resolved examples)

- **Every claimed widening event has one durable go/no-go record tied to exact evidence and roster state.** Clean
  widening-decision entries cover the canonical alpha / beta / release-candidate / stable / long-term-support /
  correction-reissue decision kinds and the first release-center / shiproom / executive-steering /
  program-governance / support surfaces, an object-incomplete example degrades, and no clean widening-decision
  entry published an incomplete record.
- **Later incident or support review can reconstruct the decision without reading ad hoc meeting notes.** A
  widen-on-stale-record example and an unbound example degrade, a clean documented-before-widening
  widening-decision entry is present, and no clean entry is unbound or widens on a stale record.
- **Shiproom and correction-line flows consume the same go/no-go record rather than duplicating decision state.**
  Clean ring-history entries cover the ring-history / prior-blocker / packet-freshness snapshot scopes with full
  resolution-form coverage while providing the complete record — the resolved coverage identity, the preserved
  signoff, and the named on-call roster — and a snapshot that would imply green while its preserved evidence is
  stale or drop the ring-history evidence degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_widening_decision_and_ring_history_registries -- support-export
cargo run -p aureline-ui --example dump_m5_widening_decision_and_ring_history_registries -- csv
cargo run -p aureline-ui --example dump_m5_widening_decision_and_ring_history_registries -- report
cargo run -p aureline-ui --example dump_m5_widening_decision_and_ring_history_registries -- widening-decision-table
cargo run -p aureline-ui --example dump_m5_widening_decision_and_ring_history_registries -- fixture-widening-decision-beta-narrowed
cargo run -p aureline-ui --example dump_m5_widening_decision_and_ring_history_registries -- fixture-ring-history-preview-narrowed
```
