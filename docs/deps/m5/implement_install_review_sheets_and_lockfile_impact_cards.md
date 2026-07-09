# Implement install-review sheets and lockfile-impact cards

Status: Implemented (M05-975, batch B115)

This contract narrows two components frozen in
[`m5-package-management-component-matrix`](freeze_the_m5_package_management_component_matrix.md)
(M05-972) — the `install_review_sheet` and the `lockfile_impact_card` — into one
implemented, export-safe packet with two co-equal control vectors. Together they
preview the real dependency-mutation blast radius **before** Aureline writes any
manifest or lockfile, so package mutation flows never hide manifest writes,
lockfile churn, peer conflicts, or validation expectations behind a generic
confirm.

- Boundary schema: [`schemas/ui/m5-install-review-lockfile-controls.schema.json`](../../../schemas/ui/m5-install-review-lockfile-controls.schema.json)
- Producer: `aureline_deps::current_install_review_lockfile_export`
- Release proof: [`artifacts/release/m5-install-review-lockfile-proof/`](../../../artifacts/release/m5-install-review-lockfile-proof/)
- Protected fixtures: [`fixtures/ui/m5-install-review-lockfile-controls/`](../../../fixtures/ui/m5-install-review-lockfile-controls/)

## Install-review sheets

Every `InstallReviewSheet` reuses the frozen `M5PackageComponent` tag (gated to
`install_review_sheet`) and answers, from the sheet alone:

- **Operation** (`operation`: `install` / `update` / `remove`) — a remove is
  never flattened into a generic "apply change". The packet requires all three
  operations to be represented (`operation_coverage_missing`).
- **Affected manifests** (`affected_manifests`, always required and non-empty —
  no hiding a manifest write) and **affected lockfiles** (`affected_lockfiles`).
- **Version delta** (`version_delta`, required) and **peer / runtime shifts**
  (`peer_conflict_count`, `peer_runtime_shift_note`).
- **Validation expectations** (`validation_tasks`, required and non-empty) and
  **registry / auth state** (`registry_auth_state_note`, required).
- **Checkpoint / rollback actions** (`checkpoint_action_label`,
  `rollback_action_label`) offered before the write, with a rollback posture
  constrained to staged-review or write-back-behind-a-checkpoint (a review sheet
  never writes directly).

The change breadth is *derived*, never asserted, by
`resolve_review_change_breadth(peer_conflict_count, affected_manifest_count,
affected_lockfile_count, transitive_churn_count, is_grouped)`. A change is
`broad_change` when it must resolve a peer conflict, regenerates more than one
lockfile, or crosses the broad transitive-churn threshold; `grouped_change` when
it touches several manifests, is explicitly grouped, or crosses the grouped
threshold; and `small_single` otherwise. The sheet's `change_breadth` and
`warrants_deeper_inspection` must match the derived values
(`change_breadth_misrepresented`), so a broad change can never read as a small
isolated one — this is the AC's "quantify whether the change is small, grouped,
or broad enough to warrant deeper inspection". The packet requires all three
breadths to be demonstrated (`breadth_coverage_missing`).

## Lockfile-impact cards

Every `LockfileImpactCard` reuses the frozen `M5PackageComponent` tag (gated to
`lockfile_impact_card`) and answers, from the card alone:

- **Resolver identity** (`resolver_label`, `resolver_version`, both required).
- **Affected lockfiles** (`affected_lockfiles`, required and non-empty).
- **Churn** (`direct_change_count`, `transitive_churn_count`) with a derived
  `churn_magnitude` (`no_churn` / `narrow_churn` / `moderate_churn` /
  `broad_churn`) and a required `churn_note` whenever there is any churn.
- **Platform / tool-version sensitivity** (`platform_sensitive`,
  `tool_version_sensitive`, `platform_tool_note`).
- **Regenerate-versus-edit write mode** (`write_mode`:
  `regenerate_whole_lockfile` / `edit_in_place_entries` / `no_lockfile_write`)
  with a required `write_mode_note`.

Churn magnitude is *derived* by `resolve_lockfile_churn(direct_change_count,
transitive_churn_count, platform_sensitive, tool_version_sensitive)` so lockfile
churn is quantified, never understated (`churn_magnitude_misrepresented`). The
rollback posture is *derived* from the write mode: a `regenerate_whole_lockfile`
regenerates rather than accepting manual edits
(`regenerate_only_no_manual_edit`), an `edit_in_place_entries` write is a
write-back behind a durable checkpoint, and a `no_lockfile_write` card mutates
nothing. A card whose posture disagrees with its write mode fails
(`card_rollback_posture_inconsistent`), so a regenerate-only lockfile can never
claim a manual-edit write-back. The packet requires both regenerate and edit
modes to be represented (`write_mode_coverage_missing`).

## Coverage and boundary

Beyond the per-vector coverage rules above, the packet carries the batch
guardrail `no_generic_confirm_language` (the trust-review invariant that no
generic confirm conceals scope, churn, peers, or validation). Raw manifest
bodies, raw lockfile bodies, registry credentials, private registry URLs, and
live registry responses never cross this boundary; the export is scanned for
forbidden material (`raw_boundary_material_in_export`).

## Regenerating artifacts

```
GEN_INSTALL_REVIEW_LOCKFILE_ARTIFACTS=1 cargo test -p aureline-deps --lib \
  implement_install_review_sheets_and_lockfile_impact_cards::tests::generate_artifacts
```
