# M5 portable-shelf and reopen-parity registries

Implement lane over the frozen [M5 change-object / patch-stack / landing matrix][matrix]
(`m5_change_object_patch_stack_and_landing_matrix`). It makes the matrix's `portable_shelf`
object class operable — as a portable shelf / bundle form for a change object or landing candidate
and the reopen-parity posture that keeps it honest when reopened — by carrying resolved, honest
projections of two registries so the Git, patch-stack / queue, review, provider-landing, help / docs,
and support / export surfaces inherit one canonical portable-shelf descriptor and one reopen-parity
posture rather than a hand-authored parallel prose that has to be kept consistent. It closes the gap
between the already-landed change-object records, patch-stack / queue and landing-candidate review,
worktree-manager rows and cleanup previews, review bundles, and provider mutation boundaries and the
export / import continuity contract the source set now expects: stacked work is made portable and
recoverable, and the bundle ID, diff refs, evidence refs, review-pack version, redaction profile, and
import / reopen status stay visible metadata instead of implicit.

The goal: give change objects a portable shelf / bundle form so stacked work can survive browser
handoff, offline follow-up, support escalation, incident bridge, and review export without requiring one
specific code host or cloud service.

## Registry-A — portable shelf / bundle

One durable, canonical portable shelf / bundle per change object or landing candidate, carrying:

- the **bundle ID** and the **diff refs**, disclosed rather than inferred from ambient branch state;
- the **evidence refs** and the **review-pack version** the bundle was captured at;
- the **redaction profile** applied so the export stays safe for browser, support, or offline handoff;
- the **import / reopen status** so an exported shelf that is imported and reopened is never treated as
  still live on a provider;
- the exported-versus-imported-versus-reopened state, so a user can export and later reopen a change
  object or landing candidate without losing diff / evidence identity;
- the resolution-form coverage (canonical object, accessible summary, audit record).

A portable shelf that cannot bind its bundle ID to its diff / evidence refs, that is a hand-copied
per-entry assumption instead of tracing to the shared registry, or that publishes an incomplete object
degrades honestly instead of letting a local imported shelf read as a live provider object. The registry
reuses the matrix `m5-portable-shelf.schema.json` domain schema.

## Registry-B — reopen parity

Each portable shelf keeps its explicit **reopen-parity** posture available, so an imported shelf stays
blocked from reading as a live provider object for background agents and broad automation unless the user
explicitly reopens and confirms it. Exported shelves must reopen as honest local / imported state rather
than pretending to remain a live provider object: the parity names the handoff channel the shelf travelled
(browser companion, support escalation, incident bridge, offline follow-up, or review export), the reopen
state (exported, imported, reopened as local, reopened as provider-linked, stale, or redacted), the
reopened truth posture (local-only, provider-linked, stale provider link, or redacted profile), the
reopening actor (an explicit user command, a background agent, or broad automation), and whether the diff
/ evidence identity is preserved across export and reopen — so an imported shelf is never surfaced as
provider-authoritative while its provider link is stale, local-only, or redacted. The registry reuses the
newly minted `m5-portable-shelf-reopen-parity.schema.json` domain schema.

## Acceptance criteria proven by the resolved examples

1. Users can export and later reopen a change object or landing candidate without losing diff / evidence
   identity: a portable shelf missing its bundle ID or diff / evidence refs degrades instead of reading as
   a recoverable shelf, and the bundle ID, diff refs, evidence refs, review-pack version, and import /
   reopen status stay visible in the UI projection, the CSV / export, and the support packet.
2. Imported shelves clearly show local-only, provider-linked, stale, or redacted state instead of
   overclaiming current hosted truth: the handoff channel, the reopen state, and the reopened truth
   posture stay distinct instead of collapsing into a generic status pill, and an imported shelf whose
   provider link is stale or whose profile is redacted keeps the diff / evidence identity preserved for
   reopen or retry through the recovery checkpoint.
3. No change object is exported or reopened without an explicit bundle identity, stack members are never
   silently reordered, collapsed, or retargeted, and nothing lands from an overclaimed imported shelf; the
   reopen-parity registry keeps each parity posture distinct and never lets a local imported estimate read
   as a provider-authoritative object.

Raw paths, raw provider payloads, secret values, and private endpoints never cross this boundary.
The Rust validator in `crates/aureline-ui` is the authoritative gate; the checked-in combined
registries schema
(`schemas/teamwork/m5-portable-shelf-and-reopen-parity-registries.schema.json`)
documents the shape.

[matrix]: ../../crates/aureline-ui/src/m5_change_object_patch_stack_and_landing_matrix/mod.rs
