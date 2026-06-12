# Command governance contract fixture

This fixture set exercises the frozen command-governance packet owned by
`aureline_commands::freeze_command_governance_contract`.

`command_governance_contract_packet.json` covers:

- the closed descriptor-field, invocation-session-field, result-packet-field,
  and lifecycle-dependency vocabularies;
- the downgrade rule set that Help/About, release, docs, and support consumers
  must honor;
- ten feature-family rows spanning notebook, data/API, profiler, docs/browser,
  pipeline, framework-pack, companion, sync, incident, and infrastructure
  command surfaces; and
- per-surface rows for desktop, CLI, AI, recipe, extension, and
  browser-companion routes, including auto-narrowed rows when proof freshness,
  lifecycle dependencies, or parity truth drop below the Stable lane.

Verify the checked packet with:

```sh
cargo test -p aureline-commands freeze_command_governance_contract --no-fail-fast
```

Regenerate the checked artifact and fixture after intentional changes with:

```sh
cargo test -p aureline-commands freeze_command_governance_contract::tests::emit_artifact -- --ignored
```
