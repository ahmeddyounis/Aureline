# Breadcrumb Segment and Action Contract

Breadcrumbs are local structure chrome. They explain where the
active editor target sits inside the current workspace, file path,
and symbol hierarchy; they are not a second sidebar, a hidden search
surface, or an alternate command registry.

Machine-readable companions:

- [`/schemas/navigation/breadcrumb_segment.schema.json`](../../schemas/navigation/breadcrumb_segment.schema.json)
  defines the breadcrumb projection record consumed by renderer,
  accessibility, parity, and support tooling.
- [`/fixtures/navigation/breadcrumb_examples/`](../../fixtures/navigation/breadcrumb_examples/)
  contains worked examples for path-only, mixed path-plus-symbol,
  overflow, and stale or unavailable symbol states.

This contract composes with:

- [`/docs/navigation/navigation_and_saved_query_contract.md`](../navigation/navigation_and_saved_query_contract.md)
  for durable `navigation_breadcrumb_trail_record` identity,
  target refs, freshness, and drift posture.
- [`/docs/ux/navigation_and_escalation_contract.md`](./navigation_and_escalation_contract.md)
  for the `route.breadcrumbs` scope, escalation tier, and rule
  that breadcrumbs must not replace broad workspace search or
  structural browsing.
- [`/docs/ux/quick_open_contract.md`](./quick_open_contract.md)
  for the shared target truth and quick-open handoff model.
- [`/docs/commands/command_descriptor_contract.md`](../commands/command_descriptor_contract.md)
  and
  [`/docs/commands/command_graph_and_ui_slots_seed.md`](../commands/command_graph_and_ui_slots_seed.md)
  for command identity, slot attachment, labels, shortcuts, and
  disabled-reason semantics.
- [`/docs/filesystem/filesystem_identity_vocabulary.md`](../filesystem/filesystem_identity_vocabulary.md)
  for presentation path, canonical object, alias, and copy-path
  behavior.

If this document disagrees with a source product document in
`.t2/docs/`, that source wins and this contract plus the schema and
fixtures must update in the same change.

## Boundary

The durable breadcrumb trail remains
`navigation_breadcrumb_trail_record`. This contract freezes the
**projection** that editor chrome renders from that durable trail:
segment anatomy, visual emphasis, truncation, overflow, keyboard
reachability, context actions, and cursor-driven symbol update rules.

Breadcrumb projection records must point back to the durable trail by
opaque ref. They may materialize short labels for display, but they
must not carry raw absolute paths, raw symbol bodies, or private
provider payloads.

Out of scope:

- final widget styling, icon choice, animation, or layout code;
- symbol indexing quality, parser or LSP implementation details;
- file-tree implementation details; and
- creating new command descriptors. Breadcrumb actions cite command
  IDs and route IDs so the command registry can own the final command
  records.

## Canonical Source Chain

Every breadcrumb segment is a projection over an owning truth record.

| Source | Owns | Breadcrumb may project |
| --- | --- | --- |
| Navigation breadcrumb trail | segment order, target refs, surface, freshness | local ancestry and current leaf identity |
| Workspace / VFS identity | presentation path, canonical object, alias state | root badge, path segment labels, copy-path posture |
| Structure provider | symbol path, provider freshness, partiality | symbol ancestry, stale or unavailable state chips |
| Command graph | action identity, labels, enablement, shortcuts | segment menus, reveal, copy, sibling, inspect actions |
| Sidebar / tree contract | reveal target and structural readiness | reveal-in-sidebar result and partial tree disclosure |
| Quick open | large sibling-set handoff and target lookup | scoped handoff when a breadcrumb menu would become browsing |

If a breadcrumb projection disagrees with any source, the projection is
wrong. The renderer corrects, downgrades, or blocks the segment instead
of inventing a local answer.

## Modes

`breadcrumb_mode_class` has three editor modes:

| Mode | Segment sequence | Use |
| --- | --- | --- |
| `file_path` | root, folder path, current file | Path-local orientation when symbol data is absent, disabled, or intentionally hidden. |
| `symbol_path` | file, symbol containers, leaf symbol, with root or folders condensed when needed | Symbol-local orientation in narrow editor groups where the file identity remains present but directory ancestry is secondary. |
| `mixed_path_plus_symbol` | root, folders, file, symbol containers, leaf symbol | Default rich editor orientation when both file and symbol truth are available. |

Mode is a projection choice only. It does not change target refs,
navigation history, command identity, or sidebar state.

## Segment Anatomy

The editor breadcrumb projection recognizes these segment types:

| Segment type | Source trail class | Target kind | Required behavior |
| --- | --- | --- | --- |
| `workspace_root` | `workspace_root_segment` | `file_path_target` | Preserves root identity, including multi-root or remote badges when ambiguity affects trust or save target. |
| `folder_path` | `folder_segment` | `file_path_target` | Represents one folder ancestor. Older folders overflow before file or leaf symbols lose clarity. |
| `file` | `file_segment` | `file_path_target` | Represents the active file or file-local container. It remains visible in symbol and mixed modes. |
| `symbol_container` | `container_symbol_segment` | `symbol_declaration_target` | Represents an enclosing class, module, test, function, or other structural container. |
| `symbol_leaf` | `leaf_symbol_segment` | `symbol_declaration_target` | Represents the current symbol leaf when known. It is the strongest segment in symbol or mixed modes. |

Every segment carries:

- stable `segment_id` and contiguous `segment_index` matching the
  durable trail order;
- `navigation_target_ref` from the navigation-artifacts contract;
- `display` fields for visible label, full-label recovery, emphasis,
  truncation, and state chips;
- `segment_state` for current-leaf, freshness, resolution, and keyboard
  reachability; and
- command-backed `actions`.

## Current Leaf

The current leaf is the strongest visual and accessibility target:

- in `file_path`, the current leaf is the `file` segment;
- in `symbol_path` or `mixed_path_plus_symbol`, the current leaf is
  the deepest resolved `symbol_leaf`; if the symbol leaf is unavailable,
  the `file` segment becomes the current leaf and the symbol state is
  exposed as unavailable;
- current leaf segments must never be moved into overflow;
- current leaf labels may truncate only to a recognizable form with
  full-label recovery on focus, tooltip, screen-reader description, and
  overflow/detail surfaces; and
- state chips such as stale, read-only, generated, remote, or provider
  unavailable may sit beside the leaf, but may not visually overpower or
  replace the leaf identity.

## Overflow and Truncation

Overflow preserves orientation before it preserves literal completeness.
The priority order is:

1. current leaf;
2. active file;
3. root badge when root ambiguity affects trust, host, or save target;
4. nearest symbol container;
5. nearer folder ancestors;
6. older folder ancestors.

Rules:

- older folder segments move to the overflow menu before the current
  file or current symbol truncates beyond recognition;
- symbol containers truncate after folder overflow has been attempted,
  except in `symbol_path` mode where symbol ancestry is the point of the
  mode;
- the overflow control is keyboard reachable and opens a menu containing
  hidden segments in canonical order;
- hidden segments keep their own target refs and actions in the
  projection record; and
- an ellipsis without an inspectable list of hidden segment targets is
  non-conforming.

## Keyboard and Assistive Technology

Breadcrumbs are keyboard-first local chrome:

- the trail uses one roving tab stop by default;
- Left / Right move by visible segment order, Home / End move to first
  and current-leaf segment, Enter or Space activates the focused
  segment, and Shift+F10 or the platform menu key opens the segment
  menu;
- overflow menus are reachable by Tab and by arrow navigation from
  adjacent segments;
- every segment announces label, type, current-leaf state, stale or
  unavailable state, and whether a context menu exists;
- focus returns to the invoking segment or nearest surviving ancestor
  after a menu, quick-open handoff, reveal action, or failed action; and
- reduced-motion or low-resource modes may simplify rendering but must
  keep the same keyboard path and state disclosure.

## Segment Actions

Segment actions resolve through the shared command graph. A breadcrumb
menu may narrow visible actions for local context, but it must not mint
surface-local verbs, labels, shortcuts, or side-effect semantics.

Required action classes:

| Action type | Applies to | Rule |
| --- | --- | --- |
| `open_target` | every resolved segment | Jumps to the segment target through the navigation target model and appends normal navigation history when it moves the editor. |
| `reveal_in_sidebar` | root, folder, file, symbol when a tree can represent it | Opens or focuses the sidebar route and selects the same canonical target. It may reveal a placeholder if tree readiness is partial. |
| `copy_presentation_path` | root, folder, file | Copies the presentation path where policy admits it. When aliases exist, copy posture follows the filesystem identity contract. |
| `copy_canonical_target_id` | every segment | Copies an opaque target or anchor ID, not raw private payload. |
| `open_containing_folder` | file and symbol segments | Opens the containing folder target; it does not browse beyond the active workspace scope. |
| `navigate_sibling` | folder or symbol segments with a known sibling set | Moves among siblings under the same parent. Large sets hand off to quick open seeded with the parent scope. |
| `inspect_structure_state` | stale, partial, unavailable, blocked, generated, remote, or provider-specific segments | Opens the nearest same-surface explanation or inspector command. |
| `open_provider_target` | provider-backed targets when available | Uses provider-specific "open in..." command routes only when target, trust, and policy are revalidated. |

Disabled actions remain selectable when they explain missing provider,
policy, stale target, restricted mode, or unavailable structure. Hidden
actions may be omitted only when discoverability is not useful and the
command descriptor allows omission.

### Reveal In Sidebar

Reveal-in-sidebar is a local handoff, not a tree crawl:

- the command receives the segment's `navigation_target_ref`;
- the sidebar resolves the same canonical target under current scope,
  trust, and policy;
- if the file tree or outline tree is still warming, the sidebar opens
  with a placeholder row and readiness disclosure instead of pretending
  the target is absent;
- if the target lies outside the current workset, the sidebar shows the
  typed outside-scope state and an explicit widen or switch action when
  allowed; and
- reveal must not silently retarget from a stale symbol to a nearby
  sibling.

### Copy Path Or ID

Copy actions are representation-labeled:

- `copy_presentation_path` copies the path the user opened where safe;
- aliases, remote roots, case variants, generated files, and restricted
  roots use the same disclosure vocabulary as tabs, quick open, and
  filesystem identity surfaces;
- `copy_canonical_target_id` copies opaque refs suitable for issues,
  support packets, command arguments, or diagnostics; and
- raw absolute paths, raw provider URLs, and raw symbol bodies are not
  emitted by the breadcrumb projection record.

### Sibling Navigation

Sibling navigation is a local convenience:

- folder siblings are scoped to the same parent folder and admitted
  workspace scope;
- symbol siblings are scoped to the same containing file and symbol
  parent;
- sibling ordering follows the structure provider or tree provider
  order, not fuzzy ranking;
- if the sibling set is too large for a compact menu, the segment action
  hands off to quick open with the parent target as scope; and
- sibling navigation cannot become the only path to broad workspace
  browsing.

## Stale, Partial, and Unavailable Structure

A breadcrumb surface must say what it knows. It must not render empty
or flat chrome as if hierarchy is exact when structure is still warming
or unavailable.

Resolution states:

| State | Meaning | Required presentation |
| --- | --- | --- |
| `resolved_current` | Target and structure are current enough for the claimed scope. | No extra chip required. |
| `resolved_stale` | Cached structure is shown below freshness floor. | Stale chip plus inspect action. |
| `provider_partial` | Some ancestors are known but the provider has not covered the full scope. | Partial chip plus what-still-works explanation. |
| `provider_unavailable` | Symbol or tree provider cannot currently serve structure. | Unavailable chip; file segment remains usable. |
| `target_unavailable` | Segment target no longer resolves. | Missing-target label; open is blocked or rerouted through drift review. |
| `scope_unavailable` | Segment exists outside current workset, trust, remote, or docs-pack scope. | Scope chip plus explicit widen/load/reach action when allowed. |
| `policy_blocked` | Policy forbids resolving or showing the target. | Policy chip and disabled-reason detail. |

If a stale or unavailable symbol is still displayed, it must carry its
state disclosure. If no symbol segment can be honestly displayed, the
file segment becomes the leaf and the unavailable symbol path is exposed
through an inspectable state action.

## Cursor-Driven Symbol Updates

Symbol breadcrumbs follow the caret, but they must not churn:

- explicit navigation, file switch, or symbol jump updates the trail
  immediately after target resolution;
- ordinary cursor movement is debounced before replacing the current
  symbol leaf;
- the default debounce window should be short enough for orientation
  after the user pauses typing or moving, typically 100-200 ms, with a
  bounded truth lag around 500 ms during sustained cursor movement;
- cursor-only symbol changes do not append navigation-history entries;
- while a provider recomputes, the previous symbol path may remain with
  a stale or revalidating state chip, or the file leaf may take over if
  the prior symbol no longer resolves;
- a provider delta that invalidates an ancestor must update or remove
  the affected segment before any action can apply to it; and
- hover, tooltip, or menu open must not trigger a private recomputation
  path that diverges from the canonical structure provider.

## Target Snapshot Revalidation

Segment menus resolve against the current target snapshot twice:

1. when the menu opens, to decide which command-backed actions are
   enabled; and
2. immediately before invocation, to prevent stale context from applying
   to a changed file, symbol, root, workset, trust state, or provider
   target.

If the snapshot changes materially while the menu is open, affected
items refresh or become disabled with a reason. Applying a command
against stale breadcrumb context is non-conforming.

## Non-Conforming Behavior

These cases must fail closed or surface an explanation:

- a breadcrumb projection that cannot point back to a durable
  `navigation_breadcrumb_trail_record`;
- a current leaf moved into overflow or truncated past recognition;
- a segment action without a command ID;
- reveal-in-sidebar selecting a different canonical target from the
  segment;
- symbol breadcrumbs that update on every cursor tick with distracting
  visual churn;
- stale or unavailable symbols rendered as exact current structure;
- a broad sibling menu that becomes hidden workspace browsing; or
- path, symbol, quick-open, sidebar, and command routes disagreeing on
  the same target identity.

## Acceptance Checks

A conforming implementation can prove:

1. Breadcrumbs preserve local ancestry and segment actions without
   becoming a substitute for sidebar browsing.
2. Overflow keeps the current leaf recognizable and every hidden
   segment keyboard reachable.
3. Segment actions cite command IDs and one target truth model across
   file, symbol, and mixed views.
4. Mixed path-plus-symbol breadcrumbs remain readable under overflow
   and continue to point at the canonical navigation trail.
