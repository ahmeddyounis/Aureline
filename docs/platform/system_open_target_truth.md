# System-open target truth and canonical binding packet

This contract defines the canonical “system-open target” packet emitted when
Aureline receives an OS-level target from system open, file associations,
open-with, reveal-in-system-shell, native open/save dialogs, drag-drop, or
open-from-terminal flows.

The packet exists to prevent wrong-target writes caused by deep links, file
associations, network share aliases, symlinks/junctions, case variants, or
Unicode normalization variants. It preserves the literal target the user chose
while binding the request to VFS identity before any risky trust/policy decision
or durable write.

If this document disagrees with the authoritative VFS identity and save
contracts, those sources win and this packet updates in the same change:

- [`/docs/adr/0006-vfs-save-cache-identity.md`](../adr/0006-vfs-save-cache-identity.md)
- [`/docs/filesystem/filesystem_identity_vocabulary.md`](../filesystem/filesystem_identity_vocabulary.md)
- [`/docs/io/save_target_token_and_write_guarantee_contract.md`](../io/save_target_token_and_write_guarantee_contract.md)
- [`/docs/ux/desktop_affordance_contract.md`](../ux/desktop_affordance_contract.md)

## Companion artifacts

- [`/schemas/platform/system_open_target_packet.schema.json`](../../schemas/platform/system_open_target_packet.schema.json)
  — boundary schema for `system_open_target_packet_record` and
  `system_open_target_placeholder_record`.
- [`/fixtures/platform/network_share_alias_cases/`](../../fixtures/platform/network_share_alias_cases/)
  — worked fixtures for UNC/network-share aliases, case-variant collisions, and
  disconnected share recovery that must fail safe.

## 1. Goal

System-open flows MUST be able to explain, in one inspectable packet:

- what literal target the OS or dialog delivered (drive-letter/UNC/backslash,
  case-preserving, and Unicode-preserving);
- what canonical filesystem object Aureline believes is being touched;
- which aliases are known to resolve to the same canonical object; and
- whether a write is allowed, read-only, or blocked pending review/retargeting.

## 2. Packet kinds

The boundary exports two record kinds:

- `system_open_target_packet_record` — the target resolved to a canonical object
  identity and the packet can carry alias and write-posture truth.
- `system_open_target_placeholder_record` — the target could not be resolved
  (missing/unmounted, disconnected share, blocked by policy, or ambiguous) and
  the packet carries a truthful placeholder posture plus recovery actions.

No system-open path may fall back to a silent empty window. When a target is not
exactly available, the resolver MUST produce a placeholder record.

## 3. Required identity disclosure

Every system-open and native-dialog target is represented by three related
strings:

1. **Literal target** — the OS-provided string as received (e.g. `Z:\src\App.ts`,
   `\\SERVER\Share\src\App.ts`, `/Volumes/Share/src/App.ts`).
2. **Presentation path** — the VFS URI used as the presentation identity inside
   the product. It MUST preserve the user-chosen casing/normalization where
   possible and MUST NOT silently replace the literal target in UI affordances.
3. **Canonical filesystem object** — the canonical URI + strongest identity token
   the VFS will target for save and external-change decisions.

Surfaces may render compact chrome (tabs/breadcrumbs) using the literal target,
but they MUST be able to surface both the literal and canonical values before a
write-like action (save/rename/overwrite) commits.

## 4. Wrong-target prevention rules (system-open)

System-open and dialog targets MUST fail safe when any of the following are
true:

- The resolved alias set indicates the target is reachable through multiple
  strings (UNC vs mapped drive, share mount vs alternate mount, case-only
  variant, Unicode-normalization variant) **and** the root cannot provide a
  stable strongest identity token.
- The target availability is not `exact_available` (moved, missing/unmounted,
  remote unreachable, policy blocked, ambiguous).
- The resolver cannot prove that the canonical object identity the next write
  would touch is the same identity the UI believes is opened.

Fail safe means one of:

- open in read-only posture with an explicit banner and no save token; or
- block write affordances until the user reselects/locates the target; or
- converge duplicate opens onto one buffer authority and present the alias
  relationship explicitly (never as two independent writable buffers).

## 5. External-path failure degradation

When a system-open target cannot be resolved exactly, the product MUST degrade
into a truthful placeholder or recovery surface instead of an empty shell.

The placeholder packet MUST:

- preserve the literal target the user asked for;
- state the availability class (`missing_or_unmounted`, `remote_unreachable`,
  `blocked_by_policy`, `moved_or_alias_changed`, `ambiguous`, …); and
- offer recovery actions appropriate to the failure:
  - **Locate** (user selects the correct target path);
  - **Open cached context** (read-only cached view when available); and
  - **Cancel** (close/dismiss the placeholder).

## 6. Network-share and case-variant fixtures

Worked fixtures under `fixtures/platform/network_share_alias_cases/` define the
minimum safe behavior for:

- UNC and mapped-drive alias disclosure that converges to one canonical object;
- case-variant collisions on case-insensitive roots that block or downgrade
  writes until canonical identity is strong; and
- disconnected network shares that open a placeholder with locate/cached/cancel
  recovery actions.

