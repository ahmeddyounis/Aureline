# M5 retirement-review-packet and support-note-closure-gate registries

This lane forces one typed retirement review packet before a line or stable-facing surface can move to `Retired` over
the frozen [M5 retired-state matrix](./m5-retired-state-ops.md), so retirement stops being an ad hoc decision buried in
release notes and becomes a completed, inspectable proof of readiness, historical closure, and user-facing honesty. It
emits one export-safe *retirement review packet* per candidate — joining the retirement manifest, impact report, latest
compatibility / public-proof state, migration outcome summary, archival refs, exact-build snapshot refs, and any
unresolved dependent blockers to one candidate identity, plus mandatory support-note closure fields so help, support
KBs, partner notes, procurement FAQs, and incident / runbook references can be marked updated, archived, redirected, or
intentionally closed — and one typed *support-note closure gate* per candidate that blocks final retirement while the
packet is incomplete. It records the *retirement-review-packet* grammar (one classified packet field per joined fact —
exact-build snapshot ref, final compatibility / public-proof join, unresolved dependent blocker, support-note closure
status, migration outcome summary, or archival signoff ref — carrying its owning team and joined to the retirement
manifest and impact report) and the *support-note-closure-gate* grammar (the typed readiness-check scope a pre-closure
blocker sits in — incomplete-retirement-review-packet, unclosed-support-note-surface, or silently-dropped-exception,
naming the active gate reason) into registry resolvers that produce export-safe, honest projections, so support, help,
and public-proof consumers read the closure state directly from the packet instead of relying on free-text release
notes.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_retirement_review_packet_and_closure_gate_registries` (the
  authoritative validator).
- **Combined schema:**
  `schemas/program/m5-retirement-review-packet-and-closure-gate-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/program/m5-retirement-review-packet.schema.json`](../../schemas/program/m5-retirement-review-packet.schema.json)
  (minted by this lane — the review packet each retirement candidate is recorded against)
  and
  [`schemas/program/m5-support-note-closure-gate.schema.json`](../../schemas/program/m5-support-note-closure-gate.schema.json)
  (minted by this lane) as its canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-retirement-review-packet-and-closure-gate-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`). This checked-in proof is the first machine-readable
  retirement review packet — it demonstrates one review-packet / support-note-closure-gate loop end to end for the
  first retirement-bearing surfaces.
- **Narrowed fixtures:**
  `fixtures/release/m5-retirement-review-packet-and-closure-gate-registries/`
  (`retirement_review_packet_beta_narrowed.json`, `closure_gate_preview_narrowed.json`).

## Two registries

1. **Retirement review packet** (`resolve_retirement_review_packet_entry`) — publishes one packet field per retirement
   candidate: the classification (exact-build snapshot ref, final compatibility / public-proof join, unresolved
   dependent blocker, support-note closure status, migration outcome summary, archival signoff ref) and its canonical
   mode, the exact-build joins (repo rows, bundle IDs, install topology, toolchain envelope), the compatibility /
   known-limits state, the migration outcome / archival signoff, and the owning team. A clean entry names a canonical
   registry token, a classified packet field, and a retirement role, covers the canonical / accessible / audit
   resolution forms, publishes a complete object, completes its packet before the object flips to Retired, and keeps a
   public-facing support-note / migration field matched to the current compatibility / public-proof state. Otherwise it
   degrades honestly.
2. **Support-note closure gate** (`resolve_closure_gate_entry`) — surfaces a candidate's pre-closure blocker list
   before final retirement. A clean entry names a classified gate scope (incomplete-retirement-review-packet,
   unclosed-support-note-surface, or silently-dropped-exception) and provides the complete gate object; a gate that
   would run support language ahead of the closed support note, hide the blocker, or let an outstanding exception
   masquerade as covered degrades.

## Acceptance criteria (proven by resolved examples)

- **No object can move to Retired without a completed review packet that includes migration outcome, support-note
  closure status, and archival refs.** Clean review-packet entries cover the canonical exact-build-snapshot-ref /
  final-compatibility-public-proof-join / unresolved-dependent-blocker / support-note-closure-status /
  migration-outcome-summary / archival-signoff-ref fields and the first release-center / help-docs / support /
  marketplace-registry / install-update surfaces, an object-incomplete example degrades, and no clean review-packet
  entry published an incomplete object.
- **Support / help / public-proof consumers can read the closure state directly from the packet instead of relying on
  free-text release notes.** A packet flipping to Retired without completed closure degrades, an unbound example
  degrades, a clean review-packet entry is present, and no clean entry is incomplete or unbound.
- **The packet records outstanding exceptions and prevents them from being silently dropped from the retirement
  decision.** Clean support-note-closure-gate entries cover the incomplete-retirement-review-packet /
  unclosed-support-note-surface / silently-dropped-exception gate scopes with full resolution-form coverage while
  providing the complete gate object, and a gate that would keep support language ahead of the closed support note or
  drop the blocker degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_retirement_review_packet_and_closure_gate_registries -- support-export
cargo run -p aureline-ui --example dump_m5_retirement_review_packet_and_closure_gate_registries -- csv
cargo run -p aureline-ui --example dump_m5_retirement_review_packet_and_closure_gate_registries -- report
cargo run -p aureline-ui --example dump_m5_retirement_review_packet_and_closure_gate_registries -- retirement-review-packet-table
cargo run -p aureline-ui --example dump_m5_retirement_review_packet_and_closure_gate_registries -- fixture-retirement-review-packet-beta-narrowed
cargo run -p aureline-ui --example dump_m5_retirement_review_packet_and_closure_gate_registries -- fixture-closure-gate-preview-narrowed
```
