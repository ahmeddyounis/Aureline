# Workset Scope Alpha Fixture

This fixture proves the saved-scope contract for a named sparse workset:

- `aureline.workset.jsonc` is the reviewable saved workset artifact.
- `workspace_manifest.json` points at that artifact by stable workset id.
- workspace, remote, headless, support-export, navigation, and refactor consumers must preserve the same stable scope id.
- degraded consumers must keep the identity and include an explicit reason.

