# Crash-store review artifact

| Field | Value |
| --- | --- |
| Packet id | `support.m5.crash_store_viewer.v1` |
| Boundary schema | `schemas/support/crash_store_viewer.schema.json` |
| Reviewer doc | `docs/support/m5/crash_store.md` |
| Primary contract refs | `schemas/support/harden_crash_capture_exact_build_symbolication_crash_loop.schema.json`, `schemas/support/support_bundle_manifest.schema.json`, `schemas/support/export_redaction_profile.schema.json` |

## Review summary

- Crash-store rows stay local-first and metadata-safe by construction.
- Users see preview and local export actions before any reviewed upload action.
- Raw dump attachment remains explicit opt-in and is disabled when retention has
  already expired.
