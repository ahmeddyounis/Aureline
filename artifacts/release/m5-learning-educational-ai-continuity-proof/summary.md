# Learning educational-AI continuity controls

- Packet: `m5-learning-educational-ai-continuity-controls:stable:0001`
- Surface: `M5 learning educational-AI continuity: live, cached, local-only, offline, stale-pack, citation-unavailable, and not-installed states with subject-first continuity, derived trust and next-safe-action, no-hidden-apply preview/approval boundaries, safe explain verbs, and a cited source fallback while learning stays useful offline`
- Degraded components: 7 (6 not live, 1 offering a preview/approval-gated apply)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## Degraded components

- **Learning mode (live)** (learning_mode_toggle) — scope `workspace`, state `live` → trust `live_enriched`, next `proceed_in_learning`, apply `no_mutation`, source `command_reference`
- **Tip: keyboard palette (cached)** (tip_card) — scope `surface`, state `cached` → trust `cached_pack`, next `refresh_enrichment`, apply `no_mutation`, source `docs_page`
- **Exercise: rename a symbol (local-only)** (guided_exercise_step) — scope `session`, state `local_only` → trust `local_only_bounded`, next `continue_local_only`, apply `sandbox_mutation_only`, source `sandbox_target`
- **Glossary: change object (offline)** (glossary_chip_or_card) — scope `feature_family`, state `offline` → trust `offline_held`, next `retry_when_online`, apply `no_mutation`, source `file_location`
- **Explanation: apply the suggested fix (stale pack)** (safe_explanation_banner) — scope `feature_family`, state `stale_pack` → trust `stale_unverified`, next `update_docs_pack`, apply `preview_approval_required`, source `symbol_location`
- **Glossary: build farm (citation unavailable)** (glossary_chip_or_card) — scope `feature_family`, state `citation_unavailable` → trust `uncited_withheld`, next `show_uncited_explicitly`, apply `mutation_unavailable`, source `no_source`
- **Progress: guided tour (not installed)** (progress_marker) — scope `unavailable`, state `not_installed` → trust `not_installed_unavailable`, next `install_to_enable`, apply `mutation_unavailable`, source `no_source`
