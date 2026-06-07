# Stable Build Intelligence And Adapter Confidence

This contract makes build-intelligence provenance a shared runtime truth rather than a UI guess. Run, test, debug, Problems, AI evidence, CLI/headless inspect, and support export surfaces consume `BuildIntelligenceSupportExport` records from `aureline-runtime` and keep the same lane, freshness, exactness, and imported-versus-live tokens.

## Canonical Vocabulary

Discovery lanes are closed and user-visible:

- `native_adapter` / Native adapter
- `structured_protocol` / Structured protocol
- `build_event_stream` / Build-event stream
- `structured_output_import` / Structured output import
- `heuristic_fallback` / Heuristic fallback

Reduced-certainty lanes never become exact live truth by rendering convention. Imported, replayed, unresolved, or heuristic rows carry `imported_live_state_token`, `exactness_status_token`, `source_confidence_rank`, and a `high_trust_action_posture_token` on run-config cards and receipts.

## Stable Boundaries

- Tooling schema: `schemas/tooling/adapter-confidence.schema.json`
- Runtime schema: `schemas/runtime/adapter_health_strip.schema.json`
- Fixture corpus: `fixtures/tooling/m4/stabilize-build-intelligence-and-adapter-confidence`
- Runtime producer: `aureline_runtime::current_stable_adapter_confidence_support_export`

## Consumer Rules

- Target rows show the lane label, adapter/protocol identity, exactness, archetype/framework binding when known, degraded reason, and open-source/open-config actions when available.
- Adapter health strips show the adapter or protocol identity, health state, precise degraded reason, last successful refresh, repair/open-details actions, and continue-local or inspect-only actions when live dispatch is narrowed.
- Run-config cards and receipts preserve whether target identity came from live workspace inspection, imported artifacts, replayed receipts, mixed live/imported evidence, or heuristic inference.
- Rerun, replay, publish, and other high-trust actions require review, refresh, repair, or inspect-only posture whenever the target is not current live exact truth.
- Discovery refresh review uses separate added, removed, renamed, downgraded-confidence, newly-heuristic, newly-exact, and now-unresolved buckets. Consumers must not replace those buckets with a generic target-change notice.

## Verification

Run:

```sh
cargo test -p aureline-runtime --test stable_adapter_confidence_support_export
```

The test verifies the requested tooling schema and docs paths exist, loads the fixture corpus, mints the stable support export, and checks lane coverage, non-live receipt posture, discovery-diff bucket counts, and redaction safety.
