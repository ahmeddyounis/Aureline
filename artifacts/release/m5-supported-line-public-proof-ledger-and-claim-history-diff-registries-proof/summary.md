# M5 Stable-Line Correction-Report and Train-Comparison Registries

- Packet: `m5-supported-line-public-proof-ledger-and-claim-history-diff-registries:stable:0001`
- Label: `M5 public-proof-ledger and claim-history-diff registries publishing one line-by-line public-proof ledger per active supported line — one section per joined proof source: a compatibility report, a benchmark / evidence packet, a support-window statement, a known-limits set, a deprecation report, and a successor report — each bound to one supported-line identity with its freshness state, last-versus-current diff, and the exact evidence-packet refs currently backing its public claims, with rollback posture preserved so onboarding / migration / support language never runs ahead of current public proof, canonical / accessible / audit resolution-form coverage, and a machine-readable claim-history diff (freshness-change, scope-narrowing, or release-line-reassociation) that turns a stale or mismatched proof source into a typed diff event showing current-versus-previous claim-state history, naming the active diff reason across release / help, About, docs, support, and public-proof surfaces`
- Consumer surfaces: 6
- Report sections: compatibility_report_proof, benchmark_packet_proof, support_window_statement_proof, known_limits_set_proof, deprecation_report_proof, successor_report_proof, proof_source_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **shiproom**: `stable`
  - Owner: Shiproom owner
  - Scope: The shiproom resolves the line's compatibility-report proof source to one typed public-proof-ledger object — the affected line rows, joined proof source, linked evidence-packet refs, freshness state, rollback target, and owning roster — from the shared registry and proves the freshness-change diff for that line; a public-proof-ledger object missing its linked evidence and a diff that keeps support language ahead of current proof degrade honestly instead of leaving a claim to read as still current
  - Correction-report entries: 2 / claim-history-diff entries: 2
- **release_center**: `stable`
  - Owner: Release-center owner
  - Scope: The release center resolves the benchmark / evidence-packet proof source and the release-line-reassociation diff while keeping the active diff reason visible; a line widening its claim on stale proof and a resolution-form gap on a diff are caught before a screenshot can reintroduce a still-current reading
  - Correction-report entries: 2 / claim-history-diff entries: 2
- **executive_steering**: `stable`
  - Owner: Executive-steering owner
  - Scope: Executive steering resolves the known-limits-set proof source while keeping its compatibility / known-issues claim matched to current public proof and reports the claim-history-diff outcome; a public-proof-ledger entry that is a hand-copied per-entry assumption and a diff on an unclassified diff scope degrade honestly
  - Correction-report entries: 2 / claim-history-diff entries: 1
- **program_governance**: `stable`
  - Owner: Program-governance owner
  - Scope: Program governance resolves the support-window-statement proof source and the scope-narrowing diff bound to the registry; an unstated registry token on a public-proof-ledger entry is caught before it can drift
  - Correction-report entries: 2 / claim-history-diff entries: 1
- **diagnostics**: `stable`
  - Owner: Diagnostics surface owner
  - Scope: Diagnostics renders the same resolved public-proof-ledger and claim-history-diff truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied line table; the successor-report proof source and the release-line-reassociation diff stay inspectable off-renderer
  - Correction-report entries: 1 / claim-history-diff entries: 1
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved public-proof-ledger and claim-history-diff truth, so a hand-copied constant, an unstated registry token, a widen-on-stale-proof attempt, or support language running ahead of current proof is visible in evidence — a freshness change, a scope narrowing, or a release-line reassociation — rather than hidden behind a screenshot
  - Correction-report entries: 1 / claim-history-diff entries: 1
