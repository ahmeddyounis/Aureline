# Import Review Sheet, Handoff-Artifact Inspection, and Post-Entry Handoff-Card Contract

This document freezes the **import review sheet** every import
activation renders before Aureline writes files (anywhere — temporary
or durable), rehydrates state, or claims portability; the
**handoff-artifact inspection rules** that name how portable-state
packages, issue / support packets, archive imports, and bundle-like
handoff objects are inspected (with redaction and provenance labels);
and the **post-entry handoff-card contract** that summarises what
Aureline intentionally did *not* do yet, what remains blocked /
optional, and what safe next action is available.

The contract is normative. Where it disagrees with the PRD, TAD,
TDD, UI / UX Spec, or design-system style guide, those sources win
and this document plus its schemas and fixtures update in the same
change. Where a Start Center, palette, drag-drop preview, system-open
handoff, deep-link intent review, CLI / headless front-end, support
console, or migration-center mints a parallel import-review surface,
inspection record, or post-entry hand-off, this contract wins and
the surface is non-conforming.

The companion artifacts are:

- [`/schemas/ux/import_review.schema.json`](../../schemas/ux/import_review.schema.json)
  — boundary schema for the two records: `import_review_record` and
  `handoff_artifact_inspection_record`.
- [`/schemas/ux/post_entry_handoff_card.schema.json`](../../schemas/ux/post_entry_handoff_card.schema.json)
  — boundary schema for the `post_entry_handoff_card_record`.
- [`/fixtures/ux/import_handoff_cases/`](../../fixtures/ux/import_handoff_cases/)
  — worked cases for the artifact-class disclosure axes, the
  inspect-only and write-capable paths, lossy-mapping disclosure,
  machine-local exclusions, cleanup posture, the four
  handoff-inspection profiles (portable-state, support / issue,
  archive, bundle-like handoff), and the post-entry handoff-card
  outcomes (review pending, paused for later, admitted partial,
  rolled back, blocked by unsupported items, failed pending
  recovery).

This contract composes with, and does not replace:

- [`/docs/ux/project_entry_contract.md`](./project_entry_contract.md)
  and [`/schemas/ux/open_flow_sheet.schema.json`](../../schemas/ux/open_flow_sheet.schema.json)
  for the `import_artifact` open-flow sheet that hosts the import
  review. The `import_review_record` is the import-specific
  projection of the eight required disclosure axes
  (project_entry_contract.md §5.2) onto a single import activation.
  The import review never re-opens the verb matrix, never widens
  the trust posture, and never bypasses the §5.3 pre-commit
  invariants.
- [`/docs/ux/persistence_inspector_contract.md`](./persistence_inspector_contract.md)
  and [`/schemas/state/portable_state_package.schema.json`](../../schemas/state/portable_state_package.schema.json)
  for the `portable_state_package_record`,
  `portable_state_export_sheet_record`, and
  `restore_provenance_card_record`. The import review previews
  those records when the import resolves to a portable-state
  artifact; it does not redefine `artifact_class`,
  `redaction_class`, `signature_state`, `checksum_state`,
  `state_plane`, `portability_class`, `restore_level`, or
  `fidelity_label`.
- [`/docs/ux/clone_review_contract.md`](./clone_review_contract.md)
  for the parallel clone-side `clone_review_record`,
  `destination_collision_sheet_record`, and
  `post_clone_trust_stage_record` family. The post-entry
  handoff-card here is the import-side analogue of the clone-side
  post-clone trust-stage record: it is emitted *after* row
  activation and *before* any side effect; it never grants trust,
  installs dependencies, recommends extensions, or executes hooks
  / tasks.
- [`/docs/workspace/materialization_and_staging_policy.md`](../workspace/materialization_and_staging_policy.md)
  and [`/schemas/workspace/materialization_class.schema.json`](../../schemas/workspace/materialization_class.schema.json)
  for the workspace-level target-materialization vocabulary and
  temp-location disclosure rules that keep inspection, staging, and
  durable promotion distinct across import/archive/handoff flows.
- [`/docs/ux/trust_prompt_contract.md`](./trust_prompt_contract.md)
  and [`/docs/adr/0001-identity-mode-trust.md`](../adr/0001-identity-mode-trust.md)
  / [`/docs/adr/0018-workspace-trust-and-restricted-mode.md`](../adr/0018-workspace-trust-and-restricted-mode.md)
  for trust-state, authority-delta, and restricted-mode
  vocabulary. The import review previews the resulting trust
  posture; the trust transition is owned by the trust-prompt
  surface and fires only after the import-result review admits.
- [`/docs/ux/dialog_sheet_contract.md`](./dialog_sheet_contract.md)
  for the surface-class taxonomy the import review and post-entry
  handoff card render inside (`window_attached_sheet`,
  `full_sheet`, `dedicated_review_surface`,
  `cli_headless_text_block`).

## Who reads this contract

- **Shell, Start Center, palette, drag-drop, system-open, deep-link,
  CLI, support-console, and migration-center authors** wiring the
  `import` verb. Every import activation emits one
  `import_review_record`, one
  `handoff_artifact_inspection_record`, and one
  `post_entry_handoff_card_record` BEFORE Aureline writes files
  into temporary or durable locations, rehydrates state, or claims
  portability.
- **Designers** sizing the import review and handoff copy so
  Import never smuggles in trust, durable writes, state
  rehydration, or portability claims.
- **Docs, support, accessibility, and measurement authors**
  attributing import behavior to the same record family the shell
  renders, so a CLI import, a support-bundle replay, and a Start
  Center click trace to the same import-review and post-entry
  handoff-card rows.

## 1. Scope

This contract freezes:

- One `import_review_record` (§3) every import activation emits
  before Aureline writes files (anywhere), rehydrates state, or
  claims portability. The record carries the closed disclosure
  axes (§3.2): artifact class, schema / producer version,
  inspect-only versus write-path, extraction / restore target,
  lossy-mapping note, machine-local exclusions, cleanup posture,
  and export / open-raw actions.
- One `handoff_artifact_inspection_record` (§4) every import
  activation emits alongside the import review. The record carries
  the typed inspection rules each artifact class requires:
  portable-state package, issue / support packet, archive import,
  and bundle-like handoff. Each inspection profile freezes its
  redaction class, redaction posture, provenance label class, and
  raw-body inspectability rule.
- One `post_entry_handoff_card_record` (§5) every import
  activation emits AFTER the user activates an import row and
  BEFORE Aureline rehydrates state, writes durable bytes into a
  final home, or claims the artifact has been ingested. The card
  carries the closed `handoff_card_status_class`, the typed list
  of what Aureline did *not* do yet, the typed list of what
  remains blocked / optional, the safest next action, and the
  preserved provenance / exclusion disclosures the support /
  export retrieval reads later.
- The cross-surface invariants (§6) so Start Center, palette,
  drag-drop, system open, deep link, CLI / headless,
  migration-center, and support console remain semantically
  aligned with the same three records.

## 2. Out of Scope

- Importer backends. The actual archive-extraction, schema-
  migration, competitor-mapping, support-redaction, and
  portable-state-rehydration engines are owned by the upstream
  importers and the persistence-inspector / portable-state
  package contracts. This contract only freezes the typed inputs
  the import-review and post-entry handoff card render.
- Final user-facing copy and microcopy. This contract pins the
  closed sets the copy resolves against; the design-system style
  guide and shell-interaction-safety contract own the strings.
- Platform file-picker chrome for picking an artifact to import.
  The desktop-affordance contract owns that overlay; the artifact
  the user picks lands on the `import_review_record` as the
  artifact-descriptor ref.
- Workspace-trust prompt visuals, policy-review takeover,
  rollback-checkpoint inspection, and migration-result review.
  Those are owned by the trust-prompt, policy-review,
  rollback-checkpoint, and migration-review contracts; the
  post-entry handoff-card record only **routes** the user into
  them.

## 3. Import review record

Every import activation emits one `import_review_record` BEFORE
Aureline writes files into any location — ephemeral inspection
memory, labelled inspection or extraction staging, the user's
chosen destination, or a durable workspace — before any state is
rehydrated, and before the activation can claim portability. The
import review is the body the `open_flow_sheet_record` of class
`import_artifact` renders inside; it does not replace the open-flow
sheet, it is the typed projection of its `import_artifact`-specific
disclosures.

A surface that commits an import without first emitting one
`import_review_record` is non-conforming. A surface that emits the
record but commits before all required axes (§3.2) resolve is
non-conforming. A surface that unpacks or restores opaquely into a
temporary or durable location without a reviewed target and scope
statement is non-conforming.

### 3.1 Required fields

- `record_kind = import_review_record`.
- `import_review_schema_version = 1`.
- `import_review_id` (opaque, stable for the activation).
- `entry_chooser_row_ref` — back-link to the
  `entry_chooser_row_record` that activated the import.
- `open_flow_sheet_ref` — back-link to the parent
  `open_flow_sheet_record` of class `import_artifact`.
- `artifact_class_disclosure` (§3.2.1).
- `version_disclosure` (§3.2.2).
- `inspect_or_write_disclosure` (§3.2.3).
- `extraction_target_disclosure` (§3.2.4).
- `lossy_mapping_disclosure` (§3.2.5).
- `machine_local_exclusion_disclosure` (§3.2.6).
- `cleanup_posture_disclosure` (§3.2.7).
- `raw_action_disclosure` (§3.2.8).
- `handoff_artifact_inspection_ref` — opaque ref to the paired
  `handoff_artifact_inspection_record`. Required on every import
  review; an import review without a paired inspection record is
  non-conforming.
- `post_entry_handoff_card_ref` — opaque ref to the
  `post_entry_handoff_card_record` the activation will emit on
  commit. Required on every import review; an import review
  without a paired post-entry card is non-conforming.
- `trust_unchanged_until_admit` — boolean, MUST be `true`.
- `no_durable_write_before_review` — boolean, MUST be `true`.
- `no_state_rehydration_before_review` — boolean, MUST be `true`.
- `no_portability_claim_before_review` — boolean, MUST be `true`.
- `fallback_actions[]` — at least one typed fallback drawn from
  the open-flow sheet `fallback_action_class` set
  (project_entry_contract.md §5.2). An import review whose only
  affordance is "Apply now" is non-conforming.
- `next_step_decision_hooks[]` — at least one; drawn from the
  entry-restore §1.7 closed set re-exported on
  `/schemas/ux/open_flow_sheet.schema.json`.
- `presentation_label` (redaction-aware, ≤ 1024 graphemes).

Optional fields: `presentation_subtitle`,
`artifact_descriptor_ref` (opaque ref to the upstream
`artifact_descriptor` record the importer minted),
`review_requirements[]` (typed requirements; required when any
disclosure axis cannot be populated — see §3.5).

### 3.2 Required disclosure axes

Every `import_review_record` carries the eight axes below. A surface
that cannot populate an axis MUST deny the activation with a typed
`import_review_requirement` (re-using the open-flow sheet
`policy_restrictions_disclosure.required_reviews` set plus the
import-specific extensions in the schema) instead of defaulting to
a generic "Import anyway" path.

#### 3.2.1 `artifact_class_disclosure`

- `artifact_class` — closed set (schema `$defs.artifact_class`).
  Allowed values: `portable_state_package`, `portable_profile_body`,
  `handoff_packet`, `support_bundle_replay`,
  `issue_attachment_packet`, `competitor_config_root`,
  `template_or_prebuild_snapshot`, `archive_bundle_unscoped`,
  `session_restore_manifest`, `workspace_manifest_bundle`,
  `mixed_state_export_bundle`. A surface that mints a twelfth
  artifact class is non-conforming.
- `artifact_descriptor_ref` — optional opaque ref to the
  upstream descriptor.
- `artifact_family_label` — redaction-aware label. Names the
  artifact family the user sees (e.g. "Aureline portable
  workspace package", "Support bundle from this build").
- `summary` — redaction-aware label.

#### 3.2.2 `version_disclosure`

- `schema_version_class` — closed set (schema
  `$defs.schema_version_class`). Captures the relationship between
  the artifact's declared schema version and the active build.
  `schema_version_unsupported_newer` and
  `schema_version_unsupported_older` deny commit until the user
  accepts a typed `schema_migration_review_required` hook.
- `schema_version_label` — redaction-aware label naming the
  artifact's declared version.
- `producer_continuity_class` — closed set (schema
  `$defs.producer_continuity_class`). Captures the build / vendor
  identity of the producer.
- `producer_label` — redaction-aware label.
- `producer_build_label` — redaction-aware label.
- `signature_state` — re-export of `signature_state` from the
  portable-state-package schema.
- `checksum_state` — re-export of `checksum_state` from the
  portable-state-package schema.
- `summary` — redaction-aware label.

#### 3.2.3 `inspect_or_write_disclosure`

- `inspect_or_write_class` — closed set (schema
  `$defs.inspect_or_write_class`). Distinguishes:
  - `inspect_only_no_write` — no bytes land anywhere; the import
    is a metadata-only review that reads the artifact body
    in-memory or via a process-scoped tmpfs that disappears on
    cancel / exit.
  - `write_to_labelled_inspection_staging` — bytes land in a
    labelled non-durable inspection area visible to the user.
  - `write_to_labelled_extraction_staging` — bytes land in a
    labelled non-durable extraction area; this is the staging
    where extracted contents are reviewable before promotion.
  - `write_to_user_destination_pending_review` — bytes land in
    the user's chosen destination but the destination is not
    yet admitted as a durable workspace.
  - `write_to_durable_workspace_after_review` — bytes land in
    a durable workspace AFTER the user's review admitted it.
    Only allowed at the post-review commit, never as the
    pre-review default.
  - `write_to_active_workspace_after_review` — items apply
    in-place to the active workspace AFTER the user's review
    admitted them.
- `non_durable_label_visible` — boolean. MUST be `true` when
  `inspect_or_write_class` is one of
  `write_to_labelled_inspection_staging`,
  `write_to_labelled_extraction_staging`, or
  `write_to_user_destination_pending_review`. An import review
  that hides the non-durable label behind generic progress copy
  is non-conforming.
- `durable_promotion_requires_review` — boolean. MUST be `true`
  for every class except `inspect_only_no_write`'s opposite
  (durable, in-place admission has already received the review;
  but the schema enforces this on the staging classes by
  pinning the boolean to true).
- `inspection_label` — redaction-aware label. Required when the
  class names a labelled-staging variant.
- `summary` — redaction-aware label.

#### 3.2.4 `extraction_target_disclosure`

- `extraction_target_class` — closed set (schema
  `$defs.extraction_target_class`). Names *where* extracted
  contents land:
  - `no_extraction` — pure metadata review; nothing is extracted.
  - `ephemeral_inspection_only` — bytes are read into
    process-scoped tmpfs / memory and disappear on cancel /
    exit.
  - `labelled_inspection_staging` — bytes land in a labelled
    inspection area on disk (non-durable).
  - `labelled_extraction_staging` — bytes land in a labelled
    extraction staging area on disk (non-durable).
  - `user_chosen_durable_destination` — bytes land in the user's
    durable destination AFTER review.
  - `active_workspace_in_place` — items apply in-place to the
    active workspace AFTER review.
  - `preserved_prior_compare_only` — the importer preserves the
    prior workspace state for comparison only; no new durable
    bytes land.
- `extraction_target_label` — redaction-aware label. Required
  when the class names a labelled-staging or
  user-chosen-durable variant.
- `restore_target_label` — redaction-aware label. Optional;
  used when the artifact is a portable-state package whose
  restore target differs from the extraction target (e.g.
  extraction lands in staging, restore retargets the active
  profile).
- `active_workspace_ref` — opaque ref. Required when the class
  is `active_workspace_in_place`.
- `summary` — redaction-aware label.

#### 3.2.5 `lossy_mapping_disclosure`

- `lossy_mapping_class` — closed set (schema
  `$defs.lossy_mapping_class`). Names whether the import will
  drop or transform items:
  - `no_lossy_mapping` — the import is fully translatable.
  - `lossy_with_review` — the importer has flagged items as
    lossy; the user reviews each.
  - `lossy_dropped_unsupported` — the importer cannot translate
    some items and will drop them; the user reviews the list.
  - `schema_migrated_compatible` — the artifact's schema migrated
    losslessly.
  - `schema_migrated_lossy` — the artifact's schema migrated
    with fidelity loss.
  - `competitor_mapping_partial` — the competitor mapping
    translated some items but not all.
  - `manual_review_required` — the importer cannot decide and
    routes to manual review.
- `lossy_item_count_class` — coarse-grained class
  (`zero | one | few | many | unknown_until_extract`). Exact
  counts cross the boundary as `lossy_item_count_label` only.
- `unsupported_item_review_required` — boolean. MUST be `true`
  whenever `lossy_mapping_class` is one of
  `lossy_with_review`, `lossy_dropped_unsupported`,
  `schema_migrated_lossy`, `competitor_mapping_partial`, or
  `manual_review_required`.
- `summary` — redaction-aware label.

#### 3.2.6 `machine_local_exclusion_disclosure`

- `exclusion_classes[]` — non-empty subset of the closed
  `machine_local_exclusion_class` set. The list MUST include
  `no_exclusions` if and only if no machine-local exclusion
  applies; mixing `no_exclusions` with any other class is
  non-conforming. Allowed classes:
  `secret_material_excluded`, `live_handle_excluded`,
  `machine_unique_handle_excluded`,
  `credential_store_only_excluded`, `absolute_path_excluded`,
  `display_affinity_excluded`, `policy_excludes_artifact`,
  `user_declined_artifact`.
- `exclusion_count_label` — redaction-aware label. Optional
  human-friendly count for the surface.
- `summary` — redaction-aware label.

#### 3.2.7 `cleanup_posture_disclosure`

- `cleanup_posture_class` — closed set (schema
  `$defs.cleanup_posture_class`). Names what the importer will
  clean up if the user cancels, the import fails, or the user
  promotes / discards staging:
  - `no_cleanup_required` — the import is metadata-only.
  - `cleanup_on_cancel` — the importer removes inspection /
    extraction staging on cancel.
  - `cleanup_on_failure` — the importer removes staging on
    failure.
  - `retain_for_review` — the importer keeps staging
    reviewable until the user explicitly clears or promotes.
  - `retain_until_durable_promotion` — the importer keeps
    staging available until the user promotes it to a durable
    destination.
  - `manual_cleanup_required` — the importer cannot clean up
    automatically; the surface MUST disclose the required
    manual step.
  - `rollback_checkpoint_retained` — the importer also
    retains a pre-import rollback checkpoint and pairs with
    the rollback-checkpoint contract.
- `cleanup_label` — redaction-aware label.
- `rollback_checkpoint_ref` — opaque ref. Required when
  `cleanup_posture_class = rollback_checkpoint_retained`.
- `summary` — redaction-aware label.

#### 3.2.8 `raw_action_disclosure`

- `raw_actions_offered[]` — non-empty subset of the closed
  `raw_action_class` set. The set MUST include at least one of
  `cancel_no_change` or `discard_inspection`. Allowed values:
  - `open_raw_inspector` — opens the artifact body as
    inspectable bytes (read-only).
  - `export_inspection_bundle` — emits a sharable redacted
    bundle for support / handoff.
  - `export_redacted_summary` — emits a redacted text
    summary for support tickets.
  - `compare_against_active_workspace` — opens a structured
    diff against the active workspace.
  - `reveal_in_filesystem` — opens the staging area in the OS
    shell so the user can inspect it before deciding.
  - `cancel_no_change` — cancel the import; nothing changes.
  - `promote_to_durable_destination` — the named bridge from
    non-durable staging to a durable workspace; ALWAYS a
    separate reviewed step.
  - `discard_inspection` — discard the inspection staging.
  - `reroute_to_compare_before_restore` — re-route the
    activation through the compare-before-restore flow.
  - `reroute_to_review_migration_report` — re-route through
    the migration-report review.
  - `request_admin_help` — escalate to admin-help review.
- `summary` — redaction-aware label.

### 3.3 The four "no implicit unpack" booleans

Every `import_review_record` carries four pinned-true booleans
that gate the activation against silent side effects:

- `trust_unchanged_until_admit = true` — the active workspace's
  trust state MUST NOT widen as a side effect of import. Trust
  remains owned by the trust-prompt surface.
- `no_durable_write_before_review = true` — no durable bytes
  may land in the user's workspace, profile store, or active
  destination before the user commits the import review.
  Labelled non-durable staging is allowed (and must be
  disclosed); durable promotion is always a separate reviewed
  step.
- `no_state_rehydration_before_review = true` — no state may
  rehydrate (no profile retarget, no settings overwrite, no
  layout restore, no session reopen, no extension restore)
  before the user commits the review.
- `no_portability_claim_before_review = true` — the activation
  MUST NOT claim the artifact has been "imported", "applied",
  or "made portable" before the user commits. Notification,
  history, and support copy MUST disclose the staging label.

A record that emits any of these as `false` is non-conforming.

### 3.4 Temporary inspection / durable promotion / cleanup
disclosure rule

Whenever a handoff artifact is not yet being restored into its
final home, the import review MUST disclose:

1. The temporary inspection location (via
   `extraction_target_disclosure.extraction_target_class +
   extraction_target_label`).
2. The durable-target promotion path (via the
   `promote_to_durable_destination` raw action and the
   `cleanup_posture_class = retain_until_durable_promotion`
   posture).
3. The cleanup behavior (via
   `cleanup_posture_disclosure.cleanup_posture_class`) so the
   user can predict what disappears on cancel, failure, and
   exit.

A surface that hides any of (1)–(3) is non-conforming. A surface
that promotes from staging to a durable destination without first
emitting a fresh `import_review_record` whose
`inspect_or_write_class` resolves to the durable variant is
non-conforming.

### 3.5 Acceptance rules (import-row level)

1. **Import never grants trust.** An import-review row whose
   disclosure axes resolve still does not grant trust; trust
   transitions are owned by the trust-prompt surface and fire
   only after the migration-result review admits per-item
   outcomes. A row that promotes import to trusted as a side
   effect is non-conforming.
2. **Import never writes durably before review.** Durable bytes
   MUST NOT land before the user commits the review.
   Inspect-only and labelled-staging variants land bytes in
   non-durable locations and disclose the staging label.
3. **Import never rehydrates state before review.** Profile
   retarget, settings overwrite, layout restore, session
   reopen, and extension restore MUST be deferred to the
   reviewed apply.
4. **Import never claims portability before review.** A
   notification, history row, or support row that says "Imported"
   before the user reviews is non-conforming. The pre-review
   copy MUST cite the staging label.
5. **Inspect-only and write paths are visibly distinct.** A
   row whose `inspect_or_write_class = inspect_only_no_write`
   MUST NOT share a primary affordance icon / accelerator /
   accent with a write-capable row. Collapsing inspect-only
   and write paths into one "Open" affordance is non-conforming.
6. **Temporary extraction is labelled even under the hood.** A
   surface that uses temporary extraction or staged inspection
   under the hood MUST still resolve `inspect_or_write_class` to
   one of the labelled-staging variants (or
   `inspect_only_no_write` with the ephemeral target) and
   surface the staging label. A surface that hides staged
   inspection behind generic progress copy is non-conforming.
7. **Lossy mapping is disclosed.** A row whose
   `lossy_mapping_class` is anything other than
   `no_lossy_mapping` or `schema_migrated_compatible` MUST
   resolve `unsupported_item_review_required = true` and route
   the user through a typed `review_unsupported_items` or
   `review_migration_report` hook before commit.
8. **Cleanup is predictable.** A row whose
   `cleanup_posture_class = manual_cleanup_required` MUST
   surface the manual step in the import review summary; a
   surface that hides manual cleanup behind a help-link is
   non-conforming.

## 4. Handoff-artifact inspection record

Every import activation emits one
`handoff_artifact_inspection_record` alongside the
`import_review_record`. The inspection record carries the typed
inspection rules each artifact class requires, including redaction
and provenance labels. The record exists so a portable-state
package, an issue / support packet, an archive import, and a
bundle-like handoff object cannot share a single generic "we
inspected the artifact" claim.

A surface that emits an `import_review_record` without a paired
inspection record is non-conforming.

### 4.1 Required fields

- `record_kind = handoff_artifact_inspection_record`.
- `handoff_artifact_inspection_schema_version = 1`.
- `inspection_id`.
- `import_review_ref` — back-link to the parent
  `import_review_record`.
- `handoff_inspection_class` — exactly one of the closed set
  (§4.2).
- `artifact_class` — re-exported from the import-review record
  (§3.2.1).
- `provenance_label_class` — exactly one of the closed set
  (§4.3).
- `redaction_class` — re-exported from
  `/schemas/state/portable_state_package.schema.json`.
- `redaction_posture_class` — closed set (§4.4).
- `signature_state` — re-exported.
- `checksum_state` — re-exported.
- `raw_body_inspectable` — boolean. Named explicitly (rather
  than inferred from the class) so a reviewer can see at a
  glance whether the artifact body can be opened raw.
- `redacted_summary_available` — boolean. Names whether a
  redacted summary is exportable for support / handoff use.
- `summary` — redaction-aware label.

Optional fields: `artifact_descriptor_ref`,
`provenance_label` (redaction-aware label),
`producer_label` (redaction-aware label),
`competitor_mapping_ref` (required when
`handoff_inspection_class = competitor_config_inspection`),
`support_redaction_policy_ref` (required when
`handoff_inspection_class = issue_or_support_packet_inspection`).

### 4.2 `handoff_inspection_class` (closed)

- `portable_state_package_inspection` — for
  `portable_state_package`, `portable_profile_body`,
  `session_restore_manifest`, `workspace_manifest_bundle`, and
  `mixed_state_export_bundle`. The redaction class is whatever
  the package manifest declares; raw-body inspection is
  available behind redaction.
- `issue_or_support_packet_inspection` — for
  `support_bundle_replay` and `issue_attachment_packet`.
  Redaction posture MUST be one of `support_redaction_applied`,
  `redaction_skipped_by_user`, `redaction_skipped_by_policy`,
  or `redaction_partial`. The inspection record MUST carry a
  `support_redaction_policy_ref` so the support / docs author
  can attribute the redaction.
- `archive_bundle_inspection` — for
  `archive_bundle_unscoped` (a generic archive whose contents
  are reviewable but whose schema is not yet known to
  Aureline). Raw-body inspection is available; redaction
  applies only to the surfaced labels, not the archive body.
- `bundle_like_handoff_inspection` — for
  `handoff_packet` and `template_or_prebuild_snapshot`. Pairs
  with a producer-continuity disclosure on the import review.
- `competitor_config_inspection` — for
  `competitor_config_root`. The inspection record MUST carry a
  `competitor_mapping_ref` so the importer's translation
  decisions can be reviewed.
- `template_or_prebuild_inspection` — for
  `template_or_prebuild_snapshot` activations whose pre-review
  rendering needs the template's set-up actions visible
  alongside the import disclosures.

### 4.3 `provenance_label_class` (closed)

- `first_party_signed` — produced by Aureline, signed.
- `first_party_unsigned` — produced by Aureline, unsigned.
- `third_party_signed` — produced by a known third party,
  signed. Pairs with a non-zero `producer_continuity_class`.
- `third_party_unsigned` — produced by a known third party,
  unsigned.
- `support_bundle_redacted` — support / issue packet, redacted.
- `support_bundle_unredacted` — support / issue packet,
  unredacted.
- `competitor_origin_declared` — competitor config with a
  declared origin.
- `anonymous_origin` — origin not declared.
- `provenance_missing` — origin missing entirely; commit MUST
  deny until the user accepts a typed
  `provenance_review_required` hook (re-exported through
  `import_review_requirement`).

### 4.4 `redaction_posture_class` (closed)

- `default_redaction_applied` — the importer applied the
  default redaction policy.
- `support_redaction_applied` — the importer applied the
  support-bundle redaction policy.
- `competitor_redaction_applied` — the importer applied the
  competitor-mapping redaction policy.
- `redaction_skipped_by_user` — the user explicitly skipped
  redaction. The surface MUST disclose this in the import
  review summary.
- `redaction_skipped_by_policy` — policy denies redaction
  (e.g. air-gapped envelopes that cannot reach the redaction
  service).
- `redaction_unavailable` — redaction is not implemented for
  this artifact class.
- `redaction_partial` — redaction was partial; the surface
  MUST list the residual classes.

### 4.5 Acceptance rules (inspection-record level)

1. **Inspection class binds to artifact class.** The
   `handoff_inspection_class` MUST be one of the typed
   profiles in §4.2, and its allowed `artifact_class` set
   matches the §4.2 binding. A record that pairs
   `competitor_config_inspection` with `portable_state_package`
   is non-conforming.
2. **Provenance is always labelled.** A record that emits
   `provenance_label_class = provenance_missing` MUST also
   emit a typed `import_review_requirement` of
   `policy_review_required` or
   `unsupported_item_review_required` upstream. A record
   that hides the missing-provenance case is non-conforming.
3. **Support packets disclose redaction.** A record whose
   `handoff_inspection_class = issue_or_support_packet_inspection`
   MUST carry a `support_redaction_policy_ref` and a
   redaction posture drawn from the support set. A surface
   that imports a support packet without disclosing the
   redaction posture is non-conforming.
4. **Raw body inspectability is explicit.** The
   `raw_body_inspectable` boolean MUST agree with the
   `raw_action_disclosure` on the parent import review: if
   `raw_body_inspectable = true`, the parent's
   `raw_actions_offered[]` MUST include `open_raw_inspector`.
5. **Redacted summaries explicitly available.** The
   `redacted_summary_available` boolean MUST agree with the
   parent's `raw_actions_offered[]`: if `redacted_summary_available
   = true`, the parent MUST offer either
   `export_redacted_summary` or `export_inspection_bundle`.

## 5. Post-entry handoff card

Every import activation emits one `post_entry_handoff_card_record`
AFTER the user activates an import row and BEFORE Aureline
rehydrates state, writes durable bytes into a final home, or
claims the artifact has been ingested. The card answers three
questions in one place:

1. What did Aureline intentionally **not** do yet?
2. What remains blocked or optional?
3. What is the safest next action?

The card preserves the artifact's exact provenance, the machine-
local exclusions the import applied, and the recommended next
steps so a later support ticket, export, or migration retrieval
can pick up where the user left off without paraphrasing the
import.

### 5.1 Required fields

- `record_kind = post_entry_handoff_card_record`.
- `post_entry_handoff_card_schema_version = 1`.
- `card_id`.
- `import_review_ref` — back-link to the parent
  `import_review_record`.
- `handoff_artifact_inspection_ref` — back-link to the parent
  `handoff_artifact_inspection_record`.
- `handoff_card_status_class` — closed set (§5.2).
- `what_aureline_did_not_do_yet` (§5.2.1).
- `what_remains_blocked_or_optional` (§5.2.2).
- `safest_next_action_disclosure` (§5.2.3).
- `preserved_provenance_disclosure` (§5.2.4).
- `preserved_exclusions_disclosure` (§5.2.5).
- `trust_unchanged_until_admit` — boolean, MUST be `true`.
- `no_durable_write_at_handoff` — boolean, MUST be `true`.
- `no_state_rehydration_at_handoff` — boolean, MUST be `true`.
- `no_portability_claim_at_handoff` — boolean, MUST be `true`.
- `later_support_or_export_retrieval_supported` — boolean,
  MUST be `true`. The card is designed to be retrievable for
  later support / export use.
- `fallback_actions[]` — at least one typed fallback drawn
  from `fallback_action_class`.
- `next_step_decision_hooks[]` — at least one drawn from the
  entry-restore §1.7 closed set.
- `presentation_label` (redaction-aware, ≤ 1024 graphemes).
- `summary` — redaction-aware label.

Optional fields: `presentation_subtitle`,
`rollback_checkpoint_ref`, `active_workspace_ref`.

### 5.2 Closed sets

#### `handoff_card_status_class` (closed)

- `review_pending` — the user activated the import; review is
  open but not yet committed.
- `compare_pending` — comparison is pending against the
  active workspace.
- `user_paused_for_later` — the user explicitly paused the
  import. Pairs with a `set_up_later` post-handoff action.
- `import_admitted_partial` — some items were admitted; some
  remain blocked or optional.
- `import_admitted_full` — all items were admitted.
- `import_rolled_back` — the user rolled back the import.
- `import_failed_pending_recovery` — the import failed
  mid-flight; the card surfaces a recovery action.
- `import_blocked_by_policy` — admin / fleet policy denies
  the apply.
- `import_blocked_by_authority` — authority required for the
  apply is missing or expired.
- `import_blocked_by_unsupported_items` — unsupported items
  block the apply.
- `import_discarded_no_change` — the user cancelled; nothing
  changed.

#### 5.2.1 `what_aureline_did_not_do_yet`

The closed `not_yet_done_class` set names side effects the
activation has intentionally NOT performed yet. The card MUST
list every applicable class so the user can see exactly what
was deferred. Allowed values:
`no_durable_write`, `no_state_rehydration`,
`no_portability_claim`, `no_trust_grant`,
`no_dependency_restore`, `no_extension_recommendation`,
`no_hook_or_task_execution`,
`no_active_workspace_replacement`, `no_profile_retarget`,
`no_settings_overwrite`, `no_credential_admission`,
`no_remote_attach`, `no_browser_handshake`,
`no_telemetry_emission_for_admission`.

#### 5.2.2 `what_remains_blocked_or_optional`

The closed `blocked_or_optional_class` set names items that
remain blocked or optional after the post-entry handoff. The
card MUST resolve each remaining concern into one of these
classes; free-form "You may want to do X" copy is
non-conforming. Allowed values:
`trust_review_blocked`, `trust_review_optional`,
`policy_review_blocked`, `policy_review_optional`,
`tenant_review_blocked`,
`migration_review_blocked`, `migration_review_optional`,
`rollback_checkpoint_review_blocked`,
`competitor_mapping_review_blocked`,
`competitor_mapping_review_optional`,
`support_redaction_review_blocked`,
`schema_migration_review_blocked`,
`unsupported_item_review_blocked`,
`unsupported_item_review_optional`,
`destination_choice_blocked`,
`destination_choice_optional`,
`credential_handoff_blocked`,
`credential_handoff_optional`,
`remote_authority_blocked`,
`no_remaining_blockers`.

A card whose only entry is `no_remaining_blockers` MUST still
resolve a typed `safest_next_action_class` (e.g.
`no_action_required` or `keep_imported_state`).

#### 5.2.3 `safest_next_action_disclosure`

The closed `safest_next_action_class` set is the resolved
primary affordance. The post-handoff surface's primary action
MUST resolve to this value. Allowed values:
`review_migration_report`, `compare_before_restore`,
`review_unsupported_items`, `review_trust_and_open`,
`promote_to_durable_destination`, `keep_imported_state`,
`roll_back_import`, `set_up_later`, `open_minimal`,
`inspect_only`, `return_to_start_center`, `request_admin_help`,
`discard_inspection`, `no_action_required`.

The card's `post_handoff_actions_offered[]` is the non-empty
subset of this set the surface offers as fallbacks. A card
whose offered list contains only the primary action (no
fallback) is non-conforming.

#### 5.2.4 `preserved_provenance_disclosure`

The card preserves the exact artifact identity (artifact
class, provenance label class, producer label) so a later
retrieval cannot paraphrase the import. The
`exact_artifact_identity_preserved` boolean MUST be `true`; a
card that emits `false` is non-conforming.

#### 5.2.5 `preserved_exclusions_disclosure`

The card preserves the machine-local exclusions the import
applied, drawn from the same `machine_local_exclusion_class`
set as the import review. The list MUST agree with the
upstream import review's `machine_local_exclusion_disclosure`;
a card that drops an exclusion is non-conforming.

### 5.3 The four "no implicit" booleans

The card carries four pinned-true booleans:

- `trust_unchanged_until_admit = true` — the active
  workspace's trust state has NOT widened as a side effect
  of the import.
- `no_durable_write_at_handoff = true` — the post-entry
  handoff did NOT write durable bytes; durable promotion
  remains a separate reviewed step.
- `no_state_rehydration_at_handoff = true` — no state has
  rehydrated as a side effect of the handoff.
- `no_portability_claim_at_handoff = true` — the card does
  NOT claim the artifact has been "imported" or "applied"
  unless the `handoff_card_status_class` is one of
  `import_admitted_partial` or `import_admitted_full` and
  the parent import review has emitted a corresponding
  reviewed-apply record.

A card that emits any of these as `false` is non-conforming.

### 5.4 Acceptance rules (post-entry handoff-card level)

1. **No silent admission.** The card MUST NOT mark the
   activation `import_admitted_full` unless the user has
   committed the import review and the apply pipeline has
   emitted the migration-result record. A card that flips
   to admitted as a side effect of post-clone behavior is
   non-conforming.
2. **Failed imports route to roll-back.** A card whose
   `handoff_card_status_class = import_failed_pending_recovery`
   MUST include `roll_back_import` in
   `post_handoff_actions_offered[]`.
3. **Paused activations route to set-up-later.** A card
   whose `handoff_card_status_class = user_paused_for_later`
   MUST include `set_up_later` in
   `post_handoff_actions_offered[]`.
4. **Blocked activations route to review.** A card whose
   `handoff_card_status_class` is one of
   `import_blocked_by_policy`,
   `import_blocked_by_authority`, or
   `import_blocked_by_unsupported_items` MUST resolve a
   safest next action that routes the user through the
   typed review (e.g. `review_unsupported_items`,
   `review_migration_report`, `request_admin_help`).
5. **Provenance is preserved verbatim.** The card's
   `preserved_provenance_disclosure.artifact_class` and
   `provenance_label_class` MUST equal the upstream
   inspection record's values. A card that paraphrases or
   collapses the artifact class is non-conforming.
6. **Exclusions are preserved verbatim.** The card's
   `preserved_exclusions_disclosure.exclusion_classes[]` MUST
   equal the upstream import-review's
   `machine_local_exclusion_disclosure.exclusion_classes[]`.
   A card that removes an exclusion class is
   non-conforming.
7. **Later retrieval works.** A support / export retrieval
   that reads the card later MUST be able to reconstruct the
   activation's artifact class, provenance, exclusions,
   safest next action, and what remains blocked / optional
   from the card alone (without re-reading the original
   artifact bytes). A card that points at since-cleared
   staging without preserving the metadata is
   non-conforming.

## 6. Cross-surface invariants

The import review, handoff-artifact inspection, and post-entry
handoff-card records project into every chooser surface that
exposes the `import` verb (Start Center, palette, drag-drop
preview, system-open handoff, deep-link intent review, CLI /
headless preview, support console, migration center,
workspace-switcher). The following invariants keep the projection
sound:

1. **One import-review record per import activation.** Every
   import activation emits exactly one `import_review_record`. A
   surface that commits an import without emitting the record
   is non-conforming.
2. **One inspection record per import activation.** Every
   import activation emits exactly one
   `handoff_artifact_inspection_record`. A surface that emits
   an import review without a paired inspection record is
   non-conforming.
3. **One post-entry handoff-card record per import activation.**
   Every import activation emits exactly one
   `post_entry_handoff_card_record`. A surface that skips this
   record is non-conforming.
4. **Same record across surfaces.** When the same import
   activation can be reached from Start Center, palette,
   drag-drop preview, system-open handoff, deep-link intent
   review, support console, migration center, and CLI /
   headless preview, the records emitted on each surface MUST
   agree on the eight required disclosure axes (§3.2), the
   handoff-inspection class (§4.2), the provenance label class
   (§4.3), the redaction posture (§4.4), and the post-entry
   `handoff_card_status_class` / safest next action.
   Surface-local chrome (icon, accelerator, accent) may differ;
   semantics may not.
5. **CLI / headless render the same axes.** A CLI import
   preview (e.g. `aureline import --preview` /
   `aureline import --dry-run`) MUST render the same disclosure
   axes as the GUI import-review sheet. Collapsing axes (e.g.
   omitting `lossy_mapping_disclosure` because the CLI does
   not render badges) is non-conforming.
6. **Drag-drop is a chooser surface, not a shortcut.** A drop
   that resolves to an import artifact MUST emit one
   `import_review_record`, one
   `handoff_artifact_inspection_record`, and one
   `post_entry_handoff_card_record` BEFORE bytes land. A drop
   that silently materialises bytes bypasses §3.5 and §4.5 and
   is non-conforming.
7. **Deep links route through the import review.** A deep link
   that resolves to an import activation MUST emit an
   `import_review_record` (rendered inside the
   `deep_link_intent_review` open-flow sheet) before any commit;
   it MUST NOT call a private importer.
8. **Staging labels survive surface translation.** When a
   non-durable inspection / extraction staging is rendered in
   the CLI, the notification / progress row, the support
   bundle, or the history view, the staging label MUST remain
   visible. A surface that renders the durable destination as
   the staging label is non-conforming.
9. **Support console preserves redaction posture.** When the
   support console reads a post-entry handoff card later, the
   inspection record's `redaction_posture_class` MUST remain
   the same. A retrieval that downgrades
   `support_redaction_applied` to
   `default_redaction_applied` is non-conforming.

## 7. Fixture corpus

The fixture corpus under
`/fixtures/ux/import_handoff_cases/` contains worked records for
the required import-review, handoff-inspection, and post-entry
handoff-card scenarios:

- `import_review_portable_state_package_inspect_only.yaml` — an
  Aureline portable-state package activation that resolves to
  inspect-only metadata review (no bytes land); inspection class
  `portable_state_package_inspection`; post-entry card status
  `review_pending`.
- `import_review_portable_state_package_extract_then_review.yaml`
  — an Aureline portable-state package activation that extracts
  to labelled extraction staging for review;
  `inspect_or_write_class = write_to_labelled_extraction_staging`;
  post-entry card status `review_pending`; safest next action
  `review_migration_report`.
- `import_review_handoff_packet_compare_before_restore.yaml` — a
  signed handoff packet activation that resolves to compare-
  before-restore; lossy mapping `lossy_with_review`;
  rollback-checkpoint retained; post-entry card status
  `compare_pending`.
- `import_review_support_bundle_replay_redacted.yaml` — a
  support-bundle replay activation with redaction applied;
  inspection class `issue_or_support_packet_inspection`;
  redaction posture `support_redaction_applied`;
  `support_redaction_policy_ref` set; post-entry card status
  `review_pending`; safest next action `inspect_only`.
- `import_review_competitor_config_partial_mapping.yaml` — a
  competitor config import with partial mapping;
  `lossy_mapping_class = competitor_mapping_partial`;
  inspection class `competitor_config_inspection`;
  `competitor_mapping_ref` set; post-entry card status
  `import_blocked_by_unsupported_items`;
  `unsupported_item_review_blocked` is the residual blocker.
- `import_review_archive_bundle_unscoped_inspection.yaml` — an
  archive bundle whose schema is not yet known;
  `inspect_or_write_class = write_to_labelled_inspection_staging`;
  inspection class `archive_bundle_inspection`; post-entry card
  status `review_pending`; safest next action
  `compare_before_restore`.
- `post_entry_handoff_card_user_paused_for_later.yaml` — the
  user paused the import for later; status
  `user_paused_for_later`; safest next action `set_up_later`;
  later support / export retrieval supported.
- `post_entry_handoff_card_import_failed_pending_recovery.yaml` —
  the import failed mid-flight; status
  `import_failed_pending_recovery`; safest next action
  `roll_back_import`; rollback checkpoint retained.

Each fixture is a YAML document validated by the
`/schemas/ux/import_review.schema.json` or
`/schemas/ux/post_entry_handoff_card.schema.json` boundary
schema and includes a `__fixture__` prelude naming the scenario,
the record kind exercised, the disclosure axes covered, and the
contract sections asserted.

## 8. Versioning and change control

The schemas declare
`import_review_schema_version = 1`,
`handoff_artifact_inspection_schema_version = 1`, and
`post_entry_handoff_card_schema_version = 1`. Adding a new
`artifact_class`, `schema_version_class`,
`producer_continuity_class`, `inspect_or_write_class`,
`extraction_target_class`, `lossy_mapping_class`,
`machine_local_exclusion_class`, `cleanup_posture_class`,
`raw_action_class`, `import_review_requirement`,
`handoff_inspection_class`, `provenance_label_class`,
`redaction_posture_class`, `handoff_card_status_class`,
`not_yet_done_class`, `blocked_or_optional_class`, or
`safest_next_action_class` is **additive-minor** and bumps the
corresponding `*_schema_version`. Repurposing an existing value,
weakening a §3.5, §4.5, or §5.4 invariant, or relaxing the four
"no implicit unpack / write / rehydrate / portability claim"
booleans on the import review, the four "no implicit" booleans
on the post-entry handoff card, or the §3.4 temporary-inspection
disclosure rule is **breaking** and requires a new ADR row plus
a coordinated update of the project-entry contract (§5 of
`project_entry_contract.md`), the open-flow sheet schema, the
persistence-inspector contract, and the workspace-entry route
matrix.
