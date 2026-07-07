# M5 prompt-composer-component consumer fixtures

Protected fixtures for row **M05-889** — the shared consumer-adoption lane over the
frozen M5 prompt-composer component matrix. Each fixture is a full
`M5ComposerComponentConsumerPacket` minted by the seed builders in
`crates/aureline-ai/src/add_shared_inline_panel_patch_review_branch_agent_docs_help_and_companion_prompt_composer_component_consumers/seed.rs`
and validated in `tests.rs`.

| Fixture | Narrowed consumer | Qualification |
| --- | --- | --- |
| `branch_agent_beta_narrowed.json` | `branch_agent` | `beta` |
| `companion_preview_narrowed.json` | `companion` | `preview` |

Both narrowed variants keep **every** consumer visible — a narrowed qualification
never drops a consumer from the matrix. Regenerate with:

```sh
cargo run -q -p aureline-ai --bin aureline_ai_prompt_composer_component_consumers -- fixture-branch-agent-beta-narrowed > branch_agent_beta_narrowed.json
cargo run -q -p aureline-ai --bin aureline_ai_prompt_composer_component_consumers -- fixture-companion-preview-narrowed > companion_preview_narrowed.json
```

Validate against
[`schemas/ai/m5-prompt-composer-component-consumer.schema.json`](../../../../schemas/ai/m5-prompt-composer-component-consumer.schema.json).
