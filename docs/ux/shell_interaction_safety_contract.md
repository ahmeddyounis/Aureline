# Shell-level interaction-safety contract

This document is the **shell-wide contract** for focus return,
batch-scope review, preview / apply / revert, typed permission
prompts, safe preview of high-risk content classes,
representation-labeled copy / export, and responsive fallback on
protected shell surfaces. It exists so every protected surface —
editor, terminal, review, palette, search, install, remote
attach, AI apply, extension web view, collaboration surface,
support export, docs / help surface — uses **one vocabulary, one
packet shape, and one set of rules** when it asks the user to
commit to a consequence, when it returns focus after that
consequence, when it collapses chrome under narrow or split
layouts, and when bytes leave the product over copy or export.

The contract is normative. Where this document disagrees with
the source UI/UX spec it quotes, the source wins and this
document MUST be updated in the same change. Where this document
disagrees with a downstream surface's mint of its own copy, this
document wins and the surface is non-conforming.

The companion artifacts are:

- [`/schemas/ux/interaction_safety.schema.json`](../../schemas/ux/interaction_safety.schema.json)
  — boundary schema every non-owning surface reads.
- [`/fixtures/ux/interaction_safety_cases/`](../../fixtures/ux/interaction_safety_cases/)
  — worked-example corpus for at least one destructive core path
  and one externally mutating / publish-capable path.

This contract rides alongside — it does not re-mint — the
vocabularies already frozen in:

- `docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`
  — authority class and freshness hint on every rendered row.
- `docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`
  — redaction pass runs before bytes reach any persistent or
  exportable sink; no raw secret material crosses the boundary.
- `docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`
  — admin policy MAY narrow; policy MAY NOT silently widen.
- `docs/adr/0009-execution-context-and-scope.md`
  — `scope_filter_class` / `execution_context_id` re-exported;
  this contract never invents a parallel scope vocabulary.
- `docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`
  — `browser_handoff_packet` is the only path a shell action
  uses to leave the product; raw URL launches are forbidden.
- `docs/adr/0011-capability-lifecycle-and-dependency-markers.md`
  — `freshness_class`, `client_scope`, `redaction_class`
  re-exported without modification.
- `docs/adr/0012-extension-manifest-permission-publisher-policy.md`
  — extension-initiated paths ride the extension effective-
  permission surface; this contract names the actor and
  consequence class, not the permission resolver.
- `docs/adr/0013-docs-help-service-health-truth.md`
  — `citation_anchor_record` is the only anchor vocabulary;
  representation-labeled copy / export quotes those ids when
  the bytes leaving the product reference docs content.
- `docs/adr/0014-search-readiness-ranking-result-truth.md`
  — `search_result_packet_record` / `search_deep_link_record`
  drift state is re-exported as the drift axis on recovery
  surfaces; this contract does not re-mint it.
- `docs/adr/0016-shell-windowing-input-accessibility-boundary.md`
  — shell zones, focus ownership, command-entry routing, and
  text-input normalization stay shared across shortcuts, menus,
  deep links, notifications, and embedded-surface handoffs.

## Who reads this document

- **Product writers** deciding preview, apply, revert, permission-
  prompt, copy / export, and focus-return copy on any protected
  shell surface.
- **Shell / review / AI / extension / provider surface authors**
  minting packets that cross the RPC boundary to support export,
  AI evidence, mutation journal, crash dump, or claim manifest.
- **Support and parity-audit tooling** reading each axis
  mechanically — every axis is separately addressable even when
  the surface folds it into a single chip.

## One contract, six protected surfaces, one record shape

The contract applies uniformly to the protected shell surfaces
below. A surface that mints a private focus-return vocabulary,
its own preview / apply / revert phase names, its own permission
scope, its own representation class, its own responsive-fallback
ladder, or its own copy / export label is non-conforming.

| Protected surface                     | Typical interaction families                                                                                                                               |
|---------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `editor_canvas`                       | rename / refactor apply, save fidelity prompts, paste-with-review, suspicious-content reveal, inline undo / redo focus return.                             |
| `terminal_canvas`                     | multiline paste review, remote clipboard bridge, paste-then-run, remote host prompts.                                                                      |
| `review_and_diff_canvas`              | batch apply, multi-file preview / apply / revert, evidence export.                                                                                         |
| `palette_and_search_canvas`           | query-basis drift, command-scope review, policy-hidden placeholders, deep-link drift recovery.                                                             |
| `install_update_attach_canvas`        | extension install, update, publisher succession, remote attach, route share. External / publish-capable path.                                              |
| `ai_apply_canvas`                     | AI-proposed mutation preview, citation-anchored apply, low-confidence or uncited denial.                                                                   |
| `collaboration_canvas`                | presenter / follow state, grant / revoke, recording / retention. External / shared path.                                                                   |
| `provider_bearing_canvas`             | browser-handoff approval, credential-handle reveal, step-up authenticator.                                                                                 |
| `docs_help_service_health_canvas`     | representation-labeled copy of docs content; raw / rendered choice on citation-anchored rows.                                                              |
| `support_export_canvas`               | sanitised / metadata-only export; redaction envelope applies; raw bodies stripped.                                                                         |

Every interaction surface on the list above — and every future
surface that inherits a protected posture — emits an
`interaction_safety_packet_record` for every consequence-bearing
interaction. The packet is the cross-surface contract; per-
surface badges, chips, sheets, and dialogs are UI freedoms that
MUST collapse back into the packet's addressable axes on every
boundary (support export, mutation journal, AI evidence, claim
manifest, crash dump, evidence packet).

## Core axes (frozen)

Every `interaction_safety_packet_record` names exactly one value
from each axis below. Adding a value is additive-minor and bumps
`interaction_safety_schema_version`; repurposing a value is
breaking and requires a new decision row.

### Authority class

Who initiated the consequence-bearing action.

- `user_initiated_local`
- `user_initiated_shared`
- `ai_initiated`
- `extension_initiated`
- `automation_recipe_initiated`
- `collaboration_remote_initiated`
- `admin_policy_initiated`

Rules (frozen):

1. The authority class names the **initiator**, not the target.
   An AI apply that writes the user's workspace is
   `ai_initiated`; a user apply triggered from a collaboration
   session is `collaboration_remote_initiated` when the actor of
   record is a remote collaborator; the user's own in-product
   action is `user_initiated_local` / `user_initiated_shared`
   depending on whether the blast radius is local or shared.
2. A surface that cannot resolve its authority class MUST deny
   with `authority_class_unresolved` rather than defaulting to
   `user_initiated_local`.

### Consequence class

The four-class destruction / mutation vocabulary re-exported
from `UI/UX Spec §9.4 Destruction-class matrix` verbatim. The
set is closed.

- `reversible_local`
- `recoverable_durable`
- `external_shared`
- `irreversible_high_blast`

Rules (frozen):

1. Every consequence-bearing interaction names a consequence
   class before the user commits. A surface that elides the
   consequence class is non-conforming.
2. `external_shared` and `irreversible_high_blast` MUST focus
   the safest action by default; they MUST NOT occupy the
   default `Enter` path unless the user is already inside an
   explicit destructive review.
3. A surface MAY escalate a consequence class (e.g. a
   `reversible_local` paste that crosses into a production host
   becomes `external_shared`); it MAY NOT silently narrow.

### High-risk preview class

Typed preview classes that force the safe-preview posture
(sanitised / sandboxed / raw-inspection / metadata-only) before
bytes render or execute. The set is closed.

- `multiline_terminal_paste`
- `remote_clipboard_bridge`
- `paste_then_run`
- `file_drop_mass_mutation`
- `rich_active_content_render`
- `notebook_widget_payload`
- `oversized_generated_artifact`
- `install_or_update`
- `remote_attach`
- `collaboration_invite`
- `publish_external`
- `destructive_bulk_mutation`
- `secret_access`
- `browser_handoff`
- `bidi_or_invisible_formatting_reveal`
- `confusable_identifier_reveal`

Rules (frozen):

1. Every high-risk preview class renders a typed safe-preview
   disclosure on the **primary surface**. A tooltip-only or
   hover-only disclosure is non-conforming.
2. `rich_active_content_render`, `notebook_widget_payload`, and
   `oversized_generated_artifact` render sanitised / sandboxed /
   metadata-only by default; the rendered / raw / active path is
   always explicit and always reachable without leaving the
   current workflow (§9.8).
3. `install_or_update`, `remote_attach`, `collaboration_invite`,
   and `publish_external` force the stronger trust-decision
   display mode; suspicious-content labels are preserved into
   the exported packet summary (EN.2).

### Preview / apply / revert phase

The phase the packet records. The set is closed; phases MUST NOT
be collapsed into generic `done` / `failed` state.

- `propose`
- `preview`
- `apply`
- `validate`
- `keep_or_revert`

Rules (frozen):

1. Apply MAY NOT silently widen or materially change scope
   after preview. If the basis drifts, the surface MUST
   invalidate the preview, emit
   `interaction_safety_apply_basis_drifted`, and reopen review
   rather than performing a hidden best-effort apply (§9.5).
2. Partial apply, partial validation, and partial revert are
   first-class outcomes with their own user-visible states, not
   footnotes inside generic `Failed` banners.
3. Revert paths name the recovery class from `revert_class`
   (below) — they MUST NOT flatten the recovery into a vague
   `Undo` label.

### Revert class

How the recovery works on this path. The set is closed.

- `exact_undo`
- `compensating_action`
- `regenerate_from_source`
- `restore_from_checkpoint`
- `evidence_only_no_rerun`
- `no_recovery_available`

Rules (frozen):

1. `evidence_only_no_rerun` is the canonical claim for
   restored terminals, debug sessions, notebooks, API requests,
   and remote shells. A surface that silently re-runs these is
   non-conforming (§9.11).
2. `no_recovery_available` is permitted only on
   `irreversible_high_blast` consequence classes; every other
   consequence class MUST carry a concrete recovery claim.

### Permission grant scope

Re-exported from `UI/UX Spec §9.6 Trust and permission prompts`
verbatim. The set is closed.

- `once`
- `session`
- `workspace`
- `profile`
- `policy_managed`

Rules (frozen):

1. Remembered decisions are never indefinite by default for
   destructive, networked, provider-backed, or secret-bearing
   capabilities.
2. `policy_managed` grants name the policy owner, source, and
   lock state; they are reviewable but not silently editable by
   the end user.

### Authority-renewal trigger

When the existing grant is **insufficient** for the next action
and the surface MUST reprompt or step up. The set is closed.

- `target_changed`
- `route_changed`
- `actor_changed`
- `policy_source_changed`
- `policy_epoch_changed`
- `trust_state_changed`
- `scope_filter_changed`
- `grant_expired`
- `grant_revoked`
- `basis_snapshot_drifted`
- `authority_class_escalated`

Rules (frozen):

1. A grant made against target A MAY NOT apply to target B
   without a renewal; the surface MUST emit
   `interaction_safety_authority_renewal_required` and reopen
   review.
2. A route change that changes the actor (e.g. same command,
   different extension owner) triggers `actor_changed` and a
   renewal; silent forwarding is non-conforming.
3. A `policy_epoch_changed` renewal MUST quote the new epoch
   on the reissued packet; a surface that follows a pre-epoch
   grant past the change is non-conforming.

### Representation class

How the bytes leave the product on copy / export. Re-exported
from `UI/UX Spec §9.8` and `Appendix EN.3`. The set is closed.

- `raw`
- `rendered`
- `escaped`
- `sanitized`
- `sandboxed`
- `generated`
- `blocked_metadata_only`

Rules (frozen):

1. Every copy / export action names exactly one representation
   class on the packet and in the user-facing label (e.g.
   `Copy raw Markdown`, `Copy rendered preview`,
   `Export sanitized HTML snapshot`,
   `Export metadata only`).
2. `raw` preserves exact bytes; `escaped` preserves the
   source representation with metacharacters rendered safe
   (useful for pasting into logs / chat without control chars
   being interpreted); `sanitized` removes active / scriptable
   content; `sandboxed` renders within a safe boundary;
   `generated` names model-produced content that MUST carry a
   citation-anchor ref when quoting authoritative material;
   `blocked_metadata_only` withholds the raw body entirely for
   policy or safety reasons and renders only the metadata
   envelope.
3. `raw` is always reachable on suspicious-content surfaces —
   the user can never be safer in one surface simply because a
   different subsystem happens to render the same bytes
   (§9.8).
4. `generated` content without a `citation_anchor_refs` entry
   is denied under ADR-0013 (`derived_explanation_uncited`)
   when the row quotes authoritative material.
5. Representation labels survive screenshots, support exports,
   copied evidence, and review handoff artifacts (§9.8).

### Batch scope class

Re-exported from `UI/UX Spec §9.12` and `Appendix EO.3`. The set
is closed.

- `visible_rows`
- `loaded_items`
- `all_matching_query`
- `custom_identity_set`
- `query_snapshot_stale`

Rules (frozen):

1. `Select all` MUST say whether it means visible rows, loaded
   rows, or all matching items. A surface that renders an
   undisclosed `all_matching_query` selection is non-
   conforming.
2. A selection built against a query snapshot that has since
   drifted is `query_snapshot_stale`; the surface MUST mark
   the selection stale or based on a prior query snapshot
   before a destructive or broad action runs (§9.12).
3. Destructive batch previews MUST summarise
   `included`, `excluded_by_user_filter`,
   `blocked_by_policy`, `blocked_by_ownership`,
   `blocked_by_protected_path`,
   `hidden_missing_permission`, `hidden_stale_membership`,
   and `query_derived` members before commit.

### Focus-return state

Re-exported from `UI/UX Spec §19.4` and `Appendix EL.2`. The set
is closed.

- `returned_exact`
- `returned_nearest_safe_ancestor`
- `returned_current_batch_or_detail_owner`
- `returned_placeholder_announced`
- `focus_loss_denied`
- `focus_not_applicable_non_interactive`

Rules (frozen):

1. Focus MUST return to the invoking control or its row / card
   on modal confirm / cancel; if the invoker was removed, to
   the nearest safe ancestor or sibling (EL.2).
2. A surface that cannot return focus to the invoking control
   and cannot locate a safe ancestor MUST render a
   `returned_placeholder_announced` placeholder whose
   announcement explains **why** focus could not return to the
   missing surface. Silent focus loss is non-conforming.
3. `focus_loss_denied` is emitted when focus was lost because
   the shell collapsed chrome under the current responsive
   fallback; the surface MUST preserve keyboard reachability by
   rendering a persistent re-entry affordance.

### Responsive-fallback mode

Shell layout state at the time of the interaction. Re-exported
from `UI/UX Spec §6.1–§6.9` and `Appendix EP`. The set is
closed.

- `full_chrome`
- `compact_shell`
- `split_shell`
- `narrow_width_sheet`
- `very_narrow_compare`
- `zoom_400_overflow`
- `missing_extension_placeholder`
- `presentation_overlay_dimmed`

Rules (frozen):

1. A responsive fallback MAY collapse secondary chrome; it MUST
   NOT hide the `required_visible_field_class` set for the
   current consequence class (below). A compact shell that
   hides the current target, authority, or consequence class
   is non-conforming.
2. `missing_extension_placeholder` preserves the zone slot and
   names `Locate`, `Recover draft`, `Open without`, or
   `Export evidence` actions as appropriate (§9.11).
3. `presentation_overlay_dimmed` preserves focus rings and
   active-region cues on the underlying surface; decorative
   dimming MUST NOT obscure the focused interactive control
   (§19.2).

### Required-visible-field set

Fields that MUST remain visible at the same time as the confirm
action on every `external_shared` and `irreversible_high_blast`
consequence class, and on every high-risk preview class, even
under the most aggressive responsive fallback. The set is closed.

- `target_identity`
- `actor_identity`
- `authority_class_label`
- `consequence_class_label`
- `scope_statement`
- `recovery_class_label`
- `policy_source_label`
- `expiry_or_revocation_claim`
- `blocked_or_hidden_member_count`
- `representation_class_label`
- `basis_snapshot_freshness`

Rules (frozen):

1. A surface whose responsive fallback hides any field in the
   `required_visible_field_class` set for its consequence class
   MUST deny the action with
   `chrome_hid_required_field` and emit
   `interaction_safety_responsive_fallback_engaged` with the
   missing field named. Silent truncation is non-conforming.
2. Consequence blocks remain visible at the same time as the
   confirm action on destructive or cross-boundary flows; users
   should not have to memorise scrolled-away warnings (§9.4).

### Denial reason

Typed denials on the interaction-safety audit stream. Denials
fail closed; silent downgrade to a best-effort apply is
forbidden.

- `authority_class_unresolved`
- `consequence_class_unresolved`
- `preview_required_but_absent`
- `apply_basis_drifted`
- `authority_renewal_required`
- `permission_expired`
- `permission_revoked`
- `permission_grant_scope_unlabelled`
- `representation_label_missing`
- `representation_escalation_required`
- `batch_scope_class_unlabelled`
- `batch_membership_not_disclosed`
- `hidden_members_not_disclosed`
- `partial_failure_not_explained`
- `focus_return_target_lost`
- `chrome_hid_required_field`
- `safe_preview_bypassed`
- `raw_body_forbidden_on_boundary`
- `raw_url_forbidden_on_boundary`
- `citation_anchor_missing_on_generated`
- `policy_blocked`
- `interaction_safety_schema_version_lagging`

## Record shapes

The schema at
`schemas/ux/interaction_safety.schema.json` freezes the
following record shapes. This section summarises; the schema is
authoritative.

### `interaction_safety_packet_record`

Envelope for every consequence-bearing interaction. Required
fields (all separately addressable):

- `packet_id` (opaque)
- `interaction_session_id_ref`
- `surface_class`
- `authority_class`
- `actor_identity_ref` (opaque — raw identity bodies never
  cross)
- `target_identity_ref` (opaque — raw paths, raw URLs, raw
  handles never cross)
- `consequence_class`
- `preview_apply_revert_phase`
- `revert_class`
- `high_risk_preview_classes` (array; may be empty for
  `reversible_local`)
- `scope_filter_class` (re-exported from ADR-0009)
- `batch_scope` (nullable `batch_scope_record` — non-null when
  the interaction operates on more than one target)
- `permission_grant_scope_at_commit` (nullable — non-null when
  the interaction commits against a grant)
- `representation_class` (nullable — non-null whenever bytes
  leave the product on copy / export)
- `citation_anchor_refs` (array; non-empty when
  `representation_class = generated` and the row quotes
  authoritative material, or when the surface is
  `docs_help_service_health_canvas`)
- `responsive_fallback_mode`
- `visible_fields_at_commit` (array; MUST contain every field
  in the `required_visible_field_class` set applicable to the
  consequence class)
- `focus_return_state` (non-null when the interaction closed a
  surface)
- `authority_renewal_triggers_observed` (array; non-empty
  whenever the packet represents a renewal / reissue)
- `basis_snapshot_ref` (opaque)
- `freshness_class` (re-exported from ADR-0011)
- `client_scopes` (non-empty; re-exported from ADR-0011)
- `running_build_identity_ref`
- `policy_context` (re-exported from ADR-0008 / ADR-0009 /
  ADR-0001)
- `redaction_class` (re-exported from ADR-0011)
- `minted_at`

Non-conforming when:

- the consequence class is `external_shared` or
  `irreversible_high_blast` and `visible_fields_at_commit`
  omits any field in the required-visible-field set;
- the `preview_apply_revert_phase` is `apply` and the packet
  carries no `preview_packet_ref` pointing back at the
  approved preview;
- the `representation_class` is `generated` and no
  `citation_anchor_refs` are recorded while the row quotes
  authoritative material;
- `authority_renewal_triggers_observed` is non-empty and the
  packet is not itself a renewal packet.

### `preview_apply_revert_record`

Per-phase record with the shared lineage every phase quotes.
Required fields:

- `preview_apply_revert_id` (opaque)
- `interaction_session_id_ref`
- `phase` (`propose` / `preview` / `apply` / `validate` /
  `keep_or_revert`)
- `consequence_class`
- `revert_class`
- `basis_snapshot_ref`
- `target_identities_ref` (array of opaque ids)
- `included_member_count` (integer)
- `blocked_member_count` (integer)
- `hidden_member_count` (integer)
- `approval_requirements_ref` (array; empty when no step-up
  required)
- `validation_plan_ref` (nullable; required on `preview` and
  on `apply`; MAY be null on `propose`)
- `validation_outcome` (nullable;
  `not_run` / `running` / `passed` / `failed` / `mixed` /
  `manual_review_required`)
- `partial_outcome` (nullable;
  `not_partial` / `partial_success` / `partial_revert` /
  `partial_validation`)
- `policy_context`
- `running_build_identity_ref`
- `minted_at`

Non-conforming when:

- `phase = apply` and `validation_plan_ref` is null;
- `validation_outcome = passed` is recorded while
  `validation_plan_ref` is null (overclaiming confidence);
- `phase = keep_or_revert` and `revert_class` is
  `no_recovery_available` while `consequence_class` is not
  `irreversible_high_blast`.

### `batch_scope_record`

Typed batch-scope summary for every interaction whose target
set is more than one.

- `batch_scope_id` (opaque)
- `batch_scope_class` (closed set above)
- `query_basis_ref` (opaque; required when
  `batch_scope_class = all_matching_query` or
  `query_snapshot_stale`)
- `basis_snapshot_freshness`
  (`authoritative_live` / `warm_cached` / `degraded_cached` /
  `stale` / `unverified`)
- `member_summary`: map from `batch_member_status` to count
  (status vocabulary:
  `included`,
  `excluded_by_user_filter`,
  `blocked_by_policy`,
  `blocked_by_ownership`,
  `blocked_by_protected_path`,
  `hidden_missing_permission`,
  `hidden_stale_membership`,
  `query_derived`)
- `stale_query_marker` (boolean)
- `policy_context`
- `minted_at`

### `permission_prompt_record`

One record per prompt open. Required fields:

- `permission_prompt_id`
- `requester_identity_ref` (actor who asked)
- `requested_capability_class`
- `scope_target_ref` (opaque)
- `grant_scope_requested` (`once` / `session` / `workspace`
  / `profile` / `policy_managed`)
- `denial_fallback_label` (short productive-fallback text —
  e.g. `Continue with local-only review`)
- `revocation_path_label` (short inspector-path text —
  e.g. `Settings › Trust › Active grants`)
- `prior_grant_delta` (nullable; non-null when the prompt
  widens an existing grant — names the delta, not the full
  envelope)
- `high_risk_preview_classes` (array; re-exported)
- `authority_renewal_triggers` (array; non-empty on renewals)
- `policy_context`
- `minted_at`
- `decision_outcome` (nullable until the user decides;
  `granted` / `denied` / `cancelled` / `expired` /
  `revoked_before_commit`)
- `decision_minted_at` (nullable)

### `copy_export_representation_record`

Emitted whenever bytes leave the product on copy / export.

- `copy_export_id`
- `action_kind` (`copy` / `export`)
- `representation_class` (closed set above)
- `source_surface_class`
- `source_target_identity_ref`
- `citation_anchor_refs` (non-empty when
  `representation_class = generated` and the row quotes
  authoritative material)
- `redaction_class`
- `policy_context`
- `minted_at`

### `focus_return_record`

Emitted on every close / dismiss / commit that closes a surface.

- `focus_return_id`
- `invoker_identity_ref`
- `return_target_identity_ref` (nullable when
  `focus_return_state` is `focus_loss_denied` or
  `focus_not_applicable_non_interactive`)
- `focus_return_state` (closed set above)
- `announcement_label` (required on
  `returned_placeholder_announced` and
  `focus_loss_denied`)
- `responsive_fallback_mode`
- `minted_at`

### `responsive_fallback_record`

Emitted on every responsive-fallback engagement on a protected
surface.

- `responsive_fallback_id`
- `shell_zone` (`rail` / `sidebar` / `main_workspace` /
  `inspector` / `bottom_panel`)
- `responsive_fallback_mode` (closed set above)
- `collapsed_chrome_items_ref` (array of opaque ids)
- `preserved_required_fields` (array from the
  required-visible-field set)
- `dropped_fields_if_any` (array — MUST be empty for
  `external_shared` and `irreversible_high_blast`
  consequence classes)
- `minted_at`

### `interaction_safety_audit_event_record`

Every emission, denial, renewal, phase transition, drift, focus
return, responsive fallback engagement, and copy / export label
event emits a structured event on the `interaction_safety`
audit stream. Events MUST NOT carry raw bodies, raw paths, raw
URLs, raw prompt text, or raw credential material.

Frozen event ids:

- `interaction_safety_preview_emitted`
- `interaction_safety_apply_committed`
- `interaction_safety_apply_basis_drifted`
- `interaction_safety_validate_completed`
- `interaction_safety_revert_requested`
- `interaction_safety_batch_scope_labelled`
- `interaction_safety_batch_stale_query_detected`
- `interaction_safety_permission_prompt_opened`
- `interaction_safety_permission_prompt_granted`
- `interaction_safety_permission_prompt_denied`
- `interaction_safety_permission_prompt_expired`
- `interaction_safety_permission_prompt_revoked`
- `interaction_safety_authority_renewal_required`
- `interaction_safety_focus_returned`
- `interaction_safety_focus_return_target_lost`
- `interaction_safety_representation_label_rendered`
- `interaction_safety_safe_preview_engaged`
- `interaction_safety_copy_export_labelled`
- `interaction_safety_responsive_fallback_engaged`
- `interaction_safety_chrome_collapse_field_preserved`
- `interaction_safety_denial_emitted`
- `interaction_safety_schema_version_bumped`

## Example state diagrams

### Preview / apply / revert lifecycle

```
          ┌──────────┐
          │ propose  │  (actor named, plan labelled, scope estimate only)
          └────┬─────┘
               │ preview requested
               ▼
          ┌──────────┐          basis drift detected
          │ preview  │ ─────────────────────────────────────────┐
          └────┬─────┘                                          │
               │ user approves preview                          │
               ▼                                                ▼
          ┌──────────┐                               ┌──────────────────────┐
          │  apply   │                               │ apply_basis_drifted  │
          └────┬─────┘                               │  (reopen review)     │
               │ apply committed                    └─────────┬────────────┘
               ▼                                              │
          ┌──────────┐                                        │
          │ validate │ ───── failed / mixed ───┐              │
          └────┬─────┘                         │              │
               │ passed                        ▼              │
               ▼                       ┌───────────────┐      │
     ┌──────────────────┐              │ keep_or_revert│◀─────┘
     │ keep_or_revert   │ ────────────▶│  (named class)│
     └──────────────────┘              └───────────────┘
```

Rules honoured by the diagram:

- Apply never silently widens scope; a basis drift invalidates
  the preview and reopens review rather than performing a
  hidden best-effort apply.
- Validate distinguishes `not_run` / `running` / `passed` /
  `failed` / `mixed` / `manual_review_required`; a surface that
  renders `passed` while `validation_plan_ref` is null is
  non-conforming.
- `keep_or_revert` names the revert class (`exact_undo` /
  `compensating_action` / `regenerate_from_source` /
  `restore_from_checkpoint` / `evidence_only_no_rerun` /
  `no_recovery_available`).

### Permission / authority renewal lifecycle

```
   ┌──────────────────────┐
   │ prompt_opened        │
   └──────────┬───────────┘
              │
     ┌────────┴────────┐
     ▼                 ▼
┌──────────┐     ┌──────────┐
│ granted  │     │ denied   │ ──► surface renders denial_fallback_label
└────┬─────┘     └──────────┘        (e.g. "Continue with local-only review")
     │
     │ commit within grant scope
     ▼
┌──────────────────────────┐
│ authority_renewal_check  │ ◀────────────────────────────────┐
└──────────┬───────────────┘                                  │
           │                                                  │
           │  target_changed / route_changed / actor_changed /
           │  policy_source_changed / policy_epoch_changed /
           │  trust_state_changed / scope_filter_changed /
           │  grant_expired / grant_revoked /
           │  basis_snapshot_drifted / authority_class_escalated
           │                                                  │
           ▼                                                  │
┌──────────────────────────┐                                  │
│ renewal_required         │ ──► reopen prompt with delta ────┘
└──────────────────────────┘
```

Rules honoured by the diagram:

- A grant never silently crosses a target / route / actor /
  policy-source / epoch / trust / scope / expiry / revocation /
  basis / authority-class boundary. Any renewal trigger reopens
  the prompt with the delta (not the full envelope).

## Per-surface projection requirements (frozen)

| Surface                               | Required projected fields                                                                                                                                                                                         | Required disclosure                                                                                                                                                 |
|---------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `editor_canvas`                       | consequence class, revert class, focus-return state on every dismiss, representation class on any copy action.                                                                                                    | Bidi / invisible / confusable reveals render a safe-preview chip; save-fidelity prompts name the consequence class.                                                 |
| `terminal_canvas`                     | `multiline_terminal_paste` / `remote_clipboard_bridge` / `paste_then_run` are high-risk preview classes; the host boundary is in `visible_fields_at_commit`.                                                      | Paste-classification before execution is mandatory on multiline / rich / large / drop payloads.                                                                     |
| `review_and_diff_canvas`              | batch-scope record (included / excluded / blocked / hidden / query-derived), basis snapshot, authority renewal triggers on reapply.                                                                                | Stale query marker rendered inline; partial-apply / partial-revert outcomes are first-class banners.                                                                |
| `palette_and_search_canvas`           | `deep_link_drift_state` re-exported from ADR-0014 where the interaction is a deep-link follow; high-risk preview class `browser_handoff` when the interaction leaves the product.                                  | Policy-hidden placeholder rows render a typed count and repair hook; silent omission is forbidden.                                                                  |
| `install_update_attach_canvas`        | actor identity, publisher identity, policy source, consequence class `external_shared` or `irreversible_high_blast`, permission grant scope, high-risk preview class `install_or_update` / `remote_attach`.       | Stronger trust-decision display mode (EN.2); representation-labeled copy of the artifact digest and publisher identity.                                              |
| `ai_apply_canvas`                     | authority class `ai_initiated`, high-risk preview class `destructive_bulk_mutation` where applicable, representation class `generated` with `citation_anchor_refs` on any quoted authoritative material.          | An AI row without an authoritative anchor is denied under ADR-0013 (`derived_explanation_uncited`) before any bytes render on the apply surface.                     |
| `collaboration_canvas`                | authority class `collaboration_remote_initiated`, consequence class at least `external_shared`, recording / retention claim as a required visible field.                                                           | Grant / revoke / presenter / follow transitions render representation-labeled copy of the session identity on export.                                               |
| `provider_bearing_canvas`             | `browser_handoff_packet` envelope (ADR-0010) is the only exit; raw URL launches are forbidden; `step_up_required` renders as a representation-visible label on the prompt.                                        | Approval-ticket shape quoted by ref; raw credential / token material never crosses this surface.                                                                    |
| `docs_help_service_health_canvas`     | `citation_anchor_refs` on every representation-labeled copy of docs content; `representation_class` is explicit for every user-initiated copy.                                                                     | A copy action that cannot attribute its bytes to a citation anchor is denied with `citation_anchor_missing_on_generated`.                                           |
| `support_export_canvas`               | all required parity fields; `representation_class = sanitized` or `blocked_metadata_only` by default; raw bodies stripped; quoted owner packets (never re-minted).                                                 | A missing required field denies the export with `parity_field_missing` at the export boundary; raw bodies and raw URLs are never included.                          |

### Chip collapsing is a UI freedom; record addressability is mandatory

A surface MAY fold
`authority_class` / `consequence_class` / `revert_class` /
`representation_class` / `focus_return_state` into one chip for
dense rendering, provided the underlying packet retains each
axis as a separately addressable field. Support exports, AI
evidence, and parity audits read each axis independently.

### Responsive fallback — what must remain visible

The table below names, per consequence class, the minimum
required-visible-field set at commit. Responsive fallback MAY
collapse **other** chrome; it MUST NOT drop any field below.

| Consequence class                 | Required at commit (minimum)                                                                                                                                                                    |
|-----------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `reversible_local`                | `target_identity`, `consequence_class_label`, `recovery_class_label`.                                                                                                                            |
| `recoverable_durable`             | + `scope_statement`, `actor_identity`, `basis_snapshot_freshness`.                                                                                                                               |
| `external_shared`                 | + `authority_class_label`, `policy_source_label`, `blocked_or_hidden_member_count`, `expiry_or_revocation_claim` (when a grant was used).                                                        |
| `irreversible_high_blast`         | + `representation_class_label` (on any copy / export), focus-return state on dismiss, basis snapshot, and a visible no-recovery warning when `revert_class = no_recovery_available`.              |

## Audit, redaction, and boundary posture

Process-boundary constraints (frozen):

1. `interaction_safety_packet_record`,
   `preview_apply_revert_record`,
   `batch_scope_record`,
   `permission_prompt_record`,
   `copy_export_representation_record`,
   `focus_return_record`,
   `responsive_fallback_record`, and
   `interaction_safety_audit_event_record` cross the RPC
   boundary as typed payloads (ADR-0004). Raw bodies, raw
   paths, raw URLs, raw prompt text, and raw credential
   material never cross.
2. Mutation-journal entries, save manifests, support bundles,
   and evidence packets name `packet_id`,
   `preview_apply_revert_id`, `batch_scope_id`,
   `permission_prompt_id`, `copy_export_id`,
   `focus_return_id`,
   `responsive_fallback_id`, and
   `running_build_identity_ref` only.
3. Crash dumps and core files MUST NOT inherit unresolved
   interaction-safety packets; a crash that lands mid-apply
   discards the packet rather than persisting a partial axis
   set.
4. AI tool calls MUST NOT cache interaction-safety packets past
   the packet's freshness window without re-resolving; a cached
   packet that outlives its anchors is denied.

Redaction defaults (frozen):

| Sink                                 | Default inclusion                                                                                                                                                                  |
|--------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `logs_local`                         | Packet / session / preview / permission / copy-export / focus-return / responsive-fallback ids, surface class, authority class, consequence class, phase, representation class, audit-event ids. No raw bodies, paths, URLs, or prompt text. |
| `traces_local`                       | Same as `logs_local`; span names MUST NOT include raw bodies or paths.                                                                                                              |
| `support_bundle`                     | Full per-axis values, full batch-member summary, full required-visible-field set, full revert-class enumeration, full citation-anchor enumeration. Raw bodies excluded.             |
| `evidence_packet`                    | Release-relevant fields: `running_build_identity_ref`, consequence class, revert class, representation class, full `citation_anchor_refs`. Raw bodies never included.                |
| `ai_context_capture`                 | Packet / preview ids, consequence class, revert class, representation class, citation anchors. Raw bodies and prompt text never captured.                                           |
| `recipe_manifest`                    | `packet_id`, `preview_apply_revert_id`, `running_build_identity_ref`. Raw bodies forbidden.                                                                                          |
| `profile_export` / `sync`            | Same as `recipe_manifest`.                                                                                                                                                          |
| `crash_dump`                         | Opt-in only; redaction scan precedes packaging; denied by default for packets whose `policy_context` references a managed policy bundle.                                            |
| `mutation_journal_entry`             | Ids, phase, consequence class, revert class, representation class. No raw bodies or URLs.                                                                                            |
| `save_manifest` (ADR-0006)           | Same as `mutation_journal_entry`.                                                                                                                                                   |
| `claim_manifest`                     | Full per-axis values, full citation-anchor enumeration. Raw bodies never included.                                                                                                  |
| `terminal_transcript`                | `packet_id` and `surface_class` only; raw URLs require boundary-labelled confirmation before capture.                                                                                |

Overrides are narrowing only; admin policy MAY reduce inclusion
further, but MAY NOT widen beyond the frozen exclusion rules.

## Schema-of-record posture

The eventual shell / interaction-safety crate's Rust types are
the source of truth. The JSON Schema export at
`schemas/ux/interaction_safety.schema.json` is the cross-tool
boundary every non-owning surface reads. Adding a new authority
class, consequence class, high-risk preview class, preview /
apply / revert phase, revert class, permission grant scope,
authority-renewal trigger, representation class, batch scope
class, focus-return state, responsive-fallback mode,
required-visible-field class, audit-event id, or denial reason
is additive-minor and bumps
`interaction_safety_schema_version`; repurposing any existing
value is breaking and requires a new decision row.

There is no external IDL or code-generator toolchain at this
milestone; this mirrors ADR 0004 through ADR 0014.

## Non-goals at this milestone

Out of scope until a superseding decision row opens:

- Full UI implementation of preview / apply / revert sheets,
  permission prompts, focus-return traversal, copy-export
  chips, or responsive-fallback ladders. This contract reserves
  the axes; the shell / review / AI / extension / provider
  surfaces wire them later.
- The eventual shell crate's Rust types; the JSON Schema export
  reserves the boundary shape until the crate lands.
- The connected-provider flow body (ADR-0010 reserves it).
- The extension effective-permission resolver body (ADR-0012
  reserves it).
- The search deep-link resolver body (ADR-0014 reserves it).

These lines move only by opening a new decision row, not by
editing this contract.

## Reuse guarantee

This contract is reusable by shell, review, AI, extension, and
provider-bearing flows without redefining core prompt or preview
semantics. A new protected surface MUST:

1. Quote the authority / consequence / revert / representation
   vocabularies above verbatim.
2. Emit `interaction_safety_packet_record` on every
   consequence-bearing interaction; emit the per-phase
   `preview_apply_revert_record` for every phase it owns; emit
   `batch_scope_record` on every >1-target interaction; emit
   `permission_prompt_record` on every prompt; emit
   `copy_export_representation_record` on every copy / export;
   emit `focus_return_record` on every close; emit
   `responsive_fallback_record` on every fallback engagement.
3. Preserve each axis as a separately addressable field on the
   packet even when the UI folds them into one chip.
4. Honour the renewal / drift / denial posture above; silent
   widening is non-conforming.
