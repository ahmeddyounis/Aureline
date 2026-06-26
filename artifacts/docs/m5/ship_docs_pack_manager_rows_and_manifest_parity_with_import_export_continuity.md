# Docs-Pack Manager Rows And Manifest Parity

- Packet: `packet:docs_pack_manager:001`
- Surface: `workflow:docs_pack_manager_rows_and_import_export_continuity:stable`
- Promotion: `stable`
- Rows: 5 / Profiles: 6

## Managed packs

- **Workspace project docs** (`local_only`): project_docs / stable / pin `unpinned` / refresh `authoritative_live`
- **Standard library mirror** (`mirrored`): mirrored_official_docs / stable / pin `pinned_offline` / refresh `warm_cached`
- **Enterprise cookbook** (`managed`): curated_knowledge_pack / enterprise / pin `pinned_compat_window` / refresh `warm_cached`
- **Support runbook pack** (`air_gapped`): support_runbook / enterprise / pin `pinned_offline` / refresh `degraded_cached`
- **Extension docs pack** (`managed`): extension_docs_pack / beta / pin `pinned` / refresh `unverified`

## Profiles

- `docs_browser_manager`: 1 projection(s)
- `help_pane_manager`: 1 projection(s)
- `onboarding_manager`: 1 projection(s)
- `settings_docs_packs_manager`: 1 projection(s)
- `air_gapped_console`: 1 projection(s)
- `support_export`: 1 projection(s)
