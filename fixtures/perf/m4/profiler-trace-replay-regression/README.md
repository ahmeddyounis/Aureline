# Profiler, Trace, Replay, and Regression Qualification Fixtures

`qualification_manifest.json` is the canonical fixture for the performance
tooling qualification packet. It covers:

- a Stable live flamegraph row with exact mapping and profile-only replay
  degradation;
- a Stable regression summary row with baseline age, comparison key, threshold
  state, confounder badges, and open-trace/open-review actions;
- an evidence-view-only imported reverse/replay row with visible disabled chrome
  and restart/import guidance.

The Rust validator mutates this fixture in unit tests to prove that missing
session strips, imported-as-live evidence, hidden replay disabled reasons,
missing regression confounders, and unsafe raw export defaults block Stable
qualification.
