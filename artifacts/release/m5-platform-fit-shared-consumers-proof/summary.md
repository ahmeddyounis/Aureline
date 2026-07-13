# Shared Platform-Fit Consumers: One Convention Across Surfaces

- Packet: `m5-platform-fit-shared-consumers:stable:0001`
- Surface: `M5 platform-fit shared consumers (one convention across surfaces)`
- Consumer bindings: 18 (7 narrowed)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-13T00:00:00Z)

## Consumer bindings

- **Window controls and menu-bar convention** [`pf-convention-shell`]: family `platform_convention` on `shell_ui`, representation `desktop_full`, role `window_menu`
- **Window controls and menu-bar convention** [`pf-convention-docs`]: family `platform_convention` on `docs_help`, representation `desktop_full`, role `window_menu`
- **Window controls and menu-bar convention** [`pf-convention-cli`]: family `platform_convention` on `cli_export`, representation `exported_redacted`, role `window_menu`
- **Platform-native shortcut notation** [`pf-shortcut-shell`]: family `shortcut_notation` on `shell_ui`, representation `desktop_full`, role `shortcut`
- **Platform-native shortcut notation** [`pf-shortcut-settings`]: family `shortcut_notation` on `settings_ui`, representation `desktop_full`, role `shortcut`
- **Platform-native shortcut notation** [`pf-shortcut-support`]: family `shortcut_notation` on `support_export`, representation `exported_redacted`, role `shortcut`
- **File / path / reveal / save terminology** [`pf-path-settings`]: family `file_path_reveal` on `settings_ui`, representation `desktop_full`, role `path_terminology`
- **File / path / reveal / save terminology** [`pf-path-docs`]: family `file_path_reveal` on `docs_help`, representation `desktop_full`, role `path_terminology`
- **File / path / reveal / save terminology** [`pf-path-cli`]: family `file_path_reveal` on `cli_export`, representation `exported_redacted`, role `path_terminology`
- **Live theme / contrast / accent / text-scale response** [`pf-theme-shell`]: family `theme_contrast_live_change` on `shell_ui`, representation `desktop_full`, role `appearance`
- **Live theme / contrast / accent / text-scale response** [`pf-theme-settings`]: family `theme_contrast_live_change` on `settings_ui`, representation `desktop_full`, role `appearance`
- **Live theme / contrast / accent / text-scale response** [`pf-theme-product`]: family `theme_contrast_live_change` on `product_ui`, representation `remote_projected`, role `appearance`
- **Credential-store wording** [`pf-credential-auth`]: family `credential_store_wording` on `auth_ui`, representation `desktop_full`, role `credential_wording`
- **Credential-store wording** [`pf-credential-settings`]: family `credential_store_wording` on `settings_ui`, representation `desktop_full`, role `credential_wording`
- **Credential-store wording** [`pf-credential-support`]: family `credential_store_wording` on `support_export`, representation `exported_redacted`, role `credential_wording`
- **IME / dead-key / AltGr / dictation / emoji / layout input** [`pf-input-input`]: family `input_method` on `input_ui`, representation `desktop_full`, role `input_fidelity`
- **IME / dead-key / AltGr / dictation / emoji / layout input** [`pf-input-onboarding`]: family `input_method` on `onboarding`, representation `compact_narrowed`, role `input_fidelity`
- **IME / dead-key / AltGr / dictation / emoji / layout input** [`pf-input-product`]: family `input_method` on `product_ui`, representation `remote_projected`, role `input_fidelity`
