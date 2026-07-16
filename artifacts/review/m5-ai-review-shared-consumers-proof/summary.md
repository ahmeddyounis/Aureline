# Shared AI-Review Consumers: One Vocabulary Across Surfaces

- Packet: `m5-ai-review-shared-consumers:stable:0001`
- Surface: `M5 AI-review shared consumers (one vocabulary across surfaces)`
- Consumer bindings: 12 (5 narrowed)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-16T00:00:00Z)

## Consumer bindings

- **AI review finding row (one inspectable finding: class, severity / confidence, analyzed scope, lifecycle)** [`arsc-finding-review-detail`]: object `ai_review_finding_row` on `review_detail`, representation `desktop_full`, role `finding_classification`
- **AI review finding row (one inspectable finding: class, severity / confidence, analyzed scope, lifecycle)** [`arsc-finding-finding-row`]: object `ai_review_finding_row` on `finding_row`, representation `desktop_full`, role `finding_classification`
- **AI review finding row (one inspectable finding: class, severity / confidence, analyzed scope, lifecycle)** [`arsc-finding-support`]: object `ai_review_finding_row` on `support_export_packet`, representation `exported_redacted`, role `finding_classification`
- **Review scope selector (analyzed diff scope and repo-instruction / check source)** [`arsc-scope-selector`]: object `review_scope_selector` on `review_scope_selector`, representation `desktop_full`, role `analyzed_scope_disclosure`
- **Review scope selector (analyzed diff scope and repo-instruction / check source)** [`arsc-scope-ai-panel`]: object `review_scope_selector` on `ai_review_panel`, representation `desktop_full`, role `analyzed_scope_disclosure`
- **Review scope selector (analyzed diff scope and repo-instruction / check source)** [`arsc-scope-pending-tray`]: object `review_scope_selector` on `pending_review_tray`, representation `remote_projected`, role `analyzed_scope_disclosure`
- **Publish-to-review sheet (outbound publish mode and provider destination, never implicit)** [`arsc-publish-sheet`]: object `publish_to_review_sheet` on `publish_to_review_sheet`, representation `desktop_full`, role `publish_destination_disclosure`
- **Publish-to-review sheet (outbound publish mode and provider destination, never implicit)** [`arsc-publish-provider-review`]: object `publish_to_review_sheet` on `provider_publish_review`, representation `desktop_full`, role `publish_destination_disclosure`
- **Publish-to-review sheet (outbound publish mode and provider destination, never implicit)** [`arsc-publish-support`]: object `publish_to_review_sheet` on `support_export_packet`, representation `exported_redacted`, role `publish_destination_disclosure`
- **Resolution memory row (durable dismissed / published / outdated / suppressed history)** [`arsc-resolution-ledger`]: object `resolution_memory_row` on `resolution_memory_ledger`, representation `desktop_full`, role `resolution_memory_disclosure`
- **Resolution memory row (durable dismissed / published / outdated / suppressed history)** [`arsc-resolution-review-detail`]: object `resolution_memory_row` on `review_detail`, representation `compact_narrowed`, role `resolution_memory_disclosure`
- **Resolution memory row (durable dismissed / published / outdated / suppressed history)** [`arsc-resolution-support`]: object `resolution_memory_row` on `support_export_packet`, representation `exported_redacted`, role `resolution_memory_disclosure`
