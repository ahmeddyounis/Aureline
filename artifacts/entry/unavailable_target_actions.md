# Unavailable-Target Recovery Actions (Entry Surfaces)

This artifact publishes the canonical unavailable-target vocabulary and recovery action grammar used by **recent work**, **restore**, **workspace switching**, and **system/protocol reentry**. Its goal is cross-surface honesty: missing or offline targets MUST NOT look like ordinary local opens, and cached/read-only routes MUST NOT imply write safety, trust continuity, or live connectivity they do not have.

This file does **not** define final UI composition. It freezes **state → allowed action set → required disclosures** so every surface can render truthful rows, prompts, and placeholders without inventing local recovery language.

## 1. Canonical sources (quoted by reference)

The vocabularies referenced here are already frozen elsewhere; this artifact binds them into one entry/recovery corpus:

- Row-level anatomy and action ids:
  - `docs/ux/recent_work_and_restore_card_contract.md`
  - `schemas/ux/recent_work_row.schema.json`
- Restore prompt controlled labels and action grammar:
  - `docs/ux/crash_loop_and_restore_fidelity_contract.md`
- Entry / restore object model (missing-target disclosure before commit):
  - `docs/workspace/entry_restore_object_model.md`
  - `schemas/workspace/entry_and_restore_result.schema.json`
- System-open and protocol/deep-link reentry:
  - `schemas/platform/system_open_target_packet.schema.json`
  - `schemas/platform/deep_link_intent.schema.json`
- Entry-surface matrix and unavailable-target action sets:
  - `.t2/docs/Aureline_Technical_Design_Document.md` (Appendix BD)
  - `.t2/docs/Aureline_UI_UX_Spec_Document.md` (desktop integration + removable volume/share rules)

## 2. Unavailable-target situations (canonical mapping)

Each situation below MUST project a **target state** and **unavailable reason** (row/placeholder chip) and MUST constrain recovery actions to the allowed set. If a surface cannot determine the exact sub-reason, it MUST degrade to the safer umbrella reason and still offer a bounded recovery path.

### 2.1 Missing folder (local path no longer resolves)

- Canonical projection:
  - `target_state = missing_target`
  - `unavailable_reason = missing_path`
  - `write_safety_badge = writes_blocked_target_unavailable`
- Allowed actions (bounded set):
  - `locate_missing_target`
  - `open_read_only_cached_view` (only when a safe cached context exists)
  - `remove_from_recents`
  - `retry_later`

### 2.2 Moved workspace file (workspace manifest moved/renamed)

- Canonical projection:
  - `target_state = moved_target_detected`
  - `unavailable_reason = moved_root`
  - `write_safety_badge = writes_unsafe_stale_or_disconnected` (until identity is re-anchored)
- Allowed actions:
  - `locate_missing_target`
  - `open_read_only_cached_view` (when a cache exists; remains non-writing)
  - `remove_from_recents`
  - `retry_later`

### 2.3 Disconnected network share (path exists only behind a share/mount)

- Canonical projection:
  - `target_state = missing_mount` (preferred when the mount/share boundary is known)
  - `unavailable_reason = missing_mount` or `remote_disconnected` (when the remote endpoint is known but disconnected)
  - `write_safety_badge = writes_blocked_target_unavailable`
- Allowed actions:
  - `locate_missing_target` (re-anchor to a new share path or a local fallback)
  - `open_read_only_cached_view` (cached-only; never implies remount success)
  - `retry_later`
  - `remove_from_recents`

### 2.4 Missing mount (removable volume, drive letter, mount point, or fstab mount absent)

- Canonical projection:
  - `target_state = missing_mount`
  - `unavailable_reason = missing_mount`
  - `write_safety_badge = writes_blocked_target_unavailable`
- Allowed actions:
  - `locate_missing_target`
  - `open_read_only_cached_view` (cached-only)
  - `retry_later`
  - `remove_from_recents`
- Restore prompt additions (when the missing mount prevents safe restore):
  - `open_clean` (clean shell with preserved evidence; no hidden replay)
  - `skip_once` (defers restore without deleting evidence)
  - `restore_now` (MAY be present but MUST be disabled until the mount/identity is revalidated)

### 2.5 Offline SSH host (remote root unreachable)

- Canonical projection:
  - `target_state = remote_unreachable`
  - `unavailable_reason = remote_disconnected`
  - `write_safety_badge = writes_unsafe_stale_or_disconnected`
- Allowed actions:
  - `reconnect` (preferred)
  - `reauth` (when reconnect is blocked by auth)
  - `open_read_only_cached_view` (cached-only; no implied remote parity)
  - `retry_later`
  - `remove_from_recents`

### 2.6 Expired managed session (authority ticket/lease expired)

- Canonical projection:
  - `target_state = authority_expired`
  - `unavailable_reason = authority_expired`
  - `write_safety_badge = writes_require_revalidation` (or stricter when policy blocks reauth)
- Allowed actions:
  - `reauth` (preferred)
  - `reconnect` (when a new ticket can be minted without widening scope)
  - `open_read_only_cached_view` (metadata/evidence only)
  - `retry_later`
  - `remove_from_recents`

### 2.7 Revoked permission (policy/trust/admin revocation)

- Canonical projection:
  - `target_state = policy_blocked`
  - `unavailable_reason = policy_blocked`
  - `write_safety_badge = writes_blocked_policy`
- Allowed actions:
  - `open_read_only_cached_view` (only when policy allows cached inspection)
  - `open_restricted` (when a restricted local-only posture is supported)
  - `export_evidence` (when required by policy for escalation)
  - `retry_later`
  - `remove_from_recents`

## 3. Recovery action route contract (required disclosures per choice)

For every unavailable-target recovery surface (row, prompt, placeholder, deep-link review), each offered recovery action MUST carry the four disclosure axes below. These are the badges that prevent “looks like a normal open” failure modes.

### 3.1 Identity disclosure (required)

Each action MUST disclose the target identity being acted upon without relying on raw absolute paths:

- Required fields (minimum):
  - `target_kind` and `root_kind`
  - an opaque identity reference (for example a recent-work id, workspace ref, deep-link target ref, or filesystem identity token ref)
- Rules:
  - if identity is stale or missing, the action MUST disclose that it is re-anchoring (Locate) or re-validating (Reconnect/Reauth) rather than resuming silently.

### 3.2 Trust disclosure (required)

Each action MUST disclose trust posture:

- Required fields (minimum):
  - `trust_state` (trusted/restricted/pending evaluation), plus any “requires review/revalidation” posture carried by the owning record.
- Rules:
  - read-only cached routes MUST NOT claim trust continuity for remote/managed targets; they are inspection-only until a live target is revalidated.

### 3.3 Stale-state disclosure (required)

Each action MUST disclose whether it operates on live, cached, stale, or unknown state:

- Required fields (minimum):
  - `freshness_class` (or equivalent cache/age cue on the owning record)
  - when cached: a coarse age label or last-validated time bucket (never a raw sensitive timestamp in privacy-reduced mode)
- Rules:
  - cached views MUST render as cached-only even when they contain enough metadata to “look complete”.

### 3.4 Unsupported-write disclosure (required)

Each action MUST disclose write safety with a single write-safety badge:

- Required fields:
  - `write_safety_badge`
- Rules:
  - `open_read_only_cached_view` MUST always pair with a non-writing badge (`writes_blocked_cached_view_only` or stricter).
  - any route that could widen authority (reconnect/reauth) MUST block writes until revalidation completes.

## 4. Surface parity rules (no per-surface recovery language)

The same unavailable-target vocabulary MUST apply across:

- recent-work rows and restore cards (`schemas/ux/recent_work_row.schema.json`)
- restore prompts and next-step hooks (`schemas/workspace/entry_and_restore_result.schema.json`)
- workspace switcher entries (`schemas/ux/recent_work_row.schema.json`)
- system-open placeholders (`schemas/platform/system_open_target_packet.schema.json`)
- protocol/deep-link review (`schemas/platform/deep_link_intent.schema.json`)

Surfaces MAY differ in presentation (row vs modal vs sheet), but they MUST:

- use the same target-state and recovery-action ids for equivalent situations;
- avoid “Open anyway” language;
- avoid implying live connectivity or write safety on cached routes;
- emit exportable decision/outcome events using `schemas/entry/recovery_choice.schema.json`.
