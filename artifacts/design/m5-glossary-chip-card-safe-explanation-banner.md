# Glossary chips/cards and safe explanation banners

- Packet: `m5-glossary-chip-card-safe-explanation-banner-controls:stable:0001`
- Surface: `M5 glossary chips/cards and safe explanation banners: term meaning with cited file/symbol/docs source truth, freshness and source-class labels, open-related-concept actions, and an explicit explain-versus-do boundary so an explanation never implies an apply-capable action or hidden authority across claimed learning surfaces`
- Glossary chips/cards: 6 (4 not cited-current)
- Safe explanation banners: 6 (1 explain-only)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## Glossary chips and cards

- **Preview then apply** — source `cited_docs`, citation `citation_current` → `cited_current`, cited via `docs_anchor`
- **Sandbox** — source `cited_spec`, citation `citation_versioned` → `cited_current`, cited via `symbol_location`
- **Review thread** — source `cited_help_pack`, citation `citation_cached` → `cited_cached`, cited via `file_location`
- **Quiet hours** — source `community_note`, citation `citation_stale` → `cited_stale`, cited via `no_deep_link`
- **Mirror mode** — source `uncited_draft`, citation `citation_offline_unavailable` → `offline_unavailable`, cited via `no_deep_link`
- **Handoff** — source `unknown_source`, citation `citation_missing` → `uncited`, cited via `no_deep_link`

## Safe explanation banners

- **Why this result is suggested** — boundary `explain_only`, apply `no_apply` → `explain_only`, offers-do false
- **What this term means** — boundary `explain_then_offer_do`, apply `preview_available` → `preview_offered`, offers-do true
- **This change needs a preview first** — boundary `preview_required`, apply `approval_pending` → `approval_pending`, offers-do true
- **This change was approved and applied** — boundary `approval_required`, apply `applied_with_undo` → `applied_reversible`, offers-do true
- **This action is sandboxed** — boundary `sandboxed_only`, apply `blocked_apply` → `apply_withheld`, offers-do true
- **Nothing is applied without your approval** — boundary `no_hidden_apply`, apply `mutation_declined` → `apply_withheld`, offers-do true
