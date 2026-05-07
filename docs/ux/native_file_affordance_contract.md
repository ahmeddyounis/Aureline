# Native file affordance contract: dialogs, reveal-in-shell, and identity-preserving handoff

This document freezes how native file and system affordances behave in Aureline
so they preserve object identity, write-safety, and route truth instead of
becoming opaque per-platform side effects.

This contract is normative. Where it disagrees with the authoritative product
design documents in `.t2/docs/`, those sources win and this contract plus its
companion artifacts MUST update in the same change.

## Companion artifacts

- [`/schemas/platform/native_file_affordance.schema.json`](../../schemas/platform/native_file_affordance.schema.json)
  — boundary schema for worked native-file-affordance cases.
- [`/fixtures/platform/native_file_affordance_cases/`](../../fixtures/platform/native_file_affordance_cases/)
  — worked cases spanning open/save dialogs, reveal-in-shell, drag/drop,
  clipboard, file associations, open-from-terminal, and browser-originated open
  requests.

## Related contracts (source of truth)

This document does not re-mint identity, save, deep-link, or safe-preview
vocabularies. Those sources remain authoritative:

- [`/docs/ux/desktop_affordance_contract.md`](./desktop_affordance_contract.md)
  — the cross-surface desktop integration contract (deep links, file
  associations, native dialogs, drag/drop, clipboard, browser handoff, terminal,
  lifecycle recovery).
- [`/docs/platform/system_open_target_truth.md`](../platform/system_open_target_truth.md)
  and
  [`/schemas/platform/system_open_target_packet.schema.json`](../../schemas/platform/system_open_target_packet.schema.json)
  — canonical binding packet for OS-provided targets (literal target,
  presentation path, canonical object identity, alias truth, and placeholder
  recovery).
- [`/docs/io/save_target_token_and_write_guarantee_contract.md`](../io/save_target_token_and_write_guarantee_contract.md)
  — save-target token and write-guarantee classes; compare-before-write;
  wrong-target prevention and recovery posture for write-like actions.
- [`/docs/ux/file_state_badge_and_write_review_contract.md`](./file_state_badge_and_write_review_contract.md)
  — shared read-only / generated / managed / policy-locked state vocabulary and
  the write-review sheet action set.
- [`/docs/security/safe_preview_trust_classes.md`](../security/safe_preview_trust_classes.md)
  — safe-preview trust class and suspicious-content vocabulary used to keep
  “safe preview” and “safe copy/export” status inspectable across surfaces.
- [`/docs/ux/clipboard_history_contract.md`](./clipboard_history_contract.md)
  — representation-labeled clipboard, drag/drop verbs, and suspicious-content
  transfer rules.

## 1. What this contract is protecting

Native affordances are “selection and transport” surfaces. They are not sources
of truth.

Examples:

- native open/save dialogs
- reveal-in-system-shell / open-with / file associations
- open-from-terminal
- drag/drop into Aureline
- clipboard path/URI paste and copy-out
- browser-originated open requests (default browser return, downloads, or
  explicit “Open in Aureline” actions)

All of these MUST route through the same product truth model:

1. **Canonical identity binding** (what object is this, really?)
2. **Route truth** (what command/intent is being requested and why?)
3. **Trust + policy evaluation** (is the route admissible in this context?)
4. **Write-safety posture** (what can be written, under what guarantee, with
   what recovery/checkpoint posture?)
5. **Safe-preview posture** (what representation is safe to show/copy/export?)

## 2. Invariants (non-negotiables)

### 2.1 Native dialogs are selectors, not admissions

Native open/save dialogs MAY be used as pickers. They MUST NOT:

- widen trust scope;
- create a privileged write path;
- override save-target token requirements; or
- become the only place where overwrite risk / read-only / managed / generated
  state is communicated.

Dialog output (paths/URIs) is treated as **literal input**, then re-bound to
product identity before any consequence-bearing action proceeds.

### 2.2 Every affordance produces inspectable truth

For any OS-provided path/URI, Aureline MUST be able to produce one inspectable
binding packet:

- `system_open_target_packet_record` when the target can be resolved, or
- `system_open_target_placeholder_record` when it cannot.

For any consequence-bearing action (open workspace, write/overwrite, open
provider-managed object, launch browser handoff, etc.), the user MUST be shown
an in-product review surface when required by trust, policy, replay, identity
uncertainty, or write posture.

### 2.3 No silent wrong-target writes

Native affordances MUST fail safe when identity is uncertain:

- alias sets (UNC vs mapped drive, alternate mounts, symlink/junction chains),
  case-only variants, and Unicode normalization variants may not produce two
  independent writable buffers;
- ambiguous or weak identity tokens MUST downgrade to read-only and/or require
  review/retargeting before any write-like action;
- a missing/unmounted target MUST open a placeholder/recovery surface, not an
  empty shell.

### 2.4 Safe-preview status is preserved across handoff

When an affordance causes content to enter or leave Aureline (open, preview,
copy, export, share), the safe-preview trust class and suspicious-content status
MUST remain inspectable. Platform UI (dialogs, Finder/Explorer, clipboard
popover) must not erase the fact that the product is showing:

- `RawText` versus a rendered/sanitized representation;
- a constrained or policy-locked object; or
- a suspicious-content warning that changes what “Copy” means.

## 3. Native open/save dialogs vs in-product review sheets

Native dialogs are allowed for selection; in-product sheets are required for
admission when a boundary is crossed.

### 3.1 Open: selection vs admission

The native open dialog (or OS “open” event) may return a path/URI. Aureline MUST
then:

1. Bind the literal target via `system_open_target_packet*`.
2. Decide the route class (`open file`, `open workspace`, `add root`, etc.) via
   the same entry model used by Start Center, drag/drop, system open, and CLI.
3. Run trust/policy evaluation.
4. Show an in-product review/interstitial when any of the following are true:
   - target is outside current trust scope or profile scope;
   - availability is not `exact_available`;
   - alias/collision state makes write posture anything other than
     `writes_allowed`;
   - the route would widen authority or resume managed/provider state; or
   - the request originated from an untrusted or replay-sensitive source.

### 3.2 Save: pick destination vs prove write safety

Save and save-as flows MUST never let a native save dialog become the write
authority.

Required rules:

- A write-like action MUST have a save-target token (or an explicit token
  unavailability posture that blocks writes) before any durable write is issued.
- If the current object is read-only, generated, managed/mirrored, policy-locked,
  or identity-uncertain, the save flow MUST route through the write-review sheet
  contract before a write-like action can proceed.
- Overwrite risk MUST be disclosed in-product. Platform “Replace?” prompts are
  not sufficient because they do not carry trust scope, alias truth, save
  guarantee class, or recovery/checkpoint posture.

### 3.3 Reveal-in-shell: identity truth, not path folklore

Reveal-in-system-shell is an OS handoff that must remain truthful:

- reveal uses the bound presentation path and canonical identity; it MUST NOT
  reveal an alias path as if it were canonical truth;
- reveal may be blocked or downgraded (copy-only, inspect-only) when the target
  is missing/unmounted, ambiguous, policy-blocked, or safe-preview constraints
  require an in-product review before disclosure;
- reveal is never a substitute for “this is the canonical object”; alias
  disclosure remains part of the product’s identity surface.

## 4. Out-of-scope paths/URIs and trust scope mismatches

When a native affordance returns a path/URI outside the expected workspace,
profile, or trust scope:

- the target MUST bind to a truthful packet (resolved or placeholder);
- the product MUST preserve the user’s original intent (open/reveal/save) as a
  review row or placeholder; and
- the default posture MUST be read-only or blocked until the user explicitly
  admits a scope change or chooses a safe alternate target.

There is no “best effort” silent retarget to “something nearby”.

## 5. Required case coverage

The worked cases under
[`/fixtures/platform/native_file_affordance_cases/`](../../fixtures/platform/native_file_affordance_cases/)
cover the minimum required scenarios:

- local file open via native open dialog
- network share open (alias/collision disclosure)
- removable volume open (stale/missing recovery)
- generated artifact save-as (read-only + write-review)
- managed/policy-locked file save (approval/deny posture)
- suspicious-content preview + safe copy/drag-drop semantics
- browser-originated open requests (origin validation + review)

## 6. Acceptance checklist

Reviewers can accept a native-file-affordance change when:

1. Open/save/reveal/association/terminal/drag-drop/clipboard/browser flows reuse
   the same binding and review model (no parallel implementations).
2. Every OS-facing action can be traced to canonical object identity and stable
   command/intent refs.
3. Wrong-target prevention is explicit (alias/collision/availability states fail
   safe before writes).
4. Save-target tokens or explicit token-unavailable postures exist before any
   durable write.
5. Safe-preview trust class and suspicious-content status remain inspectable
   across handoff and transfer.
6. Missing/unmounted targets open placeholders with bounded recovery actions,
   never silent empty shells.

