# Warm-start choice beta fixtures

Fixtures in this directory anchor the beta warm-start choice contract documented
in
[`/docs/workspace/m3/start_center_warm_start_beta.md`](../../../../docs/workspace/m3/start_center_warm_start_beta.md).
Each JSON file validates against
[`/schemas/workspace/warm_start_choice.schema.json`](../../../../schemas/workspace/warm_start_choice.schema.json)
and is projected, validated, and rendered through
[`/crates/aureline-shell/src/start_center/warm_start_choice/mod.rs`](../../../../crates/aureline-shell/src/start_center/warm_start_choice/mod.rs).

The same object model backs the Start Center, the workspace switcher, the
CLI/headless entry review, docs/help, and the support export. Regenerate the
fixtures from the seed whenever the projection changes:

```sh
BIN=target/debug/aureline_shell_warm_start_choice
cargo build -p aureline-shell --bin aureline_shell_warm_start_choice
"$BIN" page            > fixtures/workspace/m3/template_and_resume_cards/seeded_warm_start_choice_page.json
"$BIN" support-export  > fixtures/workspace/m3/template_and_resume_cards/warm_start_choice_support_export.json
"$BIN" card "warm_start_card:template.ts_web.local"             > fixtures/workspace/m3/template_and_resume_cards/template_card.json
"$BIN" card "warm_start_card:live_resume.managed_data_workspace" > fixtures/workspace/m3/template_and_resume_cards/live_resume_card.json
"$BIN" card "warm_start_card:snapshot.ts_web.local_fresh"        > fixtures/workspace/m3/template_and_resume_cards/valid_snapshot_card.json
"$BIN" card "warm_start_card:snapshot.python_devcontainer.stale" > fixtures/workspace/m3/template_and_resume_cards/stale_snapshot_card.json
"$BIN" card "warm_start_card:clone_fresh.platform_repository"    > fixtures/workspace/m3/template_and_resume_cards/clone_fresh_card.json
```

The `seeded_page_matches_checked_in_fixture` and
`checked_in_card_fixtures_match_seeded_cards` tests pin these files bit-for-bit
to the seed, so a drift fails the corpus instead of slipping past review.

| Fixture | Record kind | Demonstrates |
|---|---|---|
| [`seeded_warm_start_choice_page.json`](./seeded_warm_start_choice_page.json) | `warm_start_choice_page_record` | The full beta page: five cards, summary counters, same-weight escape-hatch invariants, page honesty marker. |
| [`warm_start_choice_support_export.json`](./warm_start_choice_support_export.json) | `warm_start_choice_support_export_record` | The support-safe export wrapping the page; raw secret material excluded by construction. |
| [`template_card.json`](./template_card.json) | `warm_start_choice_card_record` | Local-first template card; `use_template` discloses network/extensions/setup while `open_minimal` / `set_up_later` stay same-weight and side-effect-free. |
| [`live_resume_card.json`](./live_resume_card.json) | `warm_start_choice_card_record` | Managed live resume that `requires_reauth`, attaches managed runtime, and prompts for trust — yet the default stays local and a cached read-only snapshot lane is offered. |
| [`valid_snapshot_card.json`](./valid_snapshot_card.json) | `warm_start_choice_card_record` | Fresh local snapshot: `resume_live_workspace` and `start_from_snapshot` are local read-only; only `clone_fresh` reaches the network. |
| [`stale_snapshot_card.json`](./stale_snapshot_card.json) | `warm_start_choice_card_record` | Stale snapshot (`capsule_drift`): `resume_live_workspace` is disabled, the snapshot may only be inspected read-only, and the invalidation reason is visible on the card. |
| [`clone_fresh_card.json`](./clone_fresh_card.json) | `warm_start_choice_card_record` | Remote repository clone: `clone_fresh` discloses network egress while open-minimal stays a same-weight offline path. |

Raw secrets, raw credential bodies, raw environment values, raw command lines,
machine-unique trust anchors, and uncommitted workspace edits never appear in
these fixtures.
