# Crash-Loop Screen and Restore-Fidelity Contract

This contract freezes the user-facing and machine-readable posture for
session restore, repeated startup failure, and local forensics. It
composes with the existing entry / restore object model, recent-work
restore-card contract, fault-domain restart policy, recovery ladder, and
support bundle contract:

- [`/docs/workspace/entry_restore_object_model.md`](../workspace/entry_restore_object_model.md)
- [`/docs/ux/recent_work_and_restore_card_contract.md`](./recent_work_and_restore_card_contract.md)
- [`/docs/runtime/fault_domains_and_restart_policy.md`](../runtime/fault_domains_and_restart_policy.md)
- [`/docs/support/recovery_ladder_packet.md`](../support/recovery_ladder_packet.md)
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
- [`/schemas/ux/restore_fidelity.schema.json`](../../schemas/ux/restore_fidelity.schema.json)
- [`/fixtures/ux/crash_loop_cards/`](../../fixtures/ux/crash_loop_cards/)

The goal is narrow: a restored surface must never look healthier than it
is, repeated failure must stop behind a visible recovery choice, and
forensic evidence must survive even when the prior UI cannot be safely
rehydrated.

## 1. Scope

This contract freezes:

- the controlled restore-fidelity classes every startup, diagnostics,
  support export, and docs/help surface uses;
- the crash-loop screen anatomy after bounded restart attempts are
  exhausted;
- the local-forensics surface that carries crash envelopes, logs,
  restore diagnostics, and missing-dependency placeholders;
- topology-adjustment and placeholder behavior for display, extension,
  remote, managed-session, and version-change drift; and
- the action grammar for `Restore now`, `Skip once`, `Open clean`,
  `Compare to disk`, `Open journal`, and `Safe mode`.

Out of scope:

- crash-reporting transport, watchdog implementation, symbolication
  execution, or hosted ticket submission;
- final microcopy or visual layout beyond required fields and action
  semantics; and
- changing the upstream `restore_prompt_record`,
  `forensic_packet_record`, or `support_bundle_record` shapes.

## 2. Restore-Fidelity Classes

The controlled classes are:

| Class | Meaning | Must not imply |
| --- | --- | --- |
| `Exact restore` | The window, pane tree, editor identity, dirty buffers, cursor / scroll, stable inspectors, and non-mutating panels were rehydrated under the same compatible state contract. | Runtime sessions reran, authority tickets survived, or remote work reattached. |
| `Compatible restore` | The prior session was translated through a compatible schema, version, or topology adjustment with visible notes. | Pixel-perfect layout, unchanged extensions, or exact state equivalence. |
| `Layout only` | The shell restored pane slots, titles, grouping, and last-known identity while live state stayed unavailable. | A missing extension, remote target, notebook kernel, terminal, or preview server is live. |
| `Recovered drafts` | User-authored local drafts or journals were recovered without proving that target files or services accepted the writes. | The draft was saved to disk, pushed remotely, or applied to an external service. |
| `Evidence only` | The UI state could not be safely restored; crash envelope, logs, diagnostics, transcripts, and restore manifest refs remain inspectable. | Any prior pane is ready, any task reran, or any skipped restore deleted evidence. |

These labels appear unchanged on startup cards, crash-loop screens,
diagnostics panels, support-bundle previews, exported packet summaries,
and docs/help examples. A surface may add explanatory text, but it may
not substitute a softer label such as "almost restored" or "safe enough"
when the class is lower fidelity.

## 3. Surface Rules

Startup and restore surfaces must show:

- the restore-fidelity class;
- counts for restored windows, panes, dirty buffers, placeholders, and
  evidence packets;
- any missing dependency or topology note that lowered fidelity;
- the action set available for the current class; and
- a statement that skipped or clean-open decisions retain evidence when
  they do.

Diagnostics and Project Doctor surfaces must show:

- the same class and crash ids shown at startup;
- fault-domain and suspected-actor confidence, including
  `unknown_pending_probe` when confidence is not high enough;
- restart-budget counters and visible escalation state; and
- links to local forensics, logs, support export preview, and recovery
  ladder entries.

Support export surfaces must show:

- what is embedded, exported by reference, retained locally, or excluded;
- redaction class and destination posture before export;
- exact-build symbolication state; and
- stable references to crash envelope, restore manifest, forensic
  packets, and support-bundle candidate ids.

Docs/help surfaces must use the same controlled labels and explain what
each action preserves, discards, or defers. Help text must not imply
that `Skip once`, `Open clean`, or `Open without restore` deletes the
underlying evidence.

## 4. Crash-Loop Screen Anatomy

A crash-loop screen appears after the relevant fault domain exhausts its
bounded automatic restart budget or when the shell cannot safely attempt
another restore without user choice. The screen owns the whole recovery
decision for the failing launch; it is not a toast and not a hidden log.

Required fields:

| Field | Requirement |
| --- | --- |
| Restore class | One controlled restore-fidelity class from this contract. |
| Last attempted reopen mode | The last mode attempted, such as prior session restore, safe mode, open without restore, or disable-suspect-then-restore. |
| Suspected actor | Actor class, optional actor ref, confidence class, and confidence note. Unknown confidence is valid and must be explicit. |
| Last failure summary | Crash id, crash envelope ref, exit reason, build identity ref, restore manifest ref, symbolication state, and time. |
| Restart budget | Strike window, automatic budget, count at failure, escalation state, and whether another automatic restart is allowed. |
| Local forensics | Crash envelope, logs, restore diagnostics, forensic packets, missing-dependency placeholders, redaction class, and export posture. |
| Recovery recommendation | Best next safe action, why it is recommended, what it preserves, and what it defers or narrows. |
| Actions | `Safe mode`, `Open without restore`, `Disable suspect extension` when applicable, `Export evidence`, `Open logs`, and either `Retry reopen` or a disabled retry reason. |

Non-conforming behavior:

- restarting forever behind a splash screen;
- a generic `Try again` button that hides whether the action enters
  safe mode, skips restore, disables an extension, or retries the same
  failing path;
- claiming a suspect extension with high certainty when the evidence is
  only temporal adjacency; or
- removing or hiding evidence because full UI restore failed.

## 5. Local-Forensics Surface

The local-forensics surface is present whenever crash-loop or
evidence-only recovery is present. It may render inside the crash-loop
screen, diagnostics panel, support-bundle preview, or a compact card,
but the backing record keeps these fields separate:

- `crash_envelope_ref`;
- `log_refs[]`;
- `restore_diagnostic_refs[]`;
- `forensic_packet_refs[]`;
- `support_bundle_candidate_ref`;
- `missing_dependency_placeholders[]`;
- `redaction_class`;
- `export_posture`; and
- reviewable notes for what is local-only, exported by reference, or
  omitted by default.

Local-first is the default. Raw dumps, full logs, absolute paths,
provider payloads, credentials, and raw terminal scrollback are not
exported automatically. A support bundle may reference retained local
artifacts without embedding them.

## 6. Placeholder And Topology Behavior

Placeholders preserve user orientation without claiming live readiness.

| Situation | Required behavior |
| --- | --- |
| Monitor topology changed | Clamp windows and dialogs on-screen, preserve split order where possible, and show a one-time layout-adjusted note when fidelity changed. |
| Missing extension host | Preserve surrounding pane layout and replace contributed surfaces with missing or quarantined extension placeholders. |
| Remote target unavailable | Preserve local buffers, transcripts, review state, and stale metadata as evidence; mark remote panes reconnect or reauth required. |
| Managed session expired | Preserve local shell context and evidence; require explicit reauth before managed actions resume. |
| Version or schema changed | Downgrade from exact to compatible or layout-only when state translation changed semantics; preserve prior artifact for compare/export when feasible. |
| Corrupt restorable state | Quarantine the suspect state, offer compare/export/open-journal where possible, and fall back to evidence-only or draft recovery. |

Any pane that depends on stale, cached, unavailable, or incompatible
state must render that posture in-surface until the runtime is actually
live again or the user deliberately removes the surface.

## 7. Action Grammar

Every action record names `action_id`, availability, preserved state,
discarded or deferred behavior, whether confirmation is required, and
whether evidence remains after the action.

| Action | Available when | Preserves | Discards or defers |
| --- | --- | --- | --- |
| `Restore now` | A restore prompt has at least one restorable layout, draft, checkpoint, or evidence-backed item. | Session manifest, eligible layout, dirty-buffer journals, checkpoints, and evidence refs. | No live session reattach or rerun unless separately requested. |
| `Skip once` | A restore prompt is present and user wants to enter without applying it this launch. | Restore manifest, journals, crash envelope, and evidence refs. | Auto-restore for this launch only. |
| `Open clean` | The user chooses a new clean shell instead of restoring the prior UI. | Evidence, local history, support refs, and retained journals unless explicitly cleared later. | Prior layout rehydration for this launch. |
| `Compare to disk` | A recovered draft has a target file identity and readable disk state. | Draft and disk identities. | No write or discard; comparison only. |
| `Open journal` | A recovery journal exists and policy allows local inspection. | Journal identity and evidence. | No apply, save, or delete. |
| `Safe mode` | Repeated failure, suspected extension/profile/runtime regression, or support-directed recovery. | User files, recovery journals, evidence, settings, trust store, and checkpoint refs. | Extension auto-activation, session auto-reopen, remote helper attach, AI runtime access, and background rebuild until widened. |

`Open without restore`, `Disable suspect extension`, `Export evidence`,
`Open logs`, and `Retry reopen` follow the same record shape. Retry is
disabled when it would consume another automatic restart behind the same
failing path.

## 8. Live-Readiness Rule

No restored pane or session can claim live readiness unless the backing
runtime actually survived or was explicitly reconnected under current
authority. The following phrases are non-conforming for context,
transcript, draft, or evidence restoration:

- "running again";
- "connected";
- "ready";
- "restored live"; and
- "resumed" for an expired or reauthorized session.

Use posture labels instead: `transcript restored, not rerun`,
`reconnect available`, `rerun required`, `context unavailable`,
`journal recovered`, or `evidence only`.

## 9. Acceptance Checklist

A reviewer can accept a crash-loop or restore-fidelity surface when:

1. The controlled class is visible and reused across startup,
   diagnostics, support export, and docs/help.
2. Bounded restart counters are visible after repeated failure; no
   invisible automatic loop remains.
3. Local forensics survive even when UI restore fails.
4. Missing dependency placeholders preserve topology without faking live
   capability.
5. Each action states what it preserves, discards, or defers.
6. `Skip once`, `Open clean`, and `Open without restore` explicitly
   retain evidence.
7. Support/export refs preserve crash ids, restore class, fault-domain
   id, and exact-build symbolication posture.
