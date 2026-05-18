# Prompt Composer Drills

This fixture set exercises the beta prompt-composer conformance packet owned by
`aureline_ai::prompt_composer`.

The drill covers:

- explicit mode, scope, and execution-boundary rows;
- `@file`, `@symbol`, `@root`, and `@run` mention resolution;
- stable attachment identity with freshness, trust, preview, open, and remove actions;
- slash-command parity with the canonical command graph and disabled reasons;
- overflow budget rows that keep omitted, summarized, trimmed, and route-switched context visible;
- local-first draft persistence under managed policy;
- stale attachment, unresolved mention, over-budget, policy-blocked route, and offline local-only handling without losing the draft;
- evidence, route, spend, redaction, replay, operator, support, and compliance lineage;
- preview-only branch/worktree rows that do not become autonomous apply paths.

Verify the checked packet with:

```sh
cargo test -p aureline-ai prompt_composer --no-fail-fast
```
