# Provider-object model alpha fixtures

This directory contains the protected fixture for the provider-object model
alpha. The fixture is consumed by the `aureline-provider` crate and covers:

- code-host pull-request and branch rows;
- issue/work-item rows projected from a cached provider overlay;
- CI pipeline run, check run, log, artifact, and annotation rows;
- the five user-facing modes (`local_draft_mode`, `publish_now_mode`,
  `open_in_provider_mode`, `publish_later_mode`, `inspect_only_mode`);
- continuity observations for stale-within-window, expired-beyond-window,
  offline, and revoked-or-disconnected provider state.

Verify it with:

```bash
cargo test -p aureline-provider --test provider_object_alpha
cargo run -p aureline-provider --bin aureline_provider_alpha -- \
    --provider-object-alpha --validate-only
```

The fixture is the canonical proof that offline or stale provider state
does not collapse the workflow: every degraded row carries an explicit
`degraded_action`, every continuity observation names the retained
capability, and `local_editing_preserved` stays true on every row.
