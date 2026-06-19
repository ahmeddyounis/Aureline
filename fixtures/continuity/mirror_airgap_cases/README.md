# Mirror-only and air-gapped continuity cases

This directory stores canonical JSON fixtures emitted by
`cargo run -q -p aureline-continuity --example dump_m5_mirror_airgap_continuity_fixtures`.

Every file validates against
`schemas/continuity/mirror_airgap_packet.schema.json`
(`python3 tools/validate_m5_mirror_airgap_continuity_fixtures.py`).

## Files

- `page.json` — seeded stable page. It carries one mirror-only self-hosted row,
  one air-gapped sovereign row, one self-hosted-restricted row, and one exempt
  local-only row. Every offline row declares trust-root continuity that survives
  offline, a disclosed offline import and export path, an offline-safe advisory
  and revocation source, and an explicitly governed public-fallback policy
  (prohibited, unavailable, or policy-gated). Every claimed offline row points to
  a current packet.
- `summary.json` — seeded page summary record
- `registry.json` — seeded offline-continuity registry record (per-claim-row coverage)
- `support_export.json` — support-export wrapper for the seeded page
- `case_silent_public_fallback_withdrawn.json` — a mirror-only row silently falls
  back to public endpoints; it fails closed and is withdrawn
- `case_advisory_live_public_fetch_withdrawn.json` — an air-gapped row sources
  advisories from a live public fetch; it fails closed and is withdrawn
- `case_trust_root_breaks_offline_withdrawn.json` — a mirror-only trust root
  requires a live public reissue to renew; it fails closed and is withdrawn
- `case_public_fallback_undisclosed_preview.json` — a self-hosted-restricted row
  does not state its public-fallback policy and is held at preview
- `case_trust_root_undeclared_preview.json` — a mirror-only row does not declare
  its trust-root continuity and is held at preview
- `case_mirror_never_synced_preview.json` — a mirror-only row whose mirror has
  never synced and is held at preview
- `case_packet_evidence_missing_preview.json` — a claimed air-gapped row carries
  no packet and is held at preview
- `case_mirror_stale_needs_sync_beta.json` — a mirror-only mirror has aged out
  and needs a fresh sync; it narrows to beta

## Regeneration

```sh
DIR=fixtures/continuity/mirror_airgap_cases
EX="cargo run -q -p aureline-continuity --example dump_m5_mirror_airgap_continuity_fixtures --"
$EX page > $DIR/page.json
$EX summary > $DIR/summary.json
$EX registry > $DIR/registry.json
$EX support-export > $DIR/support_export.json
$EX case-silent-public-fallback-withdrawn > $DIR/case_silent_public_fallback_withdrawn.json
$EX case-advisory-live-public-fetch-withdrawn > $DIR/case_advisory_live_public_fetch_withdrawn.json
$EX case-trust-root-breaks-offline-withdrawn > $DIR/case_trust_root_breaks_offline_withdrawn.json
$EX case-public-fallback-undisclosed-preview > $DIR/case_public_fallback_undisclosed_preview.json
$EX case-trust-root-undeclared-preview > $DIR/case_trust_root_undeclared_preview.json
$EX case-mirror-never-synced-preview > $DIR/case_mirror_never_synced_preview.json
$EX case-packet-evidence-missing-preview > $DIR/case_packet_evidence_missing_preview.json
$EX case-mirror-stale-needs-sync-beta > $DIR/case_mirror_stale_needs_sync_beta.json
```

The canonical evidence packets under `artifacts/m5/continuity/mirror_airgap/`
are regenerated from the same example (`page`, `registry`, and `support-export`).
