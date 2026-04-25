# Find/replace, rename, refactor, and imported/generated patch transaction contract

This document freezes how Aureline turns semantic edits and broad text
changes into one accountable transaction shape before any editor feature
starts mutating buffers, files, symbols, or imported patches at scale.

Every find/replace dialog, workspace-wide replace flow, rename driver,
refactor / move / extract proposal, and imported or generator-emitted
patch applicator must read the same record family. There is one
transaction packet, one preview packet, one outcome packet, one
checkpoint linkage, and one validation hook plan — not separate hidden
apply paths per surface.

Machine-readable companions:

- [`/schemas/editor/refactor_transaction.schema.json`](../../schemas/editor/refactor_transaction.schema.json)
  — `refactor_transaction_record`, the canonical transaction packet
  every refactor / replace surface emits when the edit crosses more
  than one match, more than one file, more than one symbol family, or
  any provider-backed semantic boundary.
- [`/schemas/editor/refactor_preview.schema.json`](../../schemas/editor/refactor_preview.schema.json)
  — `refactor_preview_record`, the governed preview packet that names
  match counts, exclusion rows, file-state warnings, affected scope,
  generated / protected / blocked counts, import / config change
  counts, current index freshness, and checkpoint / rollback posture
  before apply may proceed.
- [`/schemas/editor/refactor_outcome.schema.json`](../../schemas/editor/refactor_outcome.schema.json)
  — `refactor_outcome_record`, the outcome and reversal packet that
  names final state, applied / skipped / blocked counts, reversal
  class, validation hook state, and the next-action posture.
- [`/fixtures/editor/refactor_cases/`](../../fixtures/editor/refactor_cases/)
  — worked YAML fixtures covering the required scenarios.

This contract composes with and does not replace:

- [`/docs/language/provider_graph_and_arbitration_contract.md`](../language/provider_graph_and_arbitration_contract.md)
  — provider health, freshness, scope, locality, and result-provenance
  rules for the language-capable providers that back rename and
  refactor / move / extract operations.
- [`/docs/language/diagnostics_and_code_action_contract.md`](../language/diagnostics_and_code_action_contract.md)
  — diagnostic-cluster and code-action summary truth that originates
  most quick-fix and fix-all transactions and that this contract
  cites by reference rather than re-deriving.
- [`/docs/governance/truth_and_degraded_state_vocabulary.md`](../governance/truth_and_degraded_state_vocabulary.md)
  — shared `Limited`, `Stale`, `Blocked`, and related downgrade
  language reused here without forking.
- [`/docs/generated/lineage_hint_packet.md`](../generated/lineage_hint_packet.md)
  — generated-source lineage and remap-safe disclosure that this
  contract relies on to mark generated paths blocked.
- [`/docs/runtime/fault_domains_and_restart_policy.md`](../runtime/fault_domains_and_restart_policy.md)
  — provider quarantine, restart, and freshness floors that this
  contract reads before allowing apply.
- `.t2/docs/Aureline_PRD.md`, `.t2/docs/Aureline_Technical_Design_Document.md`,
  and `.t2/docs/Aureline_UI_UX_Spec_Document.md`. If those documents
  disagree with this contract, those upstream documents win and this
  contract plus the companion schemas update in the same change.

## Why freeze this now

Editor mutation is the easiest place for accidental product truth:

- a workspace replace with no preview can edit generated, protected, or
  excluded paths while still looking like a one-line typo fix;
- a rename driven from a stale or partial semantic graph can mutate
  some references, miss others, and never disclose the partial scope;
- a refactor / move / extract proposal can leave one buffer dirty,
  another file unsaved, and a third file with a half-applied import
  rewrite that the user cannot undo cleanly;
- an imported patch (SARIF auto-fix, generator output, AI diff) can
  apply in one place and silently fail anchor resolution in another;
- a fix-all action that reuses a code-action summary can claim safety
  the underlying provider never proved; and
- the user's only recovery path can be to reach for VCS state rather
  than the editor's own local-history checkpoint.

This contract makes those differences explicit before any apply path
ships. Multi-file, semantic, or provider-backed edits cannot claim
inline-safe-apply without a preview packet. Imported / generated
patches and semantic refactors share one transaction model. Outcomes
disclose reversal class and skipped / blocked members rather than
quietly partial-applying.

## Scope

Frozen at this revision:

- the operation classes (`find_replace_in_file`, `workspace_replace`,
  `rename_symbol`, `refactor_move_or_extract`,
  `imported_or_generated_patch_apply`) and the requested-scope
  vocabulary that pairs with each;
- the input and replacement vocabularies (literal text, regex,
  structural match, semantic symbol, imported patch input,
  generator-proposed patch input) and the replacement kinds that pair
  with them;
- the preview-packet fields for match counts, exclusion rows with
  typed reasons, file-state warnings, affected file / symbol /
  generated / protected / blocked counts, import or config change
  counts, current index freshness, semantic-layer state, and the
  checkpoint / rollback posture;
- the outcome and reversal vocabulary (exact undo, compensating
  revert, regenerate-first, manual-review-required, blocked, failed)
  and the per-member skipped or blocked rows that name the reason;
- the validation hook taxonomy (rerun diagnostics, rerun build / test,
  refresh semantic snapshot, rerun rename-impact query, review
  generated outputs, inspect protected paths, compare imported
  baseline delta, rerun formatter / organize-imports, no automatic
  validation available) and the replay hint set;
- the gates that prevent multi-file, semantic, or imported / generated
  patch operations from claiming inline-safe-apply without a preview
  packet, and that require checkpoint refs and rollback path classes
  for any operation that mutates beyond a single buffer; and
- redaction-aware export labels so transaction, preview, and outcome
  packets are review-safe without exposing raw match text, raw
  replacement text, raw diff bodies, raw patch hunks, raw paths, raw
  symbol bodies, raw provider logs, raw command lines, raw response
  bodies, or raw secret material.

Out of scope at this revision:

- implementing refactor engines or broad language-specific transforms;
- implementing the workspace text engine, regex engine, structural
  matcher, semantic indexer, rename driver, refactor / move / extract
  engines, or imported-patch applicator;
- the final UI affordances for preview / apply / undo / partial
  selection;
- ranking or autosuggest of refactor proposals; and
- the conflict resolution policy when an external change races a live
  transaction (named here through the
  `external_modification_detected` warning, but the reconciler is its
  own decision row).

## 1. Operation classes

Every refactor or replace surface picks exactly one operation class
per transaction. The class is the load-bearing field: it decides the
preview requirement floor, the rollback path floor, and the apply
posture floor. Adding a new class is additive-minor and bumps
`refactor_transaction_schema_version`.

### 1.1 `find_replace_in_file`

A find / replace driven inside the active buffer or one named buffer.
The match set is local to one document. Inline apply is admissible
when the input is literal or regex with no capture groups crossing
file or symbol boundaries, the buffer is clean, and no provider-backed
semantic guarantee is claimed. A structured diff preview is still
admissible and is the default once the user enables "preview before
apply" in their settings. This is the only operation class that may
resolve `apply_posture_class` to `apply_inline_allowed`.

### 1.2 `workspace_replace`

A replace that crosses more than one file. The preview packet is
mandatory. The transaction MUST cite a checkpoint ref before apply
runs. Excluded scopes (workspace excludes, VCS ignore, generated-pair
paths, protected paths, dirty buffers, merge conflicts, binary or
unsupported encodings, symlink policy, unloaded roots) materialise as
exclusion rows in the preview packet, never as silently dropped
matches. The apply posture floor is `preview_before_apply_required`.

### 1.3 `rename_symbol`

A rename driven through a language provider that resolves a semantic
symbol family. The acting provider's `semantic_layer_state_class` MUST
be current or remapped (`semantic_current_exact`,
`semantic_current_remapped`, `semantic_narrowed_scope`, or
`semantic_cached_recent`). A rename driven from
`syntactic_or_text_only`, `semantic_stale_epoch_mismatch`, or
`imported_patch_only` providers MUST NOT advance under
`apply_inline_allowed` and SHOULD route through
`rename_impact_preview_required`. Partial scope (e.g. graph warm only
for the active workset) materialises as
`semantic_narrowed_scope` plus an exclusion row whose reason is
`excluded_due_to_semantic_index_partial`. The rollback class is
typically `exact_undo_via_local_history_checkpoint` when checkpoint
capture succeeded, otherwise `compensating_revert_via_workspace_diff`.

### 1.4 `refactor_move_or_extract`

A refactor / move / extract proposal that mutates one or more files.
Same provider freshness floor as rename. The preview packet MUST
disclose generated-pair, protected-path, import-graph change, and
config-graph change counts. If the operation modifies an
import / config edge, the preview MUST include
`import_or_config_change_count` greater than zero and the validation
plan MUST include `rerun_formatter_or_organize_imports` or
`rerun_build_or_test`.

### 1.5 `imported_or_generated_patch_apply`

A patch that did not originate inside the editor: a SARIF auto-fix
import, a generator-emitted diff, an AI-proposed multi-file diff, or
a `git apply`-style imported patch. The transaction MUST cite a
non-null `imported_patch_ref` on the input descriptor or the
replacement descriptor (whichever is the patch carrier). The preview
packet MUST disclose anchor resolution status: any unresolved hunks
land an `imported_patch_anchor_unresolved` exclusion row and the
preview completeness class drops to
`preview_partial_due_to_imported_patch_unverified`. The rollback path
class for this operation class falls into one of:
`exact_undo_via_local_history_checkpoint` (if the editor took a
checkpoint before apply), `compensating_revert_via_workspace_diff`,
`imported_patch_inverse_required` (if the source patch tool can
produce an inverse), or `manual_review_required_no_automatic_path`.

## 2. Requested scope

`requested_scope_class` declares what the user asked for, not what
the preview ultimately materialised. The scope vocabulary covers:

- `current_selection_only` — the active selection in the active
  buffer;
- `current_file_whole_document` — the active buffer or a single named
  document;
- `open_buffers_only` — the set of currently open documents (excluded
  files in this set must still appear as exclusion rows so the user
  sees why an open buffer was skipped);
- `named_path_set` — an explicit include path set, expressed via an
  opaque `include_path_set_ref`;
- `workspace_full` — the full workspace, subject to workspace
  excludes, VCS ignore, generated-pair filters, protected-path
  filters, and provider freshness floors;
- `semantic_symbol_graph_scope` — the union of references the language
  provider proves for the targeted symbol family at the current
  semantic-layer state;
- `imported_patch_declared_scope` — the file / hunk set declared in
  the imported patch packet; and
- `generated_pair_family_scope` — the family of generator-paired
  files when the operation explicitly opts in to generator-aware
  replace.

The `scope_descriptor` MUST flag `respects_workspace_excludes`,
`respects_vcs_ignore`, `limited_to_open_buffers`, and
`follows_symlinks` so reviewers can verify the user's request did
not silently widen.

## 3. Preview packet

A preview packet is the mandatory bridge between proposal and apply
for every operation class except `find_replace_in_file` running
inline-safe. The packet exposes:

### 3.1 Match counts

`match_counts` carries `total_match_count`, `matched_file_count`,
`matched_symbol_count`, `skipped_match_count`, and
`ambiguous_match_count`. None of these may be hidden behind a
"more results not shown" affordance: counts are exact across the
materialised scope.

### 3.2 Exclusion rows

`exclusion_rows` enumerate every reason the materialised scope is
narrower than the requested scope. The vocabulary covers:

- `excluded_by_user_path_filter` — the user typed an explicit
  exclusion;
- `excluded_by_workspace_excludes` — workspace excludes filtered the
  path;
- `excluded_by_vcs_ignore` — VCS ignore filtered the path;
- `excluded_as_generated_pair_path` — the path is a known generator
  output;
- `excluded_as_protected_or_policy_path` — policy or trust marked the
  path protected;
- `excluded_due_to_provider_freshness_floor` — the acting provider's
  freshness was below the operation's floor;
- `excluded_due_to_semantic_index_partial` — the semantic index has
  not yet covered the path;
- `excluded_due_to_imported_patch_anchor_unresolved` — the patch
  hunk's anchor could not be resolved against the current source;
- `excluded_due_to_external_modification` — the file changed under
  the editor since match collection;
- `excluded_due_to_open_buffer_dirty` — the path has unsaved changes
  the user has not asked to include;
- `excluded_due_to_merge_conflict` — the path carries a merge
  conflict marker the user has not resolved;
- `excluded_due_to_binary_or_unsupported_encoding`;
- `excluded_due_to_symlink_policy`; and
- `excluded_due_to_unloaded_root` — a workspace root the user has
  not loaded yet.

Each exclusion row carries a count, an optional opaque
`excluded_path_set_ref`, and a reviewable summary so support exports
can describe the row without leaking raw paths.

### 3.3 File-state warnings

`file_state_warnings` enumerate the situations that did not exclude
the path but do change the apply posture: dirty buffers, external
modifications, merge conflicts, case-only path collisions, mixed or
uncertain encodings, mixed line endings, binary or large file
thresholds, generated-pair presence, protected path presence,
imported-patch anchor drift, and symlink traversal.

### 3.4 Impact counts

`impact_counts` reports `affected_file_count`,
`affected_symbol_count`, `affected_anchor_count`,
`generated_path_count`, `protected_path_count`, `blocked_path_count`,
`import_or_config_change_count`, and `downstream_consumer_count`.
Reviewers read these counts before the diff body, not after.

### 3.5 Index freshness

`index_freshness` carries the acting provider's `freshness_class`,
`semantic_layer_state_class`, `language_provider_family`,
`locality_class`, and the current epoch bindings. A
`semantic_stale_epoch_mismatch` or `imported_patch_only` provider
cannot ride a `preview_complete_for_full_scope` completeness class.

### 3.6 Checkpoint and rollback posture

`checkpoint_descriptor` declares whether the operation requires a
checkpoint, the opaque `checkpoint_ref` if one was captured, and the
`rollback_path_class`. Operations that mutate beyond a single buffer
(`workspace_replace`, `rename_symbol`, `refactor_move_or_extract`,
`imported_or_generated_patch_apply`) MUST resolve
`checkpoint_required` to true, MUST cite a non-null `checkpoint_ref`,
and MUST resolve `rollback_path_class` away from
`no_safe_rollback_available`. The rollback path classes are:

- `exact_undo_via_local_history_checkpoint` — the editor's local
  history can restore prior content;
- `compensating_revert_via_workspace_diff` — apply succeeded in part,
  reversal is achievable via a workspace-level revert;
- `regenerate_first_then_replay` — a generator must regenerate
  outputs before the transaction can replay;
- `manual_review_required_no_automatic_path` — no safe automatic
  reversal exists;
- `imported_patch_inverse_required` — the source patch tool must
  produce an inverse patch; and
- `no_safe_rollback_available` — reserved for inspect-only
  transactions; never admissible when `checkpoint_required = true`.

### 3.7 Preview completeness

`preview_completeness_class` is one of:

- `preview_complete_for_full_scope` — every match in the materialised
  scope is preview-rendered, and no exclusion row carries a
  freshness-floor, semantic-index-partial, imported-patch-unresolved,
  or unloaded-root reason;
- `preview_partial_due_to_index_floor` — the semantic index has not
  yet covered the full scope;
- `preview_partial_due_to_provider_quarantine` — the acting provider
  is in quarantine or restart;
- `preview_partial_due_to_imported_patch_unverified` — the imported
  patch carrier could not be verified;
- `preview_partial_due_to_excluded_paths` — the materialised scope
  is narrower than requested for any user-or-policy reason; or
- `preview_blocked_pending_user_review` — the preview cannot proceed
  until the user reviews a generated-or-protected, policy, or trust
  decision.

A preview that names any non-zero generated, protected, blocked, or
import / config change count and is not marked
`preview_complete_for_full_scope` MUST also carry the matching
exclusion rows so the count is attributable.

## 4. Transaction record

`refactor_transaction_record` is the canonical packet that links
input, replacement, scope, acting provider, transaction state,
atomicity, safety, mutation scope, preview requirement, apply
posture, blocking reasons, the preview / outcome / checkpoint /
approval-ticket refs, the originating code-action summary or
diagnostic cluster refs (when applicable), and the validation plan.

### 4.1 Atomicity

`atomicity_class` is one of:

- `atomic_apply_or_none` — every member commits, or no member does;
- `per_file_atomic` — each file commits or skips independently;
- `per_match_atomic` — each match commits or skips independently
  (admissible only for `find_replace_in_file` and
  `workspace_replace`);
- `best_effort_with_skipped_members` — partial apply is admissible
  and surfaces every skipped member in the outcome packet; and
- `imported_patch_atomicity_inherited` — the imported patch carrier
  declares atomicity and the editor inherits it.

### 4.2 Safety class

`safety_class` is the floor for `apply_posture_class`. Roughly:

- `trivia_safe` — typo / whitespace / single-anchor literal;
- `local_text_safe` — buffer-local text only;
- `single_file_semantic` — one file, one symbol family, language
  provider current;
- `cross_file_semantic` — multiple files, language provider current;
- `workspace_text_broad` — workspace-wide text replace;
- `imported_or_generated_patch` — patch carrier is not the editor;
- `generated_or_protected` — operation touches generated or protected
  paths; or
- `unknown_or_unstable` — provider quarantine, freshness floor unmet,
  or anchors unverified.

`unknown_or_unstable` and `generated_or_protected` MUST NOT advance
under `apply_inline_allowed`.

### 4.3 Mutation scope and apply posture gates

The transaction record's `allOf` gates encode the cross-cutting rules:

1. Multi-file or provider-backed operations cannot claim safe-apply.
   `workspace_replace`, `rename_symbol`, `refactor_move_or_extract`,
   and `imported_or_generated_patch_apply` MUST resolve
   `preview_requirement_class` outside `not_required_local_inline` /
   `inline_summary` and `apply_posture_class` outside
   `apply_inline_allowed`.
2. Imported or generated patch operations MUST cite a non-null
   `imported_patch_ref` on either the input or replacement
   descriptor.
3. Rename and refactor / move / extract operations MUST run on a
   current or remapped semantic layer; syntactic-only,
   stale-epoch-mismatch, or imported-patch-only providers MUST NOT
   advance under `apply_inline_allowed`.
4. Any non-empty `blocking_reason_classes` set MUST resolve
   `transaction_state_class` into a blocked, manual-review,
   regenerate-first, or failed state and MUST NOT report
   `applied_full` or `applied_partial_with_skipped_members`.
5. Applied or rolled-back transactions MUST cite a non-null
   `outcome_packet_ref`.

### 4.4 Validation plan

`validation_plan` carries one or more `validation_hook_class`
entries (rerun diagnostics, rerun build / test, refresh semantic
snapshot, rerun rename-impact query, review generated outputs,
inspect protected paths, compare imported baseline delta, rerun
formatter / organize-imports, or no automatic validation available)
plus replay hints (`replay_against_same_execution_context`,
`replay_against_new_semantic_epoch`, `export_review_packet`,
`attach_support_bundle_evidence`, `requires_rule_metadata_refresh`,
`rebuild_imported_patch_from_source`).

## 5. Outcome record

`refactor_outcome_record` is emitted whenever a transaction
completes, partially completes, blocks, rolls back, or fails. It
carries:

- `final_state_class`: one of `applied_full`,
  `applied_partial_with_skipped_members`, `blocked_no_changes_made`,
  `rolled_back_full`, `rolled_back_partial`,
  `failed_compensating_revert_required`, `manual_review_required`,
  or `regenerate_first_required`.
- `reversal_class`: one of
  `exact_undo_via_local_history_checkpoint`,
  `compensating_revert_via_workspace_diff`,
  `regenerate_first_then_replay`, `imported_patch_inverse_required`,
  `manual_review_required_no_automatic_path`, or
  `no_safe_reversal_available`.
- `outcome_counts`: applied / skipped / blocked / rolled-back match,
  file, and symbol counts.
- `skipped_or_blocked_members`: per-row reason classes (user
  deselection, external modification, dirty buffer, merge conflict,
  generated path, protected / policy path, provider freshness floor,
  semantic index partial, imported patch anchor unresolved, imported
  patch signature unverified, binary / unsupported encoding,
  symlink policy, unloaded root, approval ticket required, step-up
  auth required, validation hook failure, IO error, provider crash).
- `validation_state`: one of `hooks_unavailable_no_validation_path`,
  `hooks_available_pending_run`, `hooks_run_passed`,
  `hooks_run_failed`, `hooks_skipped_by_user`,
  `hooks_skipped_due_to_policy`, or
  `hooks_partial_some_passed_some_failed`.
- `next_action_class`: `ready_for_validation`,
  `user_review_pending_skipped_members`,
  `user_review_pending_blocked_members`,
  `regenerate_first_then_re_apply`,
  `open_compensating_revert_review`, `manual_review_required`, or
  `no_further_action_required`.

The outcome `allOf` gates encode:

1. `applied_full` and `applied_partial_with_skipped_members` MUST
   cite a non-null `checkpoint_ref` so the local-history undo path
   stays attributable.
2. `applied_partial_with_skipped_members` MUST list at least one
   skipped-or-blocked member row.
3. `failed_compensating_revert_required` MUST resolve
   `reversal_class` to `compensating_revert_via_workspace_diff` or
   `imported_patch_inverse_required` and MUST cite a non-null
   `compensating_revert_ref`.
4. `regenerate_first_required` MUST resolve `reversal_class` to
   `regenerate_first_then_replay`; `manual_review_required` MUST
   resolve to `manual_review_required_no_automatic_path`;
   `blocked_no_changes_made` MUST report zero applied-match,
   applied-file, and applied-symbol counts.

## 6. Boundary discipline

Every record family in this contract carries opaque ids, typed
vocabulary, epoch bindings, and export-safe summaries only. The
following NEVER cross the boundary:

- raw match text or raw replacement text;
- raw diff bodies, raw patch hunks, or raw rollback artifacts;
- raw paths, raw symbol bodies, raw imported-patch bodies;
- raw provider logs, raw command lines, raw response bodies;
- raw operator identity strings; or
- raw secret material.

`redaction_class` is one of `metadata_safe_default`,
`operator_only_restricted`, `internal_support_restricted`, or
`signing_evidence_only`. Support exports default to
`internal_support_restricted` for the preview and outcome packets
when any generated, protected, or imported-patch row is present.

`policy_context` carries `policy_epoch`, `trust_state`, and
`execution_context_id` so reviewers can locate the policy bundle and
execution-context the transaction ran under.

## 7. Cross-walk to schemas

| Concern | Field path |
|---|---|
| Operation class | `refactor_transaction_record.operation_class` |
| Requested scope | `refactor_transaction_record.scope_descriptor.requested_scope_class` |
| Input shape | `refactor_transaction_record.input_descriptor` |
| Replacement shape | `refactor_transaction_record.replacement_descriptor` |
| Acting provider | `refactor_transaction_record.acting_provider` |
| Atomicity | `refactor_transaction_record.atomicity_class` |
| Safety floor | `refactor_transaction_record.safety_class` |
| Apply posture | `refactor_transaction_record.apply_posture_class` |
| Preview requirement | `refactor_transaction_record.preview_requirement_class` |
| Blocking reasons | `refactor_transaction_record.blocking_reason_classes` |
| Match counts | `refactor_preview_record.match_counts` |
| Exclusion rows | `refactor_preview_record.exclusion_rows` |
| File-state warnings | `refactor_preview_record.file_state_warnings` |
| Affected counts | `refactor_preview_record.impact_counts` |
| Index freshness | `refactor_preview_record.index_freshness` |
| Checkpoint posture | `refactor_preview_record.checkpoint_descriptor` |
| Preview completeness | `refactor_preview_record.preview_completeness_class` |
| Final state | `refactor_outcome_record.final_state_class` |
| Reversal class | `refactor_outcome_record.reversal_class` |
| Skipped / blocked rows | `refactor_outcome_record.skipped_or_blocked_members` |
| Validation state | `refactor_outcome_record.validation_state` |
| Next action | `refactor_outcome_record.next_action_class` |

## 8. Worked fixtures

The `/fixtures/editor/refactor_cases/` directory carries worked
fixtures for the required scenarios:

- `workspace_replace_excluded_scopes.yaml` —
  `refactor_preview_record` for a workspace replace whose preview
  declares user-path-filter, generated-pair, protected-path, and
  dirty-buffer exclusions, and whose preview completeness is
  `preview_partial_due_to_excluded_paths`.
- `partial_rename_stale_graph.yaml` —
  `refactor_transaction_record` for a rename driven by a language
  server whose semantic-layer state has narrowed to active workset
  scope, requiring `rename_impact_preview_required` and
  `preview_before_apply_required`.
- `generated_file_blocked_refactor.yaml` —
  `refactor_outcome_record` for a refactor / move / extract that
  blocked because the generated-pair file family was protected,
  with `final_state_class = blocked_no_changes_made` and
  `reversal_class = manual_review_required_no_automatic_path`.
- `imported_multi_file_patch_with_checkpoint.yaml` —
  `refactor_transaction_record` for an imported multi-file patch
  whose `imported_patch_ref` is bound on the replacement descriptor,
  whose `checkpoint_ref` is captured pre-apply, and whose preview
  packet ref is named so the apply path can be replayed.

Each fixture carries only opaque transaction / preview / outcome /
checkpoint / approval-ticket / review-packet / provider / epoch /
policy / execution-context / patch / symbol / path-set / member-set
handles plus typed vocabulary and reviewable summaries. None of the
fixtures carry raw match text, raw replacement text, raw diff bodies,
raw patch hunks, raw paths, raw symbol bodies, raw provider logs,
raw command lines, raw response bodies, or raw secret material.

## 9. Acceptance restated

This contract is satisfied when:

- multi-file or provider-backed edits cannot claim safe-apply without
  a preview packet that names scope and blocked / generated /
  protected impact (enforced by the
  `refactor_transaction_record.allOf` gate on operation class versus
  preview requirement and apply posture);
- imported / generated patches and semantic refactors share one
  accountable transaction model rather than separate hidden apply
  paths (enforced by the single `refactor_transaction_record` shape
  and the `imported_patch_ref` requirement on
  `imported_or_generated_patch_apply`); and
- the worked fixtures cover the four required scenarios at the file
  paths named in section 8.

Adding a new operation class, scope class, input kind, replacement
kind, transaction state, atomicity class, safety class, mutation
scope, preview requirement, apply posture, blocking reason,
validation hook class, replay hint, source kind, support class,
freshness class, semantic-layer state, locality class, language
provider family, exclusion reason, file-state warning, completeness
class, rollback path, final-state class, reversal class,
skipped-or-blocked reason, validation-hook state, next-action class,
or epoch role is additive-minor and bumps the matching schema
`*_schema_version` const. Repurposing an existing value is breaking
and requires a new decision row.
