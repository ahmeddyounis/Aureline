# Stable version-window freeze — CLI, schema, API, manifest

This document is the reviewer-facing companion for the gated stable version-window
freeze:

- [`/artifacts/release/stable_version_windows.json`](../../artifacts/release/stable_version_windows.json)
- schema: [`/schemas/release/stable_version_windows.schema.json`](../../schemas/release/stable_version_windows.schema.json)
- proof packet:
  [`/artifacts/release/m4/stable_version_windows_proof_packet.md`](../../artifacts/release/m4/stable_version_windows_proof_packet.md)

The freeze is the **canonical truth** for the version window each public interface
surface commits to for the release line and the deprecation packet that governs how
older versions leave that window. The other stable launch-control artifacts answer
adjacent questions — the [stable claim manifest](./stable_claim_manifest.md) decides
the single canonical label each *subject* publishes, and the
[stable proof index](./stable_proof_index.md) decides whether each launch-blocking
*requirement* is proven. This freeze answers the interface question: **which version
window does each CLI, schema, API, and manifest surface commit to, and is that window
actually frozen?** Downstream dashboards, docs, Help/About surfaces, release packets,
and support exports MUST ingest this freeze by `window_id` and render its
`version_window` and `frozen_label` rather than minting their own per-surface version
or maturity wording.

## Surfaces, windows, deprecation packets, claims — one row each

Each `row` is one `(surface, public claim)` binding. It names:

- the interface **surface** it freezes — `surface_kind` (`cli`, `schema`, `api`, or
  `manifest`), `surface_ref`, `surface_summary`, and whether it is `release_blocking`;
- the stable **version window** — `version_window` (`floor_version`,
  `current_version`, `ceiling_version`, `compatibility_posture`);
- the **deprecation packet** that governs the window — `deprecation_packet` (a
  `packet_id` and a list of `deprecations`, each naming the deprecated version, what
  supersedes it, the announcement date, the removal target version and date, the
  migration ref, and the status);
- the **freeze packet** that proves the freeze — `freeze_packet` (id, packet ref, the
  stable-proof-index registration ref, captured-at date, freshness SLO, SLO state,
  and evidence refs);
- the **waiver** (if any) that holds it provisionally — `waiver`;
- the public **claim** it backs — `claim_ref` (a stable-claim-manifest entry) and
  `claim_label`, the canonical lifecycle label that entry publishes.

## The claim ceiling — no per-surface widening

`claim_label` is a **hard ceiling**: a row may freeze the public claim at its label
or narrow below it, but its `frozen_label` may never be **wider** (stronger) than the
public claim's canonical label. This is what makes the freeze *ingest* the claim
manifest rather than restate it — the CI gate reads the stable claim manifest named by
`claim_manifest_ref` and fails when a row's `claim_label` is not the label the claim
manifest publishes for the entry named by `claim_ref`. The freeze reuses the stable
claim level vocabulary — `lts`, `stable`, `beta`, `preview`, `withdrawn` — rather than
minting per-surface labels.

## The launch cutline

The cutline fixes the boundary between a surface whose freeze backs a Stable (or LTS)
claim and one narrowed below it:

```
lts > stable   |   beta > preview > withdrawn   (below the cutline)
```

A surface freezes a label at or above the cutline only when its freeze packet is
within its freshness SLO, any waiver it relies on is unexpired, its deprecation packet
is complete with no overdue removal, its surface-level version evidence is complete,
an owner has signed off, and the public claim it backs is itself at or above the
cutline. A surface that loses any of those drops to a label below the cutline and
never freezes a label wider than the public claim's canonical label.

## Version windows

Each `version_window` pins three versions and a compatibility posture:

- `floor_version` — the oldest version still supported within the stable window.
- `current_version` — the current frozen version for the release line.
- `ceiling_version` — the newest version the window admits before a window-widening
  bump.
- `compatibility_posture` — one of `backward_compatible`, `additive_only`,
  `frozen_no_change`, `breaking_major_only`.

The gate and the typed model both require the window to be ordered
`floor_version <= current_version <= ceiling_version` under dotted-numeric comparison,
so a window cannot pin an inconsistent range.

## Deprecation packets

Each surface carries a `deprecation_packet` — a `packet_id` and a (possibly empty)
list of `deprecations`. A deprecation notice is **complete** only when it names the
deprecated version, what supersedes it, the announcement date, the removal target
version and date, and a migration ref. A surface with an incomplete notice carries the
`deprecation_packet_incomplete` reason and narrows.

The gate performs the **deprecation-removal-overdue automation** the typed model
cannot: against the freeze `as_of` date it reads each notice's `removal_target_date`
and `status`, and a notice whose removal date has passed without a `removed` status
makes the surface overdue — it must carry the `deprecation_removal_overdue` reason and
narrow, and a frozen surface may not carry an overdue removal.

## Freeze states

The `window_state` is the verdict for that surface:

- `frozen` — a captured, within-SLO freeze packet and a complete deprecation packet
  back the public claim at its full label, owner-signed.
- `frozen_on_waiver` — backs the claim's label only because an active, unexpired
  waiver covers a recorded freeze gap.
- `unfrozen_unbacked` — the surface evidence or deprecation packet is incomplete, or
  owner sign-off is absent; the surface is not frozen and the label must narrow.
- `unfrozen_claim_narrowed` — the public claim it backs is itself below the cutline,
  so the freeze inherits that ceiling and narrows.
- `unfrozen_stale` — the freeze packet breached its freshness SLO (or is missing); the
  surface is not frozen and the label must narrow.
- `unfrozen_waiver_expired` — the surface relied on a waiver that has expired; the
  label must narrow.
- `unfrozen_deprecation_overdue` — a deprecation passed its removal target without
  removal; the window cannot freeze until the removal lands.

A narrowing row MUST drop below the cutline and name at least one active gap reason. A
frozen row MUST back the public claim's canonical label cleanly — within-SLO captured
packet, complete deprecation packet, owner sign-off, **no** active gap reason.

## Packet-freshness SLO {#freeze-freshness-slo}

Each row's `freeze_packet` carries a `freshness_slo`:

- `target_max_age_days` — the SLO: the packet may be at most this many days old.
- `warn_within_days` — when the days remaining before the target drop to this or
  below, the packet is `due_for_refresh` (a warning, not yet a breach).
- `slo_register_ref` — the ref into this register that defines the target.

The freshness SLO register for this freeze pins one target per freeze packet:
`target_max_age_days` of 45 with a `warn_within_days` window of 10. A packet older than
the target is `breached`; a packet whose remaining headroom is within the warn window
is `due_for_refresh`; a packet with no capture is `missing`. The CI gate recomputes the
state from `captured_at` against the freeze `as_of` date and fails when a declared state
is fresher than the clock allows, or when a frozen row rides a packet whose recomputed
state is `breached` or `missing`. A Stable freeze cannot quietly outlive its packet.

## Gap reasons and the publication automation

The closed reason vocabulary (mirrored in the schema and the typed model) is:

- `claim_label_narrowed`
- `surface_capability_absent`
- `freeze_evidence_incomplete`
- `deprecation_packet_incomplete`
- `deprecation_removal_overdue`
- `freeze_packet_freshness_breached`
- `freeze_packet_missing`
- `waiver_expired`
- `owner_signoff_missing`

Each `freeze_rule` names one reason as its `trigger_reason`, the public-claim labels
it watches (`applies_to_labels`), a `default_action`, and whether it
`blocks_publication`. A rule **fires** when any watched row carries its trigger reason.
Every gap reason has a rule watching for it, so a gap reason can never fire without a
corresponding publication gate.

The `claim_label_narrowed` rule is intentionally **non-blocking**: a row narrowed only
because the public claim it backs is already below the cutline is expected inheritance,
not a freeze defect — the stable claim manifest already holds that claim upstream. The
remaining reasons describe a surface that *could* freeze a Stable window (its public
claim is canonically Stable) but is not frozen, so they block publication.

## CLI/schema/API/manifest coverage

`release_blocking_surface_refs` is the closed set of surface refs the freeze must
cover. The CI gate fails closed when a declared release-blocking surface has no
covering `release_blocking: true` row, when a release-blocking row's `surface_ref` is
not declared, when a surface ref repeats, or when any of the four surface kinds — CLI,
schema, API, or manifest — has no row at all. The release line cannot freeze some
surfaces and silently leave a whole interface kind unfrozen.

## Publication verdict

The `publication` block records the verdict for the version-window freeze. It is
`hold` when any blocking freeze rule fires and `proceed` otherwise. The
`blocking_rule_ids` and `blocking_window_ids` enumerate the firing rules and the rows
that triggered them (only rows whose public claim is at or above the cutline count).
The gate recomputes all three and the summary, and fails on any drift.

At this revision three release-line surfaces that back public claims still published
Stable are themselves unfrozen — the state-snapshot schema's freeze packet breached its
freshness SLO, the update-manifest format carries an overdue deprecation removal, and
the rollback-orchestration API lost its provisional waiver — so three blocking freeze
rules fire and stable version-window publication is held. That is the honest posture
for a pre-implementation repository: the aggregate public claims are optimistic, and
the freeze narrows the interface-level truth beneath them.

## CI gate

Run:

```sh
python3 ci/check_stable_version_windows.py --repo-root .
```

The gate fails when a closed vocabulary or the cutline drifts; when a version window is
disordered; when a row that is narrowed does not drop below the cutline or fails to
name a reason; when a frozen row carries an active gap reason, rides a stale or
uncaptured packet, carries an incomplete deprecation packet, or lacks owner sign-off;
when a row freezes a label wider than its public claim's ceiling; when a row's
`claim_label` disagrees with the stable claim manifest; when a release-blocking surface
is uncovered, a surface kind is absent, or a surface ref repeats; when a packet's
declared SLO state overstates its freshness against `as_of`; when a surface holds on an
expired waiver; when a deprecation removal is overdue but undeclared (or declared but
not actually overdue); when the publication verdict, blocking sets, or summary counts
drift; or when a referenced artifact does not exist. It also runs negative drills and
the checked-in fixture cases under
[`/fixtures/release/stable_version_windows/`](../../fixtures/release/stable_version_windows/),
and writes a validation capture to
[`/artifacts/release/captures/stable_version_windows_validation_capture.json`](../../artifacts/release/captures/stable_version_windows_validation_capture.json).

Shiproom and release tooling can fail publication directly from this artifact:

```sh
python3 ci/check_stable_version_windows.py --repo-root . --require-proceed
```

This exits non-zero (code 2) whenever the recomputed publication verdict is `hold`,
distinct from an invalid-artifact failure (code 1).

The typed Rust consumer
(`aureline_release::stable_version_windows::current_stable_version_windows`) reads the
same freeze and runs the same structural cross-check, and exposes a redaction-safe
`support_export_projection()` for Help/About and support surfaces, so
`cargo test -p aureline-release` enforces these invariants without a cargo build in CI.

## Update rules

1. Land the upstream stable claim manifest entry, the surface-level version inventory,
   refreshed freeze packets, completed deprecation packets, and waivers first; point
   each row's `claim_ref`, `claim_label`, `version_window`, `deprecation_packet`,
   `freeze_packet`, and `waiver` at the canonical records.
2. Set each row's `window_state`, `active_gap_reasons`, `slo_state`, and `frozen_label`
   to the honest posture. A surface whose packet breached its freshness SLO or is
   missing, whose deprecation packet is incomplete or carries an overdue removal, whose
   waiver expired, whose evidence is incomplete, or whose owner has not signed narrows
   below the cutline; a surface whose public claim is already below the cutline narrows
   by inheritance.
3. Recompute the `publication` block and `summary`, then run
   `python3 ci/check_stable_version_windows.py --repo-root .` and commit the
   regenerated capture in the same change set.
4. If delivery proves a narrower window than planned, narrow the frozen label and
   update the freeze — do not paper over the gap with prose.
