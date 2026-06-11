# M5 feature-family guided tours, glossary packs, contextual help, and command-backed learning rails — release evidence

Reviewer-facing evidence packet for the M5 learnability lane. Every marketed M5
depth family exposes in-product, command-backed glossary, guided-tour, and
contextual-help assets that remain usable offline and on mirrored profiles, point
back to authoritative commands and docs, and never bypass preview/approval. A
family that cannot prove that posture is explicitly narrowed below Stable with a
named reason rather than inheriting an adjacent green row.

Canonical machine sources (do not clone status text from this packet — ingest the JSON):

- Schema: [`/schemas/learning/m5-feature-family-learning-rails.schema.json`](../../../../schemas/learning/m5-feature-family-learning-rails.schema.json)
- Fixture: [`/fixtures/ux/m5/guided-tours/m5_feature_family_learning_manifest.json`](../../../../fixtures/ux/m5/guided-tours/m5_feature_family_learning_manifest.json)
- Public doc: [`/docs/help/m5/m5-feature-family-learning-rails.md`](../../../../docs/help/m5/m5-feature-family-learning-rails.md)
- Reuses: [`/schemas/learning/guided-learning-contracts.schema.json`](../../../../schemas/learning/guided-learning-contracts.schema.json)
- Typed source: `aureline_learning::m5_feature_family_learning_rails`
- Headless emitter: `aureline_learning_m5_feature_family_rails`
- Test: `cargo test -p aureline-learning m5_feature_family`

## The learnability matrix

| Family | Verdict | Command-backed | Mirror parity | Narrowing reason |
|---|---|---|---|---|
| `notebook` | **qualified_stable** | yes | live_authoritative | — |
| `request_workspace` | **qualified_stable** | yes | live_authoritative | — |
| `database_workspace` | **qualified_stable** | yes | live_authoritative | — |
| `profiler_trace` | **qualified_stable** | yes | live_authoritative | — |
| `docs_browser` | **qualified_stable** | yes | mirror_synced_disclosed | — |
| `preview` | **narrowed_beta** | yes | local_only_disclosed | learning pack not yet mirror-synced |
| `template_scaffold` | **qualified_stable** | yes | live_authoritative | — |
| `companion` | **narrowed_beta** | yes | cached_disclosed | tour anchors cached (not live-authoritative) |
| `sync_offboarding` | **qualified_stable** | yes | live_authoritative | — |

**Overall manifest verdict: narrowed_beta** — the `preview` mirror-parity gap and
the `companion` cached tour anchor each propagate to the overall verdict; all
other families qualify Stable individually.

## What this packet proves

1. **Reuse, not feature-local coachmarks.** Each family's bundle is built from the
   stable guided-learning objects — `glossary_pack`, `tour_package`,
   `exercise_rail`, and `progress_snapshot` records from the M4 qualification
   contract — extended with a contextual help card set and a mirror-parity
   posture. No hidden tutorial-only surface is minted.

2. **Command-backed, not tutorial-only.** Every bundle's
   `in_product_command_backed_path` is true and every contextual help card's
   `command_backed` is true: teaching steps reuse the same command ids, preview
   sheets, and approval paths as ordinary work. AI chat and browser handoff are
   never the only learning path.

3. **No bypass of preview/approval.** Every tour package, help card, and exercise
   rail keeps `explain_apply_class` separated (`read_only` or
   `apply_requires_approval`, never `conflated`), and every exercise rail's
   `apply_steps_reversible` is true. Learning rails teach the real product
   behavior, not a privileged shortcut.

4. **Offline and mirror parity, explicitly labeled.** Every bundle's
   `mirror_parity.silent_dead_link_on_stale` is false and
   `explicit_freshness_disclosed` is true. Local-only, air-gapped, and mirrored
   profiles surface glossary and guided-help state with a named freshness label
   (`live_authoritative`, `cached_disclosed`, `mirror_synced_disclosed`,
   `local_only_disclosed`) instead of silently degrading to dead links. The
   `preview` family is honest about not yet being mirror-synced and is narrowed.

5. **User-owned, private, exportable progress.** Each family's progress snapshot is
   `progress_local_by_default`, not `repo_visible`, not
   `telemetry_grade_read_access`, `survives_restart`, and `safe_for_support_export`.
   Progress, dismissal, and resume state is user-owned on the same terms as the
   existing learning-mode assets.

## How the verdict is derived

`derive_bundle_verdict` meets the verdicts of a family's glossary pack, tour
package, contextual help cards, exercise rail, and progress snapshot, then narrows
further if the mirror-parity posture fails or no in-product command-backed path
exists. An unclaimed family is `absent` rather than narrowed. The manifest's
`overall_verdict` is the strictest verdict across all claimed families. Stored
verdicts are re-derived and checked by `validate_m5_feature_family_learning`, so a
hand-edited fixture that disagrees with its own evidence fails validation.

## How to reproduce

```sh
cargo test -p aureline-learning m5_feature_family
cargo run -q -p aureline-learning --bin aureline_learning_m5_feature_family_rails -- validate
cargo run -q -p aureline-learning --bin aureline_learning_m5_feature_family_rails -- summary
```
