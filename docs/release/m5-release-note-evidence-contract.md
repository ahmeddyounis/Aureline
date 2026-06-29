# M5 release-note evidence contract

This contract freezes the typed release-note evidence rows the update center, what's-new panel, Help
center, docs/Help, release center, and support export consume to turn release notes into
**evidence-backed change communication** rather than one generic announcement stream. It is the
communication layer alongside the [update-center summary
objects](m5-update-center-summary-contract.md) (*what is changing, and did it verify*) and the
[change-impact cards](m5-change-impact-card-contract.md) (*what will the change do before restart*):
these rows answer *does this note actually reduce risk* — and they do only when they separate marketing
prose from a controlled change class, the affected scope, and a direct link to the relevant setting,
import, or rollback surface.

It does **not** create a second release-note system outside the existing release / Help publication
pipeline. It is the canonical source the existing surfaces ingest.

- Packet schema: [`schemas/release/m5-release-note-evidence-row.schema.json`](../../schemas/release/m5-release-note-evidence-row.schema.json)
- Controlled vocabulary: [`artifacts/release/m5-release-note-vocabulary.md`](../../artifacts/release/m5-release-note-vocabulary.md)
- Published inventory: [`artifacts/release/m5-release-note-evidence.json`](../../artifacts/release/m5-release-note-evidence.json)
- Release-grade parity proof: `artifacts/release/m5-release-note-proof/release-note-evidence.json` (+ `.md`)
- Machine-readable per-note export: [`artifacts/release/m5-release-note-evidence.csv`](../../artifacts/release/m5-release-note-evidence.csv)
- Per-state fixtures: `fixtures/release/whats-new-and-migration/`
- Producer crate / module: `crates/aureline-release` → `m5_release_note_evidence`
- Headless emitter: `aureline_release_m5_release_note_evidence`

## What a row carries

Each release note is a typed evidence row rather than free prose. The packet stores the change class,
controlled labels, refs, and routable message ids only — no headline or body text — so marketing is
separated from evidence by construction. Every row carries:

- a **change class** drawn from one frozen vocabulary so a routine docs touch-up can never read like a
  behavior change:

  | Change class | Readiness | Owner role |
  |---|---|---|
  | `docs_only` | `informational` | `docs_owner` |
  | `compatibility` | `informational` | `compatibility_owner` |
  | `behavioral` | `action_recommended` | `product_behavior_owner` |
  | `policy` | `action_recommended` | `policy_owner` |
  | `deprecated` | `action_recommended` | `deprecation_owner` |
  | `migration_required` | `action_required` | `migration_owner` |
  | `admin_action_required` | `action_required` | `admin_owner` |
  | `security` | `action_required` | `security_response_owner` |
  | `breaking` | `action_required` | `public_interface_owner` |

- one or more **evidence links** — an `evidence_packet`, `security_advisory`, `migration_doc`,
  `certification_delta`, `rollback_control`, `setting_surface`, `import_surface`, or `docs_page` — each
  flagged as a `direct_action` (an in-app surface the user acts on) and/or `substantive_evidence` (a
  packet / advisory / migration doc / certification delta / rollback control, as opposed to a bare docs
  pointer);
- the **affected scope** — the artifact classes, deployment profiles, and channels the change touches,
  with `support_sensitive` derived from the channels (`stable` / `lts`); and
- a **what's-new card** that is always dismissible and reopenable from the update center and Help, and
  that never blocks typing, save, restore, or recovery-critical workflows.

## The lane's guardrails

A row's gate is its change class's **communication-severity gate** — `governed` for
`docs_only` / `compatibility`, `narrowed` for `behavioral` / `policy` / `deprecated`, and `blocked` for
`migration_required` / `admin_action_required` / `security` / `breaking`. The gate maps one-to-one to a
readiness (`informational`, `action_recommended`, `action_required`); it classifies how much action a
note asks for and **never blocks a workflow**.

`ReleaseNoteEvidenceSet::validate` enforces the lane's invariants, so a tampered packet that drops a
required link or flips a blocking flag is rejected:

- **Evidence, not prose** — a behavior-changing or security-sensitive note (any class except
  `docs_only`) must carry at least one substantive evidence link (`missing_evidence_link` otherwise).
- **Direct action link** — a `breaking`, `migration_required`, or `admin_action_required` note must link
  directly to a setting / import / rollback surface (`missing_direct_action_link` otherwise).
- **Security advisory** — a `security` note must link to an advisory
  (`security_note_missing_advisory` otherwise).
- **Never blocks** — a what's-new card that sets any of `blocks_typing` / `blocks_save` /
  `blocks_restore` / `blocks_recovery` is rejected (`whats_new_card_blocks_workflow`).
- **Reopenable** — a what's-new card that is not dismissible / reopenable from both the update center and
  Help is rejected (`whats_new_card_not_reopenable`).

The schema mirrors the first two guardrails as conditional `allOf` rules, so the checked-in JSON fails
validation in CI as well as in code.

## How consumers read the rows

Each consumer binds the note ids it reads and **derives** its readiness, disclosed change classes,
disclosed scope, and gaps from the rows — there is no hand-maintained per-consumer status. The
six claimed consumers — `update_center`, `whats_new_panel`, `help_center`, `docs_help`,
`release_center`, `support_export` — read this one packet, so the app, docs/Help, and exported summaries
speak one vocabulary and one schema. A note that recommends action narrows every consumer that reads it;
an action-required note raises them to `action_required`. The packet-level `action_gate` aggregates the
per-consumer decisions and is the one place release / shiproom tooling reads whether any published note
asks the user to act.

## Evidence-completeness honesty

The packet-level `coverage` block discloses how many notes carry substantive evidence
(`notes_with_substantive_evidence`) and direct-action links (`notes_with_direct_action_link`) against
how many require them, with `all_required_links_present` true only when every note that needs evidence
or a direct link has it. `all_cards_reopenable` and `all_cards_non_blocking` restate the what's-new
guarantees for the whole set.

## Export safety

The packet carries metadata, refs, and routable message ids only — no credential bodies, raw provider
payloads, or free-form prose — so the same set is exportable and reviewable outside the app. The JSON,
the Markdown report, and the per-note CSV all render byte-identically across the desktop, CLI / headless,
docs/Help, and offline-export channels.

## Drills

Four fixtures exercise the acceptance criteria:

- `fixtures/release/whats-new-and-migration/notes_representative.json` — a representative release with
  one evidence-backed, action-linked note per change class, every what's-new card active and reopenable;
- `notes_dismissed_reopenable.json` — every what's-new card is dismissed but stays reopenable from the
  update center and Help (the reopenability criterion);
- `notes_docs_only.json` — a routine docs / compatibility release that leaves every consumer
  informational; and
- `notes_security_and_migration.json` — a focused security / migration release whose security note links
  to an advisory and whose migration and breaking notes link directly to setting / import / rollback
  surfaces (the evidence-backed and direct-link criteria).

## Regenerating

```sh
cargo run -q -p aureline-release --bin aureline_release_m5_release_note_evidence -- registry  > artifacts/release/m5-release-note-evidence.json
cargo run -q -p aureline-release --bin aureline_release_m5_release_note_evidence -- proof     > artifacts/release/m5-release-note-proof/release-note-evidence.json
cargo run -q -p aureline-release --bin aureline_release_m5_release_note_evidence -- markdown  > artifacts/release/m5-release-note-proof/release-note-evidence.md
cargo run -q -p aureline-release --bin aureline_release_m5_release_note_evidence -- csv       > artifacts/release/m5-release-note-evidence.csv
cargo run -q -p aureline-release --bin aureline_release_m5_release_note_evidence -- variant canonical          > fixtures/release/whats-new-and-migration/notes_representative.json
cargo run -q -p aureline-release --bin aureline_release_m5_release_note_evidence -- variant dismissed          > fixtures/release/whats-new-and-migration/notes_dismissed_reopenable.json
cargo run -q -p aureline-release --bin aureline_release_m5_release_note_evidence -- variant docs_only          > fixtures/release/whats-new-and-migration/notes_docs_only.json
cargo run -q -p aureline-release --bin aureline_release_m5_release_note_evidence -- variant security_migration > fixtures/release/whats-new-and-migration/notes_security_and_migration.json
```
