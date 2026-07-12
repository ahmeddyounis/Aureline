# Shared Editor-Inline Component Consumers: One Vocabulary Across Surfaces

- Packet: `m5-editor-inline-shared-consumers:stable:0001`
- Surface: `M5 editor-inline shared consumers`
- Consumer bindings: 21 (13 narrowed)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-12T00:00:00Z)

## Consumer bindings

- **src/main.rs (editor tab)** [`eb-tab-editor`]: component `editor_tab` on `editor_ui`, representation `desktop_full`, state `modified`
- **src/main.rs (editor tab)** [`eb-tab-support`]: component `editor_tab` on `support_export`, representation `exported_redacted`, state `modified`
- **src/main.rs:42 gutter marker** [`eb-gutter-editor`]: component `gutter` on `editor_ui`, representation `desktop_full`, state `modified`
- **src/main.rs:42 gutter marker** [`eb-gutter-diff`]: component `gutter` on `diff_ui`, representation `compact_narrowed`, state `modified`
- **src/main.rs:88 diagnostic** [`eb-diag-diagnostics`]: component `diagnostic_decoration` on `diagnostics_ui`, representation `desktop_full`, state `outdated`
- **src/main.rs:88 diagnostic** [`eb-diag-editor`]: component `diagnostic_decoration` on `editor_ui`, representation `compact_narrowed`, state `outdated`
- **src/main.rs:88 diagnostic** [`eb-diag-support`]: component `diagnostic_decoration` on `support_export`, representation `exported_redacted`, state `outdated`
- **src/main.rs:88 code action** [`eb-chip-editor`]: component `code_action_chip` on `editor_ui`, representation `desktop_full`, state `inferred_fix`
- **src/main.rs:88 code action** [`eb-chip-diagnostics`]: component `code_action_chip` on `diagnostics_ui`, representation `compact_narrowed`, state `inferred_fix`
- **pr-12 src/main.rs diff** [`eb-diff-diff`]: component `diff_view` on `diff_ui`, representation `desktop_full`, state `modified`
- **pr-12 src/main.rs diff** [`eb-diff-review`]: component `diff_view` on `review_ui`, representation `remote_projected`, state `modified`
- **pr-12 src/main.rs diff** [`eb-diff-cli`]: component `diff_view` on `cli_export`, representation `exported_redacted`, state `modified`
- **pr-12 comment 3 thread** [`eb-thread-review`]: component `review_thread` on `review_ui`, representation `desktop_full`, state `resolved`
- **pr-12 comment 3 thread** [`eb-thread-support`]: component `review_thread` on `support_export`, representation `exported_redacted`, state `resolved`
- **pr-12 comment 3 thread** [`eb-thread-product`]: component `review_thread` on `product_ui`, representation `remote_projected`, state `resolved`
- **chat-9 message 2 card** [`eb-card-ai`]: component `ai_message_card` on `ai_ui`, representation `desktop_full`, state `review_required`
- **chat-9 message 2 card** [`eb-card-notebook`]: component `ai_message_card` on `notebook_ui`, representation `compact_narrowed`, state `review_required`
- **chat-9 message 2 card** [`eb-card-support`]: component `ai_message_card` on `support_export`, representation `exported_redacted`, state `review_required`
- **chat-9 message 2 evidence** [`eb-evid-ai`]: component `evidence_timeline` on `ai_ui`, representation `desktop_full`, state `export_safe_evidence`
- **chat-9 message 2 evidence** [`eb-evid-notebook`]: component `evidence_timeline` on `notebook_ui`, representation `compact_narrowed`, state `export_safe_evidence`
- **chat-9 message 2 evidence** [`eb-evid-cli`]: component `evidence_timeline` on `cli_export`, representation `exported_redacted`, state `export_safe_evidence`
