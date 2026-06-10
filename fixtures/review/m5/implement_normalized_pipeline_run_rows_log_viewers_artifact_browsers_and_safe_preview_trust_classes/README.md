# Normalized Pipeline Run Rows, Log Viewers, Artifact Browsers, and Safe-Preview Trust-Class Fixtures

These fixtures are valid, export-safe packets that exercise the labeling and
narrowing behavior the canonical support export keeps green. Each one keeps the
trust-review and consumer-projection invariants satisfied and proof freshness
valid — the difference is which states are narrowed and why.

## log_retention_expired_offline.json

The run finished while the surface is now offline, so its log `stream_state` is
`unavailable` with an explicit truncation label and a `denied_no_open_path`
open path, and its archive artifact is `stale` with retention expired and a
`download_only_no_in_product_open` path. Demonstrates that a log whose retention
expired is labeled rather than presented as complete, and that stale or
retention-expired bytes narrow the safe-open path out of any in-product render.

## run_unknown_status_provider_owned.json

The deployment provider returned a status the contract does not recognise, so
the run is `unknown` with `unverified` freshness, carries explicit attention
reasons, and its partial log and unverified artifact both resolve to
`open_in_safe_preview_metadata_only`. Demonstrates that a provider-owned unknown
status is never flattened into `failed` or `succeeded`, stays explicit, and
keeps every open path off live bytes while freshness is unverified.
