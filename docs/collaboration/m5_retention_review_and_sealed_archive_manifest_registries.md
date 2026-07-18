# M5 retention-review and sealed-archive-manifest registries

Implement lane over the frozen [M5 collaboration-control component matrix][matrix]
(`m5_shared_terminal_debug_control_grant_presenter_consent_retention_and_session_restore_view_matrix`).
It makes the matrix's explicit retention review operable — as durable,
resolved records — and adds the sealed-archive manifest, by carrying honest projections of two
registries so the claimed M5 shared terminal / debug view, companion-follow flow, control-grant prompt,
paste / secret guard, support / export packets, and help / docs surfaces inherit one canonical retention
review and one sealed-archive manifest rather than a hand-authored parallel prose that has to be kept
consistent. It closes the gap between the already-landed session-policy, control-grant, presenter-handoff,
and paste / secret-guard lanes and the explicit retention / evidence contract the source
set now expects: collaboration evidence stays honest about what is retained, exportable, deletable, and
policy-bounded after a session ends.

## Registry-A — retention review

One durable, canonical retention review per session, carrying:

- the retention mode — live-only, metadata audit, replayable text / comment timeline, or elevated
  support / regulatory evidence — so each session names exactly what it keeps;
- the disclosed retention envelope and the export / delete-right posture shown at join time and at
  retention-change time, so a retention change cannot begin silently;
- the consent-renewal and guest-scope-widening posture, and the visible retention badge kept mechanically
  distinct from ordinary presence and follow state;
- the single-review binding, so the retention review on one session never reads as consent on another;
- the resolution-form coverage (canonical object, accessible summary, audit record).

A review that cannot bind its retention envelope / export / delete rights to its session / retention
scope, that is a hand-copied per-entry assumption instead of tracing to the shared registry, or that
publishes an incomplete object degrades honestly instead of letting a retention change read as
pre-approved. The registry reuses the matrix `m5-collaboration-retention-sheet.schema.json` domain schema.

## Registry-B — sealed-archive manifest

The typed sealed-archive manifest a participant reads before any archive is trusted — the retention mode
it was captured under, the content address that pins the exact bytes, the policy label bounding export and
delete rights, the archive outcome (disclosed awaiting consent, consent renewal required, guest scope
widened, export / delete-right disclosed, sealed archive created, or blocked with no consent), and the
audit-safe attribution of who created or changed it — plus the fresh, visible consent event any retention
change must raise: a disclose, a consent-renewal, a guest-scope-widen, an export / delete-right change, or
a sealed-archive outcome. The manifest describes sealed archives by content-addressed, policy-labeled
identity rather than generic "session recording saved" language, and never copies a raw session body,
command text, variable body, or clipboard content into logs or exports. A retention-change or
guest-scope-widen event stays attributable in-session and on export. The registry mints the
`m5-sealed-archive-manifest.schema.json` domain schema.

## Acceptance criteria proven by the resolved examples

1. Users can tell at join time and at retention-change time exactly what will be retained and what
   export / delete rights apply: a review that would let a session be retained under a live-only,
   metadata-audit, replayable text / comment timeline, or elevated support / regulatory evidence mode
   without disclosing its retention envelope and export / delete rights, or without an explicit
   policy / consent posture and visible badge, degrades instead of reading as a clean, disclosed review.
2. Sealed archives are described by content-addressed, policy-labeled manifests instead of generic
   "session recording saved" language: the retention mode, content address, policy label, archive
   outcome, and attribution stay visible in the UI projection, the CSV / export, and the support packet
   instead of collapsing into a generic status pill — and never carry raw session, command, variable-body,
   or clipboard material.
3. Collaboration presence never implies consent to retain, no consent renewal or guest-scope widening is
   skipped where required, and retention never widens silently on restore or companion follow; the
   registries keep each retention-review and sealed-archive-manifest dimension distinct.

Raw session bodies, raw command text, variable bodies, clipboard contents, and private endpoints never
cross this boundary. The Rust validator in `crates/aureline-ui` is the authoritative gate; the checked-in
combined registries schema
(`schemas/collaboration/m5-retention-review-and-sealed-archive-manifest-registries.schema.json`)
documents the shape.

[matrix]: ../../crates/aureline-ui/src/m5_shared_terminal_debug_control_grant_presenter_consent_retention_and_session_restore_view_matrix/mod.rs
