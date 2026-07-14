# M5 Acquisition-Evidence and Partial-Recovery Registries

- Packet: `m5-acquisition-evidence-and-partial-recovery-registries:stable:0001`
- Label: `M5 acquisition-evidence and partial-recovery registries with one stable acquisition-evidence packet resolving per acquisition path, the evidence staying visible with no partial content presented as a healthy full checkout and partial-not-full status disclosed before any partial-describing packet, canonical / accessible / audit resolution-form coverage, and the complete recovery-action-kind / recovery-site / state-consequence / lineage-consequence / explicit-action-requirement / attribution partial-recovery object across acquisition-engine, git, trust, diagnostics, CLI, and support surfaces`
- Consumer surfaces: 6
- Evidence kinds: clone_fetch_transcript, warnings_and_retries, resulting_root_identity, omitted_or_unfetched_state, bootstrap_checkpoint, evidence_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last refresh: 2026-07-14T00:00:00Z)

## Consumer surfaces

- **acquisition_engine**: `stable`
  - Owner: Acquisition-engine owner
  - Scope: The acquisition engine resolves the clone-fetch-transcript evidence kind to one stable packet — transcript reference, warnings and retries, resulting root identity, omitted-or-unfetched state, bootstrap checkpoint, and evidence provenance — from the shared registry and derives the resume-acquisition partial-recovery action gated behind an explicit resume; an evidence packet missing its transcript reference and a discard action that would remove partial state merely because an acquisition was interrupted degrade honestly instead of reading as a clean pass
  - Acquisition-evidence entries: 2 / partial-recovery entries: 2
- **git_service**: `stable`
  - Owner: Git-service owner
  - Scope: The git service resolves the warnings-and-retries evidence kind while keeping the partial state visible, and renders the discard-partial-state partial-recovery action gated behind an explicit discard with a disclosed cleanup; a resolution-form gap on an evidence packet and on a recovery action is caught before a screenshot can reintroduce a false-truth reading
  - Acquisition-evidence entries: 2 / partial-recovery entries: 2
- **trust_service**: `stable`
  - Owner: Trust-service owner
  - Scope: The trust service reports the resulting-root-identity evidence kind and the open-read-only-partial-root partial-recovery action without manual reconstruction; a partial-describing evidence packet that would present partial content as a healthy full checkout before partial-not-full status is disclosed is caught as an overclaim
  - Acquisition-evidence entries: 2 / partial-recovery entries: 1
- **diagnostics**: `stable`
  - Owner: Diagnostics surface owner
  - Scope: Diagnostics resolves the omitted-or-unfetched-state evidence kind while keeping it visible and bound to the registry, and renders the inert-status-report partial-recovery action; an evidence packet that is a hand-copied per-entry assumption and a recovery action on an unclassified class degrade honestly
  - Acquisition-evidence entries: 2 / partial-recovery entries: 2
- **cli_export**: `stable`
  - Owner: CLI-export owner
  - Scope: The CLI export renders the same resolved acquisition-evidence and partial-recovery truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied recovery table
  - Acquisition-evidence entries: 2 / partial-recovery entries: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved acquisition-evidence and partial-recovery truth without embedding raw secrets, so a hand-copied constant, an unstated registry token, a partial content presented as a healthy full checkout, or a partial state left invisible is visible in evidence rather than hidden behind a screenshot
  - Acquisition-evidence entries: 2 / partial-recovery entries: 1
