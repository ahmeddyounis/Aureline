# Safe-preview and recovery seed

This document is the reviewer-facing entry point for the safe-mode entry
path, the recovery-ladder rung stubs, the suspicious-content detector, and
the representation-labeled safe preview that the live shell projects when
the normal start or save sequence cannot prove it is safe to run.

It seeds the runtime substrate. It does not replace the cross-surface
safe-preview / copy / export depth contract, which is owned by a later lane
and continues to live in:

- [`/docs/security/safe_preview_trust_classes.md`](./safe_preview_trust_classes.md)
  — frozen safe-preview trust-class vocabulary.
- [`/docs/security/suspicious_content_packet.md`](./suspicious_content_packet.md)
  — shared-detector contract and safe-preview affordance baseline.

This seed projects from those documents. Trust-class spellings
(`RawText`, `SanitizedRich`, `TrustedLocalActive`, `IsolatedRemoteActive`),
the suspicious-content class set, the closed annotation-mode set, and the
representation-action vocabulary are reused verbatim.

## Why a seed

Without a substrate, every later preview, recovery, or copy/export surface
would invent its own trust language, recovery rung names, and copy/export
labels. The substrate freezes:

- one safe-mode profile that names what is disabled and what is preserved;
- one recovery-ladder rung vocabulary that downstream rows extend rather
  than fork;
- one shared suspicious-content detector that emits the schema-aligned
  `suspicious_content_case_record` and `representation_transfer_record`
  payloads any preview surface can consume; and
- one crash-loop containment record that always exposes the four
  first-class offers — Open safe mode, Disable suspect extension or
  runtime, Open without restore, and Export evidence — alongside the
  cache/index repair candidate.

## Owned runtime surfaces

| Surface | Crate / module | What it owns |
|---|---|---|
| Safe-mode profile | `aureline-shell::recovery::safe_mode` | The visibly reduced startup posture — disabled capabilities, preserved state classes, entry/exit command ids, and the `safe_mode_profile_record`. |
| Recovery-ladder rungs | `aureline-shell::recovery::ladder` | The closed rung vocabulary (`safe_mode`, `suspect_extension_quarantine`, `open_without_restore`, `cache_or_index_repair`, `restricted_fallback`, `export_evidence`) and the per-rung command ids. |
| Crash-loop containment | `aureline-shell::recovery::crash_loop` | The `crash_loop_containment_record` that exposes the four first-class offers and binds the safe-mode profile to the run. |
| Save-review suspicious annotation | `aureline-shell::recovery::suspicious_save` | The save-review projection that runs the shared detector on local bytes, attaches representation-labeled copy/export records, and surfaces the safe-mode hand-off. |
| Suspicious-content detector | `aureline-content-safety` | Bidi-control, invisible-formatting, and mixed-script confusable detection plus the schema-aligned `suspicious_content_case_record`, `surface_trust_resolution_record`, and `representation_transfer_record` shapes. |

The crates above are the canonical truth source for restricted startup,
preview posture, and recovery-ladder entry. Downstream rows are allowed to
extend them additively but must not fork the vocabulary.

## Honesty invariants

Every protected-walk entry point preserves the following invariants:

1. **No silent rerun.** Auto-rerun stays forbidden across the safe-mode
   profile, the crash-loop containment record, and the save-review
   suspicious-content annotation.
2. **No state deletion as recovery.** Every recovery rung sets
   `never_deletes_state = true`. Cache/index repair is gated and requires
   confirmation; user-authored files, journals, and trust state are
   preserved verbatim.
3. **Trust widening requires review.** Safe mode and crash-loop
   containment both set `trust_widening_forbidden_without_review = true`.
   Exiting safe mode is always an explicit, reviewed user action.
4. **Raw bytes are never rewritten.** The detector annotates and never
   normalizes. Copy and export actions resolve to representation-labeled
   transfer records (`copy_raw`, `copy_escaped`, …); generic `Copy` /
   `Export` labels are non-conforming where representations differ.
5. **Evidence stays exportable.** `export_evidence` is a first-class
   crash-loop offer and `export_escalation_packet` is one of the next
   options after safe-mode entry.

## Detector outcomes

The shared detector projects one outcome per subject. The outcome is a
rendering / annotation decision, not a copy label; copy labels always name
the representation separately.

| Outcome | Trust class projection | Default transfer posture |
|---|---|---|
| `allow` | surface default (`RawText` for editors) | default per class |
| `sanitize` | `SanitizedRich` for rich surfaces; for raw-text editors the body stays `RawText` and the annotation layer is the sanitize point | `copy_raw`, `copy_escaped` (and `copy_rendered` where a rendered view exists) |
| `isolate` | `IsolatedRemoteActive` or `TrustedLocalActive` (under verified origin / connectivity / capability) | `copy_rendered` while verified, `export_sanitized_snapshot`, `export_metadata_only` |
| `block` | `RawText` or `SanitizedRich` (snapshot only) | `copy_raw`, `copy_escaped`, `export_metadata_only` |
| `route_to_system_browser` | `SanitizedRich` on the originating surface | `copy_raw`, `copy_escaped`, `copy_rendered`, `export_sanitized_snapshot` |

For the M1 detector, only `allow` and `sanitize` outcomes are emitted on
plain-text save-review payloads. Every finding is anchored to a
`location_kind` and carries explicit `reveal_affordances` so an inline
marker, codepoint inspector, raw toggle, escaped toggle, and copy-safe
representation are reachable instead of hidden behind tooltips.

## Recovery-ladder rungs

The rungs below are stubs at this milestone — they freeze the names,
command ids, and substrate guarantees so downstream rows can extend them
without re-deriving the vocabulary.

| Rung | Command id | Substrate guarantee |
|---|---|---|
| `safe_mode` | `cmd:workspace.enter_safe_mode` | Reduced surface area, extensions and auto-restore off, files and journals preserved. |
| `suspect_extension_quarantine` | `cmd:workspace.quarantine_suspect_extension` | Suspect extension or runtime disabled for this launch; bundle files preserved. |
| `open_without_restore` | `cmd:workspace.open_without_restore` | Session-restore proposal skipped; layout not auto-rehydrated. |
| `cache_or_index_repair` | `cmd:workspace.repair_cache_or_index_candidate` | Repair candidate offered; user-authored files preserved; gated behind explicit review. |
| `restricted_fallback` | `cmd:workspace.continue_in_restricted_mode` | Continue with the narrowest capability set and visible recovery chrome. |
| `export_evidence` | `cmd:workspace.export_recovery_evidence` | Evidence packet exported without leaving safe mode. |

## Protected walk

The protected walk this seed admits is:

1. **Trigger.** The shell observes a crash-loop strike-budget burn, a
   start-sequence safety failure (broken cache, suspicious content, or
   uncertain trust), or an explicit user request.
2. **Containment.** A `crash_loop_containment_record` is emitted with the
   four first-class offers and the bound safe-mode profile.
3. **Entry.** The user enters safe mode (or accepts another rung). The
   safe-mode profile is logged at
   `<recovery_root>/safe_mode_profile_latest.json` for diagnostics.
4. **Save / preview.** When a save-review surface or any other preview
   surface processes content, it runs the shared detector. Suspicious
   findings produce a save-review annotation with raw and escaped copy
   transfers, the safe-mode hand-off command id, and a status line for
   diagnostics.
5. **Exit.** Trust widening or safe-mode exit requires explicit review;
   `cmd:workspace.exit_safe_mode_after_review` is the canonical exit
   command id.

## Failure drill

The failure drill aligned with this seed is the suspicious-content /
broken-cache walk:

1. Place a fixture buffer that contains bidi, invisible, or confusable
   text on the save-review path (covered by
   `crates/aureline-shell/tests/recovery_protected_walk.rs`).
2. Confirm the save-review annotation surfaces the `sanitize` outcome,
   the bidi / invisible finding, the `copy_raw` ↔ `copy_escaped` pair,
   and the escaped preview lines (no raw bytes are rewritten).
3. Confirm the crash-loop containment record exposes Open safe mode,
   Disable suspect extension or runtime, Open without restore, and
   Export evidence as first-class offers, with cache/index repair
   gated.
4. Confirm the safe-mode profile turns extensions and auto-restore off
   without deleting any preserved state class.

## Out of scope

This seed does not freeze:

- the full extension-bisect automation or Project Doctor surfaces — they
  belong to a later milestone;
- the cross-surface safe-preview / copy / export product depth — owned
  by a later lane that consumes the records emitted here;
- whole-script confusable scoring across full Unicode tables — only the
  mixed-script signal lands here;
- the sandbox / iframe / process boundary that implements
  `IsolatedRemoteActive`.

Extending the seed (a new rung, a new detector class, a new safe-mode
explanation row) is an additive change to the modules above, not a
surface-local invention.
