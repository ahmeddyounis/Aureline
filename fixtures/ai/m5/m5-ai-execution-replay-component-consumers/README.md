# M5 AI execution/replay-component consumer fixtures

Protected fixtures for row **M05-882** — the shared consumer-adoption lane over the
frozen M5 AI execution/replay component matrix. Each fixture is a full
`M5AiExecutionReplayConsumerPacket` minted by the seed builders in
`crates/aureline-ai/src/add_shared_patch_review_evidence_inspector_branch_worktree_queue_support_export_and_docs_help_ai_execution_replay_component_consumers/seed.rs`
and validated in `tests.rs`.

| Fixture | Narrowed consumer | Qualification |
| --- | --- | --- |
| `branch_queue_beta_narrowed.json` | `branch_worktree_queue` | `beta` |
| `docs_help_preview_narrowed.json` | `docs_help` | `preview` |

Both narrowed variants keep **every** consumer visible — a narrowed qualification
never drops a consumer from the matrix. Regenerate with:

```sh
cargo run -q -p aureline-ai --bin aureline_ai_execution_replay_component_consumers -- fixture-branch-queue-beta-narrowed > branch_queue_beta_narrowed.json
cargo run -q -p aureline-ai --bin aureline_ai_execution_replay_component_consumers -- fixture-docs-help-preview-narrowed > docs_help_preview_narrowed.json
```

Validate against
[`schemas/ai/m5-ai-execution-replay-component-consumer.schema.json`](../../../../schemas/ai/m5-ai-execution-replay-component-consumer.schema.json).
