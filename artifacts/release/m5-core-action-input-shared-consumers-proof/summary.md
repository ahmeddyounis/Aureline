# Shared Core Action / Input Control Consumers: One Vocabulary Across Surfaces

- Packet: `m5-core-action-input-shared-consumers:stable:0001`
- Surface: `M5 core action / input shared consumers`
- Consumer bindings: 19 (11 narrowed)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-12T00:00:00Z)

## Consumer bindings

- **Settings apply-changes button** [`cc-button-settings`]: control `button` on `settings_ui`, representation `desktop_full`, state `default`
- **Settings apply-changes button** [`cc-button-support`]: control `button` on `support_export`, representation `exported_redacted`, state `default`
- **Request delete-row icon button** [`cc-iconbutton-forms`]: control `icon_button` on `forms_ui`, representation `desktop_full`, state `default`
- **Request delete-row icon button** [`cc-iconbutton-review`]: control `icon_button` on `review_ui`, representation `compact_narrowed`, state `default`
- **Package install split button** [`cc-split-product`]: control `split_button` on `product_ui`, representation `desktop_full`, state `default`
- **Package install split button** [`cc-split-repair`]: control `split_button` on `repair_ui`, representation `compact_narrowed`, state `default`
- **Package install split button** [`cc-split-cli`]: control `split_button` on `cli_export`, representation `exported_redacted`, state `default`
- **Provider account display-name text field** [`cc-textfield-settings`]: control `text_field` on `settings_ui`, representation `desktop_full`, state `default`
- **Provider account display-name text field** [`cc-textfield-forms`]: control `text_field` on `forms_ui`, representation `compact_narrowed`, state `default`
- **Admin policy search field** [`cc-search-search`]: control `search_field` on `search_ui`, representation `desktop_full`, state `default`
- **Admin policy search field** [`cc-search-support`]: control `search_field` on `support_export`, representation `exported_redacted`, state `default`
- **Entry starter-template combobox** [`cc-combobox-entry`]: control `combobox` on `entry_ui`, representation `desktop_full`, state `default`
- **Entry starter-template combobox** [`cc-combobox-forms`]: control `combobox` on `forms_ui`, representation `remote_projected`, state `default`
- **Entry starter-template combobox** [`cc-combobox-cli`]: control `combobox` on `cli_export`, representation `exported_redacted`, state `default`
- **Admin policy-enforcement toggle** [`cc-toggle-settings`]: control `toggle_control` on `settings_ui`, representation `desktop_full`, state `locked`
- **Admin policy-enforcement toggle** [`cc-toggle-product`]: control `toggle_control` on `product_ui`, representation `compact_narrowed`, state `locked`
- **Admin policy-enforcement toggle** [`cc-toggle-support`]: control `toggle_control` on `support_export`, representation `exported_redacted`, state `locked`
- **Start-center entry-mode segmented control** [`cc-segmented-entry`]: control `segmented_control` on `entry_ui`, representation `desktop_full`, state `default`
- **Start-center entry-mode segmented control** [`cc-segmented-review`]: control `segmented_control` on `review_ui`, representation `remote_projected`, state `default`
