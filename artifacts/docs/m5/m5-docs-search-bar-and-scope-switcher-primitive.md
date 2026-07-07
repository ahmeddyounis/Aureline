# M5 Docs-Search-Bar and Scope-Switcher Primitive

- Packet: `m5-docs-search-bar-and-scope-switcher-primitive:stable:0001`
- Label: `M5 docs-search-bar and scope-switcher primitive: corpus class, source provider, provider availability, cached/live/mirrored retrieval, version scope, and keyboard hint`
- Docs-search consumers: 5 (5 stable)
- Search-availability postures: search_live_ready, search_cached_ready, search_mirrored_ready, narrowed_provider_degraded, narrowed_policy_limited, degraded_provider_unavailable, degraded_offline_no_corpus, blocked_unknown_state
- Provider availabilities: provider_available, provider_degraded, provider_mirror_only, provider_policy_limited, provider_unavailable, provider_availability_unknown
- Retrieval modes: live_retrieval, cached_retrieval, mirrored_retrieval, offline_bundled_retrieval, no_corpus_available, retrieval_mode_unknown
- Limit reasons: provider_degraded_reduced_corpus, policy_limited_scope, provider_unavailable_offline, no_local_corpus_offline, search_state_unknown
- Proof freshness SLO: 720 hours (last refresh: 2026-07-06T00:00:00Z)

## Docs-search consumers

- **Docs-Browser Search**: `stable`
  - Owner: Docs-browser search owner
  - Scope: The docs-browser search bar renders the shared primitive so a first-party/API search with an available provider reads as live-ready, while a guide search served from a local cache reads as cached-ready rather than being shown as live
  - Worked resolutions: 2
    - `aureline-docs@1.4.0` via `provider_available` → `search_live_ready` (retrieval `live_retrieval`, freshness `live_current`, banner `ready`)
    - `guides@latest-stable` via `provider_available` → `search_cached_ready` (retrieval `cached_retrieval`, freshness `cached_offline`, banner `ready`)
- **Onboarding / Tutorial Lookup**: `stable`
  - Owner: Onboarding / tutorial lookup owner
  - Scope: The onboarding / tutorial lookup renders the shared primitive so a mirror-only lookup reads as mirrored-ready, and a lookup whose provider is degraded reads as narrowed-provider-degraded with a use-cached-corpus next action rather than an unexplained empty result
  - Worked resolutions: 2
    - `onboarding-pack@pinned-2.1` via `provider_mirror_only` → `search_mirrored_ready` (retrieval `mirrored_retrieval`, freshness `recently_synced`, banner `ready`)
    - `project-guides@this-project` via `provider_degraded` → `narrowed_provider_degraded` (retrieval `offline_bundled_retrieval`, freshness `cached_offline`, banner `provider_degraded_reduced_corpus`)
- **AI Citation-Follow**: `stable`
  - Owner: AI citation-follow owner
  - Scope: The AI citation-follow flow renders the shared primitive so a follow whose provider is policy-limited reads as narrowed-policy-limited with a request-policy-access next action, while a follow whose provider is unavailable reads as degraded-provider-unavailable with a retry-when-online next action — never an empty citation with no explanation
  - Worked resolutions: 2
    - `vendor-docs@unversioned` via `provider_policy_limited` → `narrowed_policy_limited` (retrieval `live_retrieval`, freshness `live_current`, banner `policy_limited_scope`)
    - `community-docs@nearby-3.0` via `provider_unavailable` → `degraded_provider_unavailable` (retrieval `cached_retrieval`, freshness `stale_expired`, banner `provider_unavailable_offline`)
- **Support / Help Search**: `stable`
  - Owner: Support / help search owner
  - Scope: The support / help search renders the shared primitive so a search with no local corpus while offline reads as degraded-offline-no-corpus with an import-or-hand-off next action, while a search whose provider availability has not been evaluated reads as blocked-unknown-state with a run-availability-check next action — both degrade to calm explicit messaging
  - Worked resolutions: 2
    - `release-notes@unversioned` via `provider_available` → `degraded_offline_no_corpus` (retrieval `no_corpus_available`, freshness `unknown_freshness`, banner `no_local_corpus_offline`)
    - `help-center@unversioned` via `provider_availability_unknown` → `blocked_unknown_state` (retrieval `live_retrieval`, freshness `unknown_freshness`, banner `search_state_unknown`)
- **CLI Docs Search**: `stable`
  - Owner: CLI docs-search owner
  - Scope: The CLI docs search renders the shared primitive so a headless search whose retrieval mode has not been evaluated reads as blocked-unknown-state, while a codebase-symbol search with an available provider reads as live-ready — the same corpus/provider/scope vocabulary a docs-browser reader sees, reachable without a pointer
  - Worked resolutions: 2
    - `codebase-symbols@this-project` via `provider_available` → `blocked_unknown_state` (retrieval `retrieval_mode_unknown`, freshness `unknown_freshness`, banner `search_state_unknown`)
    - `api-symbols@2.0.0` via `provider_available` → `search_live_ready` (retrieval `live_retrieval`, freshness `live_current`, banner `ready`)
