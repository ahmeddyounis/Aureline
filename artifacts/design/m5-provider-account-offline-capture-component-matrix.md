# M5 Provider-Account-Row, Project-or-Board-Mapping-Row, Sync-Behavior-Row, Offline-Capture-Row, and Privacy-Redaction-Row Component Matrix

- Packet: `m5-provider-account-offline-capture-components:stable:0001`
- Label: `M5 provider-account-row, project-or-board-mapping-row, sync-behavior-row, offline-capture-row, and privacy-redaction-row component matrix`
- Component families: 5 (5 stable)
- Account connection states: not_configured, signed_in, limited_scope, stale_session, offline_cached_read, policy_blocked
- Sync modes: live_bidirectional, read_only_mirror, manual_push, scheduled_sync, paused_sync, offline_only
- Proof freshness SLO: 720 hours (last refresh: 2026-07-07T00:00:00Z)

## Component families

- **provider_account_row**: `stable`
  - Owner: Provider-account row owner
  - Scope: One provider-account-row model naming how the acting account is identified — a personal account, an organization member, a service account, a delegated credential, an installation grant, or an unlinked identity — its connection state (not configured, signed in, limited scope, stale session, offline cached read, or policy blocked), and the tenant scope it acts within, so a user never has to infer whether Aureline can read or write right now
  - Required labels: identity, state, keyboard_route, connection_and_scope
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **project_or_board_mapping_row**: `stable`
  - Owner: Project/board mapping row owner
  - Scope: One project-or-board-mapping-row model naming where a publish will land — an issue-tracker project, a kanban board, a repository, a milestone, a label set, or an unmapped target — and how that default destination was derived (explicit user choice, inherited default, auto-matched, imported config, policy pinned, or unmapped origin), so a default publish destination is never assumed silently and never given an alternate label for its origin
  - Required labels: identity, state, keyboard_route, mapping_and_sync_mode
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **sync_behavior_row**: `stable`
  - Owner: Sync-behavior row owner
  - Scope: One sync-behavior-row model naming how local and provider truth are kept in step — live bidirectional, read-only mirror, manual push, scheduled sync, paused sync, or offline only — the effective write scope Aureline has right now (full write, comment only, status only, read only, no write, or unknown), and the state of any locally queued draft, so a user never has to infer whether Aureline can write or what remains queued locally
  - Required labels: identity, state, keyboard_route, mapping_and_sync_mode
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **offline_capture_row**: `stable`
  - Owner: Offline-capture row owner
  - Scope: One offline-capture-row model naming how a locally captured change is held — captured local, queued for publish, publish deferred, conflict held, discard pending, or synced and cleared — and the state of its queued draft, so a user always sees what remains queued locally and a pending publish is never silently dropped or shown as reconciled
  - Required labels: identity, state, keyboard_route, connection_and_scope
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **privacy_redaction_row**: `stable`
  - Owner: Privacy-redaction row owner
  - Scope: One privacy-redaction-row model naming how much of a provider-linked object will be revealed — full body visible, metadata only, redacted share, policy restricted, raw bodies withheld, or no export — and the metadata-safe export boundary it keeps, so a user always sees what support and export flows will disclose and no surface invents an alternate label for a metadata-safe export
  - Required labels: identity, state, keyboard_route, redaction_and_export_boundary
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
