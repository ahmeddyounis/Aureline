# Portable Bundle Fixtures

These JSON fixtures drive the portable change bundle and shelf contract at
[`/schemas/change/portable_bundle.schema.json`](../../../../schemas/change/portable_bundle.schema.json).
They are consumed by `aureline-change-objects`, the shell portable-bundle
inspector, and the support-export handoff projection.

| Fixture | Object | Purpose | Validation state | Open modes |
| --- | --- | --- | --- | --- |
| `offline_review_handoff.json` | `portable_bundle` | Offline review handoff | Current | Offline inspect, compare-only reopen |
| `browser_companion_handoff.json` | `portable_bundle` | Browser companion handoff | Provider overlay unavailable | Offline inspect, compare-only reopen, browser read-only |
| `incident_follow_up_stale_validation.json` | `portable_bundle` | Incident follow-up | Environment capsule stale | Offline inspect, compare-only reopen, support inspect |
| `support_export_shelf_desktop_resume.json` | `shelf_entry` | Support export | Review-pack version stale | Offline inspect, compare-only reopen, desktop resume after revalidation |

The fixtures intentionally use opaque refs and closed vocabulary tokens only.
They never include raw paths, raw remote URLs, raw credentials, live bearer
authority, secret material, or raw diff bodies.
