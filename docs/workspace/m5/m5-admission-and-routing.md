# M5 workspace admission and first-useful-work routing

This document is the contract for the `m5_admission_and_routing_packet`. The canonical packet is
checked in at `artifacts/workspace/m5/m5-admission-and-routing.json`, validated by
`schemas/workspace/m5-admission-and-routing.schema.json`, and backed by the typed model in the
`aureline-workspace` crate (`m5_admission_and_routing`).

## What the packet governs

Every M5 wedge — notebook, data/API, profiler, framework-pack, docs, companion, sync-handoff,
and opened local folder — carries a truthful **admission checkpoint** instead of a feature-local
empty state or a forced setup wizard. Each checkpoint answers, before the wedge takes over:

- **How admission classifies the workspace** — one of six distinct
  [`AdmissionClass`] states: `certified`, `probable`, `mixed`, `unknown`, `restricted`, or
  `missing_prerequisite`. These stay distinct in UI, diagnostics, and docs/help.
- **Where the classification came from** — an explicit `detection_source` source label
  (`certified_manifest`, `framework_pack_signal`, `heuristic_probe`, `mixed_signals`,
  `user_declared`, `no_signal`, `policy_restriction`, or `missing_toolchain_probe`).
- **How confident archetype detection is** — `archetype_confidence` of `certified`, `probable`,
  `mixed`, or `unknown`.
- **Where any bundle recommendation originates** — `bundle_recommendation_source`.
- **Which first-useful-work route the wedge offers** — `open_minimal`, `set_up_later`,
  `guided_setup_offered`, `local_safe_fallback`, or `restricted_browse`.

## Blocking-now, recommended-soon, optional-later

Setup is separated by urgency, not funneled. Each setup item carries a `timing`:

- **`blocking_now`** — gates the wedge's primary feature until it is done (e.g. the profiler
  needs a runtime). It still leaves minimal local-safe work available.
- **`recommended_soon`** — worth doing, but deferrable.
- **`optional_later`** — optional and deferrable indefinitely.

Open-minimal and set-up-later paths carry equal weight with guided setup wherever it is safe.
Users can defer every non-blocking item and still reach minimal local-safe work
(`local_safe_work_available`). Only an explicit policy `restricted` admission removes local-safe
work; a `missing_prerequisite` wedge keeps it.

## What stays honest

The packet is fail-closed on five honesty rules, all checked by `validate()`:

1. **No hidden setup and no forced funnel.** Every checkpoint holds four guardrail flags closed —
   `forces_wizard`, `auto_installs_packs`, `rewrites_layout_without_review`, and
   `widens_trust_without_review` are always `false` — and no setup item ever `auto_runs`. Probable
   and mixed detection therefore never auto-install packs, rewrite layouts, or widen trust without
   review.
2. **Probable or mixed is never certified.** A checkpoint may only set
   `presented_as_certified_support` when its admission class is actually `certified`.
3. **No claim out-ranks its evidence.** The admission class may never out-rank the archetype
   confidence it derives from ([`AdmissionClass::permitted_under`]), and a bundle recommendation
   may never out-rank that confidence either.
4. **The source label must fit the class.** The `detection_source` must be canonical for the
   admission class ([`DetectionSource::is_canonical_for`]).
5. **Routing fits the class and the blocking.** A `restricted_browse` route only fits a
   `restricted` class, a `local_safe_fallback` fits a blocking prerequisite or a
   `missing_prerequisite` class, and `open_minimal` / `set_up_later` / `guided_setup_offered`
   only fit non-restricted, non-missing-prerequisite classes with no blocking item.

A checkpoint that is anything other than certified, or that carries a blocking or review-gated
setup item, must also carry a caveat so the weaker assurance is never silent.

## Provenance and downstream surfaces

Archetype confidence, the bundle-recommendation source, and first-useful-work routing provenance
survive into support and help surfaces. Each checkpoint carries a `routing_provenance_ref`, an
`archetype_evidence_ref`, and a `bundle_recommendation_ref`, plus `diagnostics_ref`,
`support_export_ref`, `help_surface_ref`, `docs_badge_ref`, and `release_evidence_ref`. Those
surfaces ingest the same packet (via `export_projection()`, which is redaction-safe) rather than
re-deriving divergent status text.

## Checkpoint roll-up (as of 2026-06-11)

| Checkpoint | Wedge | Class | Source | Confidence | Route | Local-safe | Setup (blocking/soon/later) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `notebook-certified` | notebook | certified | certified_manifest | certified | guided_setup_offered | yes | 0 / 1 / 1 |
| `data-api-probable` | data/API | probable | heuristic_probe | probable | open_minimal | yes | 0 / 1 / 1 |
| `profiler-missing-prereq` | profiler | missing_prerequisite | missing_toolchain_probe | unknown | local_safe_fallback | yes | 1 / 0 / 0 |
| `framework-pack-certified` | framework-pack | certified | framework_pack_signal | certified | guided_setup_offered | yes | 0 / 1 / 1 |
| `docs-unknown` | docs | unknown | no_signal | unknown | set_up_later | yes | 0 / 0 / 1 |
| `companion-mixed` | companion | mixed | mixed_signals | mixed | open_minimal | yes | 0 / 1 / 1 |
| `sync-handoff-probable` | sync-handoff | probable | framework_pack_signal | probable | set_up_later | yes | 0 / 1 / 1 |
| `local-folder-restricted` | local folder | restricted | policy_restriction | unknown | restricted_browse | **no** | 0 / 1 / 0 |

Seven of eight checkpoints keep minimal local-safe work; only the policy-`restricted` local
folder is browse-only. Two checkpoints present as certified support (notebook, framework-pack);
the rest never do. One blocking-now item (the profiler runtime) gates a feature without removing
local-safe work, and four setup items are review-gated.

## How it is validated

The typed model parses the embedded packet and runs `validate()`, which checks the closed
vocabularies, full wedge coverage, the four guardrail flags, the no-auto-run guard, the
class/confidence and bundle/confidence ceilings, detection-source canonicality, the
certified-presentation and local-safe recomputations, route consistency, complete provenance,
per-item consistency, required caveats, and the recomputed summary. The unit tests in
`crates/aureline-workspace/src/m5_admission_and_routing/tests.rs` assert the embedded packet
validates clean and that every wedge, admission class, route, and setup timing is exercised.

[`AdmissionClass`]: ../../../crates/aureline-workspace/src/m5_admission_and_routing/mod.rs
[`AdmissionClass::permitted_under`]: ../../../crates/aureline-workspace/src/m5_admission_and_routing/mod.rs
[`DetectionSource::is_canonical_for`]: ../../../crates/aureline-workspace/src/m5_admission_and_routing/mod.rs
