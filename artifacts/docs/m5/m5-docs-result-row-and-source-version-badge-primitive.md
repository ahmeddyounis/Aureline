# M5 Docs-Result-Row and Source-Version-Badge Primitive

- Packet: `m5-docs-result-row-and-source-version-badge-primitive:stable:0001`
- Label: `M5 docs-result-row and source-version-badge primitive: result kind, source provider, source-badge class, version scope, symbol-match confidence, freshness posture, and rank reason`
- Docs-result consumers: 5 (5 stable)
- Source-badge classes: local_project_docs, workspace_spec, first_party_reference, cached_mirrored_reference, live_vendor_upstream, extension_contributed, ai_derived_explanation
- Freshness postures: current_live, recently_synced_current, cached_explicit_not_live, mirrored_explicit_not_live, stale_flagged, freshness_unknown
- Rank factors: project_doc_precedence, version_adjacency, mirror_freshness, explicit_preference, vendor_fallback, policy_scoped_ranking
- Result kinds: doc_page, api_symbol_entry, guide_section, code_symbol_anchor, changelog_entry, example_snippet
- Proof freshness SLO: 720 hours (last refresh: 2026-07-06T00:00:00Z)

## Docs-result consumers

- **Docs-Browser Result**: `stable`
  - Owner: Docs-browser result owner
  - Scope: The docs-browser result list renders the shared primitive so a live first-party doc reads as a first-party reference, a cached API reference reads as cached-explicit-not-live with a mirror-freshness rank reason, and a project-specific codebase symbol reads as local project docs that took precedence over vendor docs
  - Worked resolutions: 3
    - `Getting started` → badge `first_party_reference` (kind `doc_page`, posture `current_live`, confidence `not_symbol_scoped`, rank `default_ranking`)
    - `Client::new` → badge `cached_mirrored_reference` (kind `api_symbol_entry`, posture `cached_explicit_not_live`, confidence `strong_match`, rank `mirror_freshness`)
    - `resolve_run_context` → badge `local_project_docs` (kind `code_symbol_anchor`, posture `recently_synced_current`, confidence `exact_symbol_match`, rank `project_doc_precedence`)
- **AI-Answer Citation**: `stable`
  - Owner: AI-answer citation owner
  - Scope: The AI-answer citation renders the shared primitive so an AI-derived explanation reads as ai-derived-explanation with a version-adjacency rank reason, a stale vendor doc reads as live-vendor-upstream flagged stale with a vendor-fallback rank reason, and a mirrored API reference reads as mirrored-explicit-not-live with a policy-scoped rank reason — never a citation shown as live when it is cached, mirrored, or stale
  - Worked resolutions: 3
    - `How retries work` → badge `ai_derived_explanation` (kind `guide_section`, posture `recently_synced_current`, confidence `heuristic_match`, rank `version_adjacency`)
    - `Vendor SDK guide` → badge `live_vendor_upstream` (kind `doc_page`, posture `stale_flagged`, confidence `not_symbol_scoped`, rank `vendor_fallback`)
    - `Rate limits` → badge `cached_mirrored_reference` (kind `api_symbol_entry`, posture `mirrored_explicit_not_live`, confidence `partial_match`, rank `policy_scoped_ranking`)
- **Onboarding Step Reference**: `stable`
  - Owner: Onboarding step reference owner
  - Scope: The onboarding step reference renders the shared primitive so an extension-contributed guide reads as extension-contributed with an explicit-preference rank reason, while a cached example snippet under default ranking reads as cached-explicit-not-live with no rank-reason disclosure — the same badge/state vocabulary a docs-browser reader sees
  - Worked resolutions: 2
    - `Community setup guide` → badge `extension_contributed` (kind `guide_section`, posture `recently_synced_current`, confidence `not_symbol_scoped`, rank `explicit_preference`)
    - `Config example` → badge `cached_mirrored_reference` (kind `example_snippet`, posture `cached_explicit_not_live`, confidence `not_symbol_scoped`, rank `default_ranking`)
- **Support Answer Result**: `stable`
  - Owner: Support answer result owner
  - Scope: The support answer result renders the shared primitive so a first-party changelog entry with unknown freshness reads as first-party-reference with a freshness-unknown posture, while a cached codebase symbol whose anchor is unresolved reads as a workspace spec that is cached-explicit-not-live — both keep source and version visible without inferring certainty
  - Worked resolutions: 2
    - `Release 1.4 notes` → badge `first_party_reference` (kind `changelog_entry`, posture `freshness_unknown`, confidence `not_symbol_scoped`, rank `default_ranking`)
    - `internal::cache_key` → badge `workspace_spec` (kind `code_symbol_anchor`, posture `cached_explicit_not_live`, confidence `unresolved_symbol`, rank `default_ranking`)
- **CLI Result List**: `stable`
  - Owner: CLI result-list owner
  - Scope: The CLI result list renders the shared primitive so a first-party doc whose match is stale reads as stale-flagged even when its declared freshness is live — never shown as live — while a clean API-symbol result reads as first-party-reference current-live, the same badge/state vocabulary a docs-browser reader sees, reachable without a pointer
  - Worked resolutions: 2
    - `Deprecated flag` → badge `first_party_reference` (kind `doc_page`, posture `stale_flagged`, confidence `not_symbol_scoped`, rank `default_ranking`)
    - `Client::send` → badge `first_party_reference` (kind `api_symbol_entry`, posture `current_live`, confidence `exact_symbol_match`, rank `default_ranking`)
