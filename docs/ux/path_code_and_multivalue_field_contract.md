# Path/file field, code-backed field, and multi-value chip field contract

This document freezes the shared contract for three field shapes that
high-impact forms must not collapse into generic text inputs:

1. **Path / file / folder fields** — the user is naming a filesystem,
   archive, remote, container, or provider-owned object whose meaning
   changes with workspace root, current file, selected target, remote
   host, or local-machine basis.
2. **Code-backed fields** — the user is editing a structured fragment
   (arbitrary code, JSON, JSONC, YAML, TOML, regex, glob, SQL, HCL,
   structured settings fragment) whose syntax, schema hints, comments,
   unknown fields, diff/preview, and lossy-normalization behavior
   matter.
3. **Multi-value list / chip fields** — the user is editing a list of
   values whose add/remove/reorder semantics, duplicate handling,
   overflow visibility, bulk-paste behavior, and export/serialization
   truth must be visible and reviewable.

The contract is normative. Where this document disagrees with the
source UI / UX, filesystem-identity, security, transport, request,
package, repair, or settings contracts it cites, the source contract
wins and this document, its three schemas, and its fixtures update in
the same change. Where this document disagrees with a downstream
surface's private widget behavior, this document wins and the surface
is non-conforming.

This contract composes with, and does not replace:

- [`/docs/ux/field_rules_contract.md`](./field_rules_contract.md)
  and [`/schemas/ux/field_rule.schema.json`](../../schemas/ux/field_rule.schema.json)
  for the high-risk field-rule families (`filesystem_path_ref`,
  `code_backed_expression`, multi-value/key-value `field_display_class`)
  that these three shapes ride on top of. Every record under this
  contract names the field-rule id it composes with so a single source
  of truth governs redaction, copy/export posture, validation hooks,
  evaluation context, and unsafe-value warnings.
- [`/docs/ux/field_row_and_value_source_contract.md`](./field_row_and_value_source_contract.md)
  and [`/schemas/ux/field_row.schema.json`](../../schemas/ux/field_row.schema.json)
  for label, source pill, effective-value inspector, exact-row deep
  links, search highlighting, and apply posture. The records here
  occupy the path-basis, structured-preview, and multi-value extension
  slots that the row contract reserved.
- [`/docs/ux/forms_validation_contract.md`](./forms_validation_contract.md)
  for validation classes, staged review, probe freshness, mutation
  blocking, and stale/skipped admission.
- [`/docs/fs/path_truth_packet.md`](../fs/path_truth_packet.md),
  [`/docs/filesystem/filesystem_identity_vocabulary.md`](../filesystem/filesystem_identity_vocabulary.md),
  and [`/schemas/filesystem/save_target_token.schema.json`](../../schemas/filesystem/save_target_token.schema.json)
  for the path-truth chip, alias inspector, and canonical identity
  vocabulary that path fields cite when an alias, symlink, archive
  overlay, or managed-provider drift is detected.

Companion artifacts:

- [`/schemas/ux/path_field.schema.json`](../../schemas/ux/path_field.schema.json)
  — boundary schema for one `path_field_record`. Covers basis,
  picker/drag-drop compatibility, target-context resolution,
  missing-target warnings, and normalization disclosure.
- [`/schemas/ux/code_backed_field.schema.json`](../../schemas/ux/code_backed_field.schema.json)
  — boundary schema for one `code_backed_field_record`. Covers
  language/format, syntax-highlighting state, schema-hint state,
  unknown-field and comment preservation posture, diff/preview
  surface, and lossy-normalization disclosure.
- [`/schemas/ux/multivalue_chip_field.schema.json`](../../schemas/ux/multivalue_chip_field.schema.json)
  — boundary schema for one `multivalue_chip_field_record`. Covers
  add/remove/reorder semantics, duplicate handling, overflow
  visibility, bulk-paste behavior, and export/serialization truth.
- [`/fixtures/ux/path_code_multivalue_cases/`](../../fixtures/ux/path_code_multivalue_cases/)
  — worked YAML records for the acceptance scenarios: a local-vs-remote
  path-basis case, a JSON field with comment-preservation warning,
  and a hidden-overflow chip list with duplicates and bulk-paste
  review.

## Scope

A surface emits one or more of these records when a form field crosses
any of the following gates:

- the value is a path, file, folder, archive inner path, glob, or
  attached drop target, and the value's meaning depends on workspace
  root, current file, selected target, current working directory,
  remote host, container, local machine, or provider-owned root;
- the value is a structured fragment (arbitrary code, JSON, JSONC,
  YAML, TOML, regex, glob, SQL, HCL, settings fragment, command
  template body) whose downstream meaning depends on syntax, schema
  hints, comments, unknown fields, or canonical normalization;
- the value is a list of values shown as chips/tags, with add, remove,
  reorder, duplicate-handling, overflow, bulk-paste, drag-drop, and
  export/serialization semantics worth reviewing.

A surface that renders one of these shapes as an inert single-line
text input — without basis disclosure for paths, without syntax /
schema state for code, without overflow / duplicate / paste posture
for chips — is non-conforming. A surface MAY render a compact control
inline; the full record must remain reachable through review sheets,
CLI/headless output, support export, and validators.

## What each record carries

### Path / file / folder field

A `path_field_record` answers the following questions:

1. **Basis** — relative to what?
   - `path_basis_class`: `workspace_root`, `current_file`,
     `selected_target`, `current_working_directory`,
     `user_chosen_basis`, `absolute_anchor`,
     `archive_inner_anchor`, `provider_root`,
     `remote_host_anchor`, or `container_volume_anchor`.
   - A relative path with no resolved basis cannot apply.
2. **Authority** — interpreted against which root?
   - `path_authority_class`: `workspace_root_authority`,
     `current_file_workspace_authority`, `remote_host_authority`,
     `container_volume_authority`, `local_machine_authority`,
     `archive_overlay_authority`, `provider_owned_root_authority`,
     `generated_overlay_authority`, `support_export_scope_authority`.
   - The record names which root the path resolves under, distinct
     from the basis the user typed against.
3. **Shape** — what kind of object?
   - `path_kind_class`: `any_filesystem_object`, `file_only`,
     `folder_only`, `executable_file_only`, `archive_inner_path_only`,
     `file_or_folder`, `glob_pattern`, `content_addressed_object`.
4. **Picker and drop compatibility**
   - `picker_compatibility_class`: `picker_supported_local`,
     `picker_supported_workspace_host`, `picker_supported_remote`,
     `picker_supported_archive`, `picker_supported_provider`,
     `picker_unsupported_manual_entry_only`,
     `picker_blocked_by_policy`.
   - `drag_drop_compatibility_class`: `drag_drop_blocked`,
     `drag_drop_local_only_with_disclosure`,
     `drag_drop_workspace_host_with_disclosure`,
     `drag_drop_remote_with_disclosure`,
     `drag_drop_archive_with_disclosure`,
     `drag_drop_cross_root_requires_review`.
   - A drag/drop into a remote, container, archive, or
     provider-owned target MUST disclose whether the dropped path is
     local-client, workspace-host, remote-target, archive-internal,
     or provider-owned before apply.
5. **Missing-target state**
   - `missing_target_state_class`: `target_present_in_context`,
     `target_missing_in_context`, `target_unknown_until_resolve`,
     `target_in_archive_inspect_only`,
     `target_behind_unmounted_root`,
     `target_blocked_by_authority`,
     `target_resolution_skipped`.
   - When the state is anything other than `target_present_in_context`,
     the record names the recovery handoff (mount the root, switch
     workspace, widen the workset, request access, accept inspect-only,
     or cancel).
6. **Normalization disclosure**
   - `path_normalization_class`: `lossless`, `case_normalized`,
     `unicode_normalized`, `separator_normalized`,
     `redundant_segment_collapsed`, `symlink_resolved`,
     `alias_resolved_to_canonical`, `whitespace_trimmed`,
     `archive_inner_path_canonicalized`.
   - When normalization is lossy, the record carries
     `normalization_warning_required: true` and the raw entry is
     preserved as a ref so the user can compare to the canonical
     form. Silent canonicalization that loses the original spelling
     is non-conforming.

### Code-backed field

A `code_backed_field_record` answers:

1. **Format**
   - `code_format_class`: `arbitrary_code_fragment`,
     `json`, `jsonc`, `yaml`, `toml`, `markdown`, `regex_pattern`,
     `glob_pattern`, `sql_fragment`, `hcl_fragment`,
     `structured_settings_fragment`, `command_template_body`,
     `notebook_assertion_expression`.
2. **Syntax highlighting**
   - `syntax_highlighting_state`: `highlighting_active`,
     `highlighting_loading`, `highlighting_unavailable`,
     `highlighting_disabled_by_policy`, `highlighting_not_applicable`.
   - The record names the highlighter source so a downgraded /
     fallback highlighter is visible to the reviewer.
3. **Schema hints**
   - `schema_hint_state`: `hint_unavailable`, `hint_loading`,
     `hint_loaded`, `hint_partial`, `hint_stale`,
     `hint_unknown_field_warning_only`, `hint_strict_reject_unknown`,
     `hint_blocked_by_policy`.
4. **Unknown-field handling**
   - `unknown_field_handling_class`: `preserve_unknown_fields`,
     `warn_on_unknown_fields_preserve`, `strip_unknown_fields_with_review`,
     `reject_unknown_fields`, `not_applicable_format_has_no_schema`.
5. **Comment preservation**
   - `comment_preservation_class`: `preserve_comments`,
     `warn_comment_loss_on_canonical_write`,
     `strip_comments_with_review`, `comments_not_supported_by_format`,
     `not_applicable_format_has_no_comments`.
6. **Diff / preview**
   - `diff_preview_state`: `inline_diff_available`,
     `side_by_side_diff_available`,
     `dry_run_preview_required_for_broad_change`,
     `preview_unavailable_blocks_apply`,
     `preview_skipped_with_explicit_ack`.
   - Broad changes (whole-file rewrite, schema migration, multi-key
     replace, settings-tree rewrite) MUST surface a diff or preview
     before apply unless the record records an explicit user
     acknowledgment with a recovery handoff.
7. **Lossy normalization**
   - `code_normalization_class`: `lossless`,
     `whitespace_canonicalized`, `key_order_canonicalized`,
     `quote_style_canonicalized`, `comments_dropped`,
     `unknown_fields_stripped`, `numeric_format_canonicalized`,
     `string_escape_canonicalized`, `boolean_or_null_normalized`.
   - When normalization is lossy, the record carries
     `lossy_normalization_warning_required: true` and the raw entry
     is preserved as a ref so the original text can be inspected.

### Multi-value chip field

A `multivalue_chip_field_record` answers:

1. **Add semantics**
   - `add_method_class`: `keyboard_separator`,
     `picker_only`, `paste_with_review_required`,
     `drag_drop_with_disclosure`, `scan_from_referenced_object`,
     `blocked_read_only`.
2. **Remove semantics**
   - `remove_semantics_class`: `direct_remove_with_undo`,
     `remove_requires_review`, `remove_blocked_by_policy`,
     `remove_blocked_required_value`,
     `remove_only_in_bulk_review`.
3. **Reorder semantics**
   - `reorder_semantics_class`: `reorder_with_drag`,
     `reorder_with_keyboard`, `reorder_only_in_review`,
     `order_meaningful_no_reorder`, `order_irrelevant`,
     `order_canonicalized_on_save_with_warning`.
4. **Duplicate handling**
   - `duplicate_policy_class`: `reject_duplicates`,
     `merge_by_stable_id`, `preserve_with_warning`,
     `last_value_wins_requires_review`,
     `case_insensitive_dedupe_with_warning`,
     `not_applicable_unique_by_construction`.
5. **Overflow visibility**
   - `overflow_visibility_class`: `all_inline`,
     `virtualized_window`, `n_more_chip_with_expand`,
     `summary_count_only_inspect_required`,
     `hidden_count_blocks_apply`.
   - A list whose visible chip count diverges from its actual
     value count MUST publish the hidden count and a way to inspect
     the full list. Silent truncation is non-conforming.
6. **Bulk-paste behavior**
   - `bulk_paste_policy_class`: `bulk_paste_blocked`,
     `bulk_paste_inline_with_split_rule`,
     `bulk_paste_promotes_to_review`,
     `bulk_paste_requires_field_rule_review`.
   - The record names the split rule (newline, comma, whitespace,
     custom delimiter), trim posture, duplicate behavior, and
     normalization preview.
7. **Export / serialization truth**
   - `serialization_truth_class`: `order_preserving_array`,
     `set_canonical_unordered`, `ordered_key_value_pair_list`,
     `unordered_key_value_map`, `delimited_string_with_declared_separator`,
     `chip_handle_list_refs_only`.
   - `export_truth_class`: `export_matches_visible_order`,
     `export_canonicalizes_order_with_warning`,
     `export_redacts_chip_payload_to_handle_only`,
     `export_excluded_class_change_required`.
   - The record states whether the on-disk / on-wire shape is the
     same as the visible chip order, and whether export carries the
     chip payload or only handle refs.

## Composition with field-rule and field-row contracts

Each `path_field_record`, `code_backed_field_record`, and
`multivalue_chip_field_record` carries:

- `field_rule_record_ref` — opaque id of the governing
  `field_rule_record` (typically `filesystem_path_ref`,
  `code_backed_expression`, or a multi-value field display class).
  Redaction, copy/export posture, validation hooks, evaluation
  context, and unsafe-value warnings live on that field-rule record.
  These specialized records do not duplicate that surface; they
  refine it.
- `field_row_record_ref` — opaque id of the governing
  `field_row_record` so label, source pill, effective-value
  inspector, and exact-row deep-link land on one row identity.

A surface MAY emit a path/code/multi-value record without a paired
field-rule record only when the value is inert by the field-rule
contract's own scope rules (low-risk text). The moment the value
becomes high-risk (secret-bearing, path-authority-bearing,
evaluable, network-targeting, etc.) the field-rule record becomes
required.

## Cross-surface mapping

| Surface | Path field | Code-backed field | Multi-value chip field |
|---|---|---|---|
| Settings | Path / folder picker for active profile, current workspace, or local-machine roots; basis MUST be visible. | Settings fragment (JSON / YAML / structured) with schema hints, unknown-field warning, and diff for broad rewrites. | Allow-list / deny-list / search-roots / extension-allow chip lists with duplicate, overflow, and serialization disclosure. |
| Scaffolding | Output folder picker with workspace-root vs absolute-anchor disclosure and missing-folder handoff. | Templated config fragment with comment preservation, unknown-field handling, and dry-run preview. | Multi-target chip list (which targets to scaffold) with hidden-overflow inspect and bulk-paste review. |
| API workspaces | Request body or attachment path with archive-inner / provider-owned authority disclosure. | Request body, JSON/YAML schema, GraphQL fragment, or assertion expression with schema hints and dry-run review. | Header keys, cookie names, env-key chips, parameter chips with duplicate-by-stable-id and order-meaningful posture. |
| Package config | Manifest path / lockfile path / script-target path with workspace-root authority and missing-file handoff. | `package.json`, `Cargo.toml`, `pyproject.toml`, lockfile fragments with comment / unknown-field / lossy-normalization disclosure. | Dependency / script / extra / feature chip lists with duplicate review and serialization-truth chip. |
| Repair flows | Cache / log / artifact path drop with workspace-host execution disclosure and policy-narrowed authority. | Repair-input JSON / YAML / regex with schema hints, dry-run preview, and lossy-normalization warning. | Targets-to-repair, files-to-rotate, keys-to-clear chip lists with overflow inspect and bulk-paste review. |
| Support exports | All path records appear redacted to basis + authority + missing-state, never raw absolute paths. | All code records appear with format, schema-hint state, normalization report, never raw fragment unless the field-rule export class allows it. | All chip records appear with full count, hidden count, duplicate report, serialization class, and chip handle refs. |

## Conformance

A conforming surface:

1. Emits a `path_field_record`, `code_backed_field_record`, or
   `multivalue_chip_field_record` for any field whose shape is one of
   the three covered shapes.
2. Names the governing `field_rule_record` and `field_row_record` so
   the record composes rather than re-mints redaction, validation,
   row anatomy, source pill, and apply posture.
3. Refuses apply on a relative path with no basis, on a structured
   fragment whose schema or syntax state would block the chosen
   apply path, or on a multi-value list whose hidden count is not
   inspectable.
4. Discloses lossy normalization and preserves a raw ref so the
   original entry can be inspected.
5. Routes drag/drop, bulk paste, duplicate handling, and overflow
   inspection through the same record as ordinary manual entry.
6. Projects the same record metadata to desktop UI, CLI / headless
   output, support export, and future inspectors.

Adding an enum value is additive-minor and bumps the schema's
`schema_version`. Repurposing an existing value is breaking and
requires a new governance decision row before any surface consumes it.
