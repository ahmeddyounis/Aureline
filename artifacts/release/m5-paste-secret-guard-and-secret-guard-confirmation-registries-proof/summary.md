# M5 Paste-Secret-Guard and Secret-Guard-Confirmation Registries

- Packet: `m5-paste-secret-guard-and-secret-guard-confirmation-registries:stable:0001`
- Label: `M5 paste-secret-guard and secret-guard-confirmation registries emitting one durable machine-readable paste-secret guard per high-risk action — one typed field per record section: the risky-action class (high-risk paste, terminal broadcast, clipboard bridge, debug-evaluate, environment-variable reveal, or variable-body reveal), the disclosed scope, target, and reason, the step-up / confirm posture required, and the visible guardrail badge kept distinct from ordinary presence and follow state — each bound to one session / target scope, so a guard never drops its scope / target / reason and a risky paste or reveal never commits silently, with canonical / accessible / audit resolution-form coverage, and a machine-readable secret-guard confirmation (the confirmation or deny outcome and its audit-safe attribution, re-raised as a fresh visible guard event when a disclose, an allowed-with-confirm, a step-up, a deny, or a blocked event touches a risky action) that names the risky-action class, scope, target, and reason, and whether the action is allowed-with-confirm, requires step-up, is denied, or blocked — so a high-risk paste or reveal never proceeds without disclosing scope, target, and reason, no raw secret body ever crosses into logs or exports, and declined or blocked secret-guard events stay attributable — rather than a green summary across shared terminal / debug, companion follow, control-grant prompt, paste / secret guard, help / docs, and support / export surfaces`
- Consumer surfaces: 6
- Scope selectors: changed_files_scope, pull_request_scope, base_head_range_scope, worktree_uncommitted_scope, full_tree_scope, saved_pack_snapshot_scope, pack_scope_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **shared_terminal_debug_view**: `stable`
  - Owner: Shared-terminal-debug-view owner
  - Scope: The shared terminal / debug view resolves a high-risk action to one typed paste-secret guard — its risky-action class (high-risk paste, terminal broadcast, clipboard bridge, debug-evaluate, environment-variable reveal, or variable-body reveal), the disclosed scope, target, and reason, the step-up / confirm posture required, and the visible guardrail badge — from the shared registry and proves the secret-guard confirmation naming whether the action is allowed-with-confirm, requires step-up, is denied, or blocked; a guard missing its scope / target / reason and a confirmation that would let a risky paste or reveal proceed silently degrade honestly instead of leaking raw secrets or command text
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **collaboration_join_review_sheet**: `stable`
  - Owner: Collaboration-join-review-sheet owner
  - Scope: The join / follow review sheet reads the paste-secret guard and separately names each guard dimension — the risky-action class, the disclosed scope, target, and reason, whether the action is allowed-with-confirm, requires step-up, is denied, or blocked, and the visible guardrail — before any risky paste or reveal commits; a risky action attempting to proceed silently and a secret reveal without an explicit policy / consent posture are caught before a green summary can hide them, so a high-risk paste or reveal never commits without disclosing scope, target, and reason and presence never implies permission to reveal
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support resolves the paste-secret guard's scope / target / reason while keeping the risky-action class, the step-up posture, and the confirmation or deny outcome bound to the export, and reports the secret-guard confirmation and its state; a guard that is a hand-copied per-entry assumption and a confirmation on an unclassified guard binding degrade honestly so declined or blocked secret-guard events stay attributable and exportable as audit-safe metadata without raw secret capture, and raw secrets, command text, variable bodies, or clipboard contents stay outside the export boundary
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **control_grant_prompt**: `stable`
  - Owner: Control-grant-prompt owner
  - Scope: The control-grant prompt resolves the paste-secret guard's posture — view-first default, disclose scope / target / reason, allowed-with-confirm, step-up required, denied, or blocked — bound to the registry so the guard postures can no longer be flattened into one generic confirm dialog; an unstated scope / target / reason on a guard is caught before it can let a risky reveal read as an implicit approval
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **paste_secret_guard**: `stable`
  - Owner: Paste-secret-guard owner
  - Scope: The paste / secret guard renders the same resolved paste-secret guard and secret-guard confirmation truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied sheet; the step-up posture and the guard binding stay inspectable off-renderer so raw secrets, command text, variable bodies, or clipboard contents are never revealed without an explicit policy / consent posture and visible guardrail
  - Review-pack-record entries: 1 / review-pack-result entries: 1
- **help_docs**: `stable`
  - Owner: Help-docs owner
  - Scope: The help / docs feed carries the same resolved paste-secret guard and secret-guard confirmation truth, so a dropped scope / target / reason, an unstated risky-action class, a risky reveal masquerading as an implicit approval, or a declined or blocked event dropping its attribution is visible in evidence — a disclose event, an allowed-with-confirm event, a step-up event, a deny event, or a blocked event — rather than hidden behind a green summary
  - Review-pack-record entries: 1 / review-pack-result entries: 1
