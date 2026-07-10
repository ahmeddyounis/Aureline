# M5 Snapshot-Review-Card / Coverage-Import-Merge-Sheet Primitive

- Packet: `m5-snapshot-coverage-import-primitive:stable:0001`
- Label: `M5 snapshot-review-card / coverage-import-merge-sheet primitive: controlled snapshot artifact kinds, baseline identities, diff states, render/raw fallback modes, environment/viewport/theme/serializer/locale scope, distinct matches-baseline/diff-detected/new-snapshot/obsolete/render-unavailable/raw-text-fallback review postures, controlled coverage import sources, merge-resolution states, line-versus-branch metric kinds, distinct merged-clean/shard-omission/conflicting-overlap/partial-merge/superseded/merge-unavailable merge postures, included and excluded run scope, commit/build identity, stale-or-incompatible warnings, a required scope disclosure before an acceptance decision, a required raw fallback for an opaque artifact, a required omitted-shard disclosure before exact current truth, and bounded reveal/accept-baseline/reject-change/open-raw-fallback/export and reveal/review-run-scope/open-incompatible/export actions`
- Review consumers: 5 (5 stable)
- Snapshot postures: matches_baseline_card, diff_detected_card, new_snapshot_card, obsolete_snapshot_card, render_unavailable_card, raw_text_fallback_card
- Merge postures: merged_clean_sheet, shard_omission_sheet, conflicting_overlap_sheet, partial_merge_sheet, superseded_sheet, merge_unavailable_sheet
- Fallback modes: rendered_diff, side_by_side, raw_text_fallback, raw_text_only
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## Review consumers

- **Snapshot Review Panel**: `stable`
  - Owner: Snapshot review panel owner
  - Scope: The snapshot review panel renders the shared snapshot / golden review card so a detected diff against a committed baseline reads as an acceptance decision only when it discloses its artifact count, its environment / viewport / theme scope, and a side-by-side raw fallback rather than collapsing to a blind Accept all, while a matched baseline needs no acceptance; it renders the shared coverage-import / merge sheet so a clean local merge may be treated as exact current truth precisely because no run was omitted, nothing is stale, and nothing is incompatible
  - Worked cards: 2 / sheets: 1
    - card `snapshot-card:review-panel::checkout-visual` (`diff_detected`) -> `diff_detected_card` (acceptance `true`, scope `true`, raw fallback `true`)
    - card `snapshot-card:review-panel::settings-serializer` (`matches_baseline`) -> `matches_baseline_card` (acceptance `false`, scope `true`, raw fallback `false`)
    - sheet `merge-sheet:review-panel::checkout-coverage` (`merged_clean`) -> `merged_clean_sheet` (omission `false`, warning `false`, exact `true`)
- **Editor Snapshot Diff**: `stable`
  - Owner: Editor snapshot diff owner
  - Scope: The editor snapshot-diff surface renders the shared snapshot / golden review card so a brand-new snapshot with a pending baseline stays an acceptance decision that discloses its environment / theme scope and a side-by-side raw fallback, and it renders the shared coverage-import / merge sheet so a partial merge drawn from a cached report names the excluded shard rather than presenting the merged number as exact current truth
  - Worked cards: 1 / sheets: 1
    - card `snapshot-card:editor::new-header-dom` (`new_snapshot`) -> `new_snapshot_card` (acceptance `true`, scope `true`, raw fallback `true`)
    - sheet `merge-sheet:editor::partial-coverage` (`partial_merge`) -> `partial_merge_sheet` (omission `true`, warning `false`, exact `false`)
- **Coverage Import / Merge Panel**: `stable`
  - Owner: Coverage import / merge panel owner
  - Scope: The coverage-import / merge panel renders the shared snapshot / golden review card so an obsolete snapshot against an updated baseline reads as an obsolete-snapshot card shown through a raw / text fallback, and it renders the shared coverage-import / merge sheet so a shard omission imported from a CI artifact names both omitted shards and a conflicting overlap from an uploaded report is flagged incompatible before any merged number is treated as exact current truth
  - Worked cards: 1 / sheets: 2
    - card `snapshot-card:import-panel::obsolete-json` (`obsolete_snapshot`) -> `obsolete_snapshot_card` (acceptance `false`, scope `true`, raw fallback `true`)
    - sheet `merge-sheet:import-panel::ci-shard-omission` (`shard_omission_detected`) -> `shard_omission_sheet` (omission `true`, warning `false`, exact `false`)
    - sheet `merge-sheet:import-panel::conflicting-overlap` (`conflicting_overlap`) -> `conflicting_overlap_sheet` (omission `false`, warning `true`, exact `false`)
- **Headless / CLI Review**: `stable`
  - Owner: Headless / CLI review owner
  - Scope: The headless / CLI review surface renders the shared snapshot / golden review card so an opaque binary snapshot imported from a CI baseline whose rendered diff is unavailable still keeps a raw / text-only fallback rather than a blind accept, and it renders the shared coverage-import / merge sheet so a coverage report superseded by a newer run from a stale prior source is flagged stale — proving the same grammar works without a desktop surface
  - Worked cards: 1 / sheets: 1
    - card `snapshot-card:headless::binary-render-unavailable` (`render_unavailable`) -> `render_unavailable_card` (acceptance `false`, scope `true`, raw fallback `true`)
    - sheet `merge-sheet:headless::superseded-coverage` (`superseded_by_newer`) -> `superseded_sheet` (omission `false`, warning `true`, exact `false`)
- **Review Export**: `stable`
  - Owner: Review export owner
  - Scope: The review export renders the shared snapshot / golden review card so an inline snapshot with a missing baseline reads as a raw / text-fallback card rather than a settled accept, and it renders the shared coverage-import / merge sheet so a merge-unavailable sheet from an unknown source flagged incompatible reads with the same vocabulary a reviewer sees in the panel and the editor
  - Worked cards: 1 / sheets: 1
    - card `snapshot-card:export::inline-missing-baseline` (`raw_text_fallback`) -> `raw_text_fallback_card` (acceptance `false`, scope `true`, raw fallback `true`)
    - sheet `merge-sheet:export::merge-unavailable` (`merge_unavailable`) -> `merge_unavailable_sheet` (omission `false`, warning `true`, exact `false`)
