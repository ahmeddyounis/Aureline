# M5 operational-readiness-review and rehearsal-drill registries

This lane exercises launch-bearing lanes before widening — materializing operational-readiness-review (ORR) and
rehearsal packets with named role rosters and freshness gating rather than relying on tribal memory and implicit
rehearsals — over the frozen [M5 launch-control matrix](./m5_launch_control_contract.md). It mints two domain
contracts — [`schemas/program/m5-orr-review-packet.schema.json`](../../schemas/program/m5-orr-review-packet.schema.json)
and [`schemas/program/m5-rehearsal-drill.schema.json`](../../schemas/program/m5-rehearsal-drill.schema.json) —
and implements them as registry resolvers that produce export-safe, honest projections. It turns the *ORR /
rehearsal-packet* grammar (how each packet kind names its readiness scope, its release / advisory / support-room /
docs-comms / backup-signer role roster, and its rehearsal-freshness expiry so a stable claim can never widen on a
stale, skipped, or contradictory rehearsal packet) and the *rehearsal-drill readiness* grammar (how a
launch-bearing lane records its roster coverage with the preserved ORR signoff, named on-call roster, and
rehearsal-freshness state that justified widening) into one canonical launch-control truth the shiproom,
release-center, executive-steering, program-governance, diagnostics, docs, CLI, support, and public-proof surfaces
resolve directly instead of restating rehearsal cadence and widening approvals by hand.

- **Canonical Rust module:** `crates/aureline-ui/src/m5_orr_review_and_rehearsal_drill_registries` (the
  authoritative validator).
- **Combined schema:** `schemas/program/m5-orr-review-and-rehearsal-drill-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/program/m5-orr-review-packet.schema.json`](../../schemas/program/m5-orr-review-packet.schema.json)
  and
  [`schemas/program/m5-rehearsal-drill.schema.json`](../../schemas/program/m5-rehearsal-drill.schema.json)
  as its canonical domain contracts.
- **Checked proof:** `artifacts/release/m5-orr-review-and-rehearsal-drill-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:** `fixtures/release/m5-orr-review-and-rehearsal-drill-registries/`
  (`orr_review_beta_narrowed.json`, `rehearsal_drill_preview_narrowed.json`).

## Two registries

1. **ORR / rehearsal packet** (`resolve_orr_review_entry`) — publishes one typed ORR-packet object per packet
   kind: the packet kind and its canonical mode, the readiness scope reference, the release-owner reference, the
   advisory-owner reference, the support-room-owner reference, the docs-comms-owner reference, the backup-signer
   reference, and the rehearsal-freshness expiry. The six canonical packet kinds are the monthly ORR, the
   release-candidate ORR, the publish/rollback drill, the mixed-version drill, the advisory/revocation drill, and
   the support/incident handoff drill (plus an unclassified sentinel). A clean entry names a canonical registry
   token, a classified packet kind, and a launch-control role, covers the canonical / accessible / audit
   resolution forms, publishes a complete packet (readiness scope plus the full role roster and rehearsal-freshness
   expiry), keeps the rehearsal packet current before widening, and — for the mixed-version and
   advisory/revocation drills, whose partner/public exposure demands it — keeps the support language matched to
   rehearsal proof. Otherwise it degrades honestly: a lane that would widen on a stale or skipped rehearsal packet,
   or that runs a claim ahead of proof, degrades to
   `orr_review_widens_scope_undocumented_or_runs_claim_ahead_of_proof`, the structured blocker a
   widen-on-stale-rehearsal attempt must surface.
2. **Rehearsal-drill readiness** (`resolve_rehearsal_drill_entry`) — keeps the explicit rehearsal-drill roster
   coverage honest and queryable. A clean entry names a classified roster coverage (full roster, backup roster, or
   conditional roster) and provides the complete resolved-coverage-identity / rehearsal-evidence-ledger /
   ORR-signoff / on-call-roster / rehearsal-freshness / widening-stage / last-rehearsal-drill-revision record; a
   record that would imply green while its ORR signoff or rehearsal-freshness state is stale, drop the rehearsal
   evidence, or let a roster-coverage gap masquerade as covered degrades to
   `rehearsal_drill_drops_evidence_or_implies_green_while_stale`.

## Per-entry ORR-packet reference

Each packet kind carries its canonical mode, and the resolver publishes the full packet object, so the registry —
never an implicit rehearsal — is the single source of truth. `orr_review_object_is_complete` rejects a packet
missing any field, `orr_review_stays_documented_before_widening` rejects a packet kind that widens on a stale or
skipped rehearsal packet, and `rehearsal_drill_stays_honest` rejects a roster-coverage record that would imply
green while its evidence is dropped or a gap is unflagged.

| packet kind | packet-kind mode |
| --- | --- |
| monthly ORR | `monthly_orr_packet_kind` |
| release-candidate ORR | `release_candidate_orr_packet_kind` |
| publish/rollback drill | `publish_rollback_drill_kind` |
| mixed-version drill | `mixed_version_drill_kind` |
| advisory/revocation drill | `advisory_revocation_drill_kind` |
| support/incident handoff drill | `support_incident_handoff_drill_kind` |

A lane that widens on a stale or skipped rehearsal packet degrades to
`orr_review_widens_scope_undocumented_or_runs_claim_ahead_of_proof`, an incomplete packet degrades to
`orr_review_object_incomplete`, and a roster-coverage record that drops the evidence or implies green while stale
degrades to `rehearsal_drill_drops_evidence_or_implies_green_while_stale`, so a widen-on-stale-rehearsal attempt,
an incomplete packet, or a stale-green roster coverage can never turn release evidence green.

## Acceptance criteria (proven by resolved examples)

- **Every claimed launch-bearing M5 lane can point to current ORR and rehearsal packets.** Clean ORR-packet
  entries cover the canonical monthly-ORR / release-candidate-ORR / publish-rollback / mixed-version /
  advisory-revocation / support-incident-handoff packet kinds and the first release-center / shiproom /
  executive-steering / program-governance / support surfaces, an object-incomplete example degrades, and no clean
  ORR-packet entry published an incomplete packet.
- **Rehearsal freshness and role coverage appear as first-class blockers in shiproom and release views.** A
  widen-on-stale-rehearsal example and an unbound example degrade, a clean current-before-widening ORR-packet
  entry is present, and no clean entry is unbound or widens on a stale rehearsal packet.
- **Stable/LTS promotion is blocked automatically on affected lanes when rehearsal state is red or stale.** Clean
  rehearsal-drill entries cover the full-roster / backup-roster / conditional-roster coverages with full
  resolution-form coverage while providing the complete record — the resolved coverage identity, the preserved ORR
  signoff, and the named on-call roster — and a record that would imply green while its rehearsal state is stale or
  drop the rehearsal evidence degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_orr_review_and_rehearsal_drill_registries -- support-export
cargo run -p aureline-ui --example dump_m5_orr_review_and_rehearsal_drill_registries -- csv
cargo run -p aureline-ui --example dump_m5_orr_review_and_rehearsal_drill_registries -- report
cargo run -p aureline-ui --example dump_m5_orr_review_and_rehearsal_drill_registries -- orr-review-table
cargo run -p aureline-ui --example dump_m5_orr_review_and_rehearsal_drill_registries -- fixture-orr-review-beta-narrowed
cargo run -p aureline-ui --example dump_m5_orr_review_and_rehearsal_drill_registries -- fixture-rehearsal-drill-preview-narrowed
```
