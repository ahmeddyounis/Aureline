# M5 Operational-Readiness-Review and Rehearsal-Drill Registries

- Packet: `m5-orr-review-and-rehearsal-drill-registries:stable:0001`
- Label: `M5 operational-readiness-review and rehearsal-drill registries with one typed ORR-packet object resolving per packet kind (monthly ORR, release-candidate ORR, publish/rollback drill, mixed-version drill, advisory/revocation drill, support/incident handoff drill), each naming its readiness scope and its release / advisory / support-room / docs-comms / backup-signer role roster, a stable claim never widening on a stale or skipped rehearsal packet, rehearsal freshness and role coverage reading as first-class blockers, canonical / accessible / audit resolution-form coverage, and the complete resolved-coverage-identity / rehearsal-evidence-ledger / ORR-signoff / on-call-roster / rehearsal-freshness / widening-stage / last-rehearsal-drill-revision rehearsal-drill record across shiproom, release-center, executive-steering, program-governance, diagnostics, and support surfaces`
- Consumer surfaces: 6
- ORR / rehearsal packet kinds: monthly_orr_packet, release_candidate_orr_packet, publish_rollback_drill, mixed_version_drill, advisory_revocation_drill, support_incident_handoff_drill, packet_kind_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last refresh: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **shiproom**: `stable`
  - Owner: Shiproom owner
  - Scope: The shiproom resolves the monthly-ORR packet kind to one typed object — packet kind, readiness scope, release / advisory / support-room / docs-comms / backup-signer roster, and rehearsal-freshness expiry — from the shared registry and proves the full-roster rehearsal-drill record for the launch-bearing lane; an ORR packet missing its roster and a rehearsal-drill record that implies green while the rehearsal packet is stale degrade honestly instead of reading as a clean pass
  - ORR-packet entries: 2 / rehearsal-drill entries: 2
- **release_center**: `stable`
  - Owner: Release-center owner
  - Scope: The release center resolves the release-candidate-ORR packet kind and the conditional-roster rehearsal-drill record while keeping the rehearsal evidence visible; a lane widening on a stale or skipped rehearsal packet and a resolution-form gap on a rehearsal-drill record are caught before a screenshot can reintroduce a false-truth reading
  - ORR-packet entries: 2 / rehearsal-drill entries: 2
- **executive_steering**: `stable`
  - Owner: Executive-steering owner
  - Scope: Executive steering resolves the mixed-version-drill packet kind while keeping its partner support language matched to rehearsal proof and reports the roster-coverage rehearsal-drill record; an ORR packet that is a hand-copied per-entry assumption and a rehearsal-drill record on an unclassified roster coverage degrade honestly
  - ORR-packet entries: 2 / rehearsal-drill entries: 1
- **program_governance**: `stable`
  - Owner: Program-governance owner
  - Scope: Program governance resolves the publish-rollback-drill packet kind and the backup-roster rehearsal-drill record bound to the registry; an unstated registry token on an ORR packet is caught before it can drift
  - ORR-packet entries: 2 / rehearsal-drill entries: 1
- **diagnostics**: `stable`
  - Owner: Diagnostics surface owner
  - Scope: Diagnostics renders the same resolved ORR-packet and rehearsal-drill truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied readiness table; the support-incident-handoff-drill ORR packet and the full-roster rehearsal-drill record stay inspectable off-renderer
  - ORR-packet entries: 1 / rehearsal-drill entries: 1
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved ORR-packet and rehearsal-drill truth, so a hand-copied constant, an unstated registry token, a widen-on-stale-rehearsal attempt, or support language running ahead of rehearsal proof is visible in evidence rather than hidden behind a screenshot
  - ORR-packet entries: 1 / rehearsal-drill entries: 1
