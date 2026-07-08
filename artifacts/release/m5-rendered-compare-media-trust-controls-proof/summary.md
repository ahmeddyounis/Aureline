# Rendered Compare Viewers, Media Rails & Trust Badge Sets

- Packet: `media-compare-controls:stable:0001`
- Surface: `Rendered compare viewers, media rails, and trust badge sets`
- Rendered compare viewers: 3 (2 not directly trusted)
- Media-metadata rails: 3 (2 carry hidden-content notes)
- Redaction / trust badge sets: 3 (2 redacted)
- Proof freshness SLO: 168 hours (last refresh: 2026-07-08T00:00:00Z)

## Rendered compare viewers

- **design snapshot** [`artifact:design/home-snapshot.png`]: sandboxed_trusted — scale `1440x900 @2x, 100% scale`
- **rendered coverage report** [`artifact:report/coverage.html`]: sandboxed_untrusted — scale `980px wide, fit-to-width`
- **session video capture** [`artifact:media/session-capture.mp4`]: raw_text_fallback — scale `1920x1080, 30fps`

## Media-metadata rails

- **PNG (sRGB)** [`artifact:design/home-snapshot.png`]: dimensions 1440x900 px — no_embedded_sensitive_content (export_safe), share team_share
- **HTML report** [`artifact:report/coverage.html`]: byte_size 412 KB — embedded_content_scan_unknown (sandboxed), share local_only
- **MP4 (H.264)** [`artifact:media/session-capture.mp4`]: duration 00:00:12 — embedded_sensitive_content_present (raw_unsanitized), share local_only

## Redaction / trust badge sets

- [`artifact:design/home-snapshot.png`]: not_redacted / trusted — export posture preserved: true
- [`artifact:report/coverage.html`]: partially_redacted / sandboxed_only — export posture preserved: true
- [`artifact:media/session-capture.mp4`]: fully_redacted / untrusted — export posture preserved: true
