# Shell responsive fallback and identity-cue preservation contract

This contract freezes how the desktop shell preserves **truth-bearing
identity cues** under resize, split, multi-window, density, zoom, and
restore pressure. It turns responsive behavior from a static layout map
into an **executable fallback contract**: every cue that can collapse has
a typed fallback surface, a keyboard route, and a screen-reader-safe
recovery path.

This document is normative. Where it conflicts with the source product
documents in `.t2/docs/`, the source wins and this contract plus the
companion artifacts and fixtures MUST update in the same change.

## Machine-readable companions

- [`/artifacts/ux/zone_priority_rules.yaml`](../../artifacts/ux/zone_priority_rules.yaml)
  — priority ladders + collapse ladders for identity cues.
- [`/fixtures/ux/responsive_fallback_cases/`](../../fixtures/ux/responsive_fallback_cases/)
  — curated stress cases covering narrow widths, stacked splits, multi-monitor
  moves, density + zoom, restore, and long-title/long-path conditions.

This contract also composes with the existing shell zoning corpus:

- [`/docs/ux/shell_zone_and_density_contract.md`](./shell_zone_and_density_contract.md)
  — zones, adaptive classes, and baseline cue fallbacks (summary table).
- [`/artifacts/ux/shell_metrics.yaml`](../../artifacts/ux/shell_metrics.yaml)
  — shell-zone + identity-cue vocabularies (do not re-mint).
- [`/fixtures/ux/shell_layout_classes/`](../../fixtures/ux/shell_layout_classes/)
  — shell-wide layout rehearsals (zone visibility + cue fallbacks).

## Composes with (non-exhaustive)

- [`/docs/ux/shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md)
  and [`/schemas/ux/interaction_safety.schema.json`](../../schemas/ux/interaction_safety.schema.json)
  — `responsive_fallback_mode` vocabulary, required-visible-field classes,
  focus-return rules, and the `chrome_hid_required_field` denial posture.
- [`/docs/ux/breadcrumb_contract.md`](./breadcrumb_contract.md)
  — breadcrumb overflow priorities, keyboard route, and announcement rules.
- [`/docs/ux/tabs_editor_groups_contract.md`](./tabs_editor_groups_contract.md)
  — tab overflow, split/compare fallback, and title recovery rules.
- [`/docs/ux/status_bar_contract.md`](./status_bar_contract.md)
  — status priority ladder, overflow parity, anti-jitter, and keyboard search.
- [`/docs/ux/title_context_bar_contract.md`](./title_context_bar_contract.md)
  — workspace / trust / host / profile / route identity projected in the title.
- [`/docs/ux/durable_work_contract.md`](./durable_work_contract.md)
  — durable job rows / activity center routing used as fallback surfaces.
- [`/docs/ux/window_display_contract.md`](./window_display_contract.md)
  — window ownership, dialog attachment, and cross-monitor identity parity.

## 1. Scope

This contract freezes:

- **Zone-priority rules** for identity cues that users read continuously:
  breadcrumbs, tabs, status indicators (including durable work summaries),
  terminal headers, inspector headers, and trust/policy/recovery cues.
- **Collapse ladders** (truncate → condense → overflow → sheet/drawer → durable
  detail surface) for each cue.
- **Recovery guarantees**: where hidden information is recovered, how it is
  recovered, and how recovery remains unambiguous without hover-only UI.
- **Accessibility consequences** for each fallback state: keyboard reachability,
  focus return, and screen-reader announcements.
- **Stress-case expectations** for narrow widths, stacked splits, multi-monitor
  moves, density/zoom combinations, restored windows, and long-title/long-path
  content.

Out of scope:

- final rendering/CSS, animation polish, and platform-specific window chrome;
- per-subsystem feature completeness (e.g., symbol providers, remote probes);
- new vocabularies for zones, adaptive classes, and responsive-fallback modes.

## 2. Definitions

**Identity cue**  
A compact, continuously visible cue that carries target, route, authority,
lifecycle, or recovery truth. The canonical cue ids live in
[`/artifacts/ux/shell_metrics.yaml`](../../artifacts/ux/shell_metrics.yaml)
(`identity_cue_id_vocabulary`).

**Responsive fallback**  
A typed shell state where space constraints (width, split pressure, zoomed
text, or restore reflow) force collapse of secondary chrome into a fallback
surface. The canonical fallback-mode vocabulary is owned by the shell
interaction-safety contract (`responsive_fallback_mode`).

**Fallback surface**  
A typed recovery surface used when an identity cue can no longer remain fully
expanded in-place (e.g., overflow menu, compact-shell menu, summary chip, sheet,
drawer, status-bar aggregate, durable job/activity center row). Canonical ids
live in `shell_metrics.yaml` (`fallback_surface_id_vocabulary`).

## 3. Non-negotiables (normative)

1. **Truth survives collapse.** A responsive fallback MAY reduce decoration and
   optional detail; it MUST NOT hide or erase truth-bearing identity.
2. **No hover-only recovery.** Hidden identity is recovered through keyboard-
   reachable overflow menus, sheets, drawers, or durable rows — never through
   pointer hover as the only path.
3. **Required-visible fields never vanish at commit.** If a protected surface is
   in a state where responsive fallback would hide any required-visible-field
   class for the active consequence class, the surface MUST deny (`chrome_hid_required_field`)
   and require the user to expand the fallback surface before committing.
4. **Identity-stable transitions.** Docked→sheet and expanded→overflow swaps
   preserve the same target refs and state; they do not mint “fresh” instances
   that lose history, focus, or lifecycle truth.
5. **Keyboard completeness.** Every collapsed cue has a complete keyboard path:
   entry, navigation inside the fallback surface, activation, dismissal, and
   focus return.
6. **Screen-reader coherence.** Collapsing a cue must not remove the accessible
   name or meaning. Overflow triggers announce hidden counts and preserve
   semantics for the recovered content.
7. **No silent re-targeting.** Closing a sheet or overflow surface returns to
   the invoking target or the nearest still-valid owner. A fallback MUST NOT
   retarget to a different pane/tab silently.

## 4. Zone-priority ladder (what collapses first)

Responsive adaptation MUST follow the shell zoning contract’s priority order:

1. move optional detail out of the **right inspector** into a sheet;
2. collapse secondary **bottom-panel tabs** before shrinking the main workspace
   below minimum useful size;
3. preserve path/branch/trust/execution-target identity before promotional or
   optional content;
4. move low-frequency side tools into overflow/drawers before collapsing
   primary navigation;
5. preserve focus and keyboard continuity across docked↔sheet transitions.

This contract adds a cue-level refinement: within a zone, **ornament collapses
before identity**, and identity collapses only when its typed fallback surface
takes over.

The concrete cue priorities and collapse ladders are frozen in:
[`/artifacts/ux/zone_priority_rules.yaml`](../../artifacts/ux/zone_priority_rules.yaml).

## 5. Cue contracts (what must survive)

The cue ids and baseline fallbacks are defined in `shell_metrics.yaml`. This
section specifies **what is protected truth** for each cue and the permitted
collapse ladder.

### 5.1 Breadcrumbs (`cue.breadcrumbs`)

Protected truth (must remain visible or recovered without ambiguity):

- root identity when ambiguity affects trust, host, or save target;
- current file / item segment;
- current symbol segment when present;
- stale/unavailable symbol truth is explicitly labeled (never implied as fresh).

Allowed collapse ladder (in order):

1. truncate older intermediate segments;
2. move older intermediate segments into the breadcrumb overflow menu;
3. condense symbol ancestry *after* folder overflow has been attempted in mixed
   mode (except in symbol-path mode per breadcrumb contract).

Forbidden:

- hiding the root segment when root ambiguity is meaningful;
- moving the current leaf into overflow;
- collapsing to an ellipsis without an inspectable list of hidden segments.

Keyboard + SR consequences:

- overflow control is a focusable element with an accessible name that includes
  the hidden-segment count;
- closing the menu returns focus to the invoking segment or nearest surviving
  ancestor (never to the document body).

### 5.2 Tabs (`cue.tabs`)

Protected truth:

- active tab identity;
- dirty state cue for dirty tabs;
- pinned tab identity for pinned tabs.

Allowed collapse ladder (in order):

1. truncate labels (with full-title recovery);
2. scroll the tab strip; then overflow inactive tabs into an overflow row/menu;
3. promote compare into tabbed compare / staged peek rather than producing
   unusable narrow panes under split pressure.

Forbidden:

- dropping the active or dirty tab from the reachable set;
- converting pinned tabs into ordinary overflow rows without preserving pin state;
- any overflow surface that cannot be reached by keyboard.

Keyboard + SR consequences:

- the overflow row/menu is reachable via keyboard from the tab strip;
- full title is recoverable via keyboard-focus popover or overflow row (not hover-only);
- close/pin actions remain keyboard reachable and do not shift layout enough to
  break aim for repeated operations.

### 5.3 Status indicators and durable activity cues (`cue.status_indicators`)

This cue includes the shell’s status-bar truth plus compact durable work
summaries (for example: background jobs, indexing/test/sync work, and queue/busy
signals). When the UI uses a “strip” presentation for ongoing activity, it MUST
collapse into the same status overflow and durable work routes described here.

Protected truth:

- recovery-critical state (`restricted`, `degraded`, `blocked`, `restore required`);
- active context truth (execution target/profile/branch/trust mode);
- basis freshness cues where they affect action safety;
- blocked/suppressed/hidden counts where they affect interpretation.

Allowed collapse ladder (in order):

1. aggregate repeated ongoing work into one summary item;
2. move ambient metadata to overflow first;
3. collapse into a compact-shell status menu that names hidden counts;
4. route detail to the durable work/activity center surface for deep inspection.

Forbidden:

- overflowing or hiding recovery-critical state without a shell-owned recovery slot;
- an unlabeled overflow trigger that hides what was lost;
- status jitter that reflows the whole bar under rapidly changing values.

Keyboard + SR consequences:

- overflow trigger announces “N more status items” and is focusable;
- overflowed items remain searchable by label in the palette/status menu and
  route to the same owner surface as when visible.

### 5.4 Terminal headers (`cue.terminal_headers`)

Protected truth:

- session label (what this terminal is);
- boundary (local/remote/container/managed) and host identity;
- transcript state (live vs captured) and exit / degraded state.

Allowed collapse ladder (in order):

1. condense label text (preserve boundary + host cue);
2. move long titles into tab overflow while keeping boundary in the visible tab;
3. route full header detail into a terminal header/detail sheet.

Forbidden:

- renaming a session silently because space ran out;
- hiding exit/degraded state while still showing an interactive prompt;
- replaying latent PTY input or implying a restored terminal re-ran.

Keyboard + SR consequences:

- terminal header actions (open transcript, export reviewed transcript) remain
  reachable via keyboard from the terminal region, even if header chrome collapses.

### 5.5 Inspector headers (`cue.inspector_headers`)

Protected truth:

- target pane reference (what the inspector is describing);
- inspector kind (context/docs/symbol/details/etc);
- freshness / degraded state labels where present;
- pin-back-to-dock affordance if the inspector is sheeted.

Allowed collapse ladder (in order):

1. sheet the inspector on demand under narrow widths;
2. condense header actions into an overflow menu inside the sheet;
3. route deep detail to the owning inspector surface.

Forbidden:

- opening an inspector sheet without the target pane ref;
- silent retargeting to a different pane when the sheet closes.

Keyboard + SR consequences:

- opening and closing the sheet preserves focus return to the invoking control
  or pane; screen readers announce the sheet title including target pane identity.

### 5.6 Trust / policy / recovery-critical state (`cue.trust_policy_recovery_state`)

Protected truth:

- workspace identity;
- trust class and policy source cues;
- recovery-critical state (restore degraded, blocked by policy, read-only degraded);
- remote/host boundary where it changes command meaning.

Allowed collapse ladder (in order):

1. condense into a summary chip in the title/context bar (never icon-only when stakes are meaningful);
2. mirror into a status-bar recovery slot;
3. route detail into a narrow inspector/review surface that is reachable by keyboard.

Forbidden:

- hiding trust loss, policy blocks, broken restore, or read-only degraded state
  behind an undisclosed overflow.

### 5.7 Collaboration / presentation role badges (`cue.collaboration_presentation_role_badges`)

Protected truth:

- role badge (presenter/driver/observer/approver) and shared-control posture;
- badge persists through docked→sheet and restore reflow.

Allowed collapse ladder:

- role badge may condense into a single chip, but it remains visible and recoverable.

Forbidden:

- silently dropping role badges under responsive collapse or restore.

### 5.8 Command palette entry (`cue.command_palette_entry`)

Protected truth:

- keyboard route to open the palette;
- no timing-dependent or hover-only entry.

## 6. Stress scenarios that MUST remain truthful

The following scenarios must not cause protected truth to disappear; if a
surface cannot keep required truth visible, it MUST provide a typed fallback
surface or deny the interaction until the user expands it:

- narrow desktop widths where long paths/titles would otherwise truncate identity;
- stacked splits / compare views where another split would violate minimum widths;
- moving windows between monitors / DPI buckets where reflow triggers different
  collapse states;
- compact density combined with narrow widths (no private “extra compact” rules);
- 400% zoom or equivalent text scaling (keyboard routes and announcements remain
  usable even if fewer cues fit inline);
- restored windows where missing dependencies force placeholders (no silent
  layout collapse; identity remains present and re-entry is explicit).

The curated fixture suite lives at:
[`/fixtures/ux/responsive_fallback_cases/`](../../fixtures/ux/responsive_fallback_cases/).

