# finalize_request_workspace_and_api_request_execution_context fixture corpus

Fixture corpus for the M4 stable request-workspace and API-request execution-context reuse truth packet (`schemas/runtime/finalize_request_workspace_and_api_request_execution_context_truth.schema.json`).

Each fixture is a `RequestExecutionContextTruthPacketInput` with an `expect` block that pins the materialized packet's promotion state, finding count, lane and row-class token sets, support-class, wedge, auth-source, connection-state, streaming-response-state, consumer-surface, known-limit, downgrade-automation, and evidence-class tokens, and the support-export safety verdict. Tests in `crates/aureline-runtime/tests/finalize_request_workspace_and_api_request_execution_context_truth_packet.rs` load each case and assert that materialization matches the expectation block.

Regenerate via:

```bash
python3 tools/regenerate_finalize_request_workspace_and_api_request_execution_context_truth_packet.py
```
