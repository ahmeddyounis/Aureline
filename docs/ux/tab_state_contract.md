# Tab State Contract

This contract freezes the tab strip and tab overflow surfaces as a
governed state surface. It exists so tab meaning (what the tab is, what
state it is in, and what will happen when you close/unpin/promote it)
stays legible under overflow, restore, compare, previews, shared/followed
sessions, and blocked placeholders.

Companion artifacts:

- [`/docs/ux/tabs_editor_groups_contract.md`](./tabs_editor_groups_contract.md)
  owns editor-group identity, overflow policy, and transition records.
- [`/schemas/ux/editor_group_state.schema.json`](../../schemas/ux/editor_group_state.schema.json)
  defines the multi-tab snapshot/transition packet shape.
- [`/schemas/ux/tab_item.schema.json`](../../schemas/ux/tab_item.schema.json)
  exports the single-tab record shape used by tab strips and overflow rows.
- [`/fixtures/ux/tab_states/`](../../fixtures/ux/tab_states/)
  contains per-tab worked examples focused on state cues and accessibility.
- [`/fixtures/ux/editor_group_cases/`](../../fixtures/ux/editor_group_cases/)
  contains multi-tab snapshots and transitions that exercise overflow and restore.
- [`/docs/ux/cross_window_transfer_contract.md`](./cross_window_transfer_contract.md)
  owns cross-window move/copy/reopen preview and restore truth.

Normative sources projected here:

- `.t2/docs/Aureline_UI_UX_Spec_Document.md` requires tab state to stay legible
  under overflow and requires identity-preserving promotion from preview to full.
- `.t2/docs/Aureline_PRD.md` requires dirty state to remain attributable and
  requires restore to preserve layout identity without hidden reruns.
- [`/docs/ux/editor_document_state_contract.md`](./editor_document_state_contract.md)
  owns stable state wording used in tab badges and accessibility labels.

## 1. Boundary

Tabs are working-set chrome. They do not own canonical content, policy,
trust, remote authority, notebook kernels, or runtimes. Tabs *project*
truth from the owning authority and provide stable routing among already-open
surfaces.

Rules:

1. **State is reviewable without icon-only decoding.** If a tab is modified,
   pinned, preview, read-only, shared/followed, generated, compare, or blocked,
   at least one non-icon cue MUST be present in the tab strip *or* in the first
   keyboard-reachable overflow surface.
2. **State is attributable.** Any non-default state MUST carry an attribution
   ref in the record (`state_attribution_refs`) that identifies the authority,
   provenance, policy, dependency, compare basis, or session behind the cue.
3. **The tab strip is not discovery.** Creating new editors, widening trust,
   reconnecting runtimes, and acquiring shared control route through explicit
   commands, sheets, or cards; not through silent tab state flips.

## 2. Required Tab States (Cues + Accessible Names)

The tab record is the single source of truth for the state axes below. A UI may
render a subset in the compact tab chip, but MUST render the full meaning in an
overflow row, tooltip, keyboard-focus popover, or inspector surface as
indicated.

### 2.1 Active and Inactive

- Active tabs MUST be identifiable without color alone (selected shape, underline,
  elevation, or similar), and MUST remain reachable when the strip overflows.
- Any list surface that is not inherently “selected vs not selected” (overflow menu,
  quick switch list, restore details) MUST include the token `Active` for the active
  tab row.

Accessible name contract:

- Screen readers MUST be able to detect the active tab via selection semantics
  (e.g., “selected”). When the tab is presented outside a `tablist`/selection
  context, `Active` MUST be included in the accessible name.

### 2.2 Modified (Dirty)

Dirty states include `modified`, `dirty_conflict`, `save_blocked`, and
`recovered_draft`.

Required cues:

- Dirty MUST have a non-icon cue: the word `Modified`, `Conflict`, `Save blocked`,
  or `Recovered draft` rendered directly or in the first overflow surface.
- Any dirty cue MUST name the authority class (buffer/notebook/review/generated/live preview)
  in the same surface that presents the dirty cue.

Accessible name contract:

- The accessible name MUST include the dirty token (`Modified`, `Conflict`,
  `Save blocked`, or `Recovered draft`) and MUST include the tab’s visible label text.

### 2.3 Pinned

Pinned tabs are deliberate anchors. Pinning changes close semantics and overflow
priority.

Required cues:

- Pinned MUST be visible as a structural cue (pinned section, pinned bucket, or
  persistent pin wordmark in overflow rows).
- Pinning MUST NOT erase other state cues (dirty, read-only, compare role, etc).

Accessible name contract:

- The accessible name MUST include `Pinned` when `pin_state = pinned`.

### 2.4 Preview (Disposable, Preview-Pinned, Promoted Full)

Preview state distinguishes “ephemeral hover/click preview” from “durable, user-owned
working tab”.

Required cues:

- Disposable previews MUST carry a non-icon cue (`Preview`) in the tab strip or overflow.
  A pure italic treatment is insufficient unless overflow rows also include the word `Preview`.
- When a preview is promoted to full, the UI MUST NOT look like the tab “changed into
  something else” or “replaced itself”. Promotion MUST preserve tab identity and keep
  the same tab chip position unless the user explicitly opened a different canonical object.

Accessible name contract:

- When `preview_state = disposable_preview` or `preview_pinned`, the accessible name MUST
  include `Preview`.
- When `preview_state = promoted_full`, the accessible name MUST NOT include `Preview`
  unless an additional preview-derived posture is still relevant and visible.

### 2.5 Read-Only

Read-only is a reasoned constraint; it is distinct from pinned and distinct from blocked.

Required cues:

- Read-only MUST be visible as text (`Read-only`) plus a reason when the reason is not obvious
  from surface class (policy/filesystem/generated/shared-follow).
- Read-only MUST remain legible when combined with pinned, dirty, generated, compare, and overflow.

Accessible name contract:

- The accessible name MUST include `Read-only` and SHOULD include a short reason token
  (`Policy`, `Filesystem`, `Generated`, `Following`) that matches visible text.

### 2.6 Shared / Followed

Shared/followed state is collaboration posture, not mutating authority.

Required cues:

- Shared/followed tabs MUST display the role (`Shared editing`, `Following`, `Presenting`,
  `Observing`) in a non-icon form in overflow rows and restore details.
- Moving, reopening, or restoring followed/shared tabs MUST NOT silently reacquire shared control.

Accessible name contract:

- The accessible name MUST include the role token (`Following`, `Shared editing`, etc) and
  MUST include the tab’s visible label text.

### 2.7 Generated

Generated state communicates provenance and edit posture.

Required cues:

- Generated tabs MUST display `Generated` plus an edit posture token (`Read-only`, `Edit blocked`,
  `Editable`) whenever `generated_state != authored`.
- Generated cues MUST remain visible even when dirty or pinned.

Accessible name contract:

- The accessible name MUST include `Generated` and the edit posture token used visually.

### 2.8 Compare

Compare state is not “just a diff icon”; role and basis must be reviewable.

Required cues:

- Compare tabs MUST include `Compare` plus the role token (`Source`, `Target`, `Base`,
  `Result`, `Unified`) in overflow rows and restore details.
- Compare tabs MUST keep basis/freshness attribution reachable (basis snapshot ref, stale basis,
  evidence-only restore) without requiring activation.

Accessible name contract:

- The accessible name MUST include `Compare` and the role token.

### 2.9 Blocked

Blocked tabs represent denial or missing prerequisites that prevent meaningful interaction.

Required cues:

- Blocked tabs MUST include `Blocked` and a short reason token (`Policy`, `Missing dependency`,
  `Authority required`, `Resize required`) in overflow rows or placeholder cards.
- Blocked tabs MUST expose at least one safe action (repair, open read-only, export evidence, remove pane).

Accessible name contract:

- The accessible name MUST include `Blocked` and the reason token used visually.

## 3. Close / Unpin Affordance Swap Contract

Close affordances must not silently change meaning.

Rules:

1. The primary affordance label MUST reflect its action (`Close` vs `Unpin`) in both visible
   UI (icon+tooltip or text) and accessible name.
2. If closing would trigger a dirty confirmation flow, the primary affordance MUST NOT look
   identical to “close without consequence”. A tooltip or adjacent state token MUST indicate
   the dirty posture before activation.
3. When pinned tabs suppress the close button in the chip, close MUST remain available through
   a keyboard-reachable context menu or overflow row action.

## 4. Overflow + Scroller Preservation Contract

Overflow handling is allowed to trade compactness for stability, but it must not trade away
recoverability.

Rules:

1. The active tab MUST remain reachable via keyboard without requiring pointer precision. When the
   strip is scrollable, selecting a tab MUST scroll it into view or project it into the first
   overflow surface with an `Active` token.
2. Hidden tabs MUST be recoverable: the overflow surface MUST list hidden tabs with full titles
   and state tokens, not just icons.
3. Overflow MUST preserve state meaning: overflow rows MUST include the same state tokens that would
   otherwise be visible in the chip (dirty, pinned, preview, read-only, shared/followed, generated,
   compare, blocked, restored).
4. Preview promotion, pinning, and close-affordance swaps MUST NOT “teleport” the active tab into a
   different row/menu without leaving a clear recovery path (overflow entry, keyboard focus popover,
   or breadcrumb/inspector).

## 5. Cross-Window Move, Reopen-Closed, and Restore Rules

Moves, reopens, and restores MUST preserve tab meaning. The following state MUST remain visible and
attributable after transfer:

- dirty state and authority (`dirty_state`, `dirty_authority`, attribution refs);
- shared/followed role (`collaboration_state`, attribution refs);
- compare context (compare mode/role plus basis refs);
- durable work indicators such as recovered drafts, evidence-only restore, generated lineage, and
  blocked/missing-dependency placeholders.

Rules:

1. **Move preserves identity.** Cross-window move preserves `tab_id`, `pane_id_ref`, and state axes.
2. **Reopen does not widen authority.** Reopen/restore never silently reattaches live providers,
   shared control, notebook kernels, or remote sessions.
3. **Restore preserves reviewability under overflow.** Restored tabs that land in overflow must still
   expose full titles and state tokens without requiring activation.

