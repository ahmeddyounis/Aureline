# Open-in-browser fallback fixtures

Worked fixtures for the **browser-fallback posture** row on the embedded
surface boundary card. These cases exist so reviewers can trace:

- when **Open in browser** is available as a typed handoff,
- when external open is **policy-blocked** or **offline-unavailable**, and
- how high-risk actions always return to **product-owned/native approval**
  surfaces rather than completing inside embedded or external chrome.

All fixtures in this directory are `embedded_boundary_card_record`
documents validated by:

- `/schemas/ux/embedded_boundary_card.schema.json`

Primary contract references:

- `/docs/ux/native_approval_surface_inventory.md`
- `/docs/ux/embedded_surface_boundary_cards.md`
- `/docs/web/scoped_browser_surface_matrix.md`
- `/docs/adr/0015-embedded-surface-boundary-and-auth-handoff.md`

## Cases

- `docs_help_stale_snapshot_system_browser_first.yaml`
  — `system_browser_first` posture while a docs snapshot is stale.
- `auth_confirmation_device_code_fallback_offered.yaml`
  — `device_code_fallback_offered` posture for auth confirmation.
- `marketplace_account_policy_blocked_external_open_blocked.yaml`
  — `external_open_blocked_by_policy` posture when policy forbids
  browser launch.
- `extension_hosted_offline_snapshot_external_open_unavailable.yaml`
  — `external_open_unavailable_offline` posture when offline.
- `service_dashboard_live_verified_browser_fallback_not_applicable.yaml`
  — `browser_fallback_not_applicable` posture for a local-only embedded
  dashboard.

