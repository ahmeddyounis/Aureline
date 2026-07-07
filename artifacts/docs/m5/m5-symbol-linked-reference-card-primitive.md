# M5 Symbol-Linked Reference-Card Primitive

- Packet: `m5-symbol-linked-reference-card-primitive:stable:0001`
- Label: `M5 symbol-linked reference-card primitive: initiating code anchor, symbol anchor, linkage strength, source provider, version scope, cited revision, and freshness posture`
- Reference-card consumers: 5 (5 stable)
- Linkage strengths: exact_symbol_linkage, nearby_version_linkage, project_specific_linkage, keyword_fallback_linkage, heuristic_linkage, unresolved_no_linkage
- Freshness postures: current_live, recently_synced_current, cached_explicit_not_live, mirrored_explicit_not_live, stale_flagged, freshness_unknown
- Symbol anchors: function_symbol, type_symbol, module_symbol, field_or_method, macro_symbol, unresolved_anchor
- Proof freshness SLO: 720 hours (last refresh: 2026-07-06T00:00:00Z)

## Reference-card consumers

- **Editor Hover / Peek**: `stable`
  - Owner: Editor hover/peek reference-card owner
  - Scope: The editor hover/peek reference card keeps the initiating file/symbol anchor visible so an exact live function match reads as exact-symbol linkage, a nearby vendor type reads as nearby-version linkage, and an unresolved anchor served from a mirror reads as a keyword fallback — never an exact symbol match
  - Worked resolutions: 3
    - `Client::send` from `src/client.rs::Client::send` → linkage `exact_symbol_linkage` (anchor `function_symbol`, posture `current_live`, resolved `true`)
    - `Widget` from `src/ui/widget.rs::Widget` → linkage `nearby_version_linkage` (anchor `type_symbol`, posture `recently_synced_current`, resolved `true`)
    - `retry helpers` from `src/net/retry.rs::retry_backoff` → linkage `keyword_fallback_linkage` (anchor `unresolved_anchor`, posture `cached_explicit_not_live`, resolved `false`)
- **Docs-Browser Card**: `stable`
  - Owner: Docs-browser reference-card owner
  - Scope: The docs-browser reference card renders the shared primitive so a project-specific codebase symbol reads as project-specific linkage, a mirror-served type reads as heuristic linkage that is mirrored-explicit-not-live, and an exact live module match reads as exact-symbol linkage — the same anchor/linkage vocabulary the editor shows
  - Worked resolutions: 3
    - `resolve_run_context` from `crates/aureline-shell/src/run.rs::resolve_run_context` → linkage `project_specific_linkage` (anchor `function_symbol`, posture `recently_synced_current`, resolved `true`)
    - `Config` from `src/config.rs::Config` → linkage `heuristic_linkage` (anchor `type_symbol`, posture `mirrored_explicit_not_live`, resolved `true`)
    - `logging module` from `src/logging/mod.rs::logging` → linkage `exact_symbol_linkage` (anchor `module_symbol`, posture `current_live`, resolved `true`)
- **AI-Explanation Card**: `stable`
  - Owner: AI-explanation reference-card owner
  - Scope: The AI-explanation reference card renders the shared primitive so an AI-derived field/method reads as nearby-version linkage with an unknown-freshness posture, an unresolved stale anchor reads as unresolved-no-linkage flagged stale, and an exact local macro reads as exact-symbol linkage — never an AI paraphrase that hides how weak the linkage is
  - Worked resolutions: 3
    - `Backoff::max_delay` from `src/net/retry.rs::Backoff::max_delay` → linkage `nearby_version_linkage` (anchor `field_or_method`, posture `freshness_unknown`, resolved `true`)
    - `legacy config keys` from `src/config/legacy.rs::LEGACY_KEYS` → linkage `unresolved_no_linkage` (anchor `unresolved_anchor`, posture `stale_flagged`, resolved `false`)
    - `declare_component macro` from `src/macros.rs::declare_component` → linkage `exact_symbol_linkage` (anchor `macro_symbol`, posture `current_live`, resolved `true`)
- **Onboarding Reference Card**: `stable`
  - Owner: Onboarding reference-card owner
  - Scope: The onboarding reference card renders the shared primitive so an exact live first-party function reads as exact-symbol linkage, while a cached community plugin type reads as heuristic linkage that is cached-explicit-not-live — the same anchor/linkage/freshness vocabulary a docs-browser reader sees
  - Worked resolutions: 2
    - `quickstart main` from `examples/quickstart.rs::main` → linkage `exact_symbol_linkage` (anchor `function_symbol`, posture `current_live`, resolved `true`)
    - `PluginRegistry` from `src/plugins/mod.rs::PluginRegistry` → linkage `heuristic_linkage` (anchor `type_symbol`, posture `cached_explicit_not_live`, resolved `true`)
- **Support Evidence Card**: `stable`
  - Owner: Support evidence reference-card owner
  - Scope: The support evidence reference card renders the shared primitive so an exact release-notes field decided by policy reads as exact-symbol linkage with a recently-synced posture, while a stale vendor type whose upstream was unavailable reads as heuristic linkage flagged stale — both keep the initiating anchor and source descriptors so identity survives the support/AI evidence path
  - Worked resolutions: 2
    - `VERSION` from `src/version.rs::VERSION` → linkage `exact_symbol_linkage` (anchor `field_or_method`, posture `recently_synced_current`, resolved `true`)
    - `VendorClient` from `src/vendor/auth.rs::VendorClient` → linkage `heuristic_linkage` (anchor `type_symbol`, posture `stale_flagged`, resolved `true`)
