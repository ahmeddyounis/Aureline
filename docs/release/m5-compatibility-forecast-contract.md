# M5 compatibility-forecast and migration-assistant contract

This contract freezes the typed compatibility forecast the update center, migration
assistant, release center, admin console, and support export inspect **before restart or
rollout widening** to see how a staged M5 update will drift the subjects Aureline
qualifies, and the migration-assistant task rows that clear that drift. It is the
lifecycle layer alongside the [change-impact
cards](m5-change-impact-card-contract.md): the cards answer *what this update will do to my
workspace once I restart*; this sheet answers *which qualified subjects will drift out of
compatibility on the stable / beta / preview / LTS lines, and what concrete migration steps
clear that drift before a stable-facing surface breaks*.

It does **not** build a universal package solver, a new extension dependency model, or a
new remote negotiation protocol. It consumes the M5 compatibility, skew, and schema-window
data Aureline already owns and makes it actionable, labelling what it cannot know — and
what lies outside its claimed window — honestly.

- Packet schema: [`schemas/release/m5-compatibility-forecast.schema.json`](../../schemas/release/m5-compatibility-forecast.schema.json)
- Migration-task-row schema: [`schemas/release/m5-migration-task-row.schema.json`](../../schemas/release/m5-migration-task-row.schema.json)
- Published inventory: [`artifacts/release/m5-compatibility-forecast.json`](../../artifacts/release/m5-compatibility-forecast.json)
- Release-grade parity proof: `artifacts/release/m5-migration-assistant-proof/compatibility-forecast.json` (+ `.md`)
- Machine-readable per-task export: [`artifacts/release/m5-migration-tasks.csv`](../../artifacts/release/m5-migration-tasks.csv)
- Per-state fixtures: `fixtures/release/compatibility-forecast/`
- Producer crate / module: `crates/aureline-release` → `m5_compatibility_forecast`
- Headless emitter: `aureline_release_m5_compatibility_forecast`

## What the forecast covers

Each qualified subject gets one forecast rather than being collapsed into one generic "an
update is available" line. The six subject families are separated explicitly:

| Subject | Primary artifact class | Owner role |
|---|---|---|
| `certified_archetype` | `workspace_state` | `certification_owner` |
| `extension_sdk_range` | `extension_packs` | `extension_sdk_owner` |
| `extension_manifest_range` | `extension_packs` | `extension_manifest_owner` |
| `remote_agent_skew` | `core_runtime` | `remote_helper_owner` |
| `public_export_reader` | `schema_contracts` | `export_contract_owner` |
| `public_schema_reader` | `schema_contracts` | `schema_contract_owner` |

Each subject carries one **line forecast** per compatibility line — `stable`, `beta`,
`preview`, `lts` — so its drift is never collapsed into a channel-agnostic verdict.

## Drift class, confidence, and the gate

Each line forecast carries a **drift class** (least→most severe) and a **forecast
confidence** (best→worst):

| Drift class | Meaning | Implied gate (if certain) |
|---|---|---|
| `no_drift` | No drift forecast | governed |
| `compatible_within_window` | Compatible, within the supported window | governed |
| `deprecation_scheduled` | Compatible now; a deprecation is scheduled | narrowed |
| `migration_required` | A migration / range bump is required | narrowed |
| `breaking_drift` | Breaks without a migration | blocked |

| Confidence | Meaning | Gate cap |
|---|---|---|
| `qualified` | Within the claimed window, inputs available | blocked |
| `likely` | Mostly available, well-supported | blocked |
| `estimated` | Partial inputs | narrowed |
| `unknown` | Inputs unavailable | narrowed |
| `outside_claimed_window` | Outside Aureline's claimed window | narrowed |
| `not_applicable` | Line does not apply | governed |

A line's effective gate is the drift gate **capped** by the confidence. This is the lane's
guardrail: a `breaking_drift` forecast on `estimated`, `unknown`, or
`outside_claimed_window` inputs caps at **narrowed** — it is flagged for review, never
raised as a hard failure. `CompatibilityForecastSheet::validate` rejects any packet that
tampers a capped line up to `blocked`, and the schema encodes the same `if`/`then`.

A subject's **worst-line gate** decides its readiness: `clear_to_widen`,
`review_before_widening`, or `hold_before_widening`.

## Migration-assistant task rows

Every subject whose worst-line gate is not governed (narrowed or held) **must** carry at
least one migration-assistant task row — a packet with a narrowed/held subject and no task
fails validation (`missing_migration_task`). Each row discloses:

- the **owner** role and the **subject** it clears;
- the **affected scope** — artifact classes, deployment profiles, and lines;
- **auto-fix availability** (`auto_fix_available` / `assisted_fix` / `manual_only` /
  `admin_required`);
- the **due-before boundary** (`before_apply` … `before_rollout_widening` …
  `before_end_of_support`);
- the **skip / waive policy** — `not_skippable`, `skippable_with_rationale`,
  `optional_recommended`, or `auto_resolved`;
- **rollback guidance** (`rollback_supported` / `pin_current_version` /
  `side_by_side_fallback` / `reinstall_only` / `no_rollback`); and
- the pre-emptive **actions** Aureline already offers: `pin`, `postpone`, `side_by_side`,
  `validator`, `repair`.

A task is suppressible only with a recorded rationale where its policy requires one: a
waiver of a `skippable_with_rationale` task with no rationale fails validation
(`waiver_rationale_missing`), and waiving a `not_skippable` task fails (`illegal_waiver`).

## Consumers read one sheet

The five consumer surfaces bind the subject families they read and **derive** their review
readiness and gaps from the forecasts, so all of them read this one sheet rather than
cloning drift fields locally:

| Consumer | Reads |
|---|---|
| `update_center` | all subjects |
| `migration_assistant` | all subjects |
| `release_center` | all subjects |
| `admin_console` | archetype, extension SDK / manifest, remote skew |
| `support_export` | all subjects |

A narrowed subject narrows every consumer that reads it (`review_before_widening`); a
confirmed breaking-drift subject holds them (`resolve_before_widening`); an out-of-window
subject narrows with `outside_claimed_window` rather than holding. Each consumer verdict is
recomputed from the subjects during validation, so it can never drift from a
hand-maintained status.

## Honesty and safety guarantees

- **No overstated coverage.** Subjects outside the claimed window are labelled
  (`outside_claimed_window`, with `within_claimed_window: false` and an
  `out_of_window_message_id`) and never raised as a hard failure — the guardrail for
  unqualified archetypes and third-party extensions.
- **Partial coverage disclosed.** The `coverage` block counts qualified / estimated /
  unknown / out-of-window / not-applicable line forecasts and sets `has_partial_coverage`.
- **Local-safe export.** The packet carries metadata, refs, and message ids only — no
  credential bodies or raw provider payloads — so it is exportable and reviewable outside
  the app without forcing an immediate restart.
- **Visible before widening.** The forecast and its tasks are computed and surfaced before
  the restart or rollout-widening that would commit the drift.
