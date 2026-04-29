# Test session and attempt worked cases

These fixtures anchor the contract in
[`/docs/testing/test_session_and_attempt_contract.md`](../../../docs/testing/test_session_and_attempt_contract.md)
and validate against:

- [`/schemas/testing/test_session.schema.json`](../../../schemas/testing/test_session.schema.json)
- [`/schemas/testing/test_attempt.schema.json`](../../../schemas/testing/test_attempt.schema.json)

The fixture set covers:

| Fixture | Record kind | Key coverage |
|---|---|---|
| [`local_watch_session.yaml`](./local_watch_session.yaml) | `test_session_record` | local watch session with `watch_state_class = watching`, exact local target/environment refs, ordered attempt refs, raw event refs, artifacts, and visible muted/quarantined counts |
| [`provider_imported_partial_artifact.yaml`](./provider_imported_partial_artifact.yaml) | `test_session_record` | provider CI import with `watch_state_class = partial_import`, `imported_ci_projection_class = mirrored_artifact`, omitted scopes, provider event refs, provider time basis, and mirrored artifact linkage |
| [`rerun_failed_after_source_edit.yaml`](./rerun_failed_after_source_edit.yaml) | `test_attempt_record` | failed-only rerun after source changed, preserving predecessor attempt, failed subset refs, source drift, current-context rerun time basis, and surface reconstruction refs |
| [`debug_from_test_attempt.yaml`](./debug_from_test_attempt.yaml) | `test_attempt_record` | debug-from-test launch preserving originating test attempt, selector, target/environment, debug-session ref, artifact linkage, and release/support reconstruction proof |

Fixtures MUST NOT encode raw command lines, raw stdout or stderr byte
streams, raw environment bodies, raw absolute paths, raw URLs, raw
secret values, raw test names, raw assertion bodies, raw source
excerpts, raw artifact bytes, raw provider payloads, or raw stack
traces. They use opaque refs, digests, counts, class labels, bounded
summaries, and timestamps.

Removing one of the four scenario classes is a breaking contract
coverage reduction.
