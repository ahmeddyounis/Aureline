# M5 Inline-Result-Marker Primitive

- Packet: `m5-inline-result-marker-primitive:stable:0001`
- Label: `M5 inline-result-marker primitive: pass/fail/error/timeout verdict, stability-or-flaky chip, imported/live origin class, last-result freshness, source-mapping fidelity, target/environment shorthand, attempt lineage, mute/quarantine and release impact, derived marker posture, and bounded reveal-evidence/open-recent-attempts/rerun/review-quarantine/export actions`
- Editor / notebook consumers: 5 (5 stable)
- Marker postures: quarantined_marker, unmapped_marker, approximate_mapping_marker, imported_evidence_marker, stale_result_marker, live_local_marker
- Source mappings: exact_mapping, approximate_mapping, unmapped_to_buffer, no_local_buffer
- Stability chips: stable_chip, flaky_suspected_chip, known_flaky_chip, quarantined_chip, newly_added_chip, unknown_stability_chip
- Proof freshness SLO: 720 hours (last refresh: 2026-07-07T00:00:00Z)

## Editor / notebook consumers

- **Editor Gutter Marker**: `stable`
  - Owner: Editor gutter marker owner
  - Scope: The editor-gutter marker renders the shared inline result marker so a fresh live-local pass that maps exactly to the buffer reads as the only live-local marker that may imply a current local result and exposes rerun and open-recent-attempts, and a stale live-local assertion failure degrades to a stale-result marker that still reruns but never reads as live
  - Worked markers: 2
    - `marker:auth-unit::token-refresh` (`passed` / `live_local`) → `live_local_marker` (mapping `exact_mapping`, live-certainty `true`, muted `false`)
    - `marker:pricing::round-half-even` (`failed` / `live_local`) → `stale_result_marker` (mapping `exact_mapping`, live-certainty `false`, muted `false`)
- **Editor Inline Marker**: `stable`
  - Owner: Editor inline marker owner
  - Scope: The editor inline marker renders the shared inline result marker so a live-local pass whose source drifted maps only approximately and degrades to an approximate-mapping marker rather than implying a current local result, and a live-local runtime error whose source no longer maps to the buffer reads as an unmapped marker that withholds the rerun and open-recent-attempts it cannot honestly offer
  - Worked markers: 2
    - `marker:integration::matrix-parse-11` (`passed` / `live_local`) → `approximate_mapping_marker` (mapping `approximate_mapping`, live-certainty `false`, muted `false`)
    - `marker:integration::pool-teardown` (`errored` / `live_local`) → `unmapped_marker` (mapping `unmapped_to_buffer`, live-certainty `false`, muted `false`)
- **Notebook Cell Marker**: `stable`
  - Owner: Notebook cell marker owner
  - Scope: The notebook-cell marker renders the shared inline result marker so an imported-CI timeout reads as an imported-evidence marker that withholds the local rerun it cannot honestly offer yet still opens its recent attempts, and a fresh live-local pass mapped exactly to the cell reads as a live-local marker — so imported evidence never inherits live certainty
  - Worked markers: 2
    - `marker:notebook::checkout-flow@ci` (`failed` / `imported_ci`) → `imported_evidence_marker` (mapping `exact_mapping`, live-certainty `false`, muted `false`)
    - `marker:notebook::data-load-smoke` (`passed` / `live_local`) → `live_local_marker` (mapping `exact_mapping`, live-certainty `true`, muted `false`)
- **Headless / CLI Marker**: `stable`
  - Owner: Headless CLI marker owner
  - Scope: The headless / CLI marker renders the shared inline result marker so a fresh live-local pass with no local buffer to decorate still reads as a live-local marker that reruns, and a replayed-snapshot flaky-suspected result reads as an imported-evidence marker that opens its recent attempts yet stays reduced certainty — proving the same marker grammar works headless
  - Worked markers: 2
    - `marker:contract::schema-back-compat` (`passed` / `live_local`) → `live_local_marker` (mapping `no_local_buffer`, live-certainty `true`, muted `false`)
    - `marker:e2e::nightly-regression@replay` (`flaky_suspected` / `replayed_snapshot`) → `imported_evidence_marker` (mapping `no_local_buffer`, live-certainty `false`, muted `false`)
- **Marker Report Export**: `stable`
  - Owner: Marker report export owner
  - Scope: The marker-report export renders the shared inline result marker so a team-owned quarantined live-local failure reads as a quarantined marker whose hidden-from-release impact heads it while still exposing rerun, open-recent-attempts, and review-quarantine, and a fresh live-local benchmark pass reads as a live-local marker — the same marker a reviewer reads elsewhere
  - Worked markers: 2
    - `marker:auth::login-redirect-quarantined` (`failed` / `live_local`) → `quarantined_marker` (mapping `exact_mapping`, live-certainty `false`, muted `true`)
    - `marker:bench::render-budget` (`passed` / `live_local`) → `live_local_marker` (mapping `exact_mapping`, live-certainty `true`, muted `false`)
