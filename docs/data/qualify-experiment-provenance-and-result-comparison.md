# Experiment Provenance and Result Comparison

Aureline stores experiment result truth as local-first records: run summaries,
dataset provenance, artifact lineage, environment fingerprints, comparison
labels, and export reviews. Hosted trackers can augment these records, but they
are not the hidden source of truth.

## Required Fields

Run summary cards must show the run ID, notebook/script/task/test source,
origin class, outcome, code revision, data source, environment fingerprint, and
reproducibility label before compare, open, or export actions are trusted.

Dataset provenance cards default to metadata-only disclosure. Raw samples,
full tables, and raw artifact payloads require explicit drill-down or export
review.

Artifact lineage rows show whether a figure, report, model, or exported dataset
is current, stale, imported, manually attached, or unknown.

Comparison rows carry one of four labels:

| Label | Meaning |
| --- | --- |
| Comparable | Code, data, environment, hardware, and metric schema are aligned enough for a meaningful delta. |
| Environment-skewed | Code and data are aligned, but environment or hardware changed materially. |
| Data-skewed | Environment is similar, but dataset snapshot or partition changed materially. |
| Lineage missing | Required code, metric, run, data, environment, or hardware provenance is missing or unsuitable for a fair delta. |

## Safe Export

Supported export scopes are notebook file, rendered report, metadata-only
summary, and raw artifact payload. Metadata-only summary preserves lineage and
environment fingerprint even when raw preview is blocked. Raw payloads are never
selected by default.
