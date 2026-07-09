# Rotation/revoke-event rows and export-safety banners

- Packet: `m5-rotation-revoke-export-safety-controls:stable:0001`
- Surface: `M5 rotation/revoke-event rows and export-safety banners: credential class, prior/new lifecycle state, derived continuity, impacted running sessions/queued jobs/remembered decisions, recovery next step, audit/export actions, export surface, export-safety class, reveal posture, derived redaction posture, preserved handle-class/source labels, and the raw-secret-excluded default`
- Rotation/revoke-event rows: 6 (3 are no longer usable)
- Export-safety banners: 6 (all 6 exclude raw secrets by default)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## Rotation/revoke-event rows

- **GitHub OAuth token (acme-org)** (oauth_token) — `rotation_due` → `active_current` → continuity `still_active`
- **npm registry API key (mirror)** (api_key) — `active_current` → `refresh_needed` → continuity `action_required`
- **GitLab PAT (internal)** (personal_access_token) — `active_current` → `rotation_due` → continuity `action_required`
- **Release signing key (platform-team)** (ssh_or_signing_key) — `active_current` → `revoked` → continuity `no_longer_usable`
- **Client certificate (contoso)** (client_certificate) — `refresh_needed` → `expired` → continuity `no_longer_usable`
- **Device-code grant (self)** (device_code_grant) — `active_current` → `superseded` → continuity `superseded`

## Export-safety banners

- **profile** — class `raw_secret_excluded` → posture `raw_excluded_labels_preserved` (raw secrets excluded)
- **support_bundle** — class `metadata_only` → posture `raw_excluded_labels_preserved` (raw secrets excluded)
- **handoff_packet** — class `handle_reference_only` → posture `handle_reference_only` (raw secrets excluded)
- **recipe** — class `redacted_share` → posture `redacted_or_masked` (raw secrets excluded)
- **portable_workspace** — class `endpoints_masked` → posture `redacted_or_masked` (raw secrets excluded)
- **audit_log** — class `export_blocked` → posture `fully_blocked` (raw secrets excluded)
