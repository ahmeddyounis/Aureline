# M5 Execution-Confidence Primitive: Adapter-Drift Banner, Launcher State, Launcher-State Parity, and Overwrite Guard

- Packet: `m5-execution-confidence-primitive:stable:0001`
- Label: `M5 Execution-Confidence Primitive: Adapter-Drift Banner, Launcher State, Launcher-State Parity, and Overwrite Guard`
- Execution surfaces: 5 / 5
- Adapter sources: native_build_server, native_build_event, heuristic_parse, imported_snapshot, provider_overlay, unknown
- Build verbs: build, test, run, debug, coverage, package
- Overwrite verdicts: promoted_higher_confidence, matched_existing_confidence, recorded_explicit_downgrade

## Execution surfaces

- **Adapter-drift banner**
  - Owner: Build-adapter confidence guild
  - Scope: Adapter-drift banners naming prior versus current adapter, capability delta, affected targets, and recompute / diagnostics actions
  - Worked cases: 2
    - `target:web-api:run:0002` → native_build_server → heuristic_parse (planned), confidence `low`, recorded_explicit_downgrade
    - `target:web-api:test:0003` → native_build_event → imported_snapshot (planned), confidence `medium`, recorded_explicit_downgrade
- **Execution launcher**
  - Owner: Execution-launcher guild
  - Scope: Run / test / debug launchers narrowing affordances before launch when adapter capability drops
  - Worked cases: 2
    - `target:web-api:run:0002` → native_build_server → heuristic_parse (planned), confidence `low`, recorded_explicit_downgrade
    - `target:web-api:build:0001` → native_build_server → native_build_server (live), confidence `high`, matched_existing_confidence
- **Launcher-state parity**
  - Owner: Execution-parity guild
  - Scope: Launcher-state parity carrying adapter source and confidence into problem surfaces, artifact views, and follow-on automation / AI
  - Worked cases: 2
    - `target:web-api:container:0004` → provider_overlay → provider_overlay (provider_overlay), confidence `medium`, matched_existing_confidence
    - `target:legacy-service:run:0005` → heuristic_parse → native_build_server (live), confidence `high`, promoted_higher_confidence
- **Overwrite guard**
  - Owner: Confidence-integrity guild
  - Scope: No-higher-confidence overwrite guard refusing to replace existing native / higher truth without an explicit downgrade
  - Worked cases: 2
    - `target:web-api:test:0003` → native_build_event → imported_snapshot (planned), confidence `medium`, recorded_explicit_downgrade
    - `target:reporting:replay:0006` → imported_snapshot → imported_snapshot (planned), confidence `medium`, recorded_explicit_downgrade
- **Support / export replay**
  - Owner: Support / diagnostics guild
  - Scope: Offline replay reconstructing execution confidence from an imported log for support and AI
  - Worked cases: 1
    - `target:reporting:replay:0006` → imported_snapshot → imported_snapshot (planned), confidence `medium`, recorded_explicit_downgrade
