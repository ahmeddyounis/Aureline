# M5 source-acquisition review sheets

This document describes the canonical packet that freezes the **M5 source-acquisition review
sheets** — one inspectable review sheet per M5 starter family that explains the network, disk,
trust, bootstrap, and topology consequences of a clone, open, import, or resume **before the
user commits**. It is the user-facing companion to the governed artifact at
`artifacts/workspace/m5/m5-source-acquisition-review.json` and the typed model in the
`aureline-workspace` crate (`m5_source_acquisition_review`).

This packet answers the source-acquisition honesty question for every new M5 starter family:
**does cloning, opening, importing, or resuming a workspace disclose its source-locator,
checkout-plan, topology, and cost truth — and keep its follow-up setup queue explicit and
un-run — before any irreversible network or disk action, or does it hide a sparse, shallow,
LFS, submodule, or interrupted-bootstrap state and silently widen the verb?**

## What this packet covers

The packet carries one review sheet for each M5 starter family, each pinned to one distinct
entry verb so clone, open, import, and resume never blur together:

1. **`template_starter`** (verb `open`) — a template-gallery starter materialized locally.
2. **`framework_pack_starter`** (verb `clone`) — a sparse framework-pack clone from a remote.
3. **`remote_clone_starter`** (verb `clone`) — a heavy, shallow remote clone with a nested repo.
4. **`sync_handoff`** (verb `clone`) — a partial clone served from a mirror.
5. **`companion_handoff`** (verb `import`) — a companion handoff bundle imported into a new root.
6. **`migration_import`** (verb `import`) — a migration archive from another tool.
7. **`session_resume`** (verb `resume`) — a reattached prior live session.
8. **`local_folder_open`** (verb `open`) — an existing local folder opened in place.

Each sheet answers, for its action:

- **Which verb is it?** A locked [`entry_verb`] (`clone`, `open`, `import`, or `resume`) that is
  never silently rewritten. A clone with an existing local copy stays a clone; an import of a
  resumable-looking bundle stays an import.
- **What is being acquired?** A `source_kind`, `host_or_mirror_class`, `protocol`, and
  `checkout_mode`, plus an opaque `target_path_ref` destination.
- **What will it cost, and how trusted is it?** An `expected_cost_band` (`local_no_fetch`
  through `very_heavy_fetch`) and a `trust_stage` (`first_party_trusted`,
  `trusted_continuation`, `review_required`, or `untrusted_browse_only`), disclosed before any
  transfer.
- **What is the topology?** A list of `topology_cues`, each a `cue_kind` (nested repo,
  submodule, shallow history, sparse checkout, LFS pointer, interrupted bootstrap, or omitted
  data) with a `state`, a one-step `recovery_action`, and `recoverable` /
  `blocks_first_useful_work` flags.
- **What follows setup?** A `follow_up_queue` of previewed items — submodule init, LFS hydrate,
  docs import, package-restore suggestion, index warm-up, or bundle recommendation — each with
  a `run_posture` and a `runs_implicitly` guard that is **always `false`**.
- **How is it reconstructable?** A `source_locator_ref` and a `checkout_plan_ref` carried into
  diagnostics and support export, plus `diagnostics_ref`, `support_export_ref`,
  `help_surface_ref`, `docs_badge_ref`, and `release_evidence_ref` so those surfaces ingest the
  same packet.

## The review gate

Each sheet's `review_required_before_acquisition` flag is **derived**, not asserted. A sheet
requires review before acquisition when any of these hold:

- the verb is `clone`, `import`, or `resume` (only a clean local `open` can be review-free);
- the `expected_cost_band` implies a network fetch (anything but `local_no_fetch`);
- any topology cue applies (state `active`, `pending`, or `partial`); or
- any follow-up item is a deferred setup step (any posture but `suggested`).

The recorded flag must equal this recomputed value, so a sheet can never claim it needs no
review while it hides a sparse checkout or a deferred submodule init. The single `open` sheet
with no fetch, no cue, and no deferred step — `local_folder_open` — is the local-safe baseline
that proves the review sheet is **not** a blanket gate.

## Invariants the gate enforces

- **Distinct, locked verbs.** Clone, open, import, and resume stay distinct. The recorded verb
  must be canonical for the source kind (a clone only fits a remote or mirror; an open only fits
  a local folder or template; an import only fits an archive or handoff; a resume only fits a
  live session), and every sheet is `verb_locked`. A local copy or a resumable-looking bundle
  never silently rewrites the verb.
- **Topology stays visible and recoverable.** Every cue that applies offers a one-step recovery
  or widen action, and every cue that blocks first-useful-work is recoverable, so a sparse,
  shallow, LFS, submodule, nested-repo, interrupted-bootstrap, or omitted-data state is never a
  dead end and never looks like missing or unsupported data. A cue that does not apply
  (`not_present`) offers no recovery.
- **Nothing runs implicitly.** Every follow-up item carries `runs_implicitly: false`; the queue
  previews the consequence of submodule init, LFS hydrate, docs import, package restore, index
  warm-up, or a bundle recommendation without performing it.
- **Provenance is reconstructable.** Every sheet carries a `source_locator_ref` and a
  `checkout_plan_ref`, so a wrong-target or half-bootstrap incident remains reconstructable from
  diagnostics and support export.

## Diagnostics

The `source_locator_ref` and `checkout_plan_ref` on each sheet are the diagnostics anchor: a
support reviewer can read which locator resolved the source and which checkout plan shaped the
acquisition, then cross-reference the topology cues to explain exactly what Aureline fetched,
omitted, or deferred. The `diagnostics_ref` points back at this section.

## Support export

The export projection (`M5SourceAcquisitionReviewPacket::export_projection`) is redaction-safe:
it carries the sheet id, verb, source/host/protocol/checkout/cost/trust tokens, provenance refs,
applicable cue kinds, follow-up kinds, and a `verb_canonical_and_locked` flag, with no caveats,
notes, or raw payloads. Support exports ingest this projection rather than re-deriving status
text.

## Help surface

The help/start-center surface reads each sheet's verb, cost band, trust stage, and topology cues
to render the pre-commit review before the user confirms a clone, open, import, or resume. The
`help_surface_ref` points back at this section.

## Docs badges

Docs badges reflect each sheet's review requirement and topology-cue presence so documentation
never implies a frictionless open where a sparse or interrupted entry is in play. The
`docs_badge_ref` points back at this section.

## Release evidence

Release evidence cites the packet's summary roll-up — verb distribution, sheets requiring
review, sheets with active or blocking cues, and recoverable-cue count — to show the
source-acquisition lane discloses topology and cost truth across every M5 starter family. The
`release_evidence_ref` points back at this section.

## Related packets

This packet builds on the stable source-locator, checkout-plan, bootstrap-result, and queue
truth (`stabilize_source_locator_checkout_plan_bootstrap_result_and_queue`) and is governed,
alongside the other entry lanes, by the M5 workflow-bundle and project-entry governance matrix
(`docs/workspace/m5/m5-entry-and-bundle-governance.md`).
