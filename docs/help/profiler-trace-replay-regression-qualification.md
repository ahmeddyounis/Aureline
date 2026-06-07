# Performance Evidence Qualification

Performance views show Stable wording only when their session, trace, replay,
comparison, and export packets are current.

Every qualified view starts with a session strip naming the workload,
build/runtime, capture mode, mapping quality, and whether the evidence is live,
imported, cached, or stale. Imported or cached evidence is not described as live
capture.

Replay and reverse-step controls stay visible with an exact disabled reason
when the lane exists but the backend is `record_only`, `profile_only`,
`import_view_only`, or `unsupported`. The guidance points to restart with
recording or import a supported capture.

Regression summaries keep baseline source and age, comparison key, observed
delta, threshold or waiver state, confounders, and open-trace/open-review
actions visible. Unlike workload, hardware, build mode, warm/cold, local/remote,
or power-state comparisons are advisory rather than plain pass/fail truth.

Exports default to manifests and summaries. Raw traces, memory payloads,
arguments, and environment fragments require explicit review before they leave
the local machine.
