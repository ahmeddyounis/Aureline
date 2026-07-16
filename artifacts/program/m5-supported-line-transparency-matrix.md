# M5 Supported-Line Public-Proof, Transparency-Report, Migration-Scoreboard, ORR-History, and Correction-Train-Archive Matrix

- Packet: `m5-supported-line-transparency:stable:0001`
- Label: `M5 supported-line public-proof, transparency-report, migration-scoreboard, ORR-history, and correction-train-archive matrix`
- Lines: 5 (5 stable)
- Stable-line-protection roles: freshness_window, transparency_disclosure, migration_scoreboard_currency, orr_history_retention, correction_archive_retention, public_proof_freshness, correction_history_join
- Widening stages: alpha, beta, release_candidate, stable, long_term_support
- Proof freshness SLO: 720 hours (last refresh: 2026-07-14T00:00:00Z)

## Lines

- **public_proof_ledger**: `stable`
  - Owner: Public-proof ledger owner
  - Canonical schema: `schemas/program/m5-public-proof-freshness-ledger.schema.json`
  - Scope: One public-proof ledger naming the current public-claim proof, the published compatibility report, the current support-window proof, and the freshness window met so external claims, partner reviews, and procurement checks inherit current rather than tribal truth
  - Required labels: identity, transparency_role, registry_reference, freshness_window
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **transparency_report**: `stable`
  - Owner: Transparency-report owner
  - Canonical schema: `schemas/program/m5-public-proof-freshness-ledger.schema.json`
  - Scope: One transparency report naming the upstream health reported, the compatibility health reported, the maintainer durability reported, and the export-safe public view kept so no internal-only incident or security detail ever leaks into a public-safe or partner/procurement feed
  - Required labels: identity, transparency_role, registry_reference, export_class
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **migration_scoreboard**: `stable`
  - Owner: Migration-scoreboard owner
  - Canonical schema: `schemas/program/m5-migration-scoreboard.schema.json`
  - Scope: One migration scoreboard naming the migration path scored, the migration blockers tracked, the migration-pain deltas recorded, and the scoreboard versioned so migration pain is never forgotten between release trains
  - Required labels: identity, transparency_role, registry_reference, export_class
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **orr_history_event**: `stable`
  - Owner: ORR-history archive owner
  - Canonical schema: `schemas/program/m5-supported-line-orr-history.schema.json`
  - Scope: One ORR-history event naming the ORR decision event recorded, the go/no-go outcome preserved, the support-window decision retained, and the history event archived so supported-line decisions are never lost to memory
  - Required labels: identity, transparency_role, registry_reference, line_association
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **correction_train_archive**: `stable`
  - Owner: Correction-train archive owner
  - Canonical schema: `schemas/program/m5-correction-train-archive.schema.json`
  - Scope: One correction-train archive naming the correction-train packet archived, the hotfix/backport packet archived, the advisory packet archived, and the archive packet bound to exact build identity so correction history stays durable and inspectable
  - Required labels: identity, transparency_role, registry_reference, line_association
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
