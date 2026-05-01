# Migration restore and shortcut-delta evidence packet contract

This document freezes the migration evidence packet that captures
*post-import truth*: what restored, what remapped, what needs
relearning, and what remains unsupported. It exists so a switching
claim cannot hide behind a single `imported successfully` line. The
packet aggregates importer outcomes, restore fidelity, rollback
checkpoint availability, shortcut/keymap deltas, and migration-gap
rows that affect daily usability, and binds them to the
compatibility-narrowing rules that decide when a marketed switching
path must be narrowed or carry an explicit known limit.

The contract is structural. It does not implement importer adapters,
real keymap translation engines, or restore execution. It defines the
packets every importer, restore surface, migration-center surface,
docs/help projection, and support export MUST emit when shortcut or
restore fidelity is weaker than the marketed migration path.

Companion artifacts:

- [`/schemas/migration/shortcut_delta_digest.schema.json`](../../schemas/migration/shortcut_delta_digest.schema.json)
  — boundary schema for `shortcut_delta_digest_row_record` and
  `shortcut_delta_digest_packet_record`, the dedicated digest reused
  by migration center, docs, support, and release publications.
- [`/artifacts/migration/migration_gap_scoreboard.yaml`](../../artifacts/migration/migration_gap_scoreboard.yaml)
  — seed migration-gap scoreboard with ecosystem/source rows, affected
  surfaces, exact/translated/approximated/skipped/blocked status,
  docs/help fallback, and claim-narrowing implication.
- [`/fixtures/migration/restore_shortcut_cases/`](../../fixtures/migration/restore_shortcut_cases/)
  — worked cases for restore fidelity envelopes, shortcut-delta
  digests, and compatibility-narrowing decisions across the governed
  source ecosystems.
- [`/docs/migration/first_run_import_diff_and_rollback_contract.md`](./first_run_import_diff_and_rollback_contract.md)
  — first-run dry-run import plan, preview, apply, rollback, and
  imported-profile history contract that emits the packets this
  contract aggregates.
- [`/docs/migration/migration_center_object_model.md`](./migration_center_object_model.md)
  and [`/schemas/migration/migration_session.schema.json`](../../schemas/migration/migration_session.schema.json)
  — durable migration-session, importer-outcome, lightweight
  shortcut-digest, and restore-record vocabulary the packet refines.
- [`/docs/migration/compatibility_scorecard_contract.md`](./compatibility_scorecard_contract.md),
  [`/schemas/migration/compatibility_scorecard.schema.json`](../../schemas/migration/compatibility_scorecard.schema.json),
  and [`/artifacts/migration/top_imported_workflow_rows.yaml`](../../artifacts/migration/top_imported_workflow_rows.yaml)
  — imported-extension, imported-workflow, and workflow-bundle
  scorecards the packet cites when a row is partial, blocked,
  community-only, deprecated, or replaced.
- [`/docs/migration/source_ecosystem_coverage_matrix.md`](./source_ecosystem_coverage_matrix.md)
  and [`/artifacts/migration/source_ecosystem_rows.yaml`](../../artifacts/migration/source_ecosystem_rows.yaml)
  — governed source ecosystem catalog the scoreboard rows index.
- [`/artifacts/migration/quality_bar_rubric.yaml`](../../artifacts/migration/quality_bar_rubric.yaml)
  — migration quality-bar vocabulary reused by the narrowing rules.

If this document disagrees with the PRD, technical architecture,
technical design, UI/UX spec, or the frozen migration-center,
restore-provenance, and compatibility-scorecard contracts, those
sources win and this document plus its schemas MUST be updated in the
same change.

## Scope

This contract freezes:

- a migration evidence packet covering imported workspace/profile
  state, restore fidelity after import, rollback checkpoint
  availability, unresolved gaps, and resulting compatibility or
  support-class notes;
- a shortcut-delta digest with imported command, remapped gesture,
  conflict resolution, unresolved gap, suggested alternative, and
  muscle-memory risk note so keymap changes stay visible;
- a migration-gap scoreboard with ecosystem/source row, affected
  surface, exact/translated/approximated/skipped/blocked status,
  docs/help fallback, and claim-narrowing implication;
- compatibility-narrowing rules for when restore gaps, unresolved
  gestures, or migration caveats require a narrower switching claim
  or an explicit known limit; and
- export and support linkage so migration packets, known-limits
  notes, help surfaces, and onboarding proof lanes all reflect the
  same post-import truth.

Out of scope:

- implementing importer logic or real keymap translation engines;
- broad extension compatibility implementation;
- rendering UI for any migration surface; and
- defining final product copy for every row.

## Migration evidence packet

The migration evidence packet is the post-import truth a reviewer,
support engineer, claim-manifest validator, or release reviewer reads
to know what the user gained, what changed, and what they still must
relearn or work around. Packets MUST cover all five layers:

| Layer | Source artifact | Required surfaces |
|---|---|---|
| Imported workspace and profile state | `importer_outcome_packet_record` rows grouped by domain | Domain breakdown, source-labeled rows, never collapsed into a single result |
| Restore fidelity after import | `restore_fidelity_envelope` (this contract) | One typed `fidelity_class` plus post-restore validation refs |
| Rollback checkpoint availability | `migration_restore_record` plus `first_run_import_rollback_checkpoint_record` | Typed `availability_state` and `cleanup_state`; never silently dropped |
| Unresolved gaps | `shortcut_delta_digest_packet_record` unresolved-gap rows plus `migration_gap_scoreboard` rows | Each gap carries class, alternative, docs/help refs, claim-narrowing implication |
| Compatibility/support-class notes | Compatibility scorecard row refs, known-limit refs, support-export refs | Travel as part of the packet, never reconstructed in prose later |

Rules:

1. A packet that omits any of the five layers is non-conforming. An
   empty array is allowed; an absent layer is not.
2. The packet preserves source labels at every layer. A consumer must
   be able to ask "which source profile/ecosystem produced this row"
   without reading a separate file.
3. Restore fidelity, rollback checkpoint availability, and unresolved
   gap counts MUST be visible on every projection of the packet
   (migration center summary, docs/help row, support export, release
   compatibility row). Surfaces may shorten the rendering; they may
   not hide a non-empty unresolved gap or a non-`exact_restore`
   fidelity class.
4. The packet is reusable. The same evidence packet feeds migration
   center history, docs/help, claim manifests, support exports, and
   release compatibility rows. Surfaces may not re-derive these
   numbers from a different source.

### Restore fidelity envelope

Every packet carries a `restore_fidelity_envelope` with one typed
`fidelity_class`:

| Class | Meaning |
|---|---|
| `exact_restore` | Post-restore state matches the imported source profile to the granularity the source itself preserves. |
| `compatible_restore` | Restore preserves intent through a documented mapping; small canonicalization is permitted. |
| `approximated_restore` | A semantic equivalent restored, but the user must accept that a feel-different mapping landed. |
| `manual_review_after_restore` | Restore landed but at least one row needs reviewer acceptance before it is usable. |
| `partial_restore_with_visible_gaps` | A subset restored and the remainder is visible as gap rows. |
| `restore_blocked` | The checkpoint or restore action is blocked for policy, evidence, or compatibility reasons. |
| `restore_not_applicable` | The session never wrote durable state (preview-only, dry-run, skip path). |

The envelope MUST also carry `rollback_checkpoint_availability`,
`rollback_checkpoint_ref`, `restore_record_ref`, and
`post_restore_validation_refs` so consumers can re-check the claim
without rereading the original session.

## Shortcut-delta digest

The dedicated digest is the row source migration center, docs/help,
support exports, and release compatibility rows quote when an imported
command is talked about. It refuses the `imported successfully`
shorthand by structure: every row carries the imported command, the
gesture it now resolves to, the conflict resolution, the unresolved
gap, the suggested alternative, and the muscle-memory risk note.

Required row fields:

| Field | Purpose |
|---|---|
| `imported_command_ref` and `imported_command_label` | What the source profile bound. |
| `imported_gesture` | The literal source gesture the user learned. |
| `remapped_gesture` and `remapped_target_command_ref` | When and how the destination binding diverges. |
| `delta_state` | Closed vocabulary across imported, remapped, conflict-resolved, conflict-unresolved, unresolved-gap, and skipped classes. |
| `conflict_resolution` | How the importer reconciled a chord collision (`kept_target_existing`, `kept_imported_overriding_target`, `remapped_imported_to_alternate_chord`, `deferred_to_user_review`, `blocked_by_*`, etc.). |
| `unresolved_gap` | Why the row could not resolve, drawn from `no_native_equivalent_command`, `no_native_chord_capacity`, `modal_or_leader_gap`, `extension_runtime_gap`, `policy_or_trust_gap`, `compatibility_scorecard_blocked`, etc. |
| `suggested_alternative` | The alternative path the user should rely on (`native_command_palette_path`, `leader_sequence`, `modal_editing_motion`, `menu_or_pane_route`, `command_link_in_docs`, `platform_native_gesture`, etc.). |
| `muscle_memory_risk_class` and `muscle_memory_risk_note` | Typed risk plus the human-readable reason the user will feel it. |
| `frequency_bucket` | `daily_driver`, `frequent`, `occasional`, `rare`, or `unknown`. |
| `claim_narrowing_implication` | Whether this row alone forces narrowing the marketed keymap claim. |
| `compatibility_scorecard_ref` | Linked scorecard when the row is gated by an imported-extension or imported-workflow scorecard. |
| `docs_help_refs` and `support_export_refs` | Required publication refs so the row remains visible across surfaces. |

Required packet fields:

- separated `imported_command_rows`, `remapped_gesture_rows`,
  `conflict_resolution_rows`, `unresolved_gap_rows`, and
  `skipped_rows` arrays so a downstream renderer cannot silently
  collapse one bucket into another;
- a `restore_fidelity_envelope` so the keymap claim cannot be read
  without the restore claim it depends on;
- a `muscle_memory_risk_summary` with `highest_risk_class`,
  `daily_driver_delta_count`, `high_risk_row_count`, and ordered
  notes;
- a `claim_narrowing_summary` with the packet-level
  `overall_implication_class`, the actions required from the
  narrowing-action vocabulary, and the rule refs the decision was
  based on; and
- a `publication_links` block with non-empty `docs_help_refs`,
  `support_export_refs`, `migration_report_refs`,
  `claim_manifest_refs`, and `compatibility_row_refs`.

Rules:

1. A packet whose `unresolved_gap_rows` is non-empty MUST also carry
   a non-`no_narrowing_required` `claim_narrowing_summary`. An empty
   narrowing summary with non-empty gaps is non-conforming.
2. A packet whose `restore_fidelity_envelope.fidelity_class` is not
   `exact_restore` and whose `claim_narrowing_summary.overall_implication_class`
   is `no_narrowing_required` is non-conforming. The narrowing rules
   below decide which specific implication applies.
3. A row with `delta_state` in `{conflict_resolved_keep_target,
   conflict_resolved_keep_imported, conflict_resolved_remap,
   conflict_unresolved}` MUST carry a `conflict_resolution`.
4. A row with `delta_state` in `{conflict_unresolved,
   unresolved_gap_no_native_equivalent,
   unresolved_gap_blocked_by_policy,
   unresolved_gap_blocked_by_compatibility}` MUST carry both
   `unresolved_gap` and `suggested_alternative`.
5. A row whose `frequency_bucket` is `daily_driver` and whose
   `muscle_memory_risk_class` is `high` or `critical` MUST appear in
   the packet `muscle_memory_risk_summary` notes — it cannot be
   summarized to zero in prose.

## Migration-gap scoreboard

The scoreboard freezes the row source migration center, docs/help,
release compatibility reports, claim manifests, and support exports
quote when they discuss switching gaps. Each row identifies one
ecosystem/source × affected surface decision, gives it one typed
status, names the docs/help fallback, and records the
claim-narrowing implication.

Row contract (mirrored in the artifact YAML):

| Field | Purpose |
|---|---|
| `scoreboard_row_id` | Stable `migration_gap_row:*` id quoted by docs, support, and release surfaces. |
| `source_ecosystem_id` | Governed source ecosystem id (or `mixed_imported_sources`/`generic_import` for handoff rows). |
| `source_object_ref` and `source_label` | Specific imported command, workflow, profile, or asset the row decides about. |
| `affected_surface` | Closed vocabulary across keymap, command palette, command flow, modal editing, run/debug, search, source control, terminal, theme, snippet, profile, layout, extension surface, and compatibility surface. |
| `gap_status` | Closed vocabulary: `exact`, `translated`, `approximated`, `skipped`, `blocked`, `not_applicable`. |
| `gap_class` | Why the gap exists, drawn from the `unresolved_gap_class` vocabulary plus runtime/asset/profile-specific extensions. |
| `docs_help_fallback_ref` | The docs/help row a user is sent to when this gap is hit. Required even when `gap_status` is `exact`. |
| `support_export_ref` | The support export ref this row appears under. |
| `compatibility_scorecard_ref` | Linked scorecard ref when the gap is governed by a scorecard row. |
| `claim_narrowing_implication` | One typed implication class plus a short summary so docs/release/support apply the same narrowing. |
| `evidence_state` and `source_revision` | Freshness fields so docs do not republish stale rows. |

Rules:

1. A row absent from the scoreboard is absent from the marketed
   migration claim. Surfaces MUST NOT invent gap rows that did not
   travel through the scoreboard.
2. A row with `gap_status` in `{approximated, skipped, blocked}` MUST
   carry a `claim_narrowing_implication` that is not
   `no_narrowing_required`.
3. The scoreboard is the same row source for docs, claim manifests,
   release compatibility rows, and support exports. Aggregations may
   reorder or shorten rows, but may not hide a `blocked` or
   `approximated` row to keep a higher-grade claim alive.

## Compatibility-narrowing rules

The narrowing rules decide when restore gaps, unresolved gestures, or
migration caveats require a narrower switching claim or an explicit
known limit. The vocabulary is closed and shared with the migration
quality bar rubric:

| Implication class | Required narrowing action(s) |
|---|---|
| `no_narrowing_required` | `no_action_required` only; allowed only when restore is `exact_restore` and no `approximated`/`skipped`/`blocked` row exists. |
| `narrow_to_partial_keymap_claim` | `narrow_to_partial`, optionally `keep_claim_with_caveat`. |
| `narrow_to_documented_remap_only` | `narrow_to_documented_remap_only` plus `add_known_limit_row` when the remap reverses a daily-driver gesture. |
| `narrow_to_no_keymap_claim` | `narrow_to_partial` + `add_known_limit_row`, or `move_to_docs_only` when the digest cannot project a daily-driver tier. |
| `add_explicit_known_limit` | `add_known_limit_row` plus a docs/help link in the same change set. |
| `block_marketed_keymap_path` | `block_marketed_path` and `remove_public_claim`; the marketed keymap claim is held until the blocking row clears. |

Required triggers:

1. `restore_fidelity_envelope.fidelity_class` of
   `approximated_restore`, `manual_review_after_restore`, or
   `partial_restore_with_visible_gaps` requires at least
   `narrow_to_partial_keymap_claim` (or stronger) for any keymap
   claim attached to the session.
2. `restore_fidelity_envelope.fidelity_class` of `restore_blocked`
   forces `block_marketed_keymap_path` for the affected lane until
   the blocker clears.
3. Any row with `delta_state ∈ {unresolved_gap_blocked_by_policy,
   unresolved_gap_blocked_by_compatibility}` and
   `frequency_bucket = daily_driver` forces
   `block_marketed_keymap_path` for that source ecosystem until a
   scorecard row narrows the claim.
4. Any row with `delta_state = conflict_unresolved` and
   `muscle_memory_risk_class ∈ {high, critical}` forces at least
   `narrow_to_documented_remap_only` plus a known-limit row.
5. Any scoreboard row whose `gap_status` is `approximated` and whose
   `frequency_bucket` (via the linked digest) is `daily_driver`
   forces `narrow_to_partial_keymap_claim` for the affected surface.
6. Narrowing actions MUST land in the same change set as the
   migration evidence packet. Public copy that widens past the
   packet's `claim_narrowing_summary.overall_implication_class`
   without a late-proof exception is non-conforming.

The migration-center, docs/help, claim-manifest, and release surfaces
read the narrowing summary, not the prose. A surface that picks a
weaker implication than the packet's summary is non-conforming.

## Export and support linkage

Migration packets, known-limits notes, help surfaces, and onboarding
proof lanes MUST reflect the same post-import truth. The linkage is
mechanical:

| Surface | Required linked refs |
|---|---|
| Migration center session view | `outcome_packet_ref`, `shortcut_digest_id`, `restore_record_ref`, `restore_fidelity_envelope`, `claim_narrowing_summary`, scoreboard row refs |
| Docs/help projection | `docs_help_refs` from the packet plus the docs/help fallback ref from each scoreboard row whose status is not `exact` |
| Known-limits / release notes | `claim_narrowing_summary.known_limit_refs`, plus the scoreboard rows whose implication class is `add_explicit_known_limit` or stronger |
| Onboarding portability proof | `restore_fidelity_envelope`, `imported_profile_history_ref`, `muscle_memory_risk_summary` (so the proof lane cannot claim parity it has not earned) |
| Support export packet | `publication_links.support_export_refs`, `restore_fidelity_envelope`, `unresolved_gap_rows`, scoreboard rows |
| Claim manifest projection | `publication_links.claim_manifest_refs`, `claim_narrowing_summary.overall_implication_class`, scoreboard rows |
| Issue template handoff | `publication_links.issue_template_refs`, `unresolved_gap_rows`, scoreboard rows |

Rules:

1. A surface that publishes a switching claim without quoting the
   packet's `restore_fidelity_envelope.fidelity_class` and
   `claim_narrowing_summary.overall_implication_class` is
   non-conforming.
2. A late copy or release that widens the claim above the packet's
   summary requires a late-proof exception packet; otherwise the
   widening is held until the packet refreshes.
3. The same packet, scoreboard rows, and narrowing summary travel
   through every export. A docs page, support export, and release
   compatibility row may differ in length and tone but may not
   disagree on `fidelity_class`, `gap_status`, or
   `overall_implication_class`.

## Acceptance

A migration evidence packet conforms when:

- Switching claims can explain what restored, what remapped, what
  needs relearning, and what remains unsupported by reading the
  packet alone.
- Shortcut and gesture deltas remain explicit in migration and
  support packets, never folded behind a generic
  `imported successfully` result.
- The packet, shortcut-delta digest, and migration-gap scoreboard
  drive claim narrowing whenever restore or shortcut fidelity is
  weaker than the marketed migration path, with a typed implication
  class and a same-change-set narrowing action.

## Fixture coverage

The fixture corpus under
[`/fixtures/migration/restore_shortcut_cases/`](../../fixtures/migration/restore_shortcut_cases/)
covers:

| Fixture | Primary behavior |
|---|---|
| `vscode_full_restore_keymap_partial.yaml` | Full settings/extension restore with a partial keymap claim and one daily-driver remap. |
| `jetbrains_run_debug_partial_restore.yaml` | Run/debug import landing as `manual_review_after_restore` with narrowed keymap and known-limit rows. |
| `vim_neovim_modal_blocked.yaml` | Modal editing import where Lua plugin runtime blocks the marketed keymap path until the scorecard narrows the claim. |
| `emacs_elisp_blocked_widening_held.yaml` | Elisp runtime blocks the marketed keymap path; widening is held and the public claim narrows to docs only. |
| `unresolved_conflict_daily_driver.yaml` | Unresolved chord conflict on a daily-driver gesture forcing documented remap only plus an explicit known-limit row. |
