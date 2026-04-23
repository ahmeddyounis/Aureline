# Clipboard, drag-drop, undo-group, reopen-history, and large-transfer feedback contract

This document is the **shell-wide contract** for the high-
frequency editing and navigation behaviors that every protected
Aureline surface shares — clipboard copy / paste, drag and drop,
undo / redo grouping, reopen-history and local-history lineage,
and long-running paste / import / attach / drop feedback. It
exists so editor, terminal, search, review, settings import,
install / attach, AI apply, extension web view, and future
automation surfaces use **one vocabulary, one set of rules, and
one lineage story** instead of minting surface-local conventions
for actions the user performs dozens of times an hour.

The contract is normative. Where this document disagrees with
the PRD, TAD, TDD, or UI/UX spec, those sources win and this
document plus its companion artifacts MUST update in the same
change. Where this document disagrees with a downstream
surface's private clipboard, drag-drop, history, or large-
transfer story, this document wins and the surface is non-
conforming.

The companion artifacts are:

- [`/artifacts/ux/undo_group_examples.yaml`](../../artifacts/ux/undo_group_examples.yaml)
  — worked undo-group lineage examples for grouped multi-file
  actions, AI / extension apply attribution, no-undo preview or
  checkpoint paths, and reopen / recover expectations, with at
  least one example that shows undo, reopen-history, and
  local-history checkpoint truth for the same mutating
  operation.
- [`/fixtures/ux/dragdrop_cases/`](../../fixtures/ux/dragdrop_cases/)
  — drag-and-drop worked examples covering the typed result
  verbs, insertion-preview requirements, modifier-key cues, and
  cross-window detach rules for editor, explorer, terminal,
  review, and install / attach drop zones.

This contract rides alongside — it does **not** re-mint — the
vocabularies already frozen in:

- [`/docs/ux/shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md)
  — authority class, consequence class, high-risk preview class,
  preview / apply / revert phase, revert class, permission
  grant scope, representation class, batch scope class, focus-
  return state, responsive-fallback mode, and the
  required-visible-field set. Every clipboard / drag-drop /
  large-transfer interaction in this contract resolves to a
  surface class and emits the
  `interaction_safety_packet_record` envelope; this document
  names **what clipboard / drag-drop / history rules the
  interaction MUST honour**, not a parallel safety vocabulary.
- [`/docs/adr/0003-buffer-undo-large-file.md`](../adr/0003-buffer-undo-large-file.md)
  and
  [`/artifacts/architecture/undo_class_rows.yaml`](../../artifacts/architecture/undo_class_rows.yaml)
  — undo-class ids (`text_edit`, `multi_cursor_text_edit`,
  `structural_edit`, `refactor_single_file`,
  `refactor_multi_file`, `formatter_run`,
  `save_participant_group`, `imported_change`,
  `machine_generated_change`, `migration_change`,
  `external_reload`, `decode_recovery_change`),
  compensation postures (`compensatable` / `only_revertible`),
  reserved journal fields, and the workspace-level
  multi-file-history reservation. This contract cites the
  class ids verbatim and never re-mints compensation posture.
- [`/docs/verification/source_fidelity_and_undo_packet.md`](../verification/source_fidelity_and_undo_packet.md)
  and
  [`/artifacts/io/save_rewrite_classes.yaml`](../../artifacts/io/save_rewrite_classes.yaml)
  — rewrite-class and recovery-class vocabulary
  (`exact_undo`, `compensating_rollback`,
  `regenerate_from_canonical_source`,
  `restore_from_checkpoint`, `evidence_only_no_rerun`,
  `no_recovery_available`). Clipboard / drop / large-transfer
  apply flows that produce a mutation journal entry name the
  same recovery class on the entry and on the user-facing
  label.
- [`/docs/verification/text_fidelity_packet.md`](../verification/text_fidelity_packet.md)
  — source-fidelity posture for encoding, BOM, newline mode,
  and lossy decode / re-encode. Paste flows that cannot
  round-trip bytes exactly name the same save-consequence
  class on the packet and on the clipboard review surface;
  this contract does not re-mint the fidelity taxonomy.
- [`/docs/security/suspicious_content_packet.md`](../security/suspicious_content_packet.md)
  — suspicious-content reveal vocabulary
  (`bidi_or_invisible_formatting_reveal`,
  `confusable_identifier_reveal`,
  `rich_active_content_render`, etc.). Safe representation of
  suspicious or sensitive clipboard values resolves to these
  reveal classes; this document does not mint new reveal
  vocabulary.
- [`/docs/ux/state_and_recovery_taxonomy.md`](./state_and_recovery_taxonomy.md)
  and
  [`/artifacts/ux/failure_tier_matrix.yaml`](../../artifacts/ux/failure_tier_matrix.yaml)
  — failure-tier placement (`tier.inline_issue`,
  `tier.contextual_degraded`, `tier.workflow_block`,
  `tier.session_recovery`, `tier.escalation_surface`), the
  controlled labels `Partially ready` / `Degraded` /
  `Read-only degraded`, and the ten-token degraded-state
  vocabulary. Long-running paste / import / attach / drop
  feedback lands on these tiers; this contract does not mint
  new tiers.
- [`/docs/ux/navigation_and_escalation_contract.md`](./navigation_and_escalation_contract.md)
  — inline-first escalation ladder (`inline`,
  `contextual_overlay`, `panel`, `window_attached_sheet`,
  `dialog_modal`, `full_surface_takeover`) and
  progressive-disclosure depths
  (`summary`, `detail`, `evidence`, `inspection`). Paste
  review, drop preview, and undo-group inspection escalate
  along that ladder only.
- [`/docs/ux/attention_activity_taxonomy.md`](./attention_activity_taxonomy.md)
  — delivery-surface classes, interruptibility tiers, and
  durable-job-row rules. Long-running paste / import / attach
  / drop work mirrors to the durable surface set named there;
  toast-only rendering of durable-tier work denies with
  `toast_only_forbidden_for_durable_work`.
- [`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md)
  — `browser_handoff_packet`. A clipboard payload that points
  outside the product (permalink, handoff URL) leaves via the
  handoff packet; raw URL launches are forbidden.
- [`/docs/accessibility/a11y_ime_packet_template.md`](../accessibility/a11y_ime_packet_template.md)
  — keyboard completeness, focus order / return, screen-reader,
  reduced motion, IME / bidi input, and accessibility-tree
  capture. Every clipboard action, drop affordance, undo-group
  reveal, and large-transfer progress surface reserves an
  accessibility hook from this packet.

## Who reads this document

- **Product writers** drafting copy / paste menu items,
  drag-drop result verbs, undo-history row labels, reopen
  menus, and long-paste / import / attach progress copy on any
  protected shell surface. Writers quote the variant id for
  axis alignment and let the UX Style Guide decide the final
  words.
- **Editor, terminal, explorer, search, review, settings-
  import, install-attach, and AI-apply surface authors**
  wiring named copy variants, drop zones, undo-group names,
  reopen menus, and durable progress mirrors. Authors pick
  from the closed sets here and open a new decision row rather
  than minting a surface-local verb.
- **Support, parity-audit, and diagnostics tooling** reading
  each axis mechanically — every axis is separately
  addressable even when the surface folds it into one chip or
  one history row.
- **Accessibility reviewers** reading the keyboard-completeness
  and announcement rules on clipboard / drop / large-transfer
  surfaces.

## 1. Scope

- One **clipboard representation contract** naming the default
  plain-text representation, the additive rich-text rule, the
  safe representation for suspicious or sensitive values, and
  the closed set of named copy variants (line, path,
  relative path, permalink, command id, diagnostic details,
  and peer variants).
- One **drag-and-drop contract** naming the closed set of
  result verbs, the insertion-preview requirement, the
  modifier-key cue table, and the cross-window detach rule.
- One **undo-group and reopen-history contract** for grouped
  multi-file actions, AI or extension apply attribution, the
  no-undo preview / checkpoint requirement, and the reopen /
  recover expectations on high-frequency surfaces.
- One **large-transfer feedback contract** for long paste,
  import, attach, or drop operations — including the progress
  surface, the durable mirror, the cancel / repair route, and
  the basis-drift refusal.
- One **lineage-cross-link rule** that makes undo, reopen-
  history, and local-history checkpoint claims resolve to the
  same mutation journal entry for every mutating operation.

## 2. Out of scope

- Full OS-specific clipboard integration on every platform
  during M0 (per the spec `Out of scope`). This contract pins
  the representation classes, named variants, lineage rules,
  and drop verbs; platform-adapter work decides which system
  clipboard formats carry them.
- Final microcopy and localization. The contract pins
  variants, axes, and escalation paths; the UX Style Guide and
  localization work own the rendered words.
- The palette / command-router / onboarding implementations.
  This contract reserves command ids and variant ids; the
  command registry owns the invocation surface.
- New undo classes, recovery classes, representation classes,
  failure tiers, interruptibility tiers, or escalation ladder
  rungs. Every row resolves to the already-frozen vocabularies
  in §3. A row that needs a new value opens a new decision row
  rather than landing here.

## 3. Frozen vocabulary (re-exported)

Every clipboard / drag-drop / undo-group / reopen-history /
large-transfer row in this contract resolves to values from the
following already-frozen vocabularies. Minting a value outside
these sets is non-conforming and opens a new decision row.

- **Authority, consequence, revert, representation, batch
  scope, focus-return, responsive-fallback, and denial-reason
  vocabularies** — from
  [`shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md).
- **Undo-class ids and compensation postures** — from
  [`undo_class_rows.yaml`](../../artifacts/architecture/undo_class_rows.yaml)
  (`text_edit`, `multi_cursor_text_edit`, `structural_edit`,
  `refactor_single_file`, `refactor_multi_file`,
  `formatter_run`, `save_participant_group`,
  `imported_change`, `machine_generated_change`,
  `migration_change`, `external_reload`,
  `decode_recovery_change`;
  `compensatable` / `only_revertible`).
- **Recovery-class tokens** — from
  [`source_fidelity_and_undo_packet.md`](../verification/source_fidelity_and_undo_packet.md)
  (`exact_undo`, `compensating_rollback`,
  `regenerate_from_canonical_source`,
  `restore_from_checkpoint`, `evidence_only_no_rerun`,
  `no_recovery_available`).
- **High-risk preview classes** — from
  [`shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md)
  (`multiline_terminal_paste`, `remote_clipboard_bridge`,
  `paste_then_run`, `file_drop_mass_mutation`,
  `rich_active_content_render`, `notebook_widget_payload`,
  `oversized_generated_artifact`,
  `bidi_or_invisible_formatting_reveal`,
  `confusable_identifier_reveal`, `secret_access`,
  `browser_handoff`, `destructive_bulk_mutation`).
- **Failure tiers and controlled labels** — from
  [`state_and_recovery_taxonomy.md`](./state_and_recovery_taxonomy.md)
  (`tier.inline_issue`, `tier.contextual_degraded`,
  `tier.workflow_block`, `tier.session_recovery`,
  `tier.escalation_surface`; `Partially ready`, `Degraded`,
  `Read-only degraded`).
- **Delivery-surface classes and interruptibility tiers** —
  from
  [`attention_activity_taxonomy.md`](./attention_activity_taxonomy.md)
  (`toast`, `contextual_banner`, `status_item`,
  `durable_job_row`, `attention_item`,
  `activity_center_digest_card`, `digest_group_row`;
  `tier_ambient`, `tier_transient`, `tier_durable`,
  `tier_actionable`, `tier_blocking_trust`,
  `tier_critical_safety`).
- **Escalation-ladder positions** — from
  [`navigation_and_escalation_contract.md`](./navigation_and_escalation_contract.md)
  (`inline`, `contextual_overlay`, `panel`,
  `window_attached_sheet`, `dialog_modal`,
  `full_surface_takeover`).

## 4. Truthfulness posture (normative)

Every clipboard copy, paste preview, drag-drop drop, undo-group
row, reopen-history row, and large-transfer progress surface
obeys the following posture. A surface whose rendered state
violates any of these rules is non-conforming and denies with
the stated reason.

1. **Copy and export never optimise for pretty presentation at
   the expense of safe or useful downstream text.** The default
   representation of a text selection is the exact bytes that
   the user selected (`representation_class = raw`), not a
   rendered preview, not a syntax-highlighted HTML blob, and
   not a pasted-into-Slack Markdown rewrite. Rich-text /
   rendered / sanitised forms are **additive** — they appear
   as named copy variants or as additional clipboard flavours,
   never as the silent default. A surface that silently
   replaces `raw` with `rendered` or `sanitized` denies with
   `representation_escalation_required`.
2. **Named copy variants are typed.** Every named copy variant
   resolves to a stable `copy_variant_id` from §5 and a stable
   `command_id` on the command graph. A menu item labelled
   "Copy path" that resolves to a free-form label rather than
   `copy.variant.absolute_path` denies with
   `copy_variant_id_unresolved`.
3. **A drop must disclose what it will do before it commits.**
   Every drop target renders a typed `drop_result_verb` and an
   insertion-preview before the pointer releases. A drop that
   commits without a preview (e.g. a finger-trap drop onto an
   overlapping zone) denies with
   `drop_result_verb_not_disclosed`.
4. **An undo group must disclose its scope and recovery
   class.** Every undo history row names the undo-class id,
   the originator, the recovery class, and the attribution —
   including the agent / invocation ids for AI or extension
   apply. A row rendered as `Undo` alone on an
   `only_revertible` group denies with
   `undo_label_hides_recovery_class`.
5. **No-undo paths show preview or checkpoint before commit.**
   Any path whose undo class is `only_revertible` (workspace-
   scoped refactor, save participant group, imported change,
   machine-generated change, migration change, external
   reload, decode recovery) MUST render a preview and take a
   checkpoint before the commit runs. A commit that runs
   against an `only_revertible` class without a preview or
   checkpoint denies with
   `no_undo_path_missing_preview_or_checkpoint`.
6. **Long transfers mirror to a durable surface.** Any paste /
   import / attach / drop operation that crosses the
   `tier_durable` threshold mirrors to a durable surface —
   activity centre, durable job row, or status item — even
   when the local surface also shows progress. A toast-only
   long transfer denies with
   `toast_only_forbidden_for_durable_work`.
7. **Suspicious or sensitive clipboard payloads render a safe
   preview before they commit.** Multiline terminal paste,
   remote clipboard bridge, paste-then-run, file drop mass
   mutation, rich active content, notebook widget payload,
   oversized generated artifact, bidi / invisible formatting,
   confusable identifiers, secret access, and browser handoff
   payloads render the typed safe-preview disclosure on the
   primary surface. A tooltip-only or hover-only disclosure
   denies with `safe_preview_bypassed`.
8. **Lineage is cited by id across undo, reopen, and local-
   history.** Every mutating clipboard / drop / import apply
   writes one mutation-journal entry; the undo history row,
   the reopen-history row, and the local-history checkpoint
   row for that operation cite the **same** entry id. A
   reopen-history row that invents its own id denies with
   `reopen_history_lineage_forked_per_surface`.
9. **Keyboard-reachable and announced.** Every clipboard
   variant, drop affordance, undo-group reveal, reopen-history
   row, and large-transfer progress surface MUST be
   keyboard-reachable through the focus order that the
   accessibility packet covers, and its announcement label
   MUST be addressable by assistive technology. A surface that
   renders recovery only on hover or only via a drag
   affordance denies with
   `state_row_recovery_not_keyboard_reachable`.

## 5. Clipboard representation contract

### 5.1 Default representation

- **`representation_class` at copy.** The default
  representation of any selection is `raw` — the exact bytes
  the selection contained. Encoding, BOM, newline mode, and
  final-newline posture are preserved per the text-fidelity
  packet; a surface that silently canonicalises encoding or
  newline mode on copy denies with `text_fidelity_lost_on_copy`.
- **Additive rich-text.** A surface MAY emit additional
  clipboard flavours alongside `raw` — for example a
  `rendered` HTML snapshot for display surfaces, an `escaped`
  payload for chat / logs, or a `sanitized` snapshot for
  external-shared copy. These are **additive**; the primary
  flavour remains `raw`. A surface that omits `raw` and offers
  only `rendered` / `sanitized` as the default denies with
  `representation_escalation_required`.
- **Packet emission.** Every copy action emits a
  `copy_export_representation_record`
  (`action_kind = copy`) naming the `representation_class`,
  the `source_surface_class`, the `source_target_identity_ref`,
  the `citation_anchor_refs` (when quoting authoritative
  material), and the `redaction_class`. Raw bodies, raw
  paths, and raw URLs never cross the packet boundary.

### 5.2 Safe representation for suspicious or sensitive values

- **Suspicious-content labels are preserved.** Selections that
  contain bidi / invisible formatting or confusable
  identifiers render a `bidi_or_invisible_formatting_reveal`
  or `confusable_identifier_reveal` chip on the primary
  surface before the copy action commits. The
  `high_risk_preview_classes` array on the packet carries the
  same value; a surface that drops the reveal class on copy
  denies with `safe_preview_bypassed`.
- **Secret-access material.** A surface that copies from a
  secret-bearing row (credential-handle reveal, token preview,
  managed-secret inspector) copies the **handle** via
  `representation_class = blocked_metadata_only` by default;
  the raw credential material is reachable only via the
  credential-handle reveal contract (ADR-0007). A copy action
  that dumps raw credential material denies with
  `raw_body_forbidden_on_boundary`.
- **Rich active content, notebook widgets, oversized
  generated artifacts.** Default to `sanitized` or
  `blocked_metadata_only` for these; the raw / rendered /
  active flavour is reachable via a named variant (§5.3) but
  is not the default.
- **AI-generated content that quotes authoritative material.**
  Copies with `representation_class = generated` MUST carry a
  non-empty `citation_anchor_refs`. A generated copy without
  anchors denies with `citation_anchor_missing_on_generated`.
- **External-shared and irreversible-high-blast copy.** When
  the copy target is inferred external / shared (publish
  surface, public clipboard share, browser handoff), the
  surface forces a preview sheet that renders the
  `representation_class_label` as a required-visible field
  per the responsive-fallback table.

### 5.3 Named copy variants (closed set)

Every named copy variant is a stable `copy_variant_id` with a
fixed `representation_class` default and a fixed
`command_id_ref`. Surfaces MUST cite the id; menu labels are
UX Style Guide work and MUST NOT be the only path to the
variant.

| `copy_variant_id`                | Default `representation_class`  | Intent                                                                                              |
|----------------------------------|---------------------------------|-----------------------------------------------------------------------------------------------------|
| `copy.variant.selection_raw`     | `raw`                           | The bare default — exact selected bytes, no rendering, no wrapping, no syntax highlighting.         |
| `copy.variant.line`              | `raw`                           | The entire line(s) the cursor(s) cover, even when the selection is zero-width. Preserves EOL mode. |
| `copy.variant.absolute_path`     | `raw`                           | The target's absolute presentation path (file URI or platform path).                                |
| `copy.variant.relative_path`     | `raw`                           | The target's path relative to the workspace root (never the URI).                                   |
| `copy.variant.permalink`         | `raw`                           | Stable navigation link (workspace-URI or deep-link) resolving to the same target across sessions.   |
| `copy.variant.command_id`        | `raw`                           | The stable `command_id` for a command palette / keybinding / menu entry.                            |
| `copy.variant.diagnostic_details`| `sanitized`                     | The diagnostic summary / class / code / monotonic timestamp for a failing row; redaction applies.   |
| `copy.variant.rendered_preview`  | `rendered`                      | The surface's rendered form (HTML / styled text) — additive, never the default.                     |
| `copy.variant.escaped_for_log`   | `escaped`                       | The selection with metacharacters made safe for pasting into logs or chat.                          |
| `copy.variant.reference_anchor`  | `raw`                           | A citation-anchor ref (docs / help content); MUST carry `citation_anchor_refs` non-empty.           |
| `copy.variant.support_export`    | `sanitized` / `blocked_metadata_only` | Sanitised metadata export for support bundles / evidence packets; raw bodies stripped.         |

Rules (frozen):

1. A variant MAY escalate its representation class (e.g.
   `copy.variant.line` from `raw` to `escaped` because the
   destination surface labels it as chat-safe); it MAY NOT
   silently narrow to a lossy form. Silent narrowing denies
   with `representation_escalation_required`.
2. `copy.variant.permalink` MUST resolve through the permalink
   contract — it is never a raw URL launch and never a
   free-text string. A surface that emits a bare external URL
   as a permalink denies with
   `raw_url_forbidden_on_boundary`.
3. `copy.variant.diagnostic_details` MUST preserve the
   last-failure reason (class / code / monotonic timestamp /
   recovery-rung history) per the state-and-recovery taxonomy
   §8.7.
4. `copy.variant.support_export` is the only variant permitted
   to emit `blocked_metadata_only`; every other variant MUST
   carry an addressable payload.
5. Every variant's `command_id_ref` is reachable without a
   pointer; the palette and keybinding resolver resolve the
   same id.

## 6. Drag-and-drop contract

### 6.1 Drop result verbs (closed set)

Every drop target renders exactly one of the closed set below
while the pointer is over it. Rendering a drop with no
`drop_result_verb` denies with `drop_result_verb_not_disclosed`.

- `move` — the dragged object moves from its origin to the
  drop target; the origin no longer contains it.
- `copy` — the dragged object is duplicated into the drop
  target; the origin is unchanged.
- `link` — the drop creates a reference / symbolic link / cite
  at the drop target; neither source nor target bytes change.
- `open` — the drop opens the source in the target viewer /
  surface without mutating either side.
- `import` — the drop ingests the source into the workspace
  (creating files, settings rows, or review artifacts);
  resolves through `imported_change` or
  `migration_change` undo classes.
- `attach` — the drop attaches the source as an evidence /
  attachment to the target object (review comment, support
  bundle, notebook cell).
- `reveal` — the drop reveals the source in a navigator
  (explorer / outline / breadcrumb); never mutates bytes.
- `blocked` — the drop target refuses the payload and renders
  the block reason (policy, ownership, protected path,
  unsupported on surface).

### 6.2 Modifier-key cues (closed table)

Modifier keys never change the result silently. While the
pointer is over a drop target, the surface MUST render the
resolved verb and a modifier-key cue list. A drop that resolves
one verb while rendering another denies with
`drop_result_verb_modifier_mismatch`.

| Modifier                    | Default verb over a writable target | Cue rendered |
|-----------------------------|-------------------------------------|--------------|
| (none)                      | `move` (intra-workspace, same root) | "Move"       |
| (none)                      | `copy` (cross-workspace / external) | "Copy"       |
| Option / Alt                | forces `copy`                       | "Copy (hold Option)" |
| Command / Ctrl              | forces `link`                       | "Link (hold Ctrl/⌘)" |
| Command+Option / Ctrl+Alt   | forces `import`                     | "Import (hold ⌘+Option)" |
| Shift                       | forces `open`                       | "Open (hold Shift)" |
| (any) over protected path   | `blocked`                           | "Blocked — protected path" |
| (any) over unsupported zone | `blocked`                           | "Blocked — unsupported on this surface" |

Rules (frozen):

1. The resolved verb and the cue are addressable by assistive
   technology. A cue rendered only as a cursor tint denies
   with `state_row_recovery_not_keyboard_reachable`.
2. Platform-adapter work MAY project the modifier labels into
   OS-local glyphs; the underlying resolved verb and the
   `drop_result_verb_id` do not change.

### 6.3 Insertion-preview requirement

Every drop that mutates (any verb other than `open`, `reveal`,
or `blocked`) renders an insertion preview before the pointer
releases, naming:

- The **target surface** and **slot** the payload lands in
  (`editor`/line-range, `explorer`/folder-row,
  `review`/threaded-reply, `install_attach`/attach-row,
  `terminal`/paste-buffer, etc.).
- The **payload summary** — item count, total bytes estimate,
  high-risk preview class(es) if any, and the resolved
  `representation_class` when bytes will be pasted.
- The **consequence class** (`reversible_local`,
  `recoverable_durable`, `external_shared`, or
  `irreversible_high_blast`) from the interaction-safety
  contract.
- The **recovery class** (`exact_undo`, `restore_from_checkpoint`,
  `compensating_rollback`, `evidence_only_no_rerun`,
  `no_recovery_available`, ...). A drop that advertises
  `Undo` while the resolved class is `restore_from_checkpoint`
  denies with `undo_label_hides_recovery_class`.

A drop surface that cannot render the insertion preview (e.g.
the target is outside the visible viewport and no preview
affordance exists) MUST either provide a side-car preview via
`contextual_overlay`, or refuse the drop with
`drop_result_verb_not_disclosed`.

### 6.4 Cross-window detach rules

A drop operation that detaches into a new product window (tear
off a tab, pull a panel into its own window, open a preview in
a companion window) MUST:

1. Preserve the invoker's focus anchor so focus-return after
   the drop resolves to the invoking row (`returned_exact`).
   A detach that abandons the invoker denies with
   `focus_return_target_lost`.
2. Carry the same `interaction_session_id_ref` across the
   detach so the undo-history row, reopen-history row, and
   large-transfer progress surface in the new window cite
   the same lineage.
3. Preserve the responsive-fallback posture invariants — the
   new window's `visible_fields_at_commit` MUST include every
   field required for the consequence class; a narrow new
   window that would hide a required field refuses to
   commit with `chrome_hid_required_field`.
4. Emit a `responsive_fallback_record` when the detach engages
   a different fallback mode on the new window. A detach that
   drops the record denies with
   `responsive_fallback_record_missing`.
5. Decline when the host platform cannot honour cross-window
   detach (mobile / web modes) — the drop either stays in the
   original window or refuses with a
   `unsupported_on_this_surface` reason. A silent no-op denies
   with `read_only_degraded_silent_noop`.

## 7. Undo / redo and history contract

### 7.1 Per-group axes (required)

Every undo history row carries the following axes. A row that
omits any axis denies with `undo_group_axis_missing`.

- **`undo_class_id`** — one of the class ids in
  `undo_class_rows.yaml`. `Undo text edit`, `Undo refactor`,
  `Undo AI apply`, etc. resolve to `text_edit`,
  `refactor_single_file` / `refactor_multi_file`,
  `machine_generated_change` respectively. Free-text classes
  are non-conforming.
- **`undo_group_id`** — the per-buffer group id; for
  workspace-scoped groups, paired with a
  `workspace_group_id` from the workspace-history
  reservation.
- **`originator`** — stable originator string (e.g.
  `user_keystroke`, `command:rename_symbol_workspace`,
  `ai_apply`, `save_participant:format_on_save`,
  `vfs_external_change`, `settings_import`).
- **`compensation_posture`** — `compensatable` or
  `only_revertible` (from the undo-class row). An undo label
  that reads `Undo` for an `only_revertible` group MUST also
  disclose the recovery class (`restore_from_checkpoint`,
  `compensating_rollback`, `regenerate_from_canonical_source`,
  `evidence_only_no_rerun`, `no_recovery_available`). Flat
  `Undo` on `only_revertible` denies with
  `undo_label_hides_recovery_class`.
- **`recovery_class`** — `exact_undo` on pure compensatable
  groups; otherwise the specific rewrite / recovery token.
- **`attribution`** — for `machine_generated_change`, the
  `agent_id` and `invocation_id`; for
  `refactor_multi_file`, the command id + preview handle;
  for `save_participant_group`, the ordered participant list;
  for `imported_change`, the import source and kind; for
  `migration_change`, the migration id and version pair. A
  row missing required attribution denies with
  `agent_attribution_lost` (machine) or
  `originator_attribution_lost` (others).
- **`mutation_journal_entry_ref`** — the journal entry id
  (`mutation_id`) every surface cites for lineage. This is
  the shared id (§7.6).
- **`checkpoint_refs`** — the checkpoint handles the group
  relies on for recovery; REQUIRED for every
  `only_revertible` group. A row missing the checkpoint ref
  denies with `no_undo_path_missing_preview_or_checkpoint`.
- **`review_artifact_ref`** — the durable review artifact for
  broad-scope apply paths (multi-file refactor, AI apply
  across files, settings import). REQUIRED when the
  workspace-history reservation mandates one.

### 7.2 Grouped multi-file actions

- **Per-buffer groups.** Every buffer carries its own
  `undo_group_id`. Single-buffer undo operates per buffer.
- **Workspace-level groups.** Cross-buffer groups
  (`refactor_multi_file`, `save_participant_group` spanning
  multiple buffers, `machine_generated_change` spanning
  multiple buffers, `migration_change`, and
  `imported_change` spanning multiple buffers) live in the
  workspace-level history and reference per-buffer
  `undo_group_id`s per the workspace-history reservation.
- **Promotion to preview / checkpoint.** Every workspace-
  level group MUST take a checkpoint before apply and MUST
  render a preview (the durable review artifact) the user
  approved. A workspace group without both denies with
  `no_undo_path_missing_preview_or_checkpoint`.
- **Redo invalidation.** A divergent edit in any member
  buffer drops the redo stack for the workspace group (per
  `cross_buffer_redo_stack_loss` in the undo-class rows). The
  history surface names the invalidation explicitly ("Redo
  invalidated by local edit in <other member buffer>"); a
  silent redo-stack drop denies with
  `redo_stack_dropped_without_disclosure`.

### 7.3 AI / extension apply attribution

- **Required attribution axes.** Every apply row with
  `undo_class_id = machine_generated_change` renders
  `agent_id`, `invocation_id`, `authority_class =
  ai_initiated` / `extension_initiated`, and the preview
  handle. A row missing any denies with
  `agent_attribution_lost`.
- **Citation.** AI apply rows that quote authoritative
  material render `representation_class = generated` on the
  apply packet and carry non-empty `citation_anchor_refs` on
  the derived explanation. A row without anchors denies with
  `citation_anchor_missing_on_generated`.
- **Revert class.** AI / extension apply defaults to
  `restore_from_checkpoint`; the UI labels it accordingly
  ("Roll back AI apply" rather than "Undo") and names the
  checkpoint handle on reveal.
- **Attribution preserved on export.** Support bundles,
  mutation-journal rows, and evidence packets preserve
  `agent_id` and `invocation_id` by ref. Redaction applies to
  prompt / input text; never to attribution ids.

### 7.4 No-undo preview / checkpoint requirement

Paths whose undo class is `only_revertible` — listed verbatim
here so surface authors do not guess:

- `refactor_multi_file`
- `save_participant_group`
- `imported_change`
- `machine_generated_change`
- `migration_change`
- `external_reload`
- `decode_recovery_change`

Every apply on these paths MUST:

1. Emit a preview packet (`phase = preview` on the
   `preview_apply_revert_record`) — the surface renders the
   preview the user approves before any bytes commit.
2. Take a checkpoint (`checkpoint_refs` non-empty) — the
   mutation journal entry carries the checkpoint handle.
3. Render the recovery class verbatim on the history row
   (`Roll back apply`, `Restore from checkpoint`,
   `Regenerate from canonical source`,
   `Evidence only — re-run not available`, or
   `No recovery available`). Labels that flatten to `Undo`
   deny with `undo_label_hides_recovery_class`.

### 7.5 Reopen / recover on high-frequency surfaces

- **Reopen-closed-buffer.** The editor exposes a reopen
  history with stable `mutation_journal_entry_ref` lineage.
  Reopening a buffer restores the invoker focus anchor
  (`returned_exact` on the focus-return record) and re-
  attaches to the buffer's per-buffer undo group id; the
  reopened surface MUST NOT invent a new undo group for the
  same operation.
- **Reopen-recent-workspace.** Workspace startup cites the
  entry-restore token set per the entry / restore truth
  audit. A reopen that silently replays workspace-scoped
  groups without disclosing the lineage denies with
  `reopen_history_lineage_forked_per_surface`.
- **Reopen AI / extension apply.** Reopening an AI or
  extension apply row from the activity centre or the review
  surface MUST re-cite the same `invocation_id` / `agent_id`
  / mutation journal entry; a reopen that mints a fresh id
  denies with `reopen_history_lineage_forked_per_surface`.
- **Reopen settings import / migration.** Reopening the
  migration report in the activity centre cites the same
  `migration_id` + `mutation_journal_entry_ref`; the reopen
  row offers `review_migration_report`, `roll_back_import`,
  and `keep_imported_state` next-step decision hooks from
  the frozen set.
- **Local-history compare / restore.** Every mutating row
  renders a `Compare with local history`,
  `Restore from checkpoint`, and `Reveal mutation journal
  entry` affordance (keyboard-reachable) when a checkpoint
  exists. Affordances cite the same
  `mutation_journal_entry_ref` as the undo row and the
  reopen row.

### 7.6 Lineage-cross-link rule (normative)

For every mutating clipboard / drop / apply operation, the
following identity invariants hold. A surface that violates any
invariant denies with
`reopen_history_lineage_forked_per_surface`.

1. **One mutation journal entry per operation.** The journal
   writes exactly one `mutation_journal_entry` for the
   operation; its `mutation_id` is the canonical lineage id.
2. **Same id on every derived row.** The undo-history row,
   the reopen-history row, the activity-centre durable job
   row, the local-history compare row, and the support-
   bundle / evidence-packet export all cite
   `mutation_journal_entry_ref = <mutation_id>`.
3. **Checkpoint identity follows the same id.** Every
   checkpoint created for the operation cites the same
   `mutation_id` through its `checkpoint_ref.mutation_id`
   binding; a reopen that restores from the checkpoint
   resolves back to the same entry.
4. **Recovery-class agreement.** The recovery class on the
   mutation journal entry, the recovery class rendered on
   the undo-history row, and the recovery class rendered on
   the reopen-history row MUST match. A mismatch denies with
   `recovery_class_disagreement_across_surfaces`.

The companion artifact
[`/artifacts/ux/undo_group_examples.yaml`](../../artifacts/ux/undo_group_examples.yaml)
pins at least one worked example that exercises all four
invariants for the same mutating operation.

## 8. Large-transfer feedback contract

A large transfer is any paste, import, attach, or drop whose
estimated cost crosses the `tier_durable` threshold in the
attention / activity taxonomy (the concrete byte / row /
duration threshold resolves there, not here). Large transfers
use the following feedback rules.

### 8.1 Progress surface

- **Primary surface.** A
  `top_of_pane_progress_indicator` on the destination surface
  (editor tab, terminal pane, explorer folder, review
  surface, install / attach surface). The indicator names
  **what is transferring** (item count + aggregate byte
  estimate), **the resolved consequence class**, and **the
  expected ready signal** (or an unbounded marker with an
  explicit cancel route).
- **Named phase.** The indicator labels the current phase:
  `Preparing` (classifying / probing), `Transferring`,
  `Validating` (source-fidelity / sanitiser / anti-malware
  pass), `Applying`, `Completed`, `Partial`, `Cancelled`,
  `Failed`. Phases track the preview / apply / revert
  phases on the interaction-safety packet one-to-one; a
  surface that invents `Working…` / `Loading…` for the
  phase label denies with `vague_loading_copy_on_protected_path`.

### 8.2 Durable mirror

- **Durable surface.** Every large transfer mirrors to a
  `durable_job_row` in the activity centre with a stable
  job id. The job row cites the same
  `mutation_journal_entry_ref` as the local progress surface
  (when the transfer produces a journal entry).
- **Reopen posture.** Closing or navigating away from the
  originating surface MUST NOT cancel the transfer. The job
  row remains live; on completion or failure the user can
  reopen the originating surface via the job row's deep link
  (focus returns to the invoker when resolvable).
- **Toast-only forbidden.** A long transfer that renders
  only as a toast denies with
  `toast_only_forbidden_for_durable_work`.

### 8.3 Cancel and repair routes

- **Cancel route.** Every large transfer exposes a
  keyboard-reachable `Cancel transfer` affordance on the
  progress surface and on the durable mirror. Cancellation
  resolves a **typed** outcome (`cancelled_by_user`,
  `cancelled_by_policy`, `cancelled_by_basis_drift`); a
  cancel that reverts without resolution denies with
  `load_without_cancel_or_repair`.
- **Repair route.** When the transfer's underlying subsystem
  supports repair (cache reset, re-index, reauth, reconnect),
  the progress surface exposes the `rung` via the
  recovery-ladder packet. A transfer without a repair route
  on a subsystem that has one denies with
  `recovery_route_by_freetext_only`.

### 8.4 Basis-drift refusal

- **Drift between preview and apply.** If the underlying basis
  changes between the approved preview and the apply (new
  matching files land in a rename, an import source grows,
  an attach target moves), the surface MUST NOT silently
  widen. It emits
  `interaction_safety_apply_basis_drifted`, refreshes the
  preview, and reopens the review surface. A silent widen
  denies with `apply_basis_drifted`.

### 8.5 Partial / failure outcomes

- **Partial outcomes are first-class.** `partial_success`,
  `partial_revert`, and `partial_validation` render their own
  inline-issue row on the destination surface and their own
  digest card on the activity centre; they are not footnotes
  inside a generic `Failed` banner. The row names the
  preserved / narrowed capability split per the state-and-
  recovery taxonomy.
- **Failure placement.** A large-transfer failure lands on
  exactly one placement tier per the failure-tier matrix —
  `tier.inline_issue` for a single-row failure,
  `tier.contextual_degraded` for a surface-level degradation,
  `tier.workflow_block` when the workflow cannot continue
  safely, `tier.session_recovery` when a repair pass is
  needed, `tier.escalation_surface` when external / support /
  admin help is required.
- **Last-failure reason preserved.** The failure row preserves
  the last-failure reason by ref (keyboard-reachable reveal +
  support-export field + export-safe description) per the
  state-and-recovery taxonomy §4.4 / §8.7.

## 9. Surface projection (frozen)

Every protected surface in the table below emits the
`interaction_safety_packet_record` envelope from
[`shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md)
for every clipboard / drop / large-transfer operation and
honours the clipboard / drop / history / large-transfer axes
below. A surface that mints a private vocabulary, its own
result verb, its own undo label, its own lineage id, or its
own large-transfer progress mode is non-conforming.

| Surface                        | Clipboard notes                                                                              | Drop verbs allowed                         | Undo-class outputs                                                  | Large-transfer posture                                              |
|--------------------------------|----------------------------------------------------------------------------------------------|--------------------------------------------|---------------------------------------------------------------------|---------------------------------------------------------------------|
| `editor_canvas`                | Default `raw` copy; `copy.variant.line`, `copy.variant.rendered_preview`, `copy.variant.diagnostic_details`; bidi / confusable reveals on suspicious paths.| `move`, `copy`, `link`, `open`, `attach`, `import`, `blocked` | `text_edit`, `multi_cursor_text_edit`, `structural_edit`, `refactor_single_file`, `formatter_run`, `save_participant_group`, `external_reload`, `decode_recovery_change`. | Long paste (multi-file drop, oversized generated artifact) routes through durable mirror; basis-drift refusal. |
| `terminal_canvas`              | `multiline_terminal_paste`, `remote_clipboard_bridge`, `paste_then_run` are high-risk preview classes; preview before execute is mandatory.| `copy`, `attach`, `blocked`                | Terminal history is `evidence_only_no_rerun` per the recovery vocabulary. | Long paste renders sanitised preview before commit; cancel route.   |
| `palette_and_search_canvas`    | `copy.variant.command_id` and `copy.variant.permalink` variants; representation labels preserved. | `copy`, `link`, `open`, `blocked`          | No mutating groups; palette-driven commands resolve to their target surface's undo class. | N/A (palette itself does not transfer bytes).                       |
| `review_and_diff_canvas`       | Default `raw`; `copy.variant.rendered_preview` for rendered diff; `copy.variant.diagnostic_details` for inline issues.| `move`, `copy`, `attach`, `import`, `blocked` | `refactor_multi_file`, `imported_change`, `machine_generated_change` via apply; preview + checkpoint mandatory. | Durable mirror for batch apply; partial outcomes first-class.       |
| `install_update_attach_canvas` | `copy.variant.support_export` on publisher-identity rows; `sanitized` default; no raw URL launches. | `attach`, `import`, `blocked`              | Install / update / attach emit `imported_change` or `machine_generated_change` on workspace state. | Install / update progress mirrors to durable job row; basis-drift refusal. |
| `ai_apply_canvas`              | `representation_class = generated` with `citation_anchor_refs`; `copy.variant.reference_anchor` for citations.| `attach`, `import`, `blocked`              | `machine_generated_change` only; preview + checkpoint mandatory; revert class `restore_from_checkpoint`. | Durable mirror for AI batch apply; partial + rollback first-class.  |
| `collaboration_canvas`         | Default `raw`; `copy.variant.permalink` for session deep links; recording / retention label preserved.| `link`, `open`, `attach`, `blocked`        | Collaboration-driven apply emits `machine_generated_change` or routes through review. | Recording / transcript transfers mirror to durable job row.         |
| `provider_bearing_canvas`      | `copy.variant.permalink` via browser-handoff packet only; raw URL launch forbidden.          | `link`, `blocked`                          | Step-up / reauth emits permission prompt records; no mutation journal entries. | Provider handoff progress mirrors to durable job row.               |
| `docs_help_service_health_canvas` | Every copy cites `citation_anchor_refs`; `copy.variant.reference_anchor` is the default path. | `copy`, `link`, `open`, `blocked`          | N/A (docs surface does not mutate workspace).                       | Doc-content snapshot download mirrors to durable mirror.            |
| `support_export_canvas`        | `copy.variant.support_export` only; `sanitized` / `blocked_metadata_only`; raw bodies stripped. | `copy`, `attach`, `blocked`                | Support export is `evidence_only_no_rerun`; no mutation.            | Support bundle build mirrors to durable job row; cancel route.      |
| `settings_import_canvas`       | Default `raw`; `copy.variant.diagnostic_details` on per-row migration failures.              | `copy`, `import`, `blocked`                | `imported_change`, `migration_change` — preview + checkpoint mandatory. | Migration progress mirrors to durable job row; partial first-class. |
| `extension_web_view_canvas`    | Extension-initiated copy / paste runs through the host channel; `representation_class` always set; generated content needs anchors. | `copy`, `attach`, `link`, `blocked`        | Extension-initiated apply emits `machine_generated_change` with `extension_initiated` authority. | Extension-initiated transfers mirror to durable job row; cancel route.|

### 9.1 Chip collapsing is a UI freedom; record addressability is mandatory

A surface MAY fold `copy_variant_id` / `drop_result_verb` /
`undo_class_id` / `recovery_class` / `representation_class`
into one chip for dense rendering, provided the underlying
records retain each axis as a separately addressable field.
Support exports, parity audits, and mutation-journal rows read
each axis independently.

## 10. Denial reasons (closed set)

Every denial fails closed; a silent downgrade to a best-effort
apply or a best-effort paste / drop / import is forbidden.

- `copy_variant_id_unresolved`
- `representation_escalation_required`
- `raw_body_forbidden_on_boundary`
- `raw_url_forbidden_on_boundary`
- `citation_anchor_missing_on_generated`
- `text_fidelity_lost_on_copy`
- `safe_preview_bypassed`
- `drop_result_verb_not_disclosed`
- `drop_result_verb_modifier_mismatch`
- `undo_group_axis_missing`
- `undo_label_hides_recovery_class`
- `no_undo_path_missing_preview_or_checkpoint`
- `agent_attribution_lost`
- `originator_attribution_lost`
- `redo_stack_dropped_without_disclosure`
- `reopen_history_lineage_forked_per_surface`
- `recovery_class_disagreement_across_surfaces`
- `toast_only_forbidden_for_durable_work`
- `vague_loading_copy_on_protected_path`
- `load_without_cancel_or_repair`
- `apply_basis_drifted`
- `responsive_fallback_record_missing`
- `chrome_hid_required_field`
- `focus_return_target_lost`
- `state_row_recovery_not_keyboard_reachable`
- `read_only_degraded_silent_noop`
- `recovery_route_by_freetext_only`

## 11. Audit, redaction, and boundary posture

1. Every clipboard action, drag-drop drop, undo-group row,
   reopen-history row, and large-transfer progress update
   crosses the RPC boundary as typed packets from
   [`shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md)
   — `interaction_safety_packet_record`,
   `preview_apply_revert_record`,
   `batch_scope_record` (for >1-target drops and batch imports),
   `copy_export_representation_record` (for copy / export),
   `focus_return_record` (on every dismiss that closes a
   surface), and
   `interaction_safety_audit_event_record`.
2. Raw clipboard bytes, raw paths, raw URLs, raw prompt text,
   and raw credential material never cross the boundary. Every
   identity is an opaque ref; every timestamp is a monotonic +
   wall-clock pair.
3. Mutation-journal entries, save manifests, support bundles,
   and evidence packets cite
   `mutation_id` / `preview_apply_revert_id` /
   `batch_scope_id` / `copy_export_id` / `focus_return_id` /
   `responsive_fallback_id` only.
4. AI tool calls MUST NOT cache clipboard payloads or drop
   previews past the owning packet's freshness window without
   re-resolving. A cached payload that outlives its anchors
   is denied.
5. Crash dumps MUST NOT persist mid-paste / mid-drop / mid-
   import clipboard or payload bytes; a crash discards the
   unresolved packet rather than persisting a partial axis
   set.

## 12. Compatibility with event-lineage, timeline, and notification-routing

This contract reserves the following integration points so the
later lineage / timeline / notification contracts bind without
renaming rows:

1. Every clipboard / drop / apply operation carries a stable
   `mutation_journal_entry_ref` and an
   `interaction_session_id_ref`; the timeline contract reads
   them directly.
2. Every promotion between failure tiers on a clipboard /
   drop / large-transfer row emits a typed promotion event
   carrying the from-tier, to-tier, trigger class, and
   evidence ref (per the state-and-recovery taxonomy).
3. Every resolved undo / reopen / local-history compare /
   restore emits a typed resolution event carrying the
   recovery class, outcome token, and preserved / narrowed
   capability deltas. Notification-routing reads the stream
   for reopen semantics.
4. Controlled labels `Partially ready` / `Degraded` /
   `Read-only degraded` render on large-transfer rows only
   when the underlying lifecycle row allows them; a surface
   that restates the label in free-form copy denies with
   `controlled_label_misapplied` or
   `controlled_label_lost_on_export`.

## 13. Acceptance mapping

- **Same contract across surfaces.** Every surface in the
  projection table §9 resolves its clipboard / drop / undo /
  reopen / large-transfer behaviours through the ids frozen
  in §3–§8. Editor, search, review, settings import, and
  future automation surfaces cite the same ids without
  restating the rules.
- **A drop discloses what it will do.** The closed result-verb
  set (§6.1), the modifier-key cue table (§6.2), and the
  insertion-preview requirement (§6.3) make
  `drop_result_verb_not_disclosed` a detectable denial rather
  than a narrative judgment.
- **An undo group discloses what it covers.** §7.1 axes, §7.4
  no-undo / preview / checkpoint rule, and §7.6 lineage-
  cross-link invariants make
  `undo_label_hides_recovery_class`,
  `no_undo_path_missing_preview_or_checkpoint`, and
  `reopen_history_lineage_forked_per_surface` detectable at
  audit time.
- **Copy / export never sacrifices downstream safety.** §4.1,
  §5.1, §5.2, and §5.3 freeze the default-raw + additive-
  rich-text rule; the denial set
  (`representation_escalation_required`,
  `text_fidelity_lost_on_copy`,
  `raw_body_forbidden_on_boundary`,
  `raw_url_forbidden_on_boundary`,
  `citation_anchor_missing_on_generated`) makes violation
  observable.
- **Shared lineage on every mutating operation.** The worked
  example in
  [`/artifacts/ux/undo_group_examples.yaml`](../../artifacts/ux/undo_group_examples.yaml)
  pins one operation whose undo-history row, reopen-history
  row, and local-history checkpoint row all cite the same
  `mutation_journal_entry_ref`.

## 14. Source anchors

- `.t2/docs/Aureline_PRD.md` — automated edit paths
  (refactor, quick fix, AI, formatter) must be previewable,
  attributable, and undoable; atomic save pipeline and save
  participants; clean-buffer auto-reload vs dirty-buffer
  merge flows; workspace-scope mutation attribution on
  grouped history entries.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` —
  transaction-grouped undo / redo; atomic rename save;
  decoding failures never discard original bytes.
- `.t2/docs/Aureline_Technical_Design_Document.md` —
  AI apply, extension refactor, multi-file replace, and
  settings import create grouped history entries with source
  attribution; broad-scope apply paths create durable review
  artifacts and checkpoints.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` —
  destruction-class matrix; trust / permission prompts;
  batch scope and query-snapshot-stale rules; representation
  labels on copy / export; focus-return; responsive
  fallback.

## 15. Linked artifacts

- Undo-group worked examples:
  [`/artifacts/ux/undo_group_examples.yaml`](../../artifacts/ux/undo_group_examples.yaml).
- Drag-and-drop worked examples:
  [`/fixtures/ux/dragdrop_cases/`](../../fixtures/ux/dragdrop_cases/).
- Shell interaction-safety contract:
  [`./shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md).
- Buffer / undo / large-file ADR:
  [`../adr/0003-buffer-undo-large-file.md`](../adr/0003-buffer-undo-large-file.md).
- Undo-class rows:
  [`../../artifacts/architecture/undo_class_rows.yaml`](../../artifacts/architecture/undo_class_rows.yaml).
- Source-fidelity / undo verification packet:
  [`../verification/source_fidelity_and_undo_packet.md`](../verification/source_fidelity_and_undo_packet.md).
- Text-fidelity verification packet:
  [`../verification/text_fidelity_packet.md`](../verification/text_fidelity_packet.md).
- State-and-recovery taxonomy:
  [`./state_and_recovery_taxonomy.md`](./state_and_recovery_taxonomy.md).
- Failure-tier matrix:
  [`../../artifacts/ux/failure_tier_matrix.yaml`](../../artifacts/ux/failure_tier_matrix.yaml).
- Navigation-and-escalation contract:
  [`./navigation_and_escalation_contract.md`](./navigation_and_escalation_contract.md).
- Attention / activity taxonomy:
  [`./attention_activity_taxonomy.md`](./attention_activity_taxonomy.md).
- Suspicious-content packet:
  [`../security/suspicious_content_packet.md`](../security/suspicious_content_packet.md).
- Connected-provider browser-handoff ADR:
  [`../adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md).
- Accessibility / IME packet template:
  [`../accessibility/a11y_ime_packet_template.md`](../accessibility/a11y_ime_packet_template.md).

## 16. Changing this contract

- Adding a named copy variant, a drop result verb, a
  modifier-key cue, a surface-projection row, an invariant, or
  a denial reason is **additive-minor** and lands here plus
  the matching artifact and fixture in the same change. Axes
  MUST resolve to already-frozen vocabulary.
- Repurposing a named copy variant id, a drop result verb,
  an undo-class id, a recovery class, or an invariant label
  is **breaking** and opens a new decision row in
  [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
- Copy and microcopy updates that do not change axes live in
  the UX Style Guide; this contract pins structure, not
  rendering.
