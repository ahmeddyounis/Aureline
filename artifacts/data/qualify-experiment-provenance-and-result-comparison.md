# Experiment Provenance and Result Comparison Qualification

Machine-readable packet:
`artifacts/data/qualify-experiment-provenance-and-result-comparison.json`

Schema:
`schemas/data/experiment-provenance.schema.json`

## Object Coverage

| Object | Current proof |
| --- | --- |
| Experiment run | Six run summary cards expose run ID, source, origin class, outcome, code revision, data refs, environment fingerprint, reproducibility label, and compare/open/export actions. |
| Dataset summary | Three dataset cards default to metadata-only disclosure and keep source class, snapshot, scope, sensitivity, location class, and schema/query summary visible. |
| Artifact lineage entry | Figure, model, report, manual attach, and unknown-lineage rows preserve producing run/import truth and never collapse to a generic success badge. |
| Result comparison row | Five comparisons carry machine-readable guard labels and visible text for comparable, environment-skewed, data-skewed, and lineage-missing states. |
| Environment fingerprint | Three fingerprint cards remain human-readable with interpreter/kernel, package/toolchain summary, target origin, hardware class, policy epoch, and freshness. |

## Comparison Drills

| Drill | Expected label | Why |
| --- | --- | --- |
| Same code, data, environment, hardware, and metric schema | Comparable | The delta can be presented as a meaningful rerun comparison. |
| Code revision changed | Lineage missing | The delta is retained as exploratory evidence and must not imply same-revision comparability. |
| Dataset snapshot changed | Data-skewed | The data partition changed and the context note stays beside the metric. |
| Environment and hardware changed | Environment-skewed | Runtime and hardware class changed even though code/data stayed aligned. |
| Imported tracker summary lacks retained lineage | Lineage missing | The comparison remains evidence-only because code, data, environment, and hardware provenance are incomplete. |

## Export Review

Export/share review separates notebook file, rendered report, metadata-only
summary, and raw artifact payload scopes. Metadata-only summary is selected by
default for support-safe handoff. Raw artifact payload is local-only, not
default-selected, and requires explicit review.

## Known Limits

This packet qualifies local-first provenance and comparison truth for promoted
notebook/data rows. It does not ship a hosted experiment tracker, and imported
tracker records stay evidence-only unless they carry recoverable run, dataset,
artifact, and environment lineage.
