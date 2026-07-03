# M5 Profiler / Topology Component Fixtures

These fixtures are the checked-in examples for
`artifacts/design/m5-profiler-topology-component-matrix.md`.

Each fixture is metadata-only and validates against its matching schema under
`schemas/ui/`. The examples preserve controlled labels for capture mode,
execution origin, profile kind, duration, mapping quality, flamegraph/icicle
filters and zoom state, call-tree symbolization, baseline comparability, workset
scope, freshness, confidence, provenance, role types, and generated-versus-
curated explainer truth.

Raw profile samples, raw trace events, raw heap objects, raw command lines, raw
local paths, credentials, provider payloads, and private user identifiers are
excluded by contract.
