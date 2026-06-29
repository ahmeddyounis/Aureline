# M5 change-impact cards contract

This contract freezes the typed change-impact cards the update center, migration
assistant, release center, team-lead review, admin console, and support export inspect
**before restart** to forecast what a staged M5 update will actually do. It is the
forecast layer alongside the [update-center summary
objects](m5-update-center-summary-contract.md): the summary answers *what is changing and
did it verify*; these cards answer *what the change will do to my workspace, profile,
schema, caches, extensions, remote helpers, toolchain, certified archetypes, and
behavior* — surfaced before the restart commits it.

It does **not** build a universal package solver or a new extension dependency model. It
forecasts impact from the M5 compatibility and schema data Aureline already owns, and
labels what it cannot know honestly.

- Packet schema: [`schemas/release/m5-change-impact-card.schema.json`](../../schemas/release/m5-change-impact-card.schema.json)
- Published inventory: [`artifacts/release/m5-change-impact-cards.json`](../../artifacts/release/m5-change-impact-cards.json)
- Release-grade parity proof: `artifacts/release/m5-change-impact-proof/change-impact-cards.json` (+ `.md`)
- Machine-readable per-card export: [`artifacts/release/m5-change-impact-cards.csv`](../../artifacts/release/m5-change-impact-cards.csv)
- Per-state fixtures: `fixtures/release/change-impact/`
- Producer crate / module: `crates/aureline-release` → `m5_change_impact_card`
- Headless emitter: `aureline_release_m5_change_impact_card`

## What the cards cover

Each forecast dimension gets its own card rather than being collapsed into one generic "an
update is available" line. The ten dimensions are separated explicitly:

| Dimension | Primary artifact class | Owner role |
|--------|------------------------|------------|
| `workspace_migration` | `workspace_state` | `workspace_state_owner` |
| `profile_migration` | `configuration` | `profile_owner` |
| `schema_migration` | `schema_contracts` | `schema_owner` |
| `cache_migration` | `core_runtime` | `cache_owner` |
| `extension_compatibility` | `extension_packs` | `extension_owner` |
| `remote_helper_skew` | `core_runtime` | `remote_helper_owner` |
| `toolchain_floor` | `language_runtimes` | `toolchain_owner` |
| `toolchain_ceiling` | `language_runtimes` | `toolchain_owner` |
| `certified_archetype` | `workspace_state` | `certification_owner` |
| `behavior_change` | `core_runtime` | `product_behavior_owner` |

Every card carries:

- a **risk class** (`no_impact`, `low_risk_cache_churn`, `compatible_with_warning`,
  `migration_required`, `habit_breaking_behavior_change`, `destructive_change`) that
  deliberately separates a routine cache rebuild from a destructive or habit-breaking
  change;
- a **forecast confidence** (`confirmed`, `likely`, `estimated`, `unknown`,
  `not_applicable`) that labels unknown inputs and partial coverage honestly;
- the **affected scope** — the artifact classes and deployment profiles the change
  touches, never narrower than the primary class;
- a **manual follow-up task** with its class, timing (`before_apply`, `before_restart`,
  `after_restart`, …), and how much Aureline can automate; and
- a **rollback or pin choice** (`rollback_supported`, `pin_current_version`,
  `side_by_side_fallback`, `reinstall_only`, `no_rollback`, `not_applicable`).

## How a card's verdict is derived

A card's gate is its **risk gate** *capped* by its **confidence**:

- the risk gate alone is `governed` for `no_impact` / `low_risk_cache_churn`, `narrowed`
  for `compatible_with_warning` / `migration_required` / `habit_breaking_behavior_change`,
  and `blocked` for `destructive_change`;
- the confidence caps the gate: `confirmed` / `likely` allow the full range, `estimated` /
  `unknown` cap at `narrowed`, and `not_applicable` caps at `governed`.

The gate maps one-to-one to a **review readiness**: `governed` → `clear_to_apply`,
`narrowed` → `review_recommended`, `blocked` → `hold_for_resolution`. Only a `blocked`
card requires a pre-restart acknowledgement.

This is the lane's **guardrail**: a high-risk forecast made on speculative inputs (an
`estimated` or `unknown` confidence) is capped at `review_recommended` and labeled with an
`unknown_input_message_id`, so speculation is never raised as a hard failure.
`ChangeImpactCardSet::validate` rejects any card that is both speculative and `blocked`.

## How consumers read the cards

Each consumer binds the dimensions it reads and **derives** its review readiness, disclosed
scope, and gaps from the cards — there is no hand-maintained per-consumer status:

| Consumer | Reads |
|----------|-------|
| `update_center` | every dimension |
| `migration_assistant` | the migration dimensions (workspace, profile, schema, cache) |
| `release_center` | every dimension |
| `team_lead_review` | every dimension |
| `admin_console` | extension, remote-helper, toolchain, archetype, behavior |
| `support_export` | every dimension |

A card that needs review narrows every consumer that reads it; a *confirmed* destructive
card holds those consumers for a pre-restart acknowledgement; a *speculative* one only
recommends review. The packet-level `release_gate` aggregates the per-consumer decisions
and is the one place release / shiproom tooling reads
`requires_pre_restart_acknowledgement`.

## Forecast coverage honesty

The packet-level `coverage` block discloses how much of the forecast is fully grounded
(`fully_forecast_cards`) versus `estimated_cards`, `unknown_input_cards`, and
`not_applicable_cards`, with `has_partial_coverage` set when any card rests on partial or
absent inputs. Partial coverage is disclosed, never implied complete.

## Export safety

The packet carries metadata, refs, and message ids only — no credential bodies or raw
provider payloads — so the impact summary is exportable and reviewable outside the app by
team leads, admins, and support without forcing an immediate restart. The JSON, the
Markdown report, and the per-card CSV all render byte-identically across the desktop,
CLI / headless, and offline-export channels.

## Drills

Three drills perturb one dimension of the canonical (all-clear) set and let the derivation
recompute every consumer:

- `fixtures/release/change-impact/cards_review_recommended.json` — a *confirmed* schema
  migration narrows the consumers that read it to `review_recommended`;
- `fixtures/release/change-impact/cards_hold_for_resolution.json` — a *confirmed*
  destructive extension change holds those consumers for a pre-restart acknowledgement;
- `fixtures/release/change-impact/cards_speculative_input.json` — a destructive behavior
  change forecast on *unknown* inputs is capped at `review_recommended` rather than
  becoming a hard failure (the guardrail).

## Regenerating

```sh
cargo run -q -p aureline-release --bin aureline_release_m5_change_impact_card -- registry  > artifacts/release/m5-change-impact-cards.json
cargo run -q -p aureline-release --bin aureline_release_m5_change_impact_card -- proof     > artifacts/release/m5-change-impact-proof/change-impact-cards.json
cargo run -q -p aureline-release --bin aureline_release_m5_change_impact_card -- markdown  > artifacts/release/m5-change-impact-proof/change-impact-cards.md
cargo run -q -p aureline-release --bin aureline_release_m5_change_impact_card -- csv       > artifacts/release/m5-change-impact-cards.csv
cargo run -q -p aureline-release --bin aureline_release_m5_change_impact_card -- variant canonical   > fixtures/release/change-impact/cards_all_clear.json
cargo run -q -p aureline-release --bin aureline_release_m5_change_impact_card -- variant review       > fixtures/release/change-impact/cards_review_recommended.json
cargo run -q -p aureline-release --bin aureline_release_m5_change_impact_card -- variant hold         > fixtures/release/change-impact/cards_hold_for_resolution.json
cargo run -q -p aureline-release --bin aureline_release_m5_change_impact_card -- variant speculative  > fixtures/release/change-impact/cards_speculative_input.json
```
