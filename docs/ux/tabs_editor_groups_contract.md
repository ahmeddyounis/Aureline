# Tabs and Editor Groups Contract

This contract freezes the working-set surface used by editor tabs,
diffs, notebooks, generated artifacts, preview panes, restored panes,
and compare views. It exists so the shell can add real editor
implementations later without changing the meaning of tab state,
editor-group identity, dirty attribution, or compare fallback.

Companion artifacts:

- [`/schemas/ux/editor_group_state.schema.json`](../../schemas/ux/editor_group_state.schema.json)
  defines the cross-tool packet shape for editor-group snapshots,
  identity-preserving transitions, and compare fallback decisions.
- [`/docs/ux/tab_state_contract.md`](./tab_state_contract.md)
  freezes per-tab state cues, accessibility naming, close-affordance swaps,
  and overflow/scroller recoverability for tab strips and overflow rows.
- [`/schemas/ux/tab_item.schema.json`](../../schemas/ux/tab_item.schema.json)
  exports the single-tab `tab_state_record` for tab strip and overflow tooling.
- [`/fixtures/ux/editor_group_cases/`](../../fixtures/ux/editor_group_cases/)
  contains seed cases for overflow, dirty / generated state, restore,
  move / split / merge identity, shared / followed groups, and compare
  width fallback.
- [`/fixtures/ux/tab_states/`](../../fixtures/ux/tab_states/)
  contains per-tab state fixtures for quick review of cues and accessible names.
- [`/docs/ux/editor_document_state_contract.md`](./editor_document_state_contract.md)
  owns the reusable document-state badge vocabulary projected through
  tabs, document headers, breadcrumbs/context rows, status surfaces,
  compare sheets, preview sheets, support bundles, accessibility labels,
  and docs screenshots.
- [`/schemas/editor/document_state_badge.schema.json`](../../schemas/editor/document_state_badge.schema.json)
  and
  [`/fixtures/editor/document_state_cases/`](../../fixtures/editor/document_state_cases/)
  publish the document-state badge schema and worked cases for recovered
  read-only, generated stale, and compare dirty-source combinations.

Normative sources projected here:

- `.t2/docs/Aureline_PRD.md` and the technical architecture /
  design documents require dirty buffers to remain authoritative
  workspace state and require restored layouts to preserve pane and tab
  identity without auto-rerunning live surfaces.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` Appendix EP requires
  the main workspace to own tabs, editor groups, and reopen-closed
  editor behavior, and requires very narrow compare views to fall back
  to tabbed compare, explicit split choice, or peek.
- [`/docs/workspace/layout_serialization_contract.md`](../workspace/layout_serialization_contract.md)
  owns skeleton-first restore, missing-dependency placeholders, and
  the separation between workspace authority and window topology.
- [`/docs/ux/shell_zone_and_density_contract.md`](./shell_zone_and_density_contract.md)
  owns adaptive classes, main-workspace minimum width, tab overflow,
  and the rule that second-group or compare violations degrade
  explicitly.
- [`/docs/ux/navigation_and_escalation_contract.md`](./navigation_and_escalation_contract.md)
  classifies tabs and editor groups as working-set routes, not primary
  discovery or authority-widening surfaces.
- [`/docs/design/design_token_component_state_vocabulary.md`](../design/design_token_component_state_vocabulary.md)
  owns shared component states such as `read_only`, `degraded`,
  `stale`, `restored`, `restricted`, and `policy_blocked`.
- [`/docs/ux/editor_document_state_contract.md`](./editor_document_state_contract.md)
  owns the stable badge-class names and canonical actions that tab
  records project when dirty, pinned, compare, recovered, generated,
  imported, read-only, mirrored, policy-locked, live-preview, conflict,
  or stale document states are active.

## 1. Boundary

Tabs and editor groups are window-topology state projected over shared
workspace authority. They show and route already-open work. They do not
own buffers, save truth, trust state, runtime authority, or policy.

| Concern | Owner | Editor-group responsibility | Must not imply |
| --- | --- | --- | --- |
| Buffer text, dirty journals, undo groups, save tokens | Workspace / buffer authority | show dirty state, link to authority ref, route save / revert commands | private per-window dirty truth |
| Pane tree, group order, tab order, focus chain, visible overflow | Window topology | preserve group identity and layout memory | closing or moving a group mutates workspace truth |
| Compare, diff, staged peek, tabbed compare posture | Shell working-set router | select a usable presentation and record fallback | unusable narrow panes are acceptable |
| Restored, generated, preview, live-preview, placeholder state | Owning subsystem plus window topology | make status visible and attributable | restored live readiness or editable generated output by default |
| Shared / followed / presentation role | Collaboration or presentation state plus window topology | show role on affected group and tabs | silent control, debug, save, or approval authority |

Rules:

1. **Identity outlives presentation.** A tab, pane, and editor group keep
   stable ids across move, split, merge, move-to-window, overflow,
   restore, and reopen flows. Labels may change; ids do not.
2. **Dirty state is attributed.** A dirty marker always names the
   authority class and source ref that owns the modification. A dirty
   mark without a buffer, notebook, generated-artifact, or review
   authority ref is non-conforming.
3. **Window topology is not authority.** A restored tab can remember
   title, group, focus, provenance, and placeholder actions, but cannot
   serialize old write, debug, remote, notebook, or preview authority as
   if that authority survived restore.
4. **Tabs are not discovery.** A tab strip may switch, close, pin,
   promote, overflow, or route an already-open target. Opening unknown
   resources still routes through command palette, quick open, sidebar,
   breadcrumbs, or explicit open commands.

## 2. Required Tab Fields

Every tab record that crosses the shell boundary MUST expose these
fields or an equivalent schema-refined shape.

| Field | Purpose | Required behavior |
| --- | --- | --- |
| `tab_id` | stable tab identity | survives moves, overflow, restore, reopen, and support export |
| `pane_id_ref` | stable pane / surface slot | remains stable when a pane degrades to placeholder or evidence only |
| `stable_content_ref` | canonical object, buffer, diff, notebook, preview, or artifact binding | never a raw path or raw capability ticket |
| `visible_label` | compact label in the strip | may truncate but must not become the only full-title source |
| `full_title` | complete title for accessibility, tooltip, overflow, support export | always reachable when the visible label truncates |
| `activity_state` | active or inactive tab | active tab remains visible or first-class in overflow |
| `dirty_state` and `dirty_authority` | clean, modified, conflict, blocked, recovered draft plus authority owner | visible through text, shape, badge, or overflow row, not icon alone |
| `pin_state` | pinned or unpinned | pinned tabs keep close / unpin affordance clarity under overflow |
| `preview_state` | disposable preview, promoted full tab, or none | preview-to-full promotion preserves `tab_id` unless the source object changes |
| `restore_state` | never restored, exact, compatible, layout-only, evidence-only, recovered draft | restored state visible on restored layouts and support export |
| `generated_state` | authored, generated read-only, generated edit-blocked, or generated editable with lineage | generated posture remains visible even when dirty or pinned |
| `compare_mode` and `compare_role` | split / tabbed / staged / metadata-only compare plus source / target / base / result role | compare labels name role and basis, not just a diff icon |
| `live_preview_state` | live, reconnecting, stale snapshot, blocked missing runtime, manual rebind | live-preview truth never relies on a green dot alone |
| `missing_dependency_posture` | missing extension, remote, runtime, authority, or non-reentrant surface | placeholder keeps tab and pane identity plus safe actions |
| `read_only_state` | writable or read-only reason | read-only remains legible with pinned, dirty, generated, or shared state |
| `collaboration_state` | local, shared editing, followed remote, presenting, observing | role badge remains visible after restore and in overflow |
| `blocked_state` | unblocked or blocked reason | blocked tabs expose the reason and repair path through overflow or placeholder |
| `close_affordance` | close / unpin / confirm / disabled / overflow-only posture | close target must not swap silently or hide dirty / pinned meaning |
| `label_truncation_state` and `full_title_recovery_surface` | truncation and title recovery | title recoverable by tooltip, keyboard focus popover, overflow row, breadcrumb, or inspector |

Rules:

1. **Visible state is textual or structural.** Dirty, restored,
   generated, compare, live-preview, read-only, shared / followed, and
   blocked states MUST have a non-color, non-icon-only cue. Icons may
   reinforce state but cannot be the only evidence.
2. **Attribution travels with state.** If a visible cue says
   `modified`, `generated`, `restored`, `live preview`, `blocked`, or
   `shared`, the record carries a typed ref to the authority,
   dependency, provenance, session, or policy source behind that cue.
3. **Preview promotion preserves history.** Promoting a disposable
   preview to a full tab retains the tab id, pane id, full title,
   active state, and authority refs. It only mints a new tab when the
   user explicitly opens a different canonical object.
4. **Pinned and read-only combine cleanly.** A pinned read-only tab
   shows both states. Pinned suppresses accidental close; read-only
   explains write limits. Neither state erases the other.

## 3. Labels, Overflow, and Full-Title Recovery

Tab labels optimize for scanning, not for being the only identity
source.

| Condition | Required fallback | Forbidden behavior |
| --- | --- | --- |
| label truncates | full title in tooltip and keyboard-focus popover, or overflow row if pointer tooltips are unavailable | only exposing full title on hover |
| tab overflows | overflow row shows active, dirty, pinned, preview, read-only, shared, blocked, generated, restored, and compare state | dropping active or dirty tabs from the reachable set |
| pinned tabs exceed visible capacity | pinned overflow section keeps order and pinned state | converting pinned tabs to ordinary overflow rows |
| dirty tab overflows | dirty count and tab row name the dirty authority | aggregate badge with no target list |
| restored placeholder overflows | row names restore posture and safe actions | hiding missing dependency until activation |

Overflow priority is:

1. active tab;
2. dirty, blocked, or save-conflict tabs;
3. pinned tabs;
4. shared / followed / presentation-role tabs;
5. compare source / target / base tabs;
6. live-preview or reconnecting tabs;
7. restored, generated, and read-only tabs;
8. remaining inactive tabs ordered by last activation.

The shell MAY collapse labels before moving tabs into overflow, but it
MUST preserve active, dirty, and pinned identity in visible chrome or in
the first keyboard-reachable overflow surface.

## 4. Close-Affordance Swap Rules

Close buttons, unpin buttons, dirty confirmations, and overflow actions
share the same hit target family but not the same meaning. The visual
target may swap only when the swap is legible.

| Tab posture | Primary affordance | Required adjacent path |
| --- | --- | --- |
| inactive clean unpinned | close | context menu includes close others, close saved, pin |
| active clean unpinned | close | focus return target recorded |
| dirty unpinned | close opens dirty confirmation or save / discard flow | save / revert route names dirty authority |
| pinned clean | unpin or close-on-secondary action according to profile | close remains reachable from context menu |
| pinned dirty | unpin is primary; close requires explicit dirty confirmation | dirty state remains visible when unpin is focused |
| read-only | close if local tab can close; write actions disabled with reason | read-only reason remains inspectable |
| shared / followed | close detaches local view only unless user owns shared session | shared role and follow consequence shown before detach |
| blocked placeholder | close / remove pane only through placeholder or overflow row | repair actions stay visible before removal |

Rules:

1. A close affordance MUST NOT become hover-only for dirty, blocked,
   restored placeholder, or shared / followed tabs.
2. Keyboard activation and pointer activation use the same command id
   and consequence class.
3. Closing a tab records whether the close was intentional so reopen
   and crash recovery can distinguish user choice from loss.

## 5. Editor-Group Identity

An editor group is a stable working-set container within a window. It is
not disposable chrome.

Every editor-group record MUST include:

- `group_id`;
- `window_id_ref`;
- `workspace_authority_ref`;
- `active_tab_id`;
- ordered `tabs`;
- group width and minimum-width posture;
- overflow policy and visible / overflowed tab refs;
- focus return target;
- topology lineage refs for support export and layout memory; and
- collaboration / follow / presentation badges affecting the group.

Rules:

1. **Split creates a sibling, not a clone.** Splitting a group mints a
   new `group_id` for the new sibling, moves or opens selected tabs by
   stable `tab_id`, and records the source group lineage. Tabs are not
   duplicated unless the command explicitly opens a second view of the
   same canonical object.
2. **Move preserves ownership.** Moving a tab between groups or windows
   changes its group membership and maybe window-local focus. It does
   not change buffer authority, dirty ownership, read-only posture, or
   save target.
3. **Merge preserves the survivor.** Merging groups chooses a survivor
   `group_id` and records merged group refs in topology lineage.
   Support export can reconstruct the pre-merge identity chain.
4. **Move-to-window preserves workspace authority.** Detaching a group
   to a new window mints a new `window_id_ref`, keeps the same
   `workspace_authority_ref`, and carries trust / policy / dirty
   status through title/context and status surfaces.
5. **Reopen closed restores identity where safe.** Reopening a closed
   tab reuses the previous `tab_id` and `stable_content_ref` when the
   target still resolves. If the target cannot resolve, the tab
   reopens as a placeholder with the original ids and safe actions.
6. **Restore is skeleton first.** Restored groups appear before heavy
   dependencies hydrate. Missing extension, remote, notebook, terminal,
   preview, or generated-artifact dependencies replace only the failing
   pane with a placeholder; surrounding group and tab order survives.

## 6. Minimum Useful Widths

The shell must not create panes that are too narrow to explain their
state.

Frozen seed values:

| Surface | Minimum useful width | Notes |
| --- | --- | --- |
| ordinary editor group | 420 px | aligns with existing shell split-heavy fixtures |
| compare pane inside split compare | 420 px per side | source and target each need identity and role labels |
| staged peek | 360 px | allowed only as an explicit temporary review surface |
| tab strip visible region | 240 px | below this, tabs collapse to overflow with active identity visible |
| restored placeholder card | 320 px | below this, placeholder opens as sheet or overflow detail |

Rules:

1. The minimum is measured after title/context bar, rail, sidebars,
   inspector, bottom panel, safe-area, and zoom constraints are applied.
2. At 400 percent zoom, the same rule applies by useful content width,
   not by raw device pixels.
3. If opening a new group, split compare, or side-by-side compare would
   put any required pane below its minimum useful width, the shell MUST
   choose one of the explicit fallback paths in Section 7.
4. A user may force a narrow split only through an explicit choice that
   states what will collapse. The forced result still cannot hide
   active, dirty, blocked, trust, or restore-critical identity.

## 7. Compare Fallback

Compare is a presentation mode over stable source and target identity.
It must remain usable before it remains side by side.

Fallback order:

1. **Split compare** when both source and target can meet minimum
   useful width and labels remain recoverable.
2. **Tabbed compare** when side-by-side would be too narrow but source,
   target, base, or result roles can be represented in a single group
   with role labels and quick switching.
3. **Staged peek** when the user is checking a small region and the peek
   can preserve target identity, basis snapshot, and focus return.
4. **Explicit choice** when more than one degraded presentation is
   plausible or the user requested a layout that would require a
   visible collapse.
5. **Deny until resize** when no presentation can preserve required
   visible state.

Compare fallback records MUST carry:

- requested mode and selected fallback;
- current main-workspace width;
- attempted per-pane width;
- minimum useful width used;
- affected group and tab refs;
- whether the user saw an explicit choice;
- source / target / base / result role refs;
- focus return target; and
- forbidden outcomes verified absent.

Forbidden outcomes:

- silently producing unusable narrow panes;
- hiding source or target identity;
- relying on icon-only compare roles;
- losing dirty / pinned / active tab state when converting to tabbed
  compare;
- closing or replacing the prior group to make room; and
- claiming live preview or notebook state was restored by compare
  presentation alone.

## 8. Restored, Generated, and Live-Preview Postures

Restored, generated, and live-preview tabs often compete for the same
small strip of chrome. They remain separate axes.

| Axis | Required wording / cue | Notes |
| --- | --- | --- |
| restored | exact, compatible, layout-only, evidence-only, or recovered draft | state persists in support export and overflow |
| generated | generated read-only, edit blocked, or editable with lineage | source / generator ref remains reachable |
| live preview | live, reconnecting, stale snapshot, missing runtime, or manual rebind | no "ready" claim without current runtime authority |
| missing dependency | named missing class and safe actions | placeholder preserves pane and tab identity |
| dirty | modified, conflict, save blocked, or recovered draft | dirty authority always named |

Rules:

1. A generated tab can be dirty only under the authority named by
   `dirty_authority`; generated read-only plus dirty requires a conflict
   or recovered-draft explanation.
2. Live preview does not make source truth editable. Source changes,
   rendered preview, and runtime state keep separate labels.
3. A restored live surface stays `evidence_only`, `manual_rebind`, or
   `placeholder` until the current runtime proves authority continuity
   or the user explicitly reconnects.

## 9. Shared and Followed Groups

Shared, followed, and presentation states are window topology and
collaboration metadata. They do not grant mutating authority by
themselves.

Rules:

1. A followed group must show who or what it follows, whether local
   breakaway is allowed, and whether commands apply to the local window
   or shared session.
2. A shared editing tab must show whether the local user can edit,
   observe, approve, or drive. Read-only and followed states remain
   distinct.
3. Moving a shared / followed group to another window preserves the
   visible role badge and carries the focus / breakaway state into the
   new window topology.
4. Reopen and restore never silently reacquire shared control.

## 10. Support Export and Layout Memory

Support export needs enough identity to reconstruct why the shell felt
stable or unstable without exporting raw private content.

Every snapshot or transition SHOULD include:

- tab, pane, group, window, topology-family, and workspace-authority
  refs;
- stable content refs and source authority refs;
- group lineage refs for split, merge, move-to-window, restore, and
  reopen;
- compare fallback decision refs;
- dirty / generated / restored / live-preview / placeholder posture;
- overflow summary and full-title recovery surface; and
- timestamps or monotonic event ids.

Raw absolute paths, raw file bodies, raw URLs, raw terminal output, raw
notebook outputs, raw preview DOM, raw credentials, and raw provider
payloads must not cross this boundary. Labels are redaction-aware and
bounded.

## 11. Acceptance Checklist

A fixture, implementation, or support-export packet conforms when:

1. Dirty, restored, generated, compare, and live-preview states are
   visible and attributable without relying on tiny icons alone.
2. Active, inactive, modified, pinned, preview, read-only, shared /
   followed, and blocked tab states stay legible when tabs overflow.
3. Close, unpin, dirty confirmation, and placeholder removal
   affordances do not silently swap meaning.
4. Full titles are keyboard-recoverable whenever labels truncate.
5. Editor groups preserve identity across split, move, merge,
   move-to-window, restore, and reopen flows.
6. Workspace authority stays separate from window topology.
7. Compare and multi-pane actions fall back explicitly before violating
   minimum useful layout rules.
8. Preview-to-full promotion preserves tab identity.
9. Pinned/read-only combinations and shared/followed states remain
   visible in overflow and restored layouts.
10. Missing dependencies produce placeholders in the same pane / tab
    slots with safe actions and retained evidence refs.
