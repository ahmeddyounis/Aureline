# M5 Docs Handoff Banner & Shared Docs-Browser Consumers

- Packet: `m5-docs-handoff-banner-and-shared-consumers:stable:0001`
- Label: `M5 docs handoff banner and shared docs-browser consumers: destination reason, in-product necessity, privacy consequence, return path, and source/version/pack context preserved across docs-browser, onboarding, glossary, AI-evidence, and support/help`
- Handoff consumers: 5 (5 stable)
- Necessities: cannot_serve_in_product, should_defer_to_canonical, user_requested_external
- Privacy consequences: stays_fully_in_product, anonymous_lookup_only, query_context_shared, identified_request_shared, external_account_and_identity_shared
- Shared components: search_bar, result_row, reference_card, source_version_badge, pack_row, stale_example_row, handoff_banner
- Proof freshness SLO: 720 hours (last refresh: 2026-07-06T00:00:00Z)

## Handoff consumers

- **Docs Browser**: `stable`
  - Owner: Docs browser owner
  - Scope: The docs browser renders the shared handoff banner so a no-local-corpus lookup reads as cannot-serve-in-product with the query context shared and the source/version preserved on return, while a user-requested open of a bundled mirror stays fully in-product — reusing the same search bar, result row, source/version badge, and pack row it renders everywhere else
  - Reused components: search_bar|result_row|source_version_badge|pack_row|handoff_banner
  - Worked handoff banners: 2
    - `external:rust-std-docs` → cannot_serve_in_product / query_context_shared (return `context_preserved_return`, context-preserved `true`, leaves-boundary `true`)
    - `external:bundled-guide-mirror` → user_requested_external / stays_fully_in_product (return `anchored_return`, context-preserved `false`, leaves-boundary `false`)
- **Onboarding Tour**: `stable`
  - Owner: Onboarding tour owner
  - Scope: The onboarding tour renders the shared handoff banner so a dynamic-rendering-only playground reads as cannot-serve-in-product with only an anonymous lookup leaving Aureline, and the tour step is preserved as the return anchor so the learner comes back where they left off — reusing the same search bar, result row, stale-example row, and handoff banner
  - Reused components: search_bar|result_row|stale_example_row|handoff_banner
  - Worked handoff banners: 1
    - `external:interactive-playground` → cannot_serve_in_product / anonymous_lookup_only (return `context_preserved_return`, context-preserved `true`, leaves-boundary `true`)
- **Glossary Card**: `stable`
  - Owner: Glossary card owner
  - Scope: The glossary card renders the shared handoff banner so an external-canonical definition reads as should-defer-to-canonical with the query context shared and the glossary term preserved on return — reusing the same symbol-reference card, source/version badge, and handoff banner it shows in hover and peek
  - Reused components: reference_card|source_version_badge|handoff_banner
  - Worked handoff banners: 1
    - `external:canonical-glossary` → should_defer_to_canonical / query_context_shared (return `context_preserved_return`, context-preserved `true`, leaves-boundary `true`)
- **AI-Evidence Follow Link**: `stable`
  - Owner: AI-evidence follow owner
  - Scope: The AI-evidence follow link renders the shared handoff banner so an auth-gated vendor portal is escalated to an identified request — never understated as no-data-leaves just because the citation was local — and an external API console reads as sharing an external account and identity; both preserve the citation's source/version on return, reusing the same result row, symbol-reference card, pack row, and handoff banner
  - Reused components: result_row|reference_card|pack_row|handoff_banner
  - Worked handoff banners: 2
    - `external:vendor-portal` → should_defer_to_canonical / identified_request_shared (return `context_preserved_return`, context-preserved `true`, leaves-boundary `true`)
    - `external:api-console` → should_defer_to_canonical / external_account_and_identity_shared (return `context_preserved_return`, context-preserved `true`, leaves-boundary `true`)
- **Support / Help**: `stable`
  - Owner: Support / help owner
  - Scope: The support / help view renders the shared handoff banner so a no-local-corpus lookup into the support knowledge base reads as cannot-serve-in-product with an identified request shared, and the ticket context is preserved on return so the handoff survives the support/export path with the same words — reusing the same pack row, stale-example row, source/version badge, and handoff banner
  - Reused components: pack_row|stale_example_row|source_version_badge|handoff_banner
  - Worked handoff banners: 1
    - `external:support-kb` → cannot_serve_in_product / identified_request_shared (return `context_preserved_return`, context-preserved `true`, leaves-boundary `true`)
