# Browser / Provider-Console Handoff Objects

- Packet: `packet:browser_provider_console_handoff_objects:001`
- Surface: `workflow:browser_provider_console_handoff_objects_with_destination_reason_privacy_consequence_and_return_anchor:stable`
- Promotion: `stable` (0 validation findings)
- Handoffs: 5 / Projections: 4

## Handoffs

- **handoff:docs_browser:tokio-spawn-anchor** (`docnode:mirror:tokio/runtime#spawn`): from `docs_browser` to `docs_or_portal_web`
   - reason `exact_anchor_unavailable_locally`: the exact anchor is not in the local mirror; the upstream portal has it
   - privacy `scoped_url_only` / trust `signed_mirror_verified` / policy `allowed_explicit`
   - return `back_to_docs_browser`: Back to the tokio::spawn peek
- **handoff:help_about:product-docs-portal** (`surface:help_about:docs-link`): from `help_about` to `docs_or_portal_web`
   - reason `source_not_mirrored`: the full getting-started portal is not mirrored in-product
   - privacy `no_context_shared` / trust `first_party_authoritative` / policy `allowed_explicit`
   - return `back_to_help_about`: Back to Help / About
- **handoff:ai_answer:provider-search** (`explanation:ai_answer:where-is-the-runtime-built`): from `ai_answer` to `ai_provider_web`
   - reason `user_requested_open_in_browser`: the reader asked to continue the search on the provider's web surface
   - privacy `query_terms_disclosed` / trust `live_provider_handoff` / policy `requires_confirmation`
   - return `back_to_ai_answer`: Back to the AI answer
- **handoff:provider_console:managed-admin** (`surface:provider_console:connected-provider`): from `provider_console_pivot` to `managed_admin_web`
   - reason `user_requested_open_in_browser`: provider account management lives only on the hosted admin console
   - privacy `isolated_session` / trust `live_provider_handoff` / policy `allowed_explicit`
   - return `back_to_provider_panel`: Back to the connected-provider panel
- **handoff:review_surface:hosted-thread** (`review-thread:pr-anchor`): from `review_surface` to `code_host_web`
   - reason `review_thread_requires_hosted_view`: the full review thread requires the hosted review view
   - privacy `scoped_url_only` / trust `live_provider_handoff` / policy `allowed_explicit`
   - return `back_to_review_panel`: Back to the review panel
