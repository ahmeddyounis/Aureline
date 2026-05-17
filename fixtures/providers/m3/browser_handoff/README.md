# Provider browser-handoff alpha fixture

This protected fixture is the launch wedge for the provider browser-handoff
alpha record family. It is consumed by:

- the Rust validator at
  [`/crates/aureline-provider/src/browser_handoff/mod.rs`](../../../../crates/aureline-provider/src/browser_handoff/mod.rs);
- the integration test at
  [`/crates/aureline-provider/tests/browser_handoff_alpha.rs`](../../../../crates/aureline-provider/tests/browser_handoff_alpha.rs);
- the alpha bin entry point at
  [`/crates/aureline-provider/src/bin/aureline_provider_alpha.rs`](../../../../crates/aureline-provider/src/bin/aureline_provider_alpha.rs)
  via `--browser-handoff-alpha`;
- the reviewer landing page at
  [`/docs/runtime/m3/provider_handoff_alpha.md`](../../../../docs/runtime/m3/provider_handoff_alpha.md);
- the JSON schema at
  [`/schemas/providers/provider_browser_handoff_alpha.schema.json`](../../../../schemas/providers/provider_browser_handoff_alpha.schema.json).

## What it covers

- a **review-lane** code-host PR handoff that is in flight
  (`launched_awaiting_return`) with an intended `return_to_local_draft_authoring`
  follow-up;
- a **runtime-lane** issue-tracker publish-later handoff that has already
  returned to the authoritative local object via a typed reconnect;
- a **provider-lane** CI annotation handoff that returned to a
  `revoked_and_local_body_wiped` truthful placeholder when the upstream grant
  was revoked;
- three import sessions whose states are `observed_fresh`,
  `stale_within_window`, and `revoked_or_disconnected`, with freshness truth
  matching each state;
- two reconnect flows covering the
  `restored_authoritative_local_object` and
  `restored_truthful_placeholder` outcomes;
- three continuity observations using the same
  `ContinuityObservationClass`, `RetainedCapabilityClass`, and
  `DegradedActionClass` vocabulary as the provider-object support export, so
  support packets read one truth across the M3 provider lanes.

## Running the validator

```bash
cargo test -p aureline-provider --test browser_handoff_alpha
cargo run -p aureline-provider --bin aureline_provider_alpha -- \
    --browser-handoff-alpha --validate-only
```
