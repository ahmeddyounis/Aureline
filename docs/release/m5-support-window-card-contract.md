# M5 support-window cards contract

This contract freezes the typed support-lifecycle cards Help/About, docs/help, the update
center, the compatibility report, support export, the admin console, and the release center
inspect to decide whether to **upgrade, pin, postpone, file a bug, or roll a channel out
broadly** — without digging through release prose or website copy. It is the
support-lifecycle layer alongside the [update-center summary
objects](m5-update-center-summary-contract.md) and the [change-impact
cards](m5-change-impact-card-contract.md): the summary answers *what is changing*, the
change-impact cards answer *what the change will do before restart*, and these cards answer
*what support, compatibility, deprecation, and end-of-support promises each channel and
boundary subject actually carries*.

It does **not** broaden Aureline's support commitments. It surfaces and governs the windows
already claimed: stable, beta, preview, nightly, and LTS no longer share one vague readiness
label once those promises exist.

- Packet schema: [`schemas/release/m5-support-window-card.schema.json`](../../schemas/release/m5-support-window-card.schema.json)
- Published inventory: [`artifacts/release/m5-support-window-cards.json`](../../artifacts/release/m5-support-window-cards.json)
- Release-grade parity proof: `artifacts/release/m5-channel-lifecycle-proof/support-window-cards.json` (+ `.md`)
- Machine-readable per-card export: [`artifacts/release/m5-support-window-cards.csv`](../../artifacts/release/m5-support-window-cards.csv)
- Per-state fixtures: `fixtures/release/support-window-and-eos/`
- Producer crate / module: `crates/aureline-release` → `m5_support_window_card`
- Headless emitter: `aureline_release_m5_support_window_card`

## What the cards cover

The packet carries two card families, both gate-bound to the shared
[descriptor/badge](../../crates/aureline-release/src/m5_descriptor_badge) vocabulary so every
surface reads one set of states.

### Channel cards — one per channel

| Channel | Identity |
|---------|----------|
| `stable` | General-availability line with the longest support commitment. |
| `beta` | Publicly announced pre-release line ahead of Stable. |
| `preview` | Gated pre-release line for early evaluation. |
| `nightly` | Automated daily line; best-effort, not a support commitment. |
| `lts` | Long-term-support line with an extended maintenance window. |

Every channel card carries:

- **channel identity** — its label and one-line description;
- a **support window** (`full_support_until`, `end_of_support_on`) and its
  **support-window state** (`full_support`, `maintenance_support`, `security_support`,
  `grace_window`, `out_of_support`);
- an **overlap window** — whether the prior version is supported in parallel, with its
  predecessor version and close date, so a user can postpone or run side-by-side;
- a **deprecation horizon** and **removal target** — the successor channel, the deprecation
  and removal dates, and the version a removal targets;
- a **pin-or-postpone path** (`stay_on_channel`, `pin_current_version`, `postpone_upgrade`,
  `move_to_successor_channel`, `side_by_side_during_overlap`, `upgrade_required`,
  `not_applicable`); and
- known **compatibility caveats**, each scoped to the artifact class it affects.

### Compatibility-subject cards — one per subject

| Subject | Primary artifact class | Owner role |
|---------|------------------------|------------|
| `workspace_profile_files` | `workspace_state` | `workspace_state_owner` |
| `extension_sdk` | `extension_packs` | `extension_sdk_owner` |
| `extension_manifest` | `extension_packs` | `extension_manifest_owner` |
| `remote_helper` | `core_runtime` | `remote_helper_owner` |
| `public_schema` | `schema_contracts` | `schema_owner` |

Every subject card carries its **end-of-support state** (`supported`, `sunset_announced`,
`deprecated`, `retired`, `removed`) and a **compatibility window** — the supported
`floor_version` → `ceiling_version`, the `current_version`, and a window **posture**
(`within_window`, `nearing_ceiling`, `outside_window`) — plus a successor reference, a
pin-or-postpone path, and compatibility caveats.

## How a card's verdict is derived

A card's gate is the **worse of its two postures**, so a card can never advertise a wider
commitment than its weakest promise:

- a channel card's gate is `worst(support_window_state, end_of_support_state)`;
- a subject card's gate is `worst(end_of_support_state, compatibility_window.posture)`.

Each state maps to a gate: `full_support` / `supported` / `within_window` → `governed`;
`maintenance_support` / `security_support` / `grace_window` / `sunset_announced` /
`deprecated` / `nearing_ceiling` → `narrowed`; `out_of_support` / `retired` / `removed` /
`outside_window` → `blocked`. The gate maps one-to-one to a **readiness**: `governed` →
`supported`, `narrowed` → `plan_migration`, `blocked` → `action_required`. Only a `blocked`
card requires a migration action.

This is the lane's **guardrail** against broadening support:
`SupportWindowCardSet::validate` rejects any card whose stored gate is *less severe* than the
weakest promise warrants (`over_broadened_commitment`).

## Deprecated and end-of-support states carry recovery guidance

A card under any lifecycle pressure (`narrowed` or `blocked`) must carry replacement,
overlap, and recovery guidance instead of a bare warning, and `carries_recovery_guidance`
records that it does:

- a **channel** card needs a named replacement (`successor_channel` or
  `replacement_message_id`), a disclosed overlap window, and an active pin-or-postpone path
  with backing refs;
- a **subject** card needs a `successor_message_id` and an active pin-or-postpone path.

`validate` raises `missing_recovery_guidance` for any pressured card that lacks it, and the
schema enforces the same with a `then: carries_recovery_guidance == true` guard for any
`narrowed` / `blocked` card.

## How consumers read the cards

Each consumer binds the channels and subjects it reads and **derives** its readiness,
profiles, and gaps from the cards — there is no hand-maintained per-consumer status. All
seven consumers (`help_about`, `docs_help`, `update_center`, `compatibility_report`,
`support_export`, `admin_console`, `release_center`) read every channel and every subject, so
**Help, the update center, and the compatibility report present the same support-window
data**. A card under pressure narrows every consumer that reads it; an out-of-support card
forces a migration action. The packet-level `release_gate` aggregates the per-consumer
decisions and is the one place release / shiproom tooling reads `requires_migration_action`.

## Lifecycle-pressure and stale-data honesty

The packet-level `coverage` block discloses `fully_supported_cards` versus `narrowing_cards`
and `blocking_cards`, with `has_lifecycle_pressure` set when any card is under pressure. The
packet `data_state` labels whether the cards are `live_verified`, `mirrored_labelled`,
`offline_cached`, `stale_banner_shown`, or `local_only_no_live_data`, so the support truth
stays honest under stale, mirrored, or no-live-data conditions.

## Export safety

The packet carries metadata, refs, and message ids only — no credential bodies or raw
provider payloads — so the support-lifecycle truth is exportable and reviewable outside the
app. The JSON, the Markdown report, and the per-card CSV all render byte-identically across
the desktop, CLI / headless, and offline-export channels.

## Drills

Three drills perturb one card of the canonical (all-supported) set and let the derivation
recompute every consumer:

- `fixtures/release/support-window-and-eos/cards_channel_deprecation.json` — the `preview`
  channel is deprecated and in its grace window (`narrowed`), carrying replacement, overlap,
  and recovery guidance, so consumers plan a migration;
- `fixtures/release/support-window-and-eos/cards_channel_end_of_support.json` — the `preview`
  channel is out of support / removed (`blocked`), carrying an upgrade path, so consumers
  require a migration action;
- `fixtures/release/support-window-and-eos/cards_subject_compatibility.json` — the
  `extension_manifest` subject is deprecated and nearing its ceiling (`narrowed`), so the
  compatibility report and the other consumers plan a migration.

## Regenerating

```sh
cargo run -q -p aureline-release --bin aureline_release_m5_support_window_card -- registry  > artifacts/release/m5-support-window-cards.json
cargo run -q -p aureline-release --bin aureline_release_m5_support_window_card -- proof     > artifacts/release/m5-channel-lifecycle-proof/support-window-cards.json
cargo run -q -p aureline-release --bin aureline_release_m5_support_window_card -- markdown  > artifacts/release/m5-channel-lifecycle-proof/support-window-cards.md
cargo run -q -p aureline-release --bin aureline_release_m5_support_window_card -- csv       > artifacts/release/m5-support-window-cards.csv
cargo run -q -p aureline-release --bin aureline_release_m5_support_window_card -- variant canonical       > fixtures/release/support-window-and-eos/cards_all_supported.json
cargo run -q -p aureline-release --bin aureline_release_m5_support_window_card -- variant deprecation     > fixtures/release/support-window-and-eos/cards_channel_deprecation.json
cargo run -q -p aureline-release --bin aureline_release_m5_support_window_card -- variant end-of-support  > fixtures/release/support-window-and-eos/cards_channel_end_of_support.json
cargo run -q -p aureline-release --bin aureline_release_m5_support_window_card -- variant subject-compat  > fixtures/release/support-window-and-eos/cards_subject_compatibility.json
```
