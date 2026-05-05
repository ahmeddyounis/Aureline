# Portable-state package, checksum/redaction manifest, and cross-machine import contract

This document freezes the cross-surface vocabulary every portable-state
export, backup, support handoff, migration, and managed-sync flow uses
when it answers four questions about a portable-state package:

- **what is in this package** — selected profile defaults, workspace
  manifests, saved views, approved docs packs, and the producer
  build/version that emitted the manifest;
- **how is integrity claimed** — checksum, signature, and the
  redaction manifest (path-redaction, host-redaction, secret/credential
  exclusion, machine-local exclusion) describing exactly what crosses
  the machine boundary;
- **what compatibility range does the package admit** — minimum and
  maximum schema version, channel floor, and platform-class allowance;
- **what cross-machine import posture does the destination get** —
  exact, compatible, downgraded, or inspect-only, plus the
  compare/export/inspect/apply actions the destination MAY offer.

The manifest is the **shared inspectable body** that every export and
backup surface emits before sync, handoff, or import features
multiply. It is not a transport, a sync engine, or a restore runtime;
it is the contract their packages MUST conform to so a reviewer,
support engineer, or migration tool can reason about a package
mechanically instead of negotiating parallel field names.

The machine-readable schema lives at:

- [`/schemas/state/portable_state_manifest.schema.json`](../../schemas/state/portable_state_manifest.schema.json)

Worked fixtures live under:

- [`/fixtures/state/portable_state_packages/`](../../fixtures/state/portable_state_packages/)

This contract composes with:

- [`/docs/state/profile_and_state_map.md`](./profile_and_state_map.md)
- [`/docs/state/migration_and_restore_playbook.md`](./migration_and_restore_playbook.md)
- [`/docs/state/durable_state_compatibility_contract.md`](./durable_state_compatibility_contract.md)
- [`/docs/state/restore_artifact_family_contract.md`](./restore_artifact_family_contract.md)
- [`/docs/state/workspace_memory_contract.md`](./workspace_memory_contract.md)
- [`/docs/ux/persistence_inspector_contract.md`](../ux/persistence_inspector_contract.md)
- [`/docs/governance/data_portability_and_exit_matrix.md`](../governance/data_portability_and_exit_matrix.md)

The manifest body sits **below** the persistence-inspector and
export-sheet review surfaces (which validate against
[`/schemas/state/portable_state_package.schema.json`](../../schemas/state/portable_state_package.schema.json))
and **above** the per-section schemas (portable profile, workspace
manifest bundle, restore-provenance record). Every selected section in
this manifest is referenced by opaque ref; the per-section schemas
remain authoritative for their own bodies.

This contract is normative. Where it disagrees with the PRD, TAD, TDD,
UI/UX spec, the data-portability matrix, or one of the upstream state
contracts, those documents win and this contract plus the manifest
schema MUST be updated in the same change. Where a downstream
sync, backup, support-export, migration, or import surface mints a
parallel package-content, redaction, compatibility, or import-posture
vocabulary, this contract wins and the surface is non-conforming.

## Why freeze this now

Portable-state drift starts when each new flow — profile sync, layout
export, support handoff, migration center, customer-managed backup —
defines its own package format. Without one frozen manifest:

- a sync engine and a support exporter both call their package "the
  bundle" while emitting incompatible content lists;
- a redaction review surface trusts a checksum the producer never
  signed and a signature the destination never verifies;
- a migration tool silently downgrades workspace truth because no
  field admits the difference between "applied through an equivalence
  map" and "intentionally omitted on this destination";
- an air-gap import opens a package whose compatibility range is
  unknowable, then guesses;
- raw absolute paths or raw hostnames smuggle across the machine
  boundary because the redaction manifest is implicit; and
- the destination cannot tell whether it just received a package meant
  for live apply, an inspection-only support packet, or a
  rollback-compare attachment.

The manifest forecloses these patterns by treating package content,
integrity, redaction, compatibility, and import posture as five
distinct contracts inside one frozen record. Once the boundary is
named, every package becomes diffable, reviewable, and explicit about
what crosses the machine boundary.

## Scope

- Freeze one `portable_state_manifest_record` shape that names
  selected sections, their state plane, portability, redaction class,
  schema version, last-write time, size estimate, checksum, and
  signature.
- Freeze the closed `section_class` vocabulary covering selected
  profile defaults, workspace manifests, saved views, approved docs
  packs, and the adjacent portable-state classes that already appear
  in the profile-and-state map.
- Freeze the `redaction_manifest` record so path-redaction,
  host-redaction, raw-secret exclusion, raw-credential exclusion,
  raw-command-line exclusion, raw-log exclusion, raw-source-content
  exclusion, and machine-local exclusion are reviewable in one place
  before export.
- Freeze the `compatibility_range` record so a destination can decide
  whether the package falls inside its admitted schema-version,
  channel, and platform-class window.
- Freeze the `import_posture_record` and the closed
  `import_posture_class` set: `exact`, `compatible`, `downgraded`,
  `inspect_only`. Every package admits exactly one posture before the
  destination decides what to do.
- Freeze the `manifest_action_id` set covering inspect, compare
  (against local or against a prior export), re-export (redacted or
  with a machine addendum), open inspect-only, apply import, and
  cancel import. Free-form actions are non-conforming.

## Out of scope

- The persistence engine, sync execution, backup transport, package
  upload, or restore runtime. The vocabulary freeze lands here;
  production surfaces compose over it later.
- Final UI copy. Display copy may render `Exact import`,
  `Compatible import`, `Downgraded import`, and `Inspect only`; the
  closed machine set is fixed.
- Repurposing of fields in the upstream profile-and-state map,
  migration-and-restore playbook, restore-artifact-family, or
  persistence-inspector contracts. This packet adds the package-level
  manifest; it does not edit the per-section bodies or the inspector
  surface contract.
- The ADR-0007 secret broker storage backend or the ADR-0010 connected
  provider/browser-handoff approval-ticket flow. The manifest references
  those bodies by opaque id only; raw bytes never appear here.

## 1. Manifest body shape

Every portable-state package MUST emit exactly one
`portable_state_manifest_record`. The record carries:

| Field group | Meaning |
|---|---|
| `manifest_id`, `manifest_label`, `created_at` | identity and producer-local timestamp; opaque to this schema |
| `package_purpose` | re-exports the persistence-inspector vocabulary: `user_portable_export`, `workspace_layout_export`, `restore_compare_export`, `support_review_export`, `migration_handoff_export` |
| `destination_class` | re-exports the persistence-inspector vocabulary: `local_file`, `workspace_artifact`, `support_bundle_attachment`, `managed_sync_service`, `customer_managed_storage`, `air_gapped_transfer` |
| `producer_build` | producer name, version, channel, platform class, instance handle. Pseudonymous; never a raw hostname |
| `compatibility_range` | minimum and maximum schema version, channel floor, platform-class allowance, optional named-class set, compatibility notes |
| `sections[]` | one row per selected section the manifest carries; see §2 |
| `redaction_manifest` | path-redaction, host-redaction, raw-secret/credential/command-line/log/source-content exclusion, machine-local exclusion status, redaction rules; see §4 |
| `machine_local_exclusions[]` | one row per machine-local artifact intentionally left out, naming the reason and the substitute class; see §4 |
| `total_size_estimate` | bytes plus precision class (`exact_preflight`, `estimated_preflight`, `bounded_upper`, `unknown_until_build`) |
| `checksum`, `signature` | integrity summaries; see §3 |
| `import_posture` | one of `exact`, `compatible`, `downgraded`, `inspect_only`, with the typed downgrade triggers, equivalence-map refs, rollback-checkpoint ref, preserved prior-artifact refs, and review-required section refs; see §5 |
| `restore_provenance_refs[]` | opaque refs to the cross-artifact `state_restore_provenance_record` bodies the import is expected to emit on apply |
| `actions` | the closed manifest action set (§6) |
| `notes` | redaction-aware free text |

Rules (frozen):

1. The manifest body MUST NOT carry raw secrets, raw tokens, raw
   provider payloads, raw absolute paths, raw hostnames, raw command
   lines, raw logs, or raw source content. Every reference is an opaque
   id resolvable through the producer's bundle table; bodies live in
   the per-section schemas.
2. `package_purpose` and `destination_class` are closed re-exports of
   the persistence-inspector vocabulary. A surface that mints
   `aux_export`, `quick_share`, or another parallel value is
   non-conforming.
3. The manifest is the **only** place inside the package that may
   declare integrity, redaction, compatibility, or import-posture
   claims for the package as a whole. A per-section body that claims
   "this section is signed" without quoting `manifest_id` is
   non-conforming.
4. `notes` is reviewable prose **after** the active redaction policy
   has already been applied; producers do not embed raw paths or hosts
   even inside the notes field.

## 2. Selected sections

The manifest's `sections[]` array enumerates exactly what the package
carries. Each row resolves to one closed `section_class` value and
carries enough metadata for the destination to decide whether to
apply, compare, or omit that section.

### 2.1 Section classes

The closed set is fixed:

| Section class | Origin | Default state plane | Default portability | Notes |
|---|---|---|---|---|
| `selected_profile_defaults` | the user profile's selected defaults snapshot | `portable_settings` | `portable` | the user-curated subset of profile defaults the export selected; never the whole profile body alone |
| `portable_profile_body` | a portable profile artifact | `portable_settings` | `portable` | composes with the portable-profile schema; referenced by ref |
| `portable_profile_machine_addendum` | the machine-bound addendum | `local_context` | `portable_with_machine_addendum` | only travels when destination admits machine-local addendum |
| `workspace_manifest` | a workspace `aureline.workspace.jsonc` | `workspace_shared_manifest` | `shared_workspace_only` | reviewed via diff/apply; never silently overwritten |
| `workset_manifest` | a workset under `.aureline/worksets/*` | `workspace_shared_manifest` | `shared_workspace_only` | as above |
| `tasks_and_launch_configs` | `.aureline/tasks.jsonc`, `.aureline/launch.jsonc` | `workspace_shared_manifest` | `shared_workspace_only` | as above |
| `extension_selection_inventory` | `extensions.selected.jsonc` | `portable_settings` | `portable` | extension lockfile rides separately as `extension_lockfile` |
| `extension_lockfile` | `.aureline/extensions.lock.json` | `workspace_shared_manifest` | `shared_workspace_only` | only travels with `workspace_layout_export` or `migration_handoff_export` |
| `extension_recommendations` | `.aureline/extensions.recommend.jsonc` | `workspace_shared_manifest` | `shared_workspace_only` | optional |
| `saved_view` | a saved view, search query, or filter set | `portable_settings` | `portable` | saved views compose over saved-query and collection-view contracts |
| `approved_docs_pack` | an approved docs pack reference | `portable_settings` | `portable` | references the docs-pack manifest; never carries doc bodies |
| `portable_layout_preset` | a layout preset under `layout_presets/*` | `portable_settings` | `portable` | window-topology-snapshot bodies travel via the restore-artifact family, not here |
| `ai_preset_selection_refs` | `ai/presets.jsonc` selection | `portable_settings` | `portable` | redaction class is at minimum `redact_value_preserve_shape` |
| `terminal_preferences` | `terminal/preferences.jsonc` | `portable_settings` | `portable` | preferences only; transcripts and restore metadata stay local-only |
| `snippets` | `snippets/*` | `portable_settings` | `portable` | redact_to ui-string-only |
| `themes_and_design_tokens` | `themes/*` | `portable_settings` | `portable` | |
| `keybindings` | `keybindings.jsonc` | `portable_settings` | `portable` | |
| `command_aliases` | `aliases.jsonc` | `portable_settings` | `portable` | |
| `profile_library_index` | `profiles/*.aureprofile.json` index | `portable_settings` | `portable` | the library index travels; individual profile bodies travel as `portable_profile_body` rows |
| `support_evidence_index` | a support evidence index | `local_context` | `local_only` | only admitted in `support_review_export` packages; the body stays local-only |
| `restore_provenance_refs` | refs to prior `state_restore_provenance_record` bodies | `portable_settings` or `local_context` | varies | informational; never live authority |

Rules (frozen):

1. The `section_class` set is closed. Adding a new section class is
   additive-minor (bump `portable_state_manifest_schema_version`) and
   MUST resolve to one row in the profile-and-state map or one row in
   an existing companion contract.
2. A section MUST resolve to exactly one `state_plane` from the
   migration-and-restore playbook (`portable_settings`,
   `local_context`, `workspace_shared_manifest`,
   `non_portable_live_authority`). A row that claims
   `non_portable_live_authority` is non-conforming inside this
   manifest; live authority is excluded by design and represented by
   reference only through `machine_local_exclusions[]` or
   `restore_provenance_refs[]`.
3. A `support_evidence_index` row MUST appear only in packages whose
   `package_purpose` is `support_review_export`. The destination class
   for those packages is restricted to `support_bundle_attachment` or
   `customer_managed_storage`; managed-sync and air-gapped destinations
   reject the row.
4. An `approved_docs_pack` row carries the docs-pack manifest ref and
   the freshness/integrity-state vocabulary the docs-pack contract
   defines. The package never inlines doc bodies or rendered docs.
5. A `saved_view` row carries the saved-view ref, the saved-query ref
   it composes over, and the redaction class. Saved-view bodies stay
   in their own schema; the manifest only quotes the ref.

### 2.2 Per-section row fields

Each `section_row` carries:

- `section_id`, `section_class`, `section_ref` — opaque ids;
- `state_plane`, `portability_class`, `redaction_class` — closed
  vocabularies re-exported from the migration-and-restore playbook
  and profile-and-state map;
- `schema_version` — the per-section schema version string the
  producer emitted;
- `last_written_at` — producer-local monotonic timestamp;
- `fidelity_label` — the producer's claim about the section
  (`exact`, `compatible`, `layout_only`, `manual_review`);
- `size_estimate`, `checksum`, `signature` — per-row integrity
  summaries;
- `source_refs[]` — opaque refs that resolved this section (for
  example, the workspace-authority checkpoint a saved view depends on
  for stable pane ids);
- `notes` — reviewable prose after redaction.

Rules (frozen):

1. A row's `redaction_class` is the floor enforced for that section.
   The package-level `redaction_manifest` MAY narrow further (for
   example, redact a `themes_and_design_tokens` row from `none` to
   `ui_string_only` for a support export) but MUST NOT widen.
2. A row whose `portability_class` is `excluded` is non-conforming
   inside `sections[]`. Excluded artifacts move to
   `machine_local_exclusions[]` so reviewers see them in one place.
3. A row's `fidelity_label` is the producer's claim. The destination's
   apply-time decision is recorded in the import-posture record (§5)
   plus the cross-artifact restore-provenance record. A row that
   silently upgrades fidelity at apply time is non-conforming.

## 3. Checksum and signature

The package manifest MUST carry one `checksum_summary` and one
`signature_summary` for the package as a whole, plus optional per-row
summaries for individual sections.

### 3.1 Checksum

`checksum_summary` fields:

- `state` — closed enum: `checksum_available`, `checksum_pending`,
  `not_applicable`, `omitted_by_policy`, `unavailable_preflight`;
- `digest_ref` — opaque ref to the digest body when available;
- `algorithm_hint` — one of `sha256`, `blake3`, `multihash`, or
  `null`.

Rules (frozen):

1. `checksum_available` requires a non-null `digest_ref`. A package
   that claims `checksum_available` with a null digest ref is
   non-conforming.
2. `checksum_pending` is admitted only during preflight (export-sheet
   review) and MUST resolve to `checksum_available` before
   `confirm_export` enables. A package emitted with `checksum_pending`
   and no path to `checksum_available` is non-conforming.
3. `not_applicable` is admitted only when `destination_class` is
   `workspace_artifact` and the workspace's own integrity story
   covers the row.
4. `omitted_by_policy` MUST cite a policy ref under `notes` so
   reviewers can audit the omission.
5. `unavailable_preflight` is admitted on the export sheet only; it
   MUST resolve before the manifest leaves the producer.

### 3.2 Signature

`signature_summary` fields:

- `state` — closed enum: `signed_verified`, `signed_unverified`,
  `unsigned`, `signature_missing`, `signature_mismatch`,
  `not_applicable`, `pending`;
- `signer_ref`, `signature_ref` — opaque refs.

Rules (frozen):

1. `unsigned` is admitted for `local_file` plain exports. The
   manifest MUST NOT imply verified provenance when `state` is
   `unsigned`.
2. `signed_unverified` is admitted on import preflight; the
   destination MUST NOT advance to `apply_import` until verification
   resolves to `signed_verified` or the user explicitly waives
   verification with a typed `signature_review_waiver` note.
3. `signature_mismatch` blocks apply at the import surface and routes
   to compare/inspect actions only.
4. `not_applicable` requires `destination_class` to be
   `workspace_artifact` whose workspace authority signs the artifact
   under its own contract.

## 4. Redaction manifest and machine-local exclusion

The redaction manifest is the package's reviewable answer to "what
crossed the machine boundary." Every package — including unsigned
local plain exports — MUST emit a `redaction_manifest`.

### 4.1 Redaction manifest fields

| Field | Closed enum | Floor for portable packages |
|---|---|---|
| `path_redaction` | `exclude_raw_paths`, `redact_to_root_handle`, `redact_to_class_label`, `preserve_for_admin_only` | `exclude_raw_paths` |
| `host_redaction` | `exclude_raw_hosts`, `redact_to_class_label`, `redact_to_remote_target_handle`, `preserve_for_admin_only` | `exclude_raw_hosts` |
| `raw_secret_exclusion_state` | `enforced`, `advisory`, `disabled` | `enforced` |
| `raw_credential_exclusion_state` | `enforced`, `advisory`, `disabled` | `enforced` |
| `raw_command_line_exclusion_state` | `enforced`, `advisory`, `disabled` | `enforced` |
| `raw_log_exclusion_state` | `enforced`, `advisory`, `disabled` | `enforced` (advisory admitted only for `support_review_export`) |
| `raw_source_content_exclusion_state` | `enforced`, `advisory`, `disabled` | `enforced` |
| `raw_provider_payload_exclusion_state` | `enforced`, `advisory`, `disabled` | `enforced` |
| `machine_local_exclusion_status` | `none`, `partial_metadata_only`, `excluded`, `placeholder_only` | `excluded` for any row whose section is `local_only` |
| `rules[]` | one row per `redaction_rule` | additive |

Rules (frozen):

1. Every `*_exclusion_state` field set to `disabled` is non-conforming
   for any `package_purpose` other than `support_review_export`, and
   even there a `disabled` value MUST be paired with an explicit
   admin-policy ref in `notes` and a destination class in
   `support_bundle_attachment` or `customer_managed_storage`.
2. `path_redaction = preserve_for_admin_only` is non-conforming for
   any `destination_class` other than `support_bundle_attachment` or
   `customer_managed_storage`. Managed-sync and air-gap destinations
   never see preserved raw paths.
3. `host_redaction = preserve_for_admin_only` follows the same rule;
   unrestricted raw hosts never cross the boundary.
4. The `rules[]` array carries reviewable prose tying each rule to
   the `section_class` it scopes (or `all_sections`). Free-form
   rule labels are non-conforming.

### 4.2 Redaction-rule rows

Each `redaction_rule_row` names:

- `rule_id` — opaque id;
- `rule_class` — closed enum (`raw_path_excluded`, `raw_host_excluded`,
  `raw_secret_excluded`, `raw_credential_excluded`,
  `raw_command_line_excluded`, `raw_log_excluded`,
  `raw_provider_payload_excluded`, `raw_source_content_excluded`,
  `raw_terminal_scrollback_excluded`, `machine_unique_handle_excluded`,
  `live_handle_excluded`);
- `enforcement_state` — `enforced`, `advisory`, `disabled`;
- `scope_class` — a `section_class` value or `all_sections`;
- `redaction_class` — re-exported from the profile-and-state map
  (`none`, `ui_string_only`, `redact_value_preserve_shape`,
  `redact_to_class_label`, `exclude_from_export`);
- `reason` — redaction-aware prose describing why the rule applies.

### 4.3 Machine-local exclusion rows

`machine_local_exclusions[]` enumerates artifacts the producer left
out because they cannot ride as portable truth. Each row carries:

- `exclusion_id`, `artifact_class` (re-exported from the
  persistence-inspector artifact-class set), `artifact_ref`;
- `state_plane`;
- `reason` — one of the closed
  `machine_local_exclusion_reason` values from the persistence
  inspector contract (e.g., `contains_secret_material`,
  `contains_live_handle`, `machine_unique_handle`,
  `credential_store_only`, `local_absolute_path`,
  `display_hint_best_effort_only`, `workspace_authority_owned_elsewhere`,
  `admin_policy_ownership`, `disposable_derived`,
  `policy_excludes_export`, `user_declined_export`,
  `unsupported_destination`);
- `substitute_class` — the closed
  `exclusion_substitute_class` value (`opaque_ref`,
  `redacted_summary`, `hash_or_count`, `source_and_freshness_label`,
  `safe_placeholder`, `metadata_only`, `omitted`);
- `redaction_class`;
- `note` — redaction-aware prose.

Rules (frozen):

1. Every machine-local artifact whose presence would change the
   destination's import posture (for example, a missing trust
   approval, a missing credential handle, or a display-affinity
   hint that affects reflow) MUST appear here. A package that elides
   the row to avoid extending the redaction manifest is
   non-conforming.
2. The `reason` and `substitute_class` vocabularies are closed
   re-exports of the persistence-inspector schema. Free-form reasons
   are non-conforming.
3. The combined inventory across `sections[]` and
   `machine_local_exclusions[]` MUST cover every artifact class the
   destination expects to see for the declared `package_purpose`.
   Silent omission is non-conforming.

## 5. Cross-machine import posture

The import-posture record is the package's pre-declared answer to
"what will the destination get if it imports this package?" Every
package admits exactly one posture before the destination decides
whether to apply, compare, or inspect.

### 5.1 Posture classes

| Display label | Machine enum | Meaning | Required handles |
|---|---|---|---|
| `Exact import` | `exact` | every selected section round-tripped without schema translation, downgrade trigger, missing dependency, manifest conflict, or review requirement | `downgrade_triggers[]` empty; `equivalence_map_refs[]` empty; `review_required_section_refs[]` empty; `rollback_checkpoint_ref` MAY be null |
| `Compatible import` | `compatible` | one or more sections translated through declared equivalence maps without blocking review; a rollback checkpoint exists before apply | `equivalence_map_refs[]` non-empty; `rollback_checkpoint_ref` non-null; `preserved_prior_artifact_refs[]` non-empty when meaning changed |
| `Downgraded import` | `downgraded` | one or more sections were excluded, narrowed, or replaced by placeholder/evidence on the destination because of missing dependencies, platform-class mismatch, channel-floor violation, or destination-unsupported sections; the destination still applies what survives | `downgrade_triggers[]` non-empty; per-affected section pairs with a `machine_local_exclusion` or `placeholder_only` row |
| `Inspect only` | `inspect_only` | the destination MUST NOT apply the package; only inspect, compare, and re-export actions are admitted | `apply_import` action is disabled with a typed disabled reason; `compare` and `open_inspect_only` enabled |

Rules (frozen):

1. The label set is closed. A surface that mints `partial`,
   `best_effort`, `safe_default`, or another parallel label is
   non-conforming.
2. `exact` is forbidden once any section needed equivalence
   translation, missing-dependency placeholder fallback, manual
   review, or any redaction class above its row's row-level floor.
3. `compatible` MUST carry `equivalence_map_refs[]` and
   `rollback_checkpoint_ref`. A package that claims `compatible`
   without those handles is non-conforming.
4. `downgraded` MUST list every typed `downgrade_trigger` that
   narrowed the result. The trigger enum is closed and shared with
   the migration-and-restore playbook plus this contract:
   - `schema_translation_required`
   - `schema_meaning_changed`
   - `missing_extension_dependency`
   - `missing_remote_session`
   - `missing_remote_authority`
   - `unsupported_display_topology`
   - `excluded_secret_material`
   - `excluded_live_handle`
   - `workspace_manifest_conflict`
   - `policy_narrowing`
   - `manual_repair_required`
   - `producer_schema_downgrade_refused`
   - `platform_class_mismatch`
   - `channel_floor_violation`
   - `compatibility_range_outside`
   - `destination_unsupported_section`
5. `inspect_only` is mandatory when `package_purpose` is
   `restore_compare_export` or `support_review_export` and the
   destination is not the original producing installation. A
   support-review export that admits `apply_import` outside its
   producing installation is non-conforming.
6. The `import_posture` record MAY narrow but MUST NOT widen the
   producer's row-level `fidelity_label` claims. A package whose
   posture is `exact` while a contained section row admits
   `compatible` or lower is non-conforming.

### 5.2 Compatibility range

`compatibility_range` is the destination's gate. Fields:

- `min_schema_version`, `max_schema_version` — opaque schema-version
  strings; the destination compares its own per-section schema
  version against this range before deciding posture;
- `channel_floor` — one of `experimental`, `beta`, `stable`, `lts`,
  or null. A destination on a channel below the floor downgrades to
  `compatible`, `downgraded`, or `inspect_only`;
- `platform_class_allowance` — closed enum: `any`,
  `machine_local_only` (only the producing machine may apply),
  `named_classes_only` (use `named_platform_classes`);
- `named_platform_classes[]` — subset of `macos`, `windows`,
  `linux`, `container`, `remote_agent`, `managed_cloud`, `other`;
- `compatibility_notes` — redaction-aware prose.

Rules (frozen):

1. A destination outside the declared range MUST set posture to
   `inspect_only` unless an admin-policy override is recorded in
   `notes` plus a typed `policy_narrowing` downgrade trigger.
2. `platform_class_allowance = machine_local_only` is admitted only
   for `package_purpose = restore_compare_export` and is the only
   case where `inspect_only` is the inevitable destination posture
   off the producing machine.

## 6. Compare and export actions

Every manifest carries a closed `actions` block. Each action row
follows the `manifest_action_record` shape: `action_id`, `enabled`,
`disabled_reason`, `consequence_class`, optional `target_ref`.

### 6.1 Action vocabulary

| Action id | Consequence class | When enabled |
|---|---|---|
| `inspect` | `read_only` | always |
| `compare` | `read_only` | when at least one prior export, local artifact, or preserved-prior-artifact ref is reachable |
| `compare_with_local` | `read_only` | when the destination installation has an existing artifact for at least one selected section |
| `compare_with_prior_export` | `read_only` | when at least one prior export ref is recorded in `notes` or in `restore_provenance_refs[]` |
| `reexport_redacted` | `builds_export` | when the producer admits re-export with a stricter redaction class than the manifest currently declares |
| `reexport_with_machine_addendum` | `builds_export` | when the package carries a `portable_profile_machine_addendum` row or one is reachable on the producing machine |
| `open_inspect_only` | `read_only` | always; opens a non-mutating inspector view of the manifest |
| `apply_import` | `requires_preview` or `requires_rollback` | when `import_posture.posture_class` is `exact`, `compatible`, or `downgraded` and the destination admits the package's `destination_class`; disabled with a typed reason for `inspect_only` |
| `cancel_import` | `cancels_without_mutation` | always |

Rules (frozen):

1. The action set is closed. A surface that mints `force_apply`,
   `merge_with_skip`, or another parallel action is non-conforming.
2. `apply_import` MUST carry a typed `disabled_reason` from the
   shared `disabled_reason_class` set when disabled
   (`policy_denied`, `requires_preview`, `requires_rollback_checkpoint`,
   `live_authority_excluded`, `package_still_building`,
   `signature_mismatch`, `compatibility_range_outside`,
   `inspect_only_posture`). Free-form reasons are non-conforming.
3. `cancel_import` MUST leave the package and the destination
   workspace untouched. A cancel action that triggers a rollback
   checkpoint or rewrites prior provenance is non-conforming.
4. `reexport_redacted` and `reexport_with_machine_addendum` produce a
   new manifest with its own `manifest_id`. Re-exporting under the
   same id is non-conforming because integrity claims would be
   ambiguous.

## 7. Conformance checklist

A portable-state package conforms when its manifest can answer:

- Which `manifest_id` and `package_purpose` declare this package?
- Which `producer_build`, `compatibility_range`, and
  `destination_class` admit the destination?
- Which selected sections are inside, with what state plane,
  portability class, redaction class, schema version, and
  per-row fidelity label?
- Which `redaction_manifest` rules and floors apply, and which
  machine-local artifacts are intentionally excluded with what
  `reason` and `substitute_class`?
- What `checksum` and `signature` summaries cover the package, and
  what is the policy ref when either is omitted?
- Which `import_posture` does the package declare, and which typed
  `downgrade_triggers[]` narrowed it?
- Which `equivalence_map_refs[]`, `rollback_checkpoint_ref`,
  `preserved_prior_artifact_refs[]`, and
  `review_required_section_refs[]` carry the apply-time story?
- Which closed actions are enabled, and what typed
  `disabled_reason` covers each disabled action?
- Which `restore_provenance_refs[]` will the apply-time flow update?

If any answer requires new vocabulary, this contract and the
companion schema are extended first, in the same change as the new
fixture under
[`/fixtures/state/portable_state_packages/`](../../fixtures/state/portable_state_packages/).

## 8. Changing this vocabulary

- **Additive-minor** changes (a new `section_class`, a new
  `redaction_rule_class`, a new `downgrade_trigger`, a new
  `manifest_action_id`, a new `machine_local_exclusion_reason` value
  inherited from upstream, a new `destination_class`) land here and
  in the manifest schema in the same change. The change MUST cite a
  motivating fixture under
  [`/fixtures/state/portable_state_packages/`](../../fixtures/state/portable_state_packages/)
  and a corresponding row in the data-portability matrix when the
  artifact class is new.
- **Repurposing** an existing `import_posture_class`,
  `section_class`, `redaction_rule_class`, `downgrade_trigger`, or
  action id is breaking and requires a governance decision row.
- The redaction floors above are minimums. A package MAY narrow the
  redaction further; widening below the floor (for example, allowing
  raw absolute paths in a managed-sync export) is non-conforming.

## 9. Reference rows

- Profile-and-state map — portable-profile artifact, state-class
  rows, portability classes, redaction classes the manifest
  re-exports.
- Migration-and-restore playbook — fidelity labels, downgrade
  triggers, preserved-prior-artifact rules the import posture quotes.
- Restore-artifact-family contract — the workspace-authority
  checkpoint and window-topology snapshot bodies the manifest
  references via `source_refs[]` (never inline).
- Workspace-memory contract — the excluded-live-authority floor the
  redaction manifest re-exports verbatim.
- Persistence-inspector contract — the artifact-class, action,
  destination-class, and machine-local-exclusion vocabularies the
  manifest reuses.
- Data-portability and exit matrix — the per-domain export
  posture, scope, deletion, and offboarding rows every concrete
  artifact class resolves through.
