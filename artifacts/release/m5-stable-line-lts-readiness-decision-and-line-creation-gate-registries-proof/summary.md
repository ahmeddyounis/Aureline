# M5 Stable-Line LTS-Readiness-Decision and Line-Creation-Gate Registries

- Packet: `m5-stable-line-lts-readiness-decision-and-line-creation-gate-registries:stable:0001`
- Label: `M5 91–180 day LTS-readiness-decision and line-creation-gate registries recording one typed decision packet per candidate line — one section per operating proof: backport branch posture, correction-line health, rollback evidence, support-window posture, mirror / air-gap continuity proof, and advisory / revocation readiness — each bound to the named decision-forum outcome and the preserved rollback / support evidence snapshot, with rollback / support posture preserved so LTS / support language never runs ahead of the current evidence, canonical / accessible / audit resolution-form coverage, and a machine-readable line-creation-gate (LTS-committed, LTS-blocked-missing-packet, or LTS-narrowed-to-stable) that gates LTS line creation and LTS-style language on a green decision packet and lets consumers explain why a line is or is not LTS-ready, naming the active gate reason across release / help, support, shiproom, executive-steering, program-governance, and public-proof surfaces`
- Consumer surfaces: 6
- Decision sections: backport_branch_posture_section, correction_line_health_section, rollback_evidence_section, support_window_posture_section, mirror_air_gap_proof_section, advisory_revocation_readiness_section, decision_section_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **shiproom**: `stable`
  - Owner: Shiproom owner
  - Scope: The shiproom resolves the candidate line's backport-branch-posture section to one typed LTS-readiness-decision object — the candidate line rows, decision section, support-window / backport / rollback / mirror-air-gap / advisory proofs, rollback target, and named forum outcome — from the shared registry and proves the LTS-committed gate for that line; an LTS-readiness-decision object missing its rollback / support evidence and a gate that keeps LTS language ahead of the current packet degrade honestly instead of leaving an LTS promise to read as earned
  - LTS-readiness-decision entries: 2 / line-creation-gate entries: 2
- **release_center**: `stable`
  - Owner: Release-center owner
  - Scope: The release center resolves the correction-line-health section and the LTS-narrowed-to-stable gate while keeping the active gate reason visible; a candidate widening its LTS language while its decision packet is unresolved and a resolution-form gap on a gate are caught before a screenshot can reintroduce an LTS-promise-as-earned reading
  - LTS-readiness-decision entries: 2 / line-creation-gate entries: 2
- **executive_steering**: `stable`
  - Owner: Executive-steering owner
  - Scope: Executive steering resolves the support-window-posture section while keeping its public LTS support-window claim matched to current rollback and support evidence and reports the line-creation-gate outcome; an LTS-readiness-decision entry that is a hand-copied per-entry assumption and a gate on an unclassified gate scope degrade honestly
  - LTS-readiness-decision entries: 2 / line-creation-gate entries: 1
- **program_governance**: `stable`
  - Owner: Program-governance owner
  - Scope: Program governance resolves the rollback-evidence section and the LTS-blocked-missing-packet gate bound to the registry; an unstated registry token on an LTS-readiness-decision entry is caught before it can drift
  - LTS-readiness-decision entries: 2 / line-creation-gate entries: 1
- **diagnostics**: `stable`
  - Owner: Diagnostics surface owner
  - Scope: Diagnostics renders the same resolved LTS-readiness-decision and line-creation-gate truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied line table; the advisory-revocation-readiness section and the LTS-narrowed-to-stable gate stay inspectable off-renderer
  - LTS-readiness-decision entries: 1 / line-creation-gate entries: 1
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved LTS-readiness-decision and line-creation-gate truth, so a hand-copied constant, an unstated registry token, an LTS-widen-without-green-packet attempt, or LTS language running ahead of the current decision packet is visible in evidence — LTS committed, LTS blocked on a missing packet, or LTS narrowed to stable — rather than hidden behind a screenshot
  - LTS-readiness-decision entries: 1 / line-creation-gate entries: 1
