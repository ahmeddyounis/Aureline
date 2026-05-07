# Warm-start chooser fixtures (freshness + revalidation truth)

Seed corpus for the contract published in:

- `artifacts/entry/warm_start_chooser_contract.md`

Machine-readable boundary schema:

- `schemas/entry/freshness_revalidation.schema.json`

Each fixture is a single YAML document exercising one of the record kinds:

- `warm_start_chooser_set_record`
- `warm_start_chooser_decision_record`
- `warm_start_chooser_outcome_record`

Every fixture:

- Renders all four warm-start lanes (resume live, start from snapshot, clone fresh, open without starter) as distinct rows; lanes may be visible-but-disabled.
- Makes snapshot/commit/session identity, freshness age, host/platform, pending updates, revalidation items, write safety, and safe fallback posture explicit before commit.
- Avoids raw absolute paths, raw hostnames, raw URLs with embedded credentials, raw secrets, raw policy bundle bodies, and raw provider payloads.

## Cases

| Fixture | Scenario focus |
| --- | --- |
| `stale_snapshot_rows.yaml` | Snapshot is stale; resume-live cannot claim liveness; chooser remains explicit about age and safe fallbacks. |
| `expired_credential_resume_requires_reauth.yaml` | Resume-live requires reauth/credential refresh; failure posture keeps restricted/read-only and snapshot/clone fallbacks available. |
| `expired_credential_decision.yaml` | Decision record choosing the resume-live lane in a reauth-required scenario. |
| `expired_credential_outcome_waiting_for_prerequisite.yaml` | Outcome record linked to a bootstrap packet while waiting for reauth/prerequisites. |
| `invalidated_prebuild_snapshot_disabled.yaml` | Snapshot lane is invalidated/missing artifacts; clone-fresh and open-without-starter remain safe fallbacks. |
| `incompatible_platform_snapshot_disabled.yaml` | Snapshot lane is incompatible with current host/toolchain; chooser shows platform mismatch and safe alternatives. |
| `offline_mirror_fallback_clone_disabled.yaml` | Offline or mirror-only startup; clone-fresh is blocked; snapshot and open-without-starter remain actionable with explicit freshness truth. |
