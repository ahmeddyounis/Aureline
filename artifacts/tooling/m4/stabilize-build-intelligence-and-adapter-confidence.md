# Stable Build Intelligence And Adapter Confidence Artifact

This artifact summarizes the checked-in adapter-confidence contract used by execution surfaces.

## Evidence

- `schemas/tooling/adapter-confidence.schema.json` defines the tooling-facing support-export boundary.
- `fixtures/tooling/m4/stabilize-build-intelligence-and-adapter-confidence/stable_adapter_confidence_contract.json` pins the expected all-lane fixture counts.
- `docs/m4/stabilize-build-intelligence-and-adapter-confidence.md` documents consumer behavior.
- `aureline_runtime::current_stable_adapter_confidence_support_export` produces the canonical runtime packet used by tests and support export projection.

## Covered In The Fixture

- Five discovery lanes: native adapter, structured protocol, build-event stream, structured output import, and heuristic fallback.
- Four degraded reasons in the all-lane scenario: version skew, control-plane outage, stale artifact, and parse ambiguity.
- Imported-versus-live states across target rows: live workspace inspection, imported artifact, replayed receipt, heuristic inference, and mixed live/imported evidence.
- Non-live receipt postures: inspect only and refresh required.
- Refresh-diff buckets: added, removed, renamed, downgraded confidence, newly heuristic, newly exact, and now unresolved.

## Verification

```sh
cargo test -p aureline-runtime --test stable_adapter_confidence_support_export
```

The verification fails if the schema/doc/fixture artifact paths are missing, if a lane or provenance token drifts, if a reduced-certainty receipt is allowed to look live, or if refresh diffs collapse into a generic change summary.
