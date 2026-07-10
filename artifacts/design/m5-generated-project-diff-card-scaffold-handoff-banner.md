# Generated-project diff cards and scaffold handoff banners

- Packet: `m5-generated-project-diff-card-scaffold-handoff-banner-controls:stable:0001`
- Surface: `M5 generated-project diff cards and scaffold handoff banners: create/modify/rename/delete counts, dependency-task-extension impact, trust state, and run-now/later/review recovery across claimed generation flows`
- Generated-project diff cards: 6 (3 blocked)
- Scaffold handoff banners: 6 (2 needing recovery)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## Generated-project diff cards

- **Generated project files** — zone `generated_only` → `generated_owned`, review `preview_ready` → `reviewable_preview`, source `template_starter`, +24/~0/»0/-0 (created/modified/renamed/deleted), deep link `template_manifest`
- **User-owned files touched** — zone `user_owned` → `user_owned`, review `review_required` → `review_required_before_write`, source `user_authored`, +0/~3/»0/-0 (created/modified/renamed/deleted), deep link `docs_anchor`
- **Regeneration conflict** — zone `generated_then_edited` → `generated_then_user_edited`, review `conflict_detected` → `conflict_blocked`, source `framework_generator`, +0/~4/»2/-0 (created/modified/renamed/deleted), deep link `docs_anchor`
- **Runtime-only output** — zone `runtime_only` → `runtime_only`, review `no_changes` → `no_changes_to_review`, source `codemod`, +0/~0/»0/-0 (created/modified/renamed/deleted), deep link `docs_anchor`
- **Mixed-ownership tree** — zone `mixed_zone` → `mixed_ownership`, review `diff_unavailable` → `diff_unavailable_blocked`, source `imported_source`, +6/~5/»0/-4 (created/modified/renamed/deleted), deep link `starter_registry_entry`
- **Unknown-ownership zone** — zone `zone_unknown` → `ownership_unknown`, review `blocked` → `diff_unavailable_blocked`, source `framework_generator`, +0/~2/»0/-0 (created/modified/renamed/deleted), deep link `policy_reference`

## Scaffold handoff banners

- **Workspace created** — outcome `create_succeeded` → `clean_create`, trust `trusted`, health `All preflight checks passing`, deep link `template_manifest`
- **Workspace partially created** — outcome `partial_bootstrap` → `partial_needs_recovery`, trust `trust_prompt_pending`, health `Some setup steps did not finish`, deep link `docs_anchor`
- **Workspace create failed** — outcome `create_failed` → `failed_needs_recovery`, trust `untrusted_blocked`, health `Create failed before completion`, deep link `policy_reference`
- **Continued without a starter** — outcome `continued_without_starter` → `continued_without_starter`, trust `restricted_trust`, health `No starter applied; workspace is empty of generated files`, deep link `docs_anchor`
- **Created empty workspace** — outcome `created_empty` → `created_empty`, trust `trust_not_applicable`, health `Empty workspace; no runnable output yet`, deep link `docs_anchor`
- **Remote provisioning pending** — outcome `provisioning_pending` → `provisioning_pending`, trust `trust_prompt_pending`, health `Local files ready; remote provisioning still in progress`, deep link `policy_reference`
