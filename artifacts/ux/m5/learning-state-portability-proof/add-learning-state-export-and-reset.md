# Learning-state export and reset — release evidence

Reviewer-facing evidence packet for the M5 learning-state portability lane. A
*learning-state export bundle* is the support/export-safe packet that ports tour,
exercise, or learning-session state out of the device — preserving the provenance
trail back to its source state, keeping the state user-owned local-first,
redacting raw payloads, and keeping a source-language escape and cached-pack
continuity visible. A *learning-state reset plan* clears a bounded slice of
learnability state with an explicit target scope, an explicit protected set (docs
packs, bookmarks, and user-authored notes are never silently deleted), and a
reversible restore. No export or reset path opens a hidden mutating tutorial
shortcut. A record that cannot prove that posture is explicitly narrowed below
Stable with a named reason rather than inheriting an adjacent green row.

Canonical machine sources (do not clone status text from this packet — ingest the JSON):

- Schema: [`/schemas/help/learning-session-export.schema.json`](../../../../schemas/help/learning-session-export.schema.json)
- Fixture: [`/fixtures/help/m5/learning-state-export-and-reset/m5_learning_state_export_and_reset.json`](../../../../fixtures/help/m5/learning-state-export-and-reset/m5_learning_state_export_and_reset.json)
- Public doc: [`/docs/m5/learning-state-portability.md`](../../../../docs/m5/learning-state-portability.md)
- Aligns with: [`/schemas/help/m5-learning-progress-snapshots.schema.json`](../../../../schemas/help/m5-learning-progress-snapshots.schema.json) (the progress state these operations carry/clear), [`/schemas/help/m5-tour-and-glossary-packages.schema.json`](../../../../schemas/help/m5-tour-and-glossary-packages.schema.json) (shared source/freshness vocabulary), and [`/schemas/learning/guided-learning-contracts.schema.json`](../../../../schemas/learning/guided-learning-contracts.schema.json) (shared verdict vocabulary)
- Typed source: `aureline_learning::learning_state_export_and_reset`
- Headless emitter: `aureline_learning_m5_learning_state_export_and_reset`
- Test: `cargo test -p aureline-learning learning_state_export_and_reset`

## The export-bundle matrix

| Bundle | Family | State | Target | Verdict | Freshness | Localization | Narrowing reason |
|---|---|---|---|---|---|---|---|
| `notebook_tour_portable` | notebook | tour_state | portable_profile | **qualified_stable** | live_authoritative | en-US (source) | — |
| `request_exercise_support_localized_cached` | request_workspace | exercise_state | support_bundle | **narrowed_beta** | cached_disclosed | en-US → fr-FR + escape | cached pack content may lag (disclosed) |
| `docs_session_portable_mirrored` | docs_browser | learning_session_state | portable_profile | **qualified_stable** | mirror_synced_disclosed | en-US (source) | — |

## The reset-plan matrix

| Plan | Clears | Protects | Restore | Verdict |
|---|---|---|---|---|
| `all_local_learnability` | tour, exercise, session, glossary, profile, hints (6) | docs packs, bookmarks, notes, model packs, checkpoints, templates (6) | command-backed, window disclosed | **qualified_stable** |
| `learning_session_only` | session (1) | docs packs, bookmarks, notes, model packs, checkpoints, templates (6) | command-backed, window disclosed | **qualified_stable** |

**Overall manifest verdict: narrowed_beta** — the localized, cached export's
disclosed cached pack narrows it, and the narrowest member propagates to the
overall verdict; the live and mirror-synced exports and both reset plans ship
Stable individually.

## What this packet proves

1. **Export carries state out without losing provenance or privacy.** Every
   bundle sets `provenance_preserved: true`, carries a non-empty
   `source_state_refs` trail back to the snapshot/profile it ported, and keeps
   `data_ownership: user_owned_local_first`. The `redaction` posture redacts raw
   payloads, credential bodies, and absolute paths, and `widens_data_sharing` is
   false — an export never broadens who can read the state. Exports are
   `user_initiated`, never silent.

2. **The source-language escape stays one step away.** A localized bundle
   (`presented_localized: true`, `en-US` → `fr-FR`) MUST set
   `escape_to_source_available: true` and carry a command-backed
   `escape_command_ref`; the schema enforces it via `if/then` and the validator
   reports a missing escape as a hard violation. Localization sets
   `preserves_provenance: true` — it changes display copy only, never identity.

3. **Cached-pack continuity stays visible.** Each bundle's `cached_pack` discloses
   its `source_class` and `freshness`. A non-live pack (`cached_disclosed`,
   `mirror_synced_disclosed`, `local_only_disclosed`, `stale_disclosed`) MUST set
   `continuity_disclosed: true`; the schema's `if/then` rules and the validator
   both reject an undisclosed non-live pack as a continuity masquerade. A disclosed
   cached/local-only/stale pack narrows to Beta with a named reason; a disclosed
   mirror-synced pack stays Stable.

4. **Reset never erases unrelated user-owned state.** Every reset plan declares an
   explicit `target_state_kinds` scope, sets `silently_deletes_outside_scope:
   false`, and lists `protected_classes` that MUST include `docs_pack`,
   `bookmark`, and `user_authored_note` (the schema requires all three via
   `contains`, and the seed also protects model packs, checkpoints, and template
   packs). A plan that drops a required protected class is a hard violation.

5. **Reset is reversible.** Every plan sets `restore_available: true`,
   `restore_window_disclosed: true`, and a command-backed `restore_command_ref` —
   reset is never a one-way door. The schema enforces the restore-window and
   command-backing via `if/then`.

6. **No hidden mutating tutorial path.** Every bundle and plan carries a
   `mutation_fence`: `introduces_tutorial_only_mutating_shortcut` and
   `bypasses_preview_approval` are false, `authority_boundary_change_allowed` is
   false, `command_graph_unchanged` is true, and any path that
   `touches_real_workspace_state` MUST set
   `uses_standard_preview_approval_when_touching_workspace: true`. Explain stays
   separate from do, and any do uses the same preview/approval model as ordinary
   work.

## How the verdict is derived

`derive_export_bundle_verdict` folds each bundle's redaction, sharing, ownership,
provenance, intent, mutation-fence, source-language-escape, and cached-pack
continuity evidence into the strictest verdict. Hard safety violations narrow to
`narrowed_preview`; a disclosed cached/local-only/stale pack narrows to
`narrowed_beta`. `derive_reset_plan_verdict` folds each plan's target scope,
protected set, scope-containment, reversibility, ownership, intent, and
mutation-fence evidence. The manifest's `overall_verdict` is the narrowest across
all bundles and plans. Stored verdicts are re-derived and checked by
`validate_m5_learning_state_export_and_reset`, so a hand-edited fixture that
disagrees with its own evidence fails validation.

## How to reproduce

```sh
cargo test -p aureline-learning learning_state_export_and_reset
cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_state_export_and_reset -- validate
cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_state_export_and_reset -- summary
```
