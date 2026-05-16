Reviewer fixtures for the request-workspace alpha contract.

Each case fixture pins one seeded scenario (the same scenarios the
headless CLI / inspector binary `aureline_shell_request_workspace`
emits) and the expected truth the runtime record, the canonical
send-inspector report, the chrome panel projection, and the headless
CLI all surface. The integration test in
`/crates/aureline-runtime/tests/request_workspace_alpha.rs` replays
every fixture through the canonical
`RequestWorkspaceAlphaRecord::send_inspector_report` projection and
asserts UI / CLI parity over target class, method, credential class,
boundary cue, readiness band, review posture, expected side-effect
tokens, and banner kinds.

Fixtures:

- `local_read_only_get.json` - local-host GET against a payments
  lookup, broker-handle credential, fresh schema, dispatch allowed.
- `remote_mutating_post_stale_schema.json` - remote (SSH) POST against
  payments refund, broker-handle credential, stale-under-week schema;
  review required.
- `managed_delete_missing_schema.json` - managed-workspace DELETE
  against payments refund, broker-handle credential, missing schema;
  dispatch blocked.
- `remote_graphql_no_auth.json` - remote (SSH) GraphQL operation
  against a public endpoint with no credentials, fresh schema; review
  required.
