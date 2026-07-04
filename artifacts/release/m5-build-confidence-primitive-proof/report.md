# M5 Build / Run Confidence Primitive: Adapter Badge, Target-Graph Row, Capability Matrix, Raw-Event Drawer, and Fallback Drawer

- Packet: `m5-build-confidence-primitive:stable:0001`
- Label: `M5 Build / Run Confidence Primitive: Adapter Badge, Target-Graph Row, Capability Matrix, Raw-Event Drawer, and Fallback Drawer`
- Confidence surfaces: 6 / 6
- Adapter sources: native_build_server, native_build_event, heuristic_parse, imported_snapshot, provider_overlay, unknown
- Build verbs: build, test, run, debug, coverage, package
- Fallback states: structured_high, structured_degraded, heuristic_fallback, imported_only, unknown

## Confidence surfaces

- **Adapter-source badge**
  - Owner: Build-adapter confidence guild
  - Scope: Adapter-source badges and confidence chips naming native versus fallback lanes
  - Worked cases: 2
    - `target:web-api:build:0001` → native_build_server (live), confidence `high`, structured
    - `target:legacy-service:run:0003` → heuristic_parse (planned), confidence `low`, fallback
- **Target-graph row**
  - Owner: Target-graph guild
  - Scope: Target-graph rows preserving stable target id, owning module / root, freshness, verbs, and required env
  - Worked cases: 2
    - `target:web-api:test:0002` → native_build_event (live), confidence `medium`, structured
    - `target:web-api:build:0001` → native_build_server (live), confidence `high`, structured
- **Capability-matrix sheet**
  - Owner: Capability-matrix guild
  - Scope: Capability-matrix sheets explaining supported verbs and downgraded actions before any run
  - Worked cases: 2
    - `target:web-api:test:0002` → native_build_event (live), confidence `medium`, structured
    - `target:web-api:container:0005` → provider_overlay (provider_overlay), confidence `medium`, structured
- **Raw-event drawer**
  - Owner: Event-interoperability guild
  - Scope: Raw-event drawers disclosing payload lineage, adapter version, and export / copy actions
  - Worked cases: 2
    - `target:legacy-service:run:0003` → heuristic_parse (planned), confidence `low`, fallback
    - `target:web-api:build:0001` → native_build_server (live), confidence `high`, structured
- **Fallback-confidence drawer**
  - Owner: Fallback-confidence guild
  - Scope: Fallback-confidence drawers naming why confidence fell and the recovery route
  - Worked cases: 2
    - `target:reporting:build:0004` → imported_snapshot (planned), confidence `medium`, fallback
    - `target:legacy-service:run:0003` → heuristic_parse (planned), confidence `low`, fallback
- **Support / export replay**
  - Owner: Support / diagnostics guild
  - Scope: Offline replay reconstructing build confidence from an imported log for support and AI
  - Worked cases: 1
    - `target:reporting:replay:0006` → imported_snapshot (planned), confidence `medium`, fallback
