# Presentation speaker notes: local-only defaults, explicit sharing, and no audience leakage

A speaker note is a presenter's private prompt attached to one waypoint of a
guided walkthrough — the thing you write to yourself so you remember what to say,
which file to open next, and which contract the audience should walk away with.
Because a note is written for the presenter and not the room, Aureline treats it
as **local and private by default**. A note only ever leaves the local machine
through an explicit, separately recorded decision, and even then its raw text
never enters a support, diagnostics, or telemetry export.

The machine truth is the speaker-note sharing model in
`crates/aureline-shell/src/presentation/speaker_notes/`. Diagnostics, support
export, and release surfaces ingest the support-export packet it builds rather
than cloning note state by hand. The canonical note object is the
[speaker-note](../../schemas/presentation/speaker-note.schema.json) boundary
object; the support-export packet's boundary schema is
[speaker-note-export](../../schemas/presentation/speaker-note-export.schema.json),
and its class tokens mirror that schema exactly.

## Local and private by default

Every note starts in the **local** scope:

- A freshly created governed note pins its scope to `local`, its retention to
  `local_only_not_retained_remotely`, its shared-state to `local_not_shared`, and
  carries no explicit-promotion marker. A note cannot start out shared.
- A local note is never retained off the machine and is never reported as
  audience visible. It lives on the presenter's in-memory model and the
  reviewable fixtures only.

## Sharing is explicit, recorded, and never inferred

A note becomes shared **only** through `promote_note_to_shared`, which is the one
path from local to shared. It fails closed:

- The promotion request must carry an explicit acknowledgement that the note will
  leave local-only state, plus a reopenable share-decision ref. An unacknowledged
  or undecided request never produces a shared note.
- Sharing is **never inferred from follow state or co-presence**. The promotion
  request takes no follower or audience input, so the fact that viewers are
  present — or following the presenter — can never, by itself, promote a note.
- A successful promotion emits a `SpeakerNoteShareRecord`: an auditable,
  reopenable record of the local → shared transition, the retention chosen, the
  explicit acknowledgement, and the guarantee that the body did not enter any
  export as part of the share.

When a note leaves local-only state, its **retention, export, and shared-state
posture become visible**:

- **Retention** — a shared note records whether it is retained in the session
  store (`shared_retained_in_session_store`) or delivered live only because policy
  disabled retention (`shared_retention_disabled_by_policy`).
- **Shared-state** — a shared note is `shared_explicitly_promoted`, distinguishing
  a deliberate share from a private aside.
- **Export** — the body-export posture is always `body_never_exported`: the raw
  prompt never crosses the support / diagnostics / telemetry boundary.

## Citations are preserved and typed

A note preserves its links to the objects it was written against — files,
symbols, docs, and graph objects — as typed `NoteCitation`s. The typed citations
stay a faithful view of the canonical note's `citation_refs`: each typed citation
references a ref the note carries, with no ref typed twice and none left untyped.
The support export reports only the **count and the kinds** of citations, never a
citation source.

## Audience and follower surfaces never see a private note

The audience projection (`project_audience_note_disclosures`) is built so a
private note cannot leak:

- Only a deliberately shared note ever produces an audience disclosure. A local
  note is dropped **by construction**, so it can never render on a follower
  surface.
- An audience disclosure carries export-safe metadata — the note id, its
  waypoint, the `shared` scope, and whether a shared body is available to the
  audience — never the body text itself in the metadata record.

## Diagnostics record scope honestly without leaking the prompt

The support-export packet carries one diagnostics row per note. Each row records,
as typed class tokens, the note's scope, its retention / shared-state / export
posture, the count and kinds of its citations, and presence flags — but **never**
the note body or a next-step cue. The packet pins three invariants true and is
rejected if any is flipped:

- `raw_note_bodies_excluded` — no row carries a note body.
- `shared_rows_explicitly_promoted` — every shared row carries an explicit
  promotion marker.
- `no_local_note_audience_visible` — no local note is reported as audience
  visible.

A presenter's notes are theirs. They stay on the local machine unless the
presenter explicitly shares them, the consequences of sharing are visible and
auditable when they do, and a raw prompt never escapes through a support packet.
