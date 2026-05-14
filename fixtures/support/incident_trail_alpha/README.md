# Alpha crash incident-trail fixtures

These fixtures exercise the alpha crash incident trail consumed by
`aureline-crash` and the support-bundle preview surface.

| File | Purpose |
|---|---|
| `crash_envelope.json` | Synthetic alpha-channel crash envelope with exact-build identity, trace IDs, fault domain, and module identities. |
| `crash_dump_manifest.json` | Metadata-only dump manifest; raw dump bytes stay local-only. |
| `symbolication_report_exact.json` | Exact-build symbolication report for every module. |
| `symbolication_report_partial.json` | Partial report that leaves the renderer source-map lane unresolved. |

The missing-symbolication state is exercised by omitting the report in
the protected Rust test rather than adding an empty pseudo-report.
