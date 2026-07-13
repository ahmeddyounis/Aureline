# Shared Decision / Feedback Primitive Consumers: One Vocabulary Across Surfaces

- Packet: `m5-decision-feedback-shared-consumers:stable:0001`
- Surface: `M5 decision / feedback shared consumers`
- Consumer bindings: 19 (11 narrowed)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-13T00:00:00Z)

## Consumer bindings

- **Provider account trust badge** [`cc-badge-settings`]: primitive `badge_chip_pill` on `settings_ui`, representation `desktop_full`, disposition `info`
- **Provider account trust badge** [`cc-badge-support`]: primitive `badge_chip_pill` on `support_export`, representation `exported_redacted`, disposition `info`
- **Help keyboard-shortcut popover** [`cc-popover-help`]: primitive `popover` on `help_ui`, representation `desktop_full`, disposition `info`
- **Help keyboard-shortcut popover** [`cc-popover-review`]: primitive `popover` on `review_ui`, representation `compact_narrowed`, disposition `info`
- **Repair confirm-destructive dialog** [`cc-dialog-review`]: primitive `dialog_sheet` on `review_ui`, representation `desktop_full`, disposition `warning`
- **Repair confirm-destructive dialog** [`cc-dialog-product`]: primitive `dialog_sheet` on `product_ui`, representation `compact_narrowed`, disposition `warning`
- **Repair confirm-destructive dialog** [`cc-dialog-cli`]: primitive `dialog_sheet` on `cli_export`, representation `exported_redacted`, disposition `warning`
- **Updates advisory banner** [`cc-banner-updates`]: primitive `banner_inline_notice` on `updates_ui`, representation `desktop_full`, disposition `warning`
- **Updates advisory banner** [`cc-banner-shell`]: primitive `banner_inline_notice` on `shell_ui`, representation `remote_projected`, disposition `warning`
- **Settings saved toast** [`cc-toast-settings`]: primitive `toast` on `settings_ui`, representation `desktop_full`, disposition `success`
- **Settings saved toast** [`cc-toast-support`]: primitive `toast` on `support_export`, representation `exported_redacted`, disposition `success`
- **Review empty-queue state** [`cc-empty-review`]: primitive `empty_state` on `review_ui`, representation `desktop_full`, disposition `info`
- **Review empty-queue state** [`cc-empty-shell`]: primitive `empty_state` on `shell_ui`, representation `compact_narrowed`, disposition `info`
- **Shell dependency-load state** [`cc-loading-shell`]: primitive `loading_state` on `shell_ui`, representation `desktop_full`, disposition `pending`
- **Shell dependency-load state** [`cc-loading-support`]: primitive `loading_state` on `support_ui`, representation `remote_projected`, disposition `pending`
- **Shell dependency-load state** [`cc-loading-cli`]: primitive `loading_state` on `cli_export`, representation `exported_redacted`, disposition `pending`
- **Provider disconnect consequence block** [`cc-consequence-settings`]: primitive `consequence_block` on `settings_ui`, representation `desktop_full`, disposition `blocked`
- **Provider disconnect consequence block** [`cc-consequence-support`]: primitive `consequence_block` on `support_ui`, representation `compact_narrowed`, disposition `blocked`
- **Provider disconnect consequence block** [`cc-consequence-support-export`]: primitive `consequence_block` on `support_export`, representation `exported_redacted`, disposition `blocked`
